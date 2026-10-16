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

## 2026-10-16 - Checkpoint 3a: live harness corrections

- The first case attempt stopped before SQL: PowerShell treated SqlConnectionStringBuilder property
  assignment as an unsupported dictionary keyword. Switched to canonical connection-string keys
  and added a synthetic round-trip regression, including a semicolon-bearing database name.
- Confirmed actual timeout behavior with a disposable network-disabled container: the client timed
  out, the owned container was removed, and no background renderer survived.
- Independent review found that HTML reruns could include stale trailing PNGs. The viewer now
  selects exactly the current page range; an injected stale-image regression covers this. Empty
  extracted words are filtered and each engine pane links its PDF.
- Validation: 13 harness checks in PowerShell 5.1 and 7; four Python comparison tests in Docker.
  The three representative live case runs then completed capture/native-live/native-frozen/Rust
  successfully. Actual PDF and visual results are recorded in the following checkpoint.

## 2026-10-16 - Checkpoint 3b: three real evaluation comparisons

- Each ready template representative passed reviewed SQL capture, original typed native printing,
  native DataSet replay and Docker Rust replay. Nine PDFs passed qpdf; every export has three pages.
- Native-live/replay PNG hashes are identical for all nine pages. Native/Rust ratings match in order:
  CR-001 17/17/0; CR-003 18/21/13; CR-005 16/12/7. Dates and paper boxes match.
- All nine comparison page pairs were inspected, with independent agent checks. Confirmed open
  differences: CR-001 orphan page-boundary heading; CR-003 line wrapping; shared baseline/row metrics.
  A suspected missing-label defect was disproved by bbox/raster checks and withdrawn.
- All case requests/partial files were cleaned up and no owned case container remains. No live
  standard procedure ran, and no school/grade/publication/template/configuration data was changed.
- Private comparison index and PDFs are ready. Shared results and the next one-case-at-a-time repair
  queue are in [initial comparisons](coverage/initial-comparisons.md). Three representatives are
  tested; the other 95 eligible students remain SQL-only evidence. No visual parity acceptance yet.

## 2026-10-16 - Checkpoint 4a: CR-001 printer resolution and row pitch

- Reproduced both engines from the frozen snapshot without SQL. The actual first glyph
  baselines match; earlier bbox offsets mostly reflected differing PDF font descriptors.
- Recovered stored DEVMODE resolution (600 x 600) despite blank printer names. Retained
  independent positive X/Y values in the model; quality enum codes do not become DPI.
- Added opt-in printer-pixel line pitch and selected it in the STC worker, preserving glyph
  sizing and first baselines. Docker supports an explicit override; the template wins otherwise.
- Corrected cumulative row drift: all 34 rating baseline origins now agree within 0.00004 pt.
  The orphan heading is restored at page 1's bottom and repeated on page 2; three pages and
  17/17/0 ordered ratings are preserved. All page pairs reviewed independently.
- Added numeric, raw decoder, old JSON, bilingual pagination, launcher and actual Chinese
  PDF regression coverage. Affected Rust suites: 845 passed; minimal text: 10 passed;
  Chinese: five passed; launcher: five checks on PS7/5.1 plus seven unchanged clock checks.
- Details and the correction to earlier bbox conclusions: [CR-001 metrics](coverage/cr-001-metrics.md).
  Horizontal kerning and remaining small geometric differences are still being investigated.

## 2026-10-16 - Checkpoint 4b: CR-001 English spacing

- Actual glyph operators isolated pair kerning as the remaining English spacing error.
  Added opt-in Basic-Latin unkerned measurement/wrapping and PDF shaping using shared ranges.
- English glyph deviation across 63 matched lines fell from 0.64045 to 0.05832 pt.
  Line breaks, all 34 ratings/baselines and the restored heading are preserved.
- All 2,015 non-ASCII glyphs retain exact text and coordinates relative to the DPI600 replay.
  All three page pairs reviewed. Arabic-only PDFs, ligatures, combining marks and Chinese
  actual-PDF transport have focused regressions; general library defaults remain unchanged.
- Details: [spacing findings and fix](coverage/cr-001-spacing.md). Signature/rule geometry
  is recorded separately; this checkpoint does not claim pixel identity.

## 2026-10-17 - Checkpoint 4c: CR-001 border and rule geometry

- Controlled native probes established 1.5-point outward ordinary SingleLine text/field
  borders and half-width anchoring for authored horizontal/vertical lines. Signature
  endpoints remain unchanged; no report/object/student-specific adjustment is used.
- Independent review narrowed the correction to verified ordinary frames and axis-aligned
  lines. Mixed styles, tight/filled/shadowed/rotated frames, two-dimensional line boxes
  and degenerate lines preserve prior geometry, with explicit regression coverage.
- Audited two public PDF reference changes: 284 coordinate lines / 142 segments only;
  all text, fonts, images, transforms and page counts remain unchanged.
- Docker full workspace: 1,615 passed. Final scope guards subsequently passed all
  348 layout tests and nine PDF reference fixtures / 15 pages. All five Chinese PDF
  tests passed in the workspace run. Strict workspace Clippy and formatting pass;
  final text tests: 23, minimal features: 11. Default nonzero DPI is now checked at
  compile time, resolving a strict documentation lint without changing behavior.
- Resource controls added after the user's memory warning: one build/test job,
  sequential 2-GiB Docker containers, D: temporary files and host RAM/disk checks.
  Reclaimed unused Linux filesystem cache; no files, volumes or user processes removed.
- [Geometry evidence](coverage/cr-001-geometry.md) records probes, limitations, review
  guards and reference audit. The fresh final pair and independent review follow in
  [the CR-001 comparison record](coverage/cr-001-parity.md).
