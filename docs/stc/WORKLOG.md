# STC checkpoint ledger

## 2026-10-16 - Existing implementation baseline

| Commit | Checkpoint |
| --- | --- |
| `0511565` | Docker language verification; Arabic direction/shaping and Chinese fallback/wrapping/extraction |
| `7dc65ba` | Experimental PDF bridge and worker; native OLE bitmap recovery; first live evaluation comparison |
| `0cecca5` | Evaluation pagination, repeated headers, conditional footers, typography and shape parity |
| `4c7fd0c` | Native field border weights and PDF device hairlines |

Detailed findings, fixes and verification are retained in [README](README.md). The latest baseline has
three matching pages and all 35 ratings in order; minor text/row metrics remain. This is close visual
parity for one template, not full Crystal compatibility. Full Docker workspace validation reported
1,585 passing tests, followed by two additional regressions in affected suites; 28 bridge contract
checks and four Chinese integration checks passed. Database-gated workspace tests were not configured.

## 2026-10-16 - Checkpoint 1: local fork and evidence policy

- Preserved the existing clone, history, `main` branch and MPL-2.0 license.
- Renamed the source remote from `origin` to `upstream`; no hosted fork or push.
- Added fork provenance, contributor instructions, coverage gates and this resumable ledger.
- Started independent read-only subagent reviews of current Parent report surfaces, SQL eligibility,
  and native/Rust comparison readiness under the user's explicit authorization.
- Validation: clean starting checkout; documentation and Git diff checks. No runtime behavior changes.
- Next checkpoint: consolidate source coverage and local SQL candidate counts, record unavailable or
  unsafe-to-run branches, then exercise isolated report cases with explicit evidence levels.

## 2026-10-16 - Checkpoint 2a: host-local report clock

- Review found Docker's UTC clock could print the previous date during the first hours of the
  Windows host's local day. Capturing culture alone did not fix CurrentDate/CurrentTime specials.
- The STC launcher now captures host DateTime.Now, the adapter validates/forwards it, and the worker
  applies one local clock to data/formulas/layout. Explicit replay clocks are supported; malformed
  values fail before data access. Existing standalone-worker fallback is documented.
- Added persistent near-midnight, override, malformed-date and transport regressions. Docker results:
  7 launcher checks, 39 adapter checks, 10 worker tests; strict worker Clippy and formatting passed.
- Normal worker/adapter binaries rebuilt. No database or PDF generation for this fix.
- Details and reproduction: [clock parity](coverage/clock-parity.md).
