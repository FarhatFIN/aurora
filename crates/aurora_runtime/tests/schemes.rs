//! Local-scheme fetch tests (§6.9 item 1): `data`, `file`, `about:blank`,
//! and the unsupported-scheme error path.

// §9.6: tests are exempt from the unwrap/expect restrictions.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use aurora_runtime::{CancelToken, fetch};

#[test]
fn data_url_plain_text_decodes() {
    let cancel = CancelToken::new();
    let outcome = fetch("data:text/html,<h1>hi</h1>", &cancel).unwrap();
    assert_eq!(outcome.status, 200);
    assert_eq!(outcome.body, b"<h1>hi</h1>".to_vec());
}

#[test]
fn data_url_base64_decodes() {
    let cancel = CancelToken::new();
    // base64("hello, base64") = "aGVsbG8sIGJhc2U2NA==".
    let outcome = fetch(
        "data:application/octet-stream;base64,aGVsbG8sIGJhc2U2NA==",
        &cancel,
    )
    .unwrap();
    assert_eq!(outcome.body, b"hello, base64".to_vec());
    assert!(outcome.charset.is_none());
}

#[test]
fn data_url_reports_charset() {
    let cancel = CancelToken::new();
    let outcome = fetch("data:text/plain;charset=windows-1252,%A0", &cancel).unwrap();
    assert_eq!(outcome.charset.as_deref(), Some("windows-1252"));
}

#[test]
fn file_url_reads_local_files() {
    let path = std::env::temp_dir().join(format!("aurora-m1-{}.html", std::process::id()));
    std::fs::write(&path, b"<p>file-ok</p>").unwrap();
    let cancel = CancelToken::new();
    let outcome = fetch(&format!("file://{}", path.display()), &cancel).unwrap();
    assert_eq!(outcome.body, b"<p>file-ok</p>".to_vec());
    std::fs::remove_file(&path).ok();
}

#[test]
fn file_url_rejects_remote_hosts_and_missing_files() {
    let cancel = CancelToken::new();
    assert!(matches!(
        fetch("file://remote.example/x", &cancel),
        Err(aurora_runtime::NetError::UnsupportedScheme)
    ));
    assert!(fetch("file:///nonexistent-aurora-test-file", &cancel).is_err());
}

#[test]
fn about_blank_is_an_empty_document() {
    let cancel = CancelToken::new();
    let outcome = fetch("about:blank", &cancel).unwrap();
    assert_eq!(outcome.status, 200);
    assert!(outcome.body.is_empty());
}

#[test]
fn unknown_schemes_fail_cleanly() {
    let cancel = CancelToken::new();
    for url in [
        "gopher://x/",
        "ftp://example.com/file",
        "javascript:void(0)",
    ] {
        assert!(matches!(
            fetch(url, &cancel),
            Err(aurora_runtime::NetError::UnsupportedScheme)
        ));
    }
}
