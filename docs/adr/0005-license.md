# ADR-0005: License — Apache-2.0

- **Status:** accepted (M2, per the project owner's instruction)
- **Context:** the workspace declared `MIT OR Apache-2.0` at M0 as a
  placeholder. The owner requires a license with a patent grant: MIT has
  none, Apache-2.0 grants an explicit one (§3) with defensive-termination
  (§3) — the standard choice for patent-relevant open source.
- **Decision:** the whole repository is licensed under **Apache-2.0**
  (`LICENSE` at the root; the workspace `license` field is
  `Apache-2.0`). The MIT option is dropped.
- **Consequences:** contributions are covered by the same grant;
  the vendored master specification, the generated tables, and the
  committed test PKI are distributed under it. Third-party crates keep
  their own licenses (the approved Tier 1 list: rustls — Apache/ISC/ MIT
  triple, flate2 — Apache/MIT, ring — ISC-style with OpenSSL exception).
