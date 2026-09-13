//! M0 smoke test (§7.2 exit criterion): the crate builds, links, and is
//! reachable through its public surface. Replaced by real integration tests
//! as each subsystem lands (§8).

#[test]
fn crate_links_and_reports_identity() {
    use aurora_platform as _;
    assert_eq!(env!("CARGO_PKG_NAME"), "aurora_platform");
    assert!(!env!("CARGO_PKG_VERSION").is_empty());
}
