# Test fixtures

Reference data whose bytes the tests depend on (§8.9).

| Directory | Contents | Source |
|---|---|---|
| `tls/` | Throwaway test PKI: `ca.{pem,key}`, leaf certs `good` (SAN DNS:localhost, IP:127.0.0.1), `expired` (notAfter 2020), `wronghost` (SAN other.example), and `untrusted-ca` + its leaf | Generated with the openssl CLI; see `docs/spec-notes/tls.md` for the regeneration commands |

The TLS keys here are test-only, carry no authority, and are committed so
the §5.3 certificate-failure matrix is reproducible everywhere.
