//! `aurora_url` behavior tests (§5.1 definition of done): parse/serialize
//! round trips, relative resolution, hosts, encoding, origins. Cases are
//! hand-written from the WHATWG URL Standard; expected outputs follow the
//! spec's algorithms, read in `docs/spec-notes/url-basic-parser.md`.

// §9.6: tests are exempt from the unwrap/expect restrictions.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use aurora_url::{Host, Origin, ParseError, Url};

fn parse(input: &str) -> String {
    Url::parse(input).unwrap().serialize(false)
}

#[test]
fn absolute_urls_parse_and_normalize() {
    assert_eq!(parse("https://example.com/"), "https://example.com/");
    assert_eq!(parse("HTTP://EXAMPLE.COM"), "http://example.com/");
    assert_eq!(parse("https://example.com:443/x"), "https://example.com/x");
    assert_eq!(
        parse("http://example.com:8080/"),
        "http://example.com:8080/"
    );
    // Trailing empty segment survives; lone dot collapses.
    assert_eq!(parse("http://example.com/a/"), "http://example.com/a/");
    assert_eq!(parse("http://example.com/a/."), "http://example.com/a/");
    assert_eq!(
        parse("http://example.com/a/b/c/./../../g"),
        "http://example.com/a/g"
    );
}

#[test]
fn backslashes_are_path_separators_for_special_schemes() {
    assert_eq!(parse(r"http://example.com\a\b"), "http://example.com/a/b");
    assert_eq!(parse(r"https:\\example.com\p"), "https://example.com/p");
    // Non-special: backslash is an ordinary (non-separator) path character.
    let url = Url::parse("foo://host/a\\b").unwrap();
    assert_eq!(url.path(), "/a\\b");
}

#[test]
fn whitespace_and_tabs_are_cleaned() {
    assert_eq!(parse("  https://example.com/  "), "https://example.com/");
    assert_eq!(parse("ht\ttp\ns://example.com/"), "https://example.com/");
}

#[test]
fn userinfo_parses_and_serializes() {
    assert_eq!(
        parse("http://user:pass@example.com/"),
        "http://user:pass@example.com/"
    );
    assert_eq!(
        parse("http://user@example.com/"),
        "http://user@example.com/"
    );
    // An empty userinfo is dropped on serialization.
    assert_eq!(parse("http://@example.com/"), "http://example.com/");
    assert_eq!(
        parse("http://us:er@example.com/")
            .split('@')
            .next()
            .unwrap_or(""),
        "http://us:er"
    );
}

#[test]
fn ports_validate() {
    assert_eq!(
        Url::parse("http://example.com:99999/"),
        Err(ParseError::InvalidPort)
    );
    assert_eq!(
        Url::parse("http://example.com:abc/"),
        Err(ParseError::InvalidPort)
    );
    // Port 0 is legal (§5.1 pitfall list).
    assert_eq!(parse("http://example.com:0/"), "http://example.com:0/");
    // Empty port is dropped.
    assert_eq!(parse("http://example.com:/x"), "http://example.com/x");
}

#[test]
fn ipv4_hosts_normalize() {
    assert_eq!(parse("http://127.0.0.1/"), "http://127.0.0.1/");
    assert_eq!(parse("http://0x7f.1/"), "http://127.0.0.1/");
    assert_eq!(parse("http://0x7f000001/"), "http://127.0.0.1/");
    assert_eq!(parse("http://0177.0.0.1/"), "http://127.0.0.1/");
    assert_eq!(parse("http://1.2.3/"), "http://1.2.0.3/");
    assert_eq!(
        Url::parse("http://256.1.1.1/"),
        Err(ParseError::InvalidDomainChar)
    );
    assert_eq!(
        Url::parse("http://1.1.1.1.1/"),
        Err(ParseError::InvalidDomainChar)
    );
}

#[test]
fn ipv6_hosts_parse_and_serialize() {
    assert_eq!(parse("http://[::1]/"), "http://[::1]/");
    assert_eq!(parse("http://[0:0::1]/"), "http://[::1]/");
    assert_eq!(parse("http://[2001:DB8::1]/"), "http://[2001:db8::1]/");
    assert_eq!(
        parse("http://[::ffff:127.0.0.1]/"),
        "http://[::ffff:7f00:1]/"
    );
    assert_eq!(
        Url::parse("http://[::1/x"),
        Err(ParseError::InvalidIpv6Unclosed)
    );
    assert_eq!(
        Url::parse("http://[1::2::3]/"),
        Err(ParseError::InvalidIpv6)
    );
}

#[test]
fn empty_host_fails_for_special_schemes_only() {
    assert_eq!(Url::parse("http://"), Err(ParseError::EmptyHost));
    assert_eq!(Url::parse("http://?q"), Err(ParseError::EmptyHost));
    // Extra slashes before the authority are consumed (validation errors
    // only): http:///x normalizes to http://x/.
    assert_eq!(parse("http:///x"), "http://x/");
    // Non-special schemes allow an empty (opaque) host (§5.1 pitfall list).
    assert_eq!(parse("foo:///x"), "foo:///x");
    assert_eq!(parse("foo://"), "foo://");
}

#[test]
fn opaque_paths_are_not_normalized() {
    let url = Url::parse("data:text/html,<h1>hi</h1>").unwrap();
    assert!(url.cannot_be_a_base());
    assert_eq!(url.path(), "text/html,<h1>hi</h1>");
    assert_eq!(url.serialize(false), "data:text/html,<h1>hi</h1>");
    assert_eq!(parse("mailto:me@example.com"), "mailto:me@example.com");
    // Fragment split off an opaque path.
    let url = Url::parse("data:abc#frag").unwrap();
    assert_eq!(url.path(), "abc");
    assert_eq!(url.fragment(), Some("frag"));
}

#[test]
fn file_urls_normalize() {
    assert_eq!(parse("file:///etc/passwd"), "file:///etc/passwd");
    assert_eq!(parse("file:/etc/passwd"), "file:///etc/passwd");
    assert_eq!(parse("file://localhost/etc/passwd"), "file:///etc/passwd");
    assert_eq!(parse("file://host/etc/passwd"), "file://host/etc/passwd");
    assert_eq!(parse(r"file://C:\x\y"), "file:///C:/x/y");
}

#[test]
fn relative_resolution_against_http_base() {
    let base = Url::parse("http://user@example.com:8080/a/b?q#f").unwrap();
    let resolve = |reference: &str| base.join(reference).unwrap().serialize(false);
    assert_eq!(resolve(""), "http://user@example.com:8080/a/b?q");
    assert_eq!(resolve("#x"), "http://user@example.com:8080/a/b?q#x");
    assert_eq!(resolve("?z"), "http://user@example.com:8080/a/b?z");
    assert_eq!(resolve("c"), "http://user@example.com:8080/a/c");
    assert_eq!(resolve("c/d"), "http://user@example.com:8080/a/c/d");
    assert_eq!(resolve("/c"), "http://user@example.com:8080/c");
    assert_eq!(resolve("../c"), "http://user@example.com:8080/c");
    assert_eq!(resolve("../../c"), "http://user@example.com:8080/c");
    assert_eq!(resolve("//other.com/p"), "http://other.com/p");
    assert_eq!(resolve("https://other.com/p"), "https://other.com/p");
    // Same-scheme relative: "http:x" resolves against the base path, with
    // the base's credentials/host/port carried over (relative state).
    assert_eq!(resolve("http:c"), "http://user@example.com:8080/a/c");
    // Credentials of the reference replace the base's.
    assert_eq!(resolve("//u@other.com/"), "http://u@other.com/");
}

#[test]
fn file_base_resolution() {
    let base = Url::parse("file:///dir/sub/x.html").unwrap();
    assert_eq!(
        base.join("y.html").unwrap().serialize(false),
        "file:///dir/sub/y.html"
    );
    assert_eq!(base.join("/top").unwrap().serialize(false), "file:///top");
}

#[test]
fn parse_serialize_round_trip_is_idempotent() {
    // §5.1 invariant: parse(serialize(u)) == u for valid u.
    let inputs = [
        "https://example.com/a/b?q=1&r=2#frag",
        "http://user:pass@example.com:8080/a%20b/?x",
        "https://[2001:db8::1]:8443/x",
        "ftp://ftp.example.com/pub/file.txt",
        "file:///C:/Users/test/doc.html",
        "data:text/plain,hello%20world",
        "mailto:someone@example.com?subject=hi",
        "ws://example.com/socket",
        "foo://opaque-host.example/path\\weird",
        "http://example.com/a/b/../c/./d%2Fe",
    ];
    for input in inputs {
        let once = Url::parse(input).unwrap();
        let serialized = once.serialize(false);
        let twice = Url::parse(&serialized).unwrap();
        assert_eq!(once, twice, "round trip of {input}");
        assert_eq!(
            serialized,
            twice.serialize(false),
            "re-serialization of {input}"
        );
    }
}

#[test]
fn query_and_fragment_encode_sets() {
    // Space encodes in query and fragment; ' only in special queries.
    assert_eq!(parse("http://a/?q= x"), "http://a/?q=%20x");
    assert_eq!(parse("http://a/?q=i'ts"), "http://a/?q=i%27ts");
    assert_eq!(parse("foo://a/?q=i'ts"), "foo://a/?q=i'ts");
    assert_eq!(parse("http://a/#f g"), "http://a/#f%20g");
}

#[test]
fn origins_per_the_standard() {
    let https = Url::parse("https://example.com/").unwrap();
    assert!(matches!(aurora_url::origin(&https), Origin::Tuple { .. }));
    assert_eq!(
        aurora_url::origin(&https).serialize(),
        "https://example.com"
    );

    let ip = Url::parse("http://127.0.0.1:8080/").unwrap();
    assert_eq!(aurora_url::origin(&ip).serialize(), "http://127.0.0.1:8080");

    // file: and non-special schemes get opaque ("null") origins.
    for input in ["file:///x", "mailto:a@b", "foo://h/"] {
        let url = Url::parse(input).unwrap();
        assert!(matches!(aurora_url::origin(&url), Origin::Opaque(_)));
        assert_eq!(aurora_url::origin(&url).serialize(), "null");
    }
}

#[test]
fn parsing_is_total_on_garbage() {
    // §4.6 class 3: no panic on external input, only typed failure.
    for input in [
        "",
        ":",
        "http://",
        "1://x",
        "://x",
        "http://exa mple.com/",
        "%",
        "http://exa%mple.com/",
        "http://[",
        "http://]x[",
        "\u{0}",
        "a b:c:d",
        "http://example.com:-1/",
        "x:",
    ] {
        let _ = Url::parse(input); // must not panic; result either way is fine
    }
    // A few of these have specific expected failures:
    assert_eq!(Url::parse(""), Err(ParseError::RelativeWithoutBase));
    assert_eq!(Url::parse("a b:c:d"), Err(ParseError::RelativeWithoutBase));
    assert_eq!(
        Url::parse("http://exa mple.com/"),
        Err(ParseError::InvalidDomainChar)
    );
    // Non-ASCII domain: IDNA placeholder, typed error (PROGRESS.md).
    assert_eq!(Url::parse("http://münchen.de/"), Err(ParseError::IdnaError));
}

#[test]
fn accessors_report_components() {
    let url = Url::parse("https://user:pw@example.com:9988/a/b?the-query#the-frag").unwrap();
    assert_eq!(url.scheme(), "https");
    assert_eq!(url.host(), Some(&Host::Domain("example.com".into())));
    assert_eq!(url.port(), Some(9988));
    assert_eq!(url.username(), "user");
    assert_eq!(url.password(), "pw");
    assert_eq!(url.path(), "/a/b");
    assert_eq!(url.query(), Some("the-query"));
    assert_eq!(url.fragment(), Some("the-frag"));
    assert!(url.is_special());
    assert!(!url.cannot_be_a_base());
    let opaque = Url::parse("mailto:x@y").unwrap();
    assert!(opaque.cannot_be_a_base());
    assert!(!opaque.is_special());
}

#[test]
fn serialize_excludes_fragment_for_the_wire() {
    // §6.9 item 2: fragment stripped on the wire.
    let url = Url::parse("https://example.com/a#secret").unwrap();
    assert_eq!(url.serialize(true), "https://example.com/a");
    assert_eq!(url.serialize(false), "https://example.com/a#secret");
}
