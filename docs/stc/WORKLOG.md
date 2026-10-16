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
