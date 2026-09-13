# TLS reading notes (§5.3)

Source: RFC 8446 (TLS 1.3) as provided by `rustls`; RFC 5280 chain
validation via the same; RFC 6797 (HSTS — lands with M12, §7.14).

## What the trust store provides

- Default: `webpki-roots` (Mozilla's root program snapshot) — hermetic, so
  tests and builds do not depend on the OS trust store (§3.3: "prefer
  webpki-roots for hermetic tests"). `rustls-native-certs` is the approved
  alternative when system-store behavior is wanted.
- Override: `Pool::with_root_certs(certs)` replaces the trust set — used by
  the TLS matrix tests (committed fixtures under `tests/fixtures/tls/`) and
  by the future `--cert-bundle` CLI flag (§7.3).
- Policy: TLS 1.2 and 1.3 only (explicit `with_protocol_versions`); no SSL
  v3/TLS 1.0/1.1; no RC4/3DES; no compression; ALPN offers `http/1.1`.
- Verification failure is a hard `NetError::Tls` with an exact text variant
  (`certificate expired`, `hostname mismatch`, `untrusted certificate
  authority`, …) — there is no click-through UX (scope decision, §5.3).
- Handshake driver: `process_new_packets` is driven directly so
  verification failures surface as typed rustls errors and map to the
  exact failure text; the deadline is 10 s (§5.2.5 phase table).

## Fixture regeneration

`tests/fixtures/tls/` holds the throwaway test PKI (generate with the
openssl CLI; keys are test-only and carry no authority):

```sh
cd tests/fixtures/tls
openssl req -x509 -newkey rsa:2048 -nodes -keyout ca.key -out ca.pem \
  -days 30 -subj "/CN=AURORA Test CA"
# leaf: good.pem (SAN DNS:localhost,IP:127.0.0.1) signed by ca
# expired.pem: same SAN, -not_before 2019… -not_after 2020…
# wronghost.pem: SAN DNS:other.example, signed by ca
# untrusted: separate self-signed untrusted-ca + leaf signed by it
```

Note: a self-signed *leaf* without `basicConstraints=CA:FALSE` is rejected
as `Other(CaUsedAsEndEntity)` before issuer checks — the untrusted case
needs a real (untrusted) CA signing a normal leaf to exercise
`UnknownIssuer`.
