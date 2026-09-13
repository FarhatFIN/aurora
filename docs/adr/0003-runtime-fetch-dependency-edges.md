# ADR-0003: `aurora_runtime` depends on `aurora_url` and `aurora_encoding`

- **Status:** accepted (M1, second session)
- **Context:** §4.3 assigns the fetch-API to `aurora_runtime` but its
  "may depend on" list omits `aurora_url` and `aurora_encoding`, without
  which the runtime cannot parse the URL it fetches (the network stack's
  `Request` carries a `Url`) nor decode response text (§7.3's
  `--dump-text` is "print decoded body"). The omission is a spec bug: no
  route exists from the shell-facing crates to URL parsing that does not
  smuggle it through another crate's API.
- **Decision:** `aurora_runtime` may depend on `aurora_url` and
  `aurora_encoding`; `scripts/check-deps.sh`'s allowed map is updated to
  match. The `aurora` facade gains no new edges (still `aurora_runtime`
  only); the headless `aurora` binary lives as a bin target of the `aurora`
  package and uses the engine exclusively through the facade.
- **Consequences:** the crate map in §4.3 is amended here (per §0.4 the
  document stays ground truth via this ADR). The bridge rule and shell
  boundary are unchanged.
