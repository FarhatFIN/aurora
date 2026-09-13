//! The loader: scheme dispatch, request exchange, redirects (§5.2.4–§5.2.5,
//! §6.9 items 1–7).

use crate::body::{self, Framing};
use crate::conn::{self, Connection, Pool};
use crate::error::{CancelToken, NetError};
use crate::message::{self, Method, Request, ResponseHead};
use aurora_url::Url;
use std::time::Instant;

const MAX_REDIRECTS: usize = 20;

/// A fetched response: head plus a fully buffered (content-decoded) body.
/// Buffering keeps the M1 tool API simple; the framing reader is the seam
/// the document pipeline will stream through.
#[derive(Clone, Debug)]
pub struct Response {
    pub status: u16,
    pub reason: String,
    pub headers: crate::headers::HeaderMap,
    /// The final URL after redirects.
    pub url: Url,
    /// The redirect chain that led to `url`, oldest first (§5.2.4).
    pub redirected_from: Vec<Url>,
    pub body: Vec<u8>,
}

/// Fetches `request` following redirects through `pool`.
///
/// # Errors
/// [`NetError`] for every failure class of the stack.
pub fn fetch(pool: &Pool, request: &Request, cancel: &CancelToken) -> Result<Response, NetError> {
    let mut current = request.clone();
    let mut chain: Vec<Url> = Vec::new();
    loop {
        let response = fetch_single(pool, &current, cancel)?;
        let is_redirect = matches!(response.status, 301 | 302 | 303 | 307 | 308);
        if !is_redirect || request.redirect == crate::message::RedirectPolicy::Error {
            let mut response = response;
            response.redirected_from = chain;
            return Ok(response);
        }
        if chain.len() >= MAX_REDIRECTS {
            return Err(NetError::TooManyRedirects);
        }
        let Some(location) = response.headers.get("location") else {
            return Ok(response);
        };
        let next_url = current
            .url
            .join(location)
            .map_err(|_| NetError::Protocol("bad redirect location"))?;
        chain.push(current.url.clone());
        // 301/302/303 → GET with the body dropped; 307/308 preserve both.
        let (method, headers) = if matches!(response.status, 301..=303) {
            (Method::Get, std::mem::take(&mut current.headers))
        } else {
            (current.method, std::mem::take(&mut current.headers))
        };
        let mut headers = headers;
        // Cross-origin redirect strips credentials (§5.2.4; cookies live in
        // §5.17 — Authorization is the M1-relevant bearer).
        if origin_key(&next_url) != origin_key(&current.url) {
            headers.remove_all("authorization");
            headers.remove_all("cookie");
        }
        current = Request {
            method,
            url: next_url,
            headers,
            redirect: current.redirect,
        };
    }
}

/// The connection-pool origin identity: scheme, host, port.
fn origin_key(url: &Url) -> String {
    format!(
        "{}|{}|{}",
        url.scheme(),
        url.host()
            .map(aurora_url::Host::serialize)
            .unwrap_or_default(),
        url.port().or_else(|| url.default_port()).unwrap_or(0)
    )
}

/// One request/response exchange (no redirect handling). A pooled
/// connection that fails once is discarded and the exchange retried on a
/// fresh connection — one clean retry, not more (§5.2.1).
fn fetch_single(
    pool: &Pool,
    request: &Request,
    cancel: &CancelToken,
) -> Result<Response, NetError> {
    let mut first = Exchange {
        reused: false,
        leftover: Vec::new(),
        pool,
        request,
        cancel,
    };
    match first.run() {
        Ok(response) => Ok(response),
        Err(error) if first.reused && is_reusable_failure(&error) => {
            let mut fresh = Exchange {
                reused: false,
                leftover: Vec::new(),
                pool,
                request,
                cancel,
            };
            fresh.run()
        }
        Err(error) => Err(error),
    }
}

fn is_reusable_failure(error: &NetError) -> bool {
    // A reused connection dying before the head completes is the classic
    // stale-keep-alive case — worth the one clean retry (§5.2.1).
    matches!(
        error,
        NetError::Connect
            | NetError::Io
            | NetError::Timeout
            | NetError::Protocol("connection closed before head completed")
    )
}

/// One request/HEAD-read/body-read pass over one connection.
struct Exchange<'a> {
    reused: bool,
    leftover: Vec<u8>,
    pool: &'a Pool,
    request: &'a Request,
    cancel: &'a CancelToken,
}

struct ExchangeOutcome {
    keep_alive: bool,
    response: Response,
}

impl Exchange<'_> {
    fn run(&mut self) -> Result<Response, NetError> {
        match self.request.url.scheme() {
            "http" | "https" => self.exchange_http(),
            _ => Err(NetError::UnsupportedScheme),
        }
    }

    fn exchange_http(&mut self) -> Result<Response, NetError> {
        let url = self.request.url.clone();
        let mut connection = match self.pool.checkout(&url) {
            Some(connection) => {
                self.reused = true;
                connection
            }
            None => conn::connect(&url, self.cancel)?,
        };
        let outcome = self.exchange_over(&mut connection)?;
        if outcome.keep_alive {
            self.pool.checkin(&url, connection);
        }
        Ok(outcome.response)
    }

    fn exchange_over(&mut self, connection: &mut Connection) -> Result<ExchangeOutcome, NetError> {
        connection.set_timeouts(conn::PHASE_HEADER_TIMEOUT)?;
        let url = self.request.url.clone();
        let encoded_head = self.request.encode_head(&wire_path(&url));
        connection
            .as_write()
            .write_all(encoded_head.as_bytes())
            .map_err(|error| body::io_error(&error))?;
        connection
            .as_write()
            .flush()
            .map_err(|error| body::io_error(&error))?;
        let head = self.read_head(connection)?;
        connection.set_timeouts(conn::PHASE_BODY_IDLE_TIMEOUT)?;
        let framing = body::framing_for(self.request.method, head.status, &head.headers);
        let mut leftover = std::mem::take(&mut self.leftover);
        let raw_body = body::read_body(connection.as_read(), framing, &mut leftover, self.cancel)?;
        self.leftover = leftover;
        let keep_alive = framing != Framing::UntilClose
            && head
                .headers
                .get("connection")
                .is_none_or(|value| !value.eq_ignore_ascii_case("close"));
        let decoded = body::decode_content_coding(head.headers.get("content-encoding"), raw_body)?;
        Ok(ExchangeOutcome {
            keep_alive,
            response: Response {
                status: head.status,
                reason: head.reason,
                headers: head.headers,
                url,
                redirected_from: Vec::new(),
                body: decoded,
            },
        })
    }

    fn read_head(&mut self, connection: &mut Connection) -> Result<ResponseHead, NetError> {
        let deadline = Instant::now() + conn::PHASE_HEADER_TIMEOUT;
        let mut buffer = std::mem::take(&mut self.leftover);
        loop {
            self.cancel.check()?;
            if let Ok((head, used)) = message::parse_head(&buffer) {
                buffer.drain(..used);
                self.leftover = buffer;
                return Ok(head);
            }
            if Instant::now() > deadline {
                return Err(NetError::Timeout);
            }
            let mut chunk = [0u8; 4 * 1024];
            let read = connection.as_read().fill(&mut chunk)?;
            if read == 0 {
                return Err(NetError::Protocol(
                    "connection closed before head completed",
                ));
            }
            buffer.extend_from_slice(&chunk[..read]);
            if buffer.len() > 256 * 1024 {
                return Err(NetError::Protocol("head exceeds 256 KiB"));
            }
        }
    }
}

/// The request target: path + query, fragment stripped (§6.9 item 2);
/// empty path becomes "/" for special schemes.
#[must_use]
pub fn wire_path(url: &Url) -> String {
    let mut target = url.path();
    if target.is_empty() {
        target.push('/');
    }
    if let Some(query) = url.query() {
        target.push('?');
        target.push_str(query);
    }
    target
}

/// A convenience for tools and tests: one-shot fetch with a fresh pool.
///
/// # Errors
/// See [`fetch`].
pub fn fetch_once(request: &Request, cancel: &CancelToken) -> Result<Response, NetError> {
    fetch(&Pool::new(), request, cancel)
}
