//! The headless AURORA driver (§7.3): `aurora --url …` fetches and prints.
//! Milestone demos script this binary end to end; it is also the simplest
//! embedder of the engine's public API.

use aurora::{CancelToken, fetch};
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut dump_bytes = false;
    let mut dump_text = false;
    let mut target: Option<String> = None;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--dump-bytes" => dump_bytes = true,
            "--dump-text" => dump_text = true,
            "--url" => target = iter.next().cloned(),
            "--version" => {
                println!("aurora {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            other => {
                if target.is_none() && !other.starts_with('-') {
                    target = Some((*other).to_owned());
                } else {
                    eprintln!("usage: aurora [--dump-bytes|--dump-text] <url | --url url>");
                    std::process::exit(2);
                }
            }
        }
    }
    if dump_bytes && dump_text {
        eprintln!("aurora: --dump-bytes and --dump-text are mutually exclusive");
        std::process::exit(2);
    }
    let Some(url) = target else {
        eprintln!("usage: aurora [--dump-bytes|--dump-text] <url | --url url>");
        std::process::exit(2);
    };

    let outcome = match fetch(&url, &CancelToken::new()) {
        Ok(outcome) => outcome,
        Err(error) => {
            eprintln!("aurora: fetch failed: {error:?}");
            std::process::exit(1);
        }
    };
    eprintln!(
        "aurora: {} {} ({})",
        outcome.status, outcome.reason, outcome.final_url
    );
    let body = if dump_text {
        outcome.text().into_bytes()
    } else {
        outcome.body.clone()
    };
    if std::io::stdout().write_all(&body).is_err() {
        // A closed stdout (e.g. piped into head) is not a fetch failure.
        std::process::exit(0);
    }
}
