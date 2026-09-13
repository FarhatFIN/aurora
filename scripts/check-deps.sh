#!/usr/bin/env bash
# Dependency policy check (§8.8, §3.2, §4.3). Machine-checks, dependency-free:
#   1. no engine crate may depend on the shell (§7.2 exit criterion);
#   2. each crate's internal dependencies are within its §4.3 "may depend on" set;
#   3. every external dependency is on the §3.3 approved list (tier rules that
#      need human judgment — "reference only in tests", "not in hot paths" —
#      are enforced in review per §9.7, not here);
#   4. nothing from the §3.3 explicitly-not-approved list appears anywhere.
set -euo pipefail
cd "$(dirname "$0")/.."
FAILED=0

allowed_deps() {
    case "$1" in
        aurora_url|aurora_encoding|aurora_dom|aurora_css|aurora_image|aurora_js|aurora_ipc) ;;
        aurora_net)      echo "aurora_url aurora_platform" ;;
        aurora_html)     echo "aurora_dom aurora_encoding" ;;
        aurora_style)    echo "aurora_css aurora_dom" ;;
        aurora_text)     echo "aurora_platform" ;;
        aurora_layout)   echo "aurora_style aurora_text aurora_dom" ;;
        aurora_paint)    echo "aurora_layout aurora_text aurora_image" ;;
        aurora_runtime)  echo "aurora_js aurora_dom aurora_net aurora_style aurora_layout aurora_paint" ;;
        aurora_storage)  echo "aurora_net aurora_platform" ;;
        aurora_security) echo "aurora_url" ;;
        aurora_platform) ;;
        aurora)          echo "aurora_runtime" ;;
        aurora_shell)    echo "aurora_runtime aurora_paint aurora_platform aurora_ipc" ;;
        *) echo "__unknown__" ;;
    esac
}

# §3.3 approved list (Tier 1 + dev-only crates).
approved_external() {
    case "$1" in
        winit|softbuffer|wgpu|rustls|rustls-pemfile|rustls-native-certs|webpki-roots|\
        harfbuzz|harfbuzz-sys|rustybuzz|fontdb|fontconfig|memmap2|\
        image|png|flate2|miniz_oxide|libc|windows-sys|objc2|\
        unicode-bidi|unicode-segmentation|unicode-normalization|\
        rayon|serde|criterion|cargo-fuzz|libfuzzer-sys|tempfile) return 0 ;;
        *) return 1 ;;
    esac
}

forbidden_external() {
    case "$1" in
        html5ever|scraper|selectors|quick-xml|swc|swc_core|boa|boa_engine|quickjs|\
        deno_core|tao|gtk|gtk4|qt|egui) return 0 ;;
        *) return 1 ;;
    esac
}

deps_of() { # $1 = Cargo.toml path, $2 = section header
    awk -v want="$2" '
        /^\[/ { insec = ($0 == "[" want "]") ; next }
        insec && /^[A-Za-z0-9_-]+[ \t]*[=]/ { name = $1; sub(/[ \t]*[=.].*$/, "", name); print name }
    ' "$1"
}

for manifest in crates/*/Cargo.toml apps/*/Cargo.toml; do
    crate=$(dirname "$manifest" | xargs basename)
    allowed=$(allowed_deps "$crate")
    [ "$allowed" = "__unknown__" ] && { echo "FAIL: $crate is not in the §4.3 map"; FAILED=1; continue; }

    while read -r dep; do
        [ -z "$dep" ] && continue
        if [ "$dep" = "aurora_shell" ] && [ "$crate" != "aurora_shell" ]; then
            echo "FAIL: engine crate $crate depends on the shell (§4.3 boundary)"
            FAILED=1
        elif case " $allowed " in *" $dep "*) true ;; *) false ;; esac; then
            : # allowed internal dependency
        elif forbidden_external "$dep"; then
            echo "FAIL: $crate depends on $dep — explicitly not approved (§3.3)"
            FAILED=1
        elif approved_external "$dep"; then
            : # Tier 1 approved; per-crate tier notes live in the crate's Cargo.toml
        else
            echo "FAIL: $crate depends on $dep — not on the §3.3 list; needs an ADR (§3.2)"
            FAILED=1
        fi
    done < <(deps_of "$manifest" dependencies; deps_of "$manifest" dev-dependencies)
done

[ "$FAILED" -eq 0 ] && echo "dependency policy: OK" || exit 1
