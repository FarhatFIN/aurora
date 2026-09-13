//! The TLS certificate-failure matrix (§5.3 definition of done): a local
//! TLS server with fixture certificates — good chain, expired cert,
//! wrong-hostname cert, untrusted root — each yielding its exact
//! `TlsFailure` variant. Verification failure is a hard failure; there is
//! no click-through (§5.3).

// §9.6: tests are exempt from the unwrap/expect restrictions.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use aurora_net::{CancelToken, Pool, fetch};
use aurora_url::Url;
use rustls_pemfile::{certs, private_key};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;

const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures/tls");

fn read_pems(
    name: &str,
) -> (
    Vec<rustls::pki_types::CertificateDer<'static>>,
    Arc<rustls::pki_types::PrivateKeyDer<'static>>,
) {
    let pem = std::fs::read(format!("{FIXTURES}/{name}.pem")).unwrap();
    let cert_list: Vec<_> = certs(&mut pem.as_slice())
        .map(|result| result.unwrap())
        .collect();
    let key_pem = std::fs::read(format!("{FIXTURES}/{name}.key")).unwrap();
    let key = Arc::new(private_key(&mut key_pem.as_slice()).unwrap().unwrap());
    (cert_list, key)
}

fn root_certs(name: &str) -> Vec<rustls::pki_types::CertificateDer<'static>> {
    certs(
        &mut std::fs::read(format!("{FIXTURES}/{name}.pem"))
            .unwrap()
            .as_slice(),
    )
    .map(|result| result.unwrap())
    .collect()
}

/// An in-process TLS server presenting `server_cert`; serves one request.
fn spawn_tls_server(server_cert: &str) -> u16 {
    let (cert_list, key) = read_pems(server_cert);
    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_list, (*key).clone_key())
        .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        let Ok(stream) = listener.incoming().next().unwrap() else {
            return;
        };
        let Ok(conn) = rustls::ServerConnection::new(Arc::new(config)) else {
            return;
        };
        let mut tls = rustls::StreamOwned::new(conn, stream);
        // Read the request head (drives the handshake), then respond.
        let mut buffer = Vec::new();
        let mut chunk = [0u8; 1024];
        loop {
            let read = tls.read(&mut chunk).unwrap_or(0);
            if read == 0 {
                return;
            }
            buffer.extend_from_slice(&chunk[..read]);
            if buffer.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
        }
        let _ = tls.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok");
        let _ = tls.flush();
    });
    port
}

fn fetch_https(cert: &str, trust: &Pool) -> Result<aurora_net::Response, aurora_net::NetError> {
    let port = spawn_tls_server(cert);
    let url = Url::parse(&format!("https://127.0.0.1:{port}/")).unwrap();
    fetch(trust, &aurora_net::Request::get(url), &CancelToken::new())
}

fn trusted_pool() -> Pool {
    Pool::with_root_certs(root_certs("ca")).unwrap()
}

#[test]
fn valid_chain_and_hostname_verifies_and_serves() {
    let response = fetch_https("good", &trusted_pool()).unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"ok".to_vec());
}

#[test]
fn expired_certificate_is_a_hard_failure() {
    let error = fetch_https("expired", &trusted_pool()).unwrap_err();
    assert_eq!(error, aurora_net::NetError::Tls("certificate expired"));
}

#[test]
fn wrong_hostname_certificate_is_a_hard_failure() {
    // The cert is trusted (signed by our CA) but names other.example.
    let error = fetch_https("wronghost", &trusted_pool()).unwrap_err();
    assert_eq!(error, aurora_net::NetError::Tls("hostname mismatch"));
}

#[test]
fn untrusted_root_is_a_hard_failure() {
    // The self-signed fixture is not in the pool's trust store.
    let error = fetch_https("untrusted", &trusted_pool()).unwrap_err();
    assert_eq!(
        error,
        aurora_net::NetError::Tls("untrusted certificate authority")
    );
}

#[test]
fn default_webpki_pool_still_reaches_the_real_world() {
    // Sanity: the default pool (webpki-roots) is unaffected by the fixture
    // work; a certificate it cannot verify fails cleanly.
    let port = spawn_tls_server("untrusted");
    let url = Url::parse(&format!("https://127.0.0.1:{port}/")).unwrap();
    let error = fetch(
        &Pool::new(),
        &aurora_net::Request::get(url),
        &CancelToken::new(),
    )
    .unwrap_err();
    assert_eq!(
        error,
        aurora_net::NetError::Tls("untrusted certificate authority")
    );
}
