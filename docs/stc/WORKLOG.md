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

## 2026-10-17 - Checkpoint 4d: rounded outer frames

- A controlled native thickness probe confirmed that ordinary unfilled rounded-box
  stroke centerlines sit half a stroke width inside the final outer bounds.
- Added the verified inset with explicit scope guards and span metadata; continuation
  fragments and final endpoints preserve it without moving rows or authored model data.
- Docker checks: 351 layout and 12 worker tests, five Chinese PDF tests, nine PDF fixtures
  / 15 pages, strict workspace Clippy and formatting all passed. No new public reference
  changes. Worker rebuilt with the bounded resource policy; host RAM recovered to 7.9 GiB.
- All three page pairs were inspected. Text, heading, signature and footer fixes remain
  confirmed. A suspected page-2 footer omission was disproved by identical line pixels.
- Deeper outline review found actual remaining Arabic punctuation/shadda positioning
  differences; these are not dismissed as extraction artifacts. Investigation continues
  from this committed checkpoint. Uniform corner-radius rounding retains a 0.05-pt residual.

## 2026-10-17 - Checkpoint 4e: paragraph RTL and Arabic marks

- Native metadata and a controlled saved-copy byte probe established the omitted
  paragraph reading-order slot. Retain optional per-paragraph direction, project the
  first paragraph for older object-level consumers, and carry direction per wrapped
  line. Preserve old serialized-model inheritance and orphan field-heading direction.
- Windows Uniscribe independently established the native exporter's reversed Arabic
  vertical-offset convention. Added an opt-in PDF policy, enabled by the STC worker,
  restricted to Arabic logical clusters; glyph selection, joining, advances and other
  scripts remain unchanged. Library defaults retain standard Krilla placement.
- Focused Docker checks: 325 reader, 354 layout, 64 model, 79 PDF unit + 1 integration.
  Tests cover mixed directions, missing/widened values, serde compatibility, actual
  Arabic marks, a positioned Latin accent, mixed Hebrew clusters and serialized PDFs.
- Strict JSON refresh audited 154 reports: 106 files receive 364 explicit LTR paragraph
  fields, with no object direction changes or unrelated edits. Git confirms 364 added
  lines and zero removed lines. No private report/text/data was added to the corpus.
- [Arabic evidence](coverage/cr-001-arabic.md) records native probes, implementation
  limits, negative-fixture correction and baseline audit. Final workspace checks and
  fresh all-page comparison are the next acceptance gate; no SQL or deployment ran.
- Final workspace sweep passed 1,590 tests and reused the 40 just-passed unchanged
  decoder parity tests: 1,630 total, zero failed/ignored. Includes all five Chinese
  PDF tests, JSON/KDL checks and public PDF reference fixtures. No live DB configured.
- Strict workspace Clippy (all targets/features, warnings denied) and formatting pass.

## 2026-10-17 - Checkpoint 4f: final text origins

- Independent review of the fresh pair at `3683e20` found three remaining measured
  causes: centered-field trailing whitespace, first-line printer ascent, and final
  per-glyph alignment measurement. Acceptance stayed open while native probes and
  all-glyph coordinate classification distinguished these from font rasterization.
- The centered LTR field omits trailing ASCII spaces before measurement/emission.
  Left/right and centered RTL fields retain their text; static centered LTR text
  keeps trailing glyphs while excluding them from its final alignment measurement.
  Leading/internal spaces, blank lines, tabs and NBSP retain their existing semantics.
- Selected printer DPI controls ascent for static text, explicit RTL fields and known RTF fields, rounded to whole twips; other fields retain prior nominal ascent.
  This supersedes checkpoint 4a's first-ascent preservation policy, not its measured
  rating-baseline results. The ordinary 1440-DPI default and PDF cell sizing remain.
- Final Center/Right alignment with zero character spacing rounds each shaped advance
  to PDF thousandths, then effective-font twips, before summing. Wrapping and painted
  advances retain their earlier behavior. Ligatures and zero-advance marks are tested.
- Focused Docker checks pass: 361 layout, 18 Page IR, 27 text, 12 worker and 12 minimal
  text tests. A page-total fixture initially omitted the existing reserved sign cell;
  corrected expectations now explicitly retain it while verifying the late suffix fix.
- Render integration checks, strict linting and independent replay measurements are
  in progress. [Current acceptance](coverage/cr-001-parity.md) owns the final results.
- Universal printer ascent was rejected after 54 field-run regressions. Controlled
  native probes establish RTL and RTF paths; CanGrow, Right alignment, their
  conjunction and a short box are insufficient general rules. Synthetic guards
  retain old field ascent for unverified formats and those negative cases.
- Two standard-field runs / 36 glyphs retain a documented 0.10-pt native baseline
  exception. No report/object-specific offset is introduced. The scoped repair
  remains a recoverable checkpoint while committed-source comparison is prepared.
- Scoped checkpoint verification passes 481 Docker tests (19 Page IR, 27 text, 367 layout,
  56 render, 12 worker), including five Chinese PDF tests. Strict workspace Clippy and
  formatting pass; worker rebuilt. The earlier minimal-feature text run passed 12.

## 2026-10-17 - Checkpoint 4g: fresh pair at 33f1998

- Regenerated original Windows Crystal and Docker Rust from clean commit `33f1998`
  at 01:22:32-01:22:58 local, using the same captured schema-bearing DataSet; no SQL.
- Both PDFs pass integrity checks, have three 595 x 841 pt pages, preserve the date and
  all 34 ordered ratings (17/17/0). All six page images were reviewed.
- Independent actual-glyph review matches 4,467 nonblank outlines/sizes, 283 runs and
  all 216 aligned runs. Ratings match baseline origins within 0.00004 pt; all 63 English
  line breaks remain, with maximum glyph X error 0.03332 pt. Overall maximum X error 0.05708 pt.
- Exactly two standard-field runs retain 0.10 pt baseline error. Other ordinary
  baselines agree within 0.00005 pt; Arabic mark rounding is at most 0.03016 pt.
  This remains an explicit open comparison, not pixel-identity acceptance.
- Private final/ holds the fresh pair, six PNGs, HTML comparison, source/input/engine
  hashes and independent review. Previous evidence is preserved in
  final-before-text-origins/; no files or Docker volumes were pruned.

## 2026-10-17 - Checkpoint 4h: growing-field content metrics

- Replaced only private copied input strings to isolate the two remaining baseline
  exceptions. Synthetic ASCII removes the footer's 0.10-pt growth-dependent shift.
  Independent synthetic Arabic in a different centered field reproduces a 0.05-pt
  shift across 34 runs / 170 glyphs; its Latin growth control has zero movement.
  Fonts, glyph outlines, horizontal positions and all other content stay unchanged.
- Growing plain LTR fields now ask for an ascent selected from the complete resolved
  content. Windows compatibility inspects actual shaped RTL glyph levels and chooses
  printer ascent only when present. Fixed fields, English/CJK content, HTML/unknown
  metadata and ordinary 1440-DPI defaults preserve their previous behavior.
- Added actual-shaper Arabic/mixed/Chinese/bidi-control tests and layout regressions
  for metadata guards, fixed footers, complete paragraphs and adapter forwarding.
  This changes no declared direction, text, wrapping, glyph advances or report data.
- Docker tests, independent source review and a new actual-PDF comparison are in progress.
  Original templates/data remain unchanged; no SQL, branches, worktrees or deployment.
- Final source checks pass: 486 focused Docker tests (19 Page IR, 29 text, 370 layout,
  56 render, 12 worker), plus 12 minimal-feature text tests. Includes all five Chinese
  PDF tests and the public reference net. Strict workspace Clippy and formatting
  pass; worker rebuilt. Independent review found no blocking source issue.
- Independent candidate replay confirms exactly two footer runs / 36 total glyphs
  (32 nonblank) move by -0.10 pt in PDF Y. Their baselines now match native exactly.
  No X/font/code/width changes; Page IR changes only the two ascent values. All 4,467
  nonblank glyph outlines/sizes match. Other ordinary baselines differ by at most
  0.00005 pt; remaining mark Y rounding is 0.03016 pt and overall X rounding 0.05708 pt.
  Fresh original/Rust regeneration from this committed source is the next gate.

## 2026-10-17 - Checkpoint 4i: final CR-001 comparison accepted

- Fresh original Windows Crystal and Linux Docker Rust exports ran at 01:38:46-01:39:09
  local from clean implementation commit `be022bd`, the unchanged original template
  and the same frozen schema-bearing DataSet. No SQL was executed.
- Both PDFs pass integrity checks, have three 595 x 841 pt pages and the same print
  date. All 34 ordered ratings (17/17/0), 63 English line breaks, 283 text runs and
  4,467 nonblank glyph outlines/sizes match. The restored boundary heading repeats
  correctly. Both previously offset footer baselines now match native exactly.
- Root and independent reviewer inspected all six final PNGs. The fresh glyph/outline
  data, Page IR, run geometry and horizontal paths equal the verified candidate;
  each engine's fresh page PNGs also equal its candidate PNGs. No blocker remains.
- Accepted for this captured case with explicit numerical tolerances: maximum glyph
  X error 0.05708 pt, ordinary baseline error 0.00005 pt, shadda Y rounding 0.03016 pt,
  and rounded-curve endpoint rounding about 0.05005 pt. Four rounded stroke bounds
  match exactly. Forty-two same-outline CMap differences explain extraction changes;
  this is not byte/pixel identity or evidence for another report case.
- Latest implementation checks: 486 focused Docker tests plus 12 minimal text tests,
  including five Chinese PDF tests; strict workspace Clippy and formatting pass.
  The earlier full 1,630-test checkpoint remains separately recorded.
- Final private PDFs, six PNGs, HTML comparison, input/source/engine hashes, resource
  record and independent review live under ignored `cr-001-parity/final/`. Previous
  comparisons are retained. Public Markdown uses only the opaque case ID.
- Resource check at 01:40:15: C: 290.38 GiB free, D: 561.49 GiB free, host RAM available
  6.88 GiB, WSL working set 0.97 GiB; no task container remains. D: holds temp/evidence;
  reused Docker volumes retain their managed location. No process kill or broad prune.
- Updated fork overview/coverage and canonical backend references. CR-003, CR-005,
  other eligible students, standard-report database rehearsal, authenticated delivery,
  Excel and broader bridge compatibility remain separate work. No Parent code changed.

## 2026-10-17 - Checkpoint 5a: CR-003 native controls and renderer repair

- Fresh baseline from clean `339404c` preserves all 52 ordered ratings over three
  pages. Independent review isolates one English wrap and two centered RTL headings;
  all 6,449 nonblank glyph outlines/sizes match before the repair.
- Native zero/one/two-prefix report copies prove that LTR leading spaces consume
  fitting width. Zero/one/two-terminal-space DataSet controls prove centered RTL
  visible text is invariant and the terminal U+0020 is not emitted. Original report,
  snapshot/schema and every unrelated cell remain unchanged. No SQL was executed.
- Windows layout now counts LTR prefixes using shaper paragraph direction; RTL prefix
  behavior remains. Centered RTL fields and late numeric rewrites prepare terminal
  spaces consistently. Existing other-alignment/static/unknown-initial guards remain.
- A new mixed-direction test exposed older character truncation from first/last
  visual glyph reconstruction. Logical min/max across all glyph clusters fixes it;
  Arabic marks, Hebrew/Arabic and constrained mixed-line tests protect the behavior.
- Red regressions captured both compatibility defects. Final source verification
  passes 473 focused Docker tests plus 12 minimal text tests, including six Chinese
  tests. Strict workspace Clippy, formatting and worker build pass; independent
  source review reports no blocker. Candidate CR-003 and CR-001 replays are next.
- Detailed native findings: coverage/cr-003-whitespace.md. Current validation and
  resource ledger: coverage/cr-003-parity.md. Private evidence and temp files use D:;
  bounded sequential Docker and idle-cache reclamation protect available RAM/C: space.

## 2026-10-17 - Checkpoint 5b: reject prefix-only candidate; prove device fitting

- Independent full-page review rejected the first candidate despite 473 passing
  tests: one English boundary wrapped early and a terminal LRM/space line shifted
  later rows by 7.05 pt. The intended wrap and RTL centering were fixed, but this
  was not an accepted comparison. Private rejected output is retained.
- Native GDI/Arial regular+bold probes match 570 hdmx glyph widths exactly across
  75/83/92ppem. At600DPI/83ppem, 4792.8twips fits while4802.4 does not fit4795;
  raw design and naive pixel rounding cannot distinguish both boundaries correctly.
- Exact font-provided device widths now drive only opted-in printable-ASCII fitting
  through the existing shaped line layout; unavailable/malformed records and other
  scripts fall back. Painted advances/public nominal width remain unchanged.
- A native one-cell terminal-space/LRM removal leaves all6,449 visible glyphs and
  positions unchanged. LTR fit separates/restores only U+0020/LRM tails, preventing
  phantom lines while preserving logical content. Other controls/marks are guarded.
- Initial corrected-candidate audit pairs all6,449 glyphs/377runs without exclusions;
  maxX0.06008pt and markY0.03016pt. All-page and extended regression gates continue.
- CR001 with the first candidate was byte-identical in PDF/IR/PNGs; the latest
  hinted-width worker gets its own replay. No SQL, original-input change or deployment.
- Final source validation passes 479 focused Docker tests (39 text, 371 layout,
  57 render, 12 worker), plus 13 minimal-feature text tests, strict workspace Clippy
  and formatting. Four in-memory synthetic font/device-table tests cover the real
  fitting path and guards without proprietary assets. All six Chinese tests pass.
- Corrected candidate all-page review passes all 72 English lines, 52 ratings and
  6,449 glyphs, with documented rounding only. Latest CR-001 Rust PDF/IR/all PNGs
  remain byte-identical to its accepted final outputs. Clean-source pair is next.
