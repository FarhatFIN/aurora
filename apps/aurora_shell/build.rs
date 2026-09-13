//! Build script: stamps the current git commit into the binary so
//! `aurora_shell --version` can report it (§7.2 demo). Outside a git tree the
//! commit falls back to "unknown" in main.rs.

use std::process::Command;

fn main() {
    // HEAD changes on branch switches; COMMIT_EDITMSG on every commit —
    // together they keep the stamped commit current without watching all of .git.
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/COMMIT_EDITMSG");
    let head = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output();
    if let Ok(output) = head
        && output.status.success()
    {
        let commit = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        if !commit.is_empty() {
            println!("cargo:rustc-env=AURORA_BUILD_COMMIT={commit}");
        }
    }
}
