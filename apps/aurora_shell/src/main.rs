//! The AURORA browser shell (§4.3): owns the window, tabs, omnibox, chrome,
//! downloads, preferences UI, and `DevTools` UI. The shell reaches the engine
//! exclusively through `aurora::WebView` (§4.1) — never engine internals.
//!
//! Thread: the UI thread (§3.7). It never blocks on I/O and never runs layout.

/// Crate version from the workspace version; `v0.1.0` is tagged at M13 (§11.6).
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Short git commit stamped by the build script; "unknown" outside a git tree.
const BUILD_COMMIT: &str = match option_env!("AURORA_BUILD_COMMIT") {
    Some(commit) => commit,
    None => "unknown",
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|arg| arg == "--version") {
        println!("aurora-shell {VERSION} (commit {BUILD_COMMIT})");
        return;
    }
    eprintln!("aurora-shell {VERSION} — the AURORA browser");
    eprintln!(
        "The window UI lands with milestone M6 (§7.8); until then only --version is supported."
    );
    eprintln!("usage: aurora-shell [--version]");
    std::process::exit(2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_report_is_well_formed() {
        assert!(!super::VERSION.is_empty());
        assert!(!super::BUILD_COMMIT.is_empty());
        assert!(
            super::VERSION
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_digit())
        );
    }
}
