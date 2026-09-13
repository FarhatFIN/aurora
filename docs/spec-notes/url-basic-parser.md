# WHATWG URL Standard — reading notes

Source: <https://url.spec.whatwg.org/> (living standard, fetched 2026-09-13).
Implemented in `aurora_url`: basic URL parser (§4.4), host/IPv4/IPv6 parsers
and serializers (§3), percent-encode sets (§1.3), origin (§4.7), the
`x-www-form-urlencoded` serializer (§5).

## Key facts the code relies on

- Input preprocessing: strip leading/trailing C0-or-space, then remove every
  tab/LF/CR. The parser is total — failure is a return value, never an abort.
- Path percent-encode set (current text): query set + `? ^ ` { }`. There is
  no separate non-special path set anymore; userinfo = path + `/ : ; = @
  [ \ ] |`.
- IPv4: fewer than four parts is a *validation error*, not a failure —
  `http://1.2.3/` → `1.2.0.3`. Hex (`0x`) and octal (leading `0`) radixes
  apply per part; `0x` with no digits is valid (zero). `09` fails (9 is not
  an octal digit) — consistent with the standard's roundtrip table.
- IPv6: the serializer has no IPv4-mapped special case in the current text —
  `::ffff:127.0.0.1` serializes as `::ffff:7f00:1`. Compression picks the
  first longest zero run (longer than one piece).
- Host parser: percent-encoded byte in a domain is a validation error only;
  the input is still percent-decoded before domain-to-ASCII. IDNA (UTS-46)
  is NOT implemented — non-ASCII domains fail with `IdnaError` (standing
  placeholder, PROGRESS.md).
- File URLs: host is the empty host (serialized between `//` and path);
  `localhost` normalizes to the empty host; Windows drive letters are
  normalized platform-independently (`C|` → `C:`).
- Serializer: `//` is emitted iff the host is non-null; default ports are
  elided; the host-null + `path[0] == ""` case emits `/.` to avoid
  re-parsing as an authority.
- Pointer model: a state processes the code point at the pointer; "decrease
  pointer by k" rewinds; the machine exits when a run leaves the pointer at
  EOF. "Start over" resets the pointer to the first code point.

## Deferred (tracked)

- IDNA/UTS-46 domain-to-ASCII (non-ASCII hosts) — placeholder in
  PROGRESS.md; unblocks internationalized domains when landed.
- Parse-error collection (validation errors) for DevTools — WBS §6.2.
