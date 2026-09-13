//! Integration tests against a local in-process mock server (§5.2
//! invariant: no external network in tests). Each test scripts raw
//! response bytes and asserts on both the request bytes the stack sent
//! and the decoded responses it produced.

// §9.6: tests are exempt from the unwrap/expect restrictions.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use aurora_net::{CancelToken, Method, Pool, RedirectPolicy, Request, fetch};
use aurora_url::Url;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A scripted server: serves each accepted connection by running the next
/// closure, which receives the stream and may exchange multiple messages
/// (keep-alive). Returns the port and the connection counter.
struct MockServer {
    port: u16,
    connections: Arc<AtomicUsize>,
}

impl MockServer {
    fn spawn(script: Arc<dyn Fn(&mut TcpStream) + Send + Sync + 'static>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let connections = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&connections);
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else { return };
                counter.fetch_add(1, Ordering::SeqCst);
                let script = Arc::clone(&script);
                std::thread::spawn(move || script(&mut stream));
            }
        });
        Self { port, connections }
    }

    fn url(&self, path: &str) -> String {
        format!("http://127.0.0.1:{port}{path}", port = self.port)
    }
}

fn read_request(stream: &mut TcpStream) -> String {
    let mut buffer = Vec::new();
    let mut byte = [0u8; 1];
    // Read until the blank line terminates the head (tests are small).
    while !buffer.ends_with(b"\r\n\r\n") {
        if stream.read(&mut byte).unwrap_or(0) == 0 {
            break;
        }
        buffer.push(byte[0]);
    }
    String::from_utf8_lossy(&buffer).into_owned()
}

fn respond(stream: &mut TcpStream, head: &str, body: &[u8]) {
    stream.write_all(head.as_bytes()).unwrap();
    stream.write_all(body).unwrap();
    stream.flush().unwrap();
}

fn get(url: &str) -> Request {
    Request::get(Url::parse(url).unwrap())
}

#[test]
fn get_with_content_length_round_trips() {
    let server = MockServer::spawn(Arc::new(|stream| {
        let request = read_request(stream);
        assert!(request.starts_with("GET /a?x=1 HTTP/1.1\r\n"));
        assert!(request.contains("Host: 127.0.0.1:"));
        assert!(request.contains("User-Agent: Aurora/0.1 (+engine project)"));
        assert!(request.contains("Accept-Encoding: gzip, deflate"));
        assert!(request.contains("Connection: keep-alive"));
        respond(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: 5\r\n\r\n",
            b"hello",
        );
    }));
    let response = fetch(
        &Pool::new(),
        &get(&server.url("/a?x=1")),
        &CancelToken::new(),
    )
    .unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"hello".to_vec());
    assert_eq!(
        response.headers.get("content-type"),
        Some("text/html; charset=utf-8")
    );
}

#[test]
fn chunked_body_decodes_and_keeps_connection_framed() {
    let server = MockServer::spawn(Arc::new(|stream| {
        read_request(stream);
        respond(
            stream,
            "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n",
            b"5\r\nhello\r\n3\r\n, w\r\n0\r\n\r\n",
        );
    }));
    let response = fetch(&Pool::new(), &get(&server.url("/")), &CancelToken::new()).unwrap();
    assert_eq!(response.body, b"hello, w".to_vec());
}

#[test]
fn gzip_content_encoding_decodes() {
    // "hello" gzipped with the reference compressor baked in as golden bytes.
    let gz: Vec<u8> = vec![
        0x1F, 0x8B, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xCB, 0x48, 0xCD, 0xC9, 0xC9,
        0x07, 0x00, 0x86, 0xA6, 0x10, 0x36, 0x05, 0x00, 0x00, 0x00,
    ];
    let server = MockServer::spawn(Arc::new(move |stream| {
        read_request(stream);
        respond(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nContent-Length: 25\r\n\r\n",
            &gz,
        );
    }));
    let response = fetch(&Pool::new(), &get(&server.url("/")), &CancelToken::new()).unwrap();
    assert_eq!(response.body, b"hello".to_vec());
}

#[test]
fn until_close_fallback_reads_to_eof() {
    let server = MockServer::spawn(Arc::new(|stream| {
        read_request(stream);
        respond(stream, "HTTP/1.1 200 OK\r\n\r\n", b"no framing at all");
        stream.shutdown(std::net::Shutdown::Write).unwrap();
    }));
    let response = fetch(&Pool::new(), &get(&server.url("/")), &CancelToken::new()).unwrap();
    assert_eq!(response.body, b"no framing at all".to_vec());
}

#[test]
fn redirect_chain_gets_followed_and_reported() {
    let server = MockServer::spawn(Arc::new(|stream| {
        // First connection: two hops.
        let first = read_request(stream);
        assert!(first.starts_with("GET /one HTTP/1.1\r\n"));
        respond(
            stream,
            "HTTP/1.1 302 Found\r\nLocation: /two\r\nContent-Length: 0\r\n\r\n",
            b"",
        );
        let second = read_request(stream);
        assert!(
            second.starts_with("GET /two HTTP/1.1\r\n"),
            "second request on same connection"
        );
        respond(
            stream,
            "HTTP/1.1 301 Moved Permanently\r\nLocation: /three\r\nContent-Length: 0\r\n\r\n",
            b"",
        );
        let third = read_request(stream);
        assert!(third.starts_with("GET /three HTTP/1.1\r\n"));
        respond(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\n",
            b"ok",
        );
    }));
    let response = fetch(&Pool::new(), &get(&server.url("/one")), &CancelToken::new()).unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"ok".to_vec());
    assert_eq!(response.url.serialize(false), server.url("/three"));
    assert_eq!(response.redirected_from.len(), 2);
    assert_eq!(
        response.redirected_from[0],
        Url::parse(&server.url("/one")).unwrap()
    );
    assert_eq!(
        response.redirected_from[1],
        Url::parse(&server.url("/two")).unwrap()
    );
}

#[test]
fn redirect_policy_error_stops_at_the_redirect() {
    let server = MockServer::spawn(Arc::new(|stream| {
        read_request(stream);
        respond(
            stream,
            "HTTP/1.1 302 Found\r\nLocation: /two\r\nContent-Length: 0\r\n\r\n",
            b"",
        );
    }));
    let mut request = get(&server.url("/one"));
    request.redirect = RedirectPolicy::Error;
    let response = fetch(&Pool::new(), &request, &CancelToken::new()).unwrap();
    assert_eq!(response.status, 302);
}

#[test]
fn cross_origin_redirect_strips_authorization() {
    // Same mock host, different port = different origin for the test.
    let origin_a = MockServer::spawn(Arc::new(|stream| {
        let request = read_request(stream);
        assert!(
            !request.contains("authorization:"),
            "credentials must be stripped"
        );
        assert!(!request.to_ascii_lowercase().contains("authorization:"));
        respond(stream, "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n", b"");
    }));
    let target_port = origin_a.port;
    let origin_b = MockServer::spawn(Arc::new(move |stream| {
        let request = read_request(stream);
        assert!(
            request
                .to_ascii_lowercase()
                .contains("authorization: secret")
        );
        respond(
            stream,
            &format!(
                "HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:{target_port}/landing\r\nContent-Length: 0\r\n\r\n"
            ),
            b"",
        );
    }));
    let mut request = get(&origin_b.url("/start"));
    request.headers.append("Authorization", "secret").unwrap();
    let response = fetch(&Pool::new(), &request, &CancelToken::new()).unwrap();
    assert_eq!(response.status, 200);
}

#[test]
fn keep_alive_reuses_the_connection_across_fetches() {
    let server = MockServer::spawn(Arc::new(|stream| {
        for path in ["/first", "/second"] {
            let request = read_request(stream);
            assert!(request.starts_with(&format!("GET {path} HTTP/1.1\r\n")));
            respond(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Length: 1\r\n\r\n",
                if path == "/first" { b"1" } else { b"2" },
            );
        }
    }));
    let pool = Pool::new();
    let cancel = CancelToken::new();
    let first = fetch(&pool, &get(&server.url("/first")), &cancel).unwrap();
    let second = fetch(&pool, &get(&server.url("/second")), &cancel).unwrap();
    assert_eq!(first.body, b"1".to_vec());
    assert_eq!(second.body, b"2".to_vec());
    assert_eq!(
        server.connections.load(Ordering::SeqCst),
        1,
        "one connection reused"
    );
}

#[test]
fn pooled_connection_failure_is_retried_once_on_a_fresh_connection() {
    static FIRST: AtomicUsize = AtomicUsize::new(0);
    FIRST.store(0, Ordering::SeqCst);
    let server = MockServer::spawn(Arc::new(|stream| {
        let request = read_request(stream);
        assert!(request.starts_with("GET / HTTP/1.1\r\n"));
        // First contact: respond then slam the connection shut so the next
        // pooled use fails; second contact: clean response.
        if FIRST.fetch_add(1, Ordering::SeqCst) == 0 {
            respond(stream, "HTTP/1.1 200 OK\r\nContent-Length: 1\r\n\r\n", b"a");
            stream.shutdown(std::net::Shutdown::Both).unwrap();
        } else {
            respond(stream, "HTTP/1.1 200 OK\r\nContent-Length: 1\r\n\r\n", b"b");
        }
    }));
    let pool = Pool::new();
    let cancel = CancelToken::new();
    let first = fetch(&pool, &get(&server.url("/")), &cancel).unwrap();
    assert_eq!(first.body, b"a".to_vec());
    let second = fetch(&pool, &get(&server.url("/")), &cancel).unwrap();
    assert_eq!(second.body, b"b".to_vec());
}

#[test]
fn head_requests_do_not_read_a_body() {
    let server = MockServer::spawn(Arc::new(|stream| {
        let request = read_request(stream);
        assert!(request.starts_with("HEAD / HTTP/1.1\r\n"));
        respond(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Length: 100\r\n\r\n",
            b"", // no body for HEAD
        );
    }));
    let mut request = get(&server.url("/"));
    request.method = Method::Head;
    let response = fetch(&Pool::new(), &request, &CancelToken::new()).unwrap();
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
}

#[test]
fn bad_protocol_is_a_value_not_a_panic() {
    let server = MockServer::spawn(Arc::new(|stream| {
        read_request(stream);
        respond(stream, "NOT-HTTP garbage\r\n\r\n", b"");
    }));
    let response = fetch(&Pool::new(), &get(&server.url("/")), &CancelToken::new());
    assert!(matches!(response, Err(aurora_net::NetError::Protocol(_))));
}

#[test]
fn ob_fold_is_rejected() {
    let server = MockServer::spawn(Arc::new(|stream| {
        read_request(stream);
        respond(
            stream,
            "HTTP/1.1 200 OK\r\nX-A: v\r\n  continued\r\n\r\n",
            b"",
        );
    }));
    let response = fetch(&Pool::new(), &get(&server.url("/")), &CancelToken::new());
    assert!(matches!(response, Err(aurora_net::NetError::Protocol(_))));
}
