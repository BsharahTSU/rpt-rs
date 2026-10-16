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

## 2026-10-16 - Checkpoint 2b: Parent inventory and isolated-case harness

- Source review traced three fixed AAJ Crystal templates and dynamic evaluation RPT policies in
  AAJ/GOPS/BAS/TerraSanta; external Gradebook remains an unverified separate application.
- Read-only SQL inventory found 38 grouped cases across eight configured/local AAJ templates and
  246 current students in those cohorts. Three first-term evaluation groups have 98 ready candidates
  (27 / 33 / 38). Other cases are retained with publication, empty-data or configuration blockers.
- The standard AAJ procedure updates grade-related tables. No execution was attempted on live data;
  a disposable database is required for faithful standard-case testing. Other local copies were
  metadata-only discovery: 51 school databases, 19 source profiles, no remote servers contacted.
- Added one-case capture/native-live/native-frozen/Docker-Rust runner and generic PDF inspection.
  Actual worker/adapter/template/snapshot hashes, scoped rows and culture/clock are recorded privately.
  Fresh run directories and owned-container cleanup prevent stale exports from becoming passes.
- Review caught and corrected UTF-8 and subprocess-exit handling under Windows PowerShell 5.1.
- Validation: 12 synthetic harness checks passed in PowerShell 7 and 5.1; three Python comparison
  regressions passed in Docker. Inventory completed in about eight seconds; scripts parsed and
  private outputs are ignored. No PDFs generated at this checkpoint.
- Prepared private manifests for one eligible representative in each ready group. The established
  reference student is retained for the third template. Next: render each case sequentially and
  record actual native export, frozen replay, Rust diagnostics and all-page visual findings.
- Details: [surfaces](coverage/parent-surfaces.md), [SQL](coverage/sql-eligibility.md),
  [runner](coverage/case-runner.md).
