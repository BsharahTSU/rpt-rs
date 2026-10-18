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

## 2026-10-17 - Checkpoint 5c: final CR-003 pair accepted

- Fresh native Windows Crystal and Docker Linux Rust exports from clean `899ca64`
  ran at 19:02:21-19:02:47. Both PDFs are valid, three 595x841pt pages, with the same
  date and 52 ordered ratings (18/21/13). Original template and snapshot hashes agree.
- Coordinator and independent reviewer inspected all six final PNGs. Full audit
  without exclusions matches 6,449 glyphs, 377 runs, 279 aligned runs and 72 English
  lines. Both headings, the target wrap, short-word boundary and terminal-LRM row
  are correct. No unresolved layout blocker remains in this captured pair.
- Measured residuals: max glyph X 0.06008pt; ordinary Y 0.00005pt; rating Y 0.00004pt;
  mark Y 0.03016pt; path rounding 0.05004pt. Seven rounded bounds agree within
  0.00005pt and six filled bounds exactly. Eighty-nine CMap differences preserve
  identical outlines. This is documented tolerance acceptance, not pixel identity.
- Fresh per-engine PDFs/PNGs, Rust IR and audit data equal the reviewed candidate.
  Latest CR-001 replay is independently confirmed byte-identical in PDF/IR/PNGs.
- Final checks: 479 focused Docker tests plus 13 minimal text tests, six Chinese
  tests, strict Clippy/formatting and worker build. No public baseline changes.
- At 19:03:43 C: had 282.98GiB free, D: 561.41GiB free, available RAM 4.28GiB;
  no container remained. D: holds temp/evidence; existing Docker volumes were reused.
- Updated fork/current coverage and canonical backend documentation. CR-005, other
  candidates, standard-report database rehearsal, Excel/subreports and authenticated
  Parent delivery remain open. Local commits only; no branches/worktrees/push/deploy.

## 2026-10-18 - Checkpoint 6a: CR-005 text/layout verified; image residual isolated

- Fresh original Windows Crystal and Docker Rust baseline from clean bf34df6 uses
  the unchanged frozen snapshot without SQL. Both have three 595x841pt pages,
  matching date and 35 ordered ratings (16/12/7). All six PNGs were inspected.
- Independent full outline/operator audit matches 4,918 glyphs, 301 runs and 59
  English lines without exclusions. Max X 0.05894pt, ordinary Y 0.00005pt,
  mark Y 0.03016pt and path rounding 0.05004pt. Shared CR-001/CR-003 fixes already
  repair the old text/row drift; no new runtime patch is justified by these findings.
- Logo inspection identifies a previously unmeasured shared residual in CR-001,
  CR-003 and CR-005: Rust exactly retains stored 24-bit BMP pixels; Crystal reduces
  2,358 source colors to 185 used indexed colors. No alternate stored logo exists.
  Exact palette generation remains unknown. Do not claim overall exact matching.
- Added synthetic combined continuation/forced-group/final-footer regression and
  Simplified/Traditional Chinese trailing-space/LRM regression. Private evidence,
  findings, source paths and remaining scope are in coverage/cr-005-parity.md.
- Use D:\tmpppppp123\rpt-rs-cr005 and bounded sequential Docker; local commits only,
  no branch/worktree/push, database activity, original-input change or deployment.
- Checkpoint validation: 430 affected Docker tests (372 layout, 58 render including seven Chinese), strict affected-crate Clippy and workspace formatting pass. Independent test review has no blockers. RAM guard stopped work at 1.60GiB; idle cache reclamation with no containers allowed safe continuation, ending at 3.94GiB available.

## 2026-10-18 - Checkpoint 6b: fresh CR-005 pair and explicit image limitation

- Fresh native/Rust PDFs from clean 3a28dae at 10:31:55-10:32:18 pass integrity,
  page/date and 35-rating checks. All six final PNGs were reviewed by coordinator
  and independent auditor; the full glyph/run/path audit reproduces baseline.
- Both fresh PDFs, Rust IR and all PNGs equal their respective reviewed baseline
  bytes. Text/layout passes below 0.06pt X / 0.031pt mark Y. Exact image matching
  remains open; native's lossy indexed logo differs from source-preserving Rust.
- Read-only source/pixel investigation excludes an alternate stored logo. Standard
  Pillow and GDI+ adaptive quantizers do not exactly reproduce native. Octree-like
  palette ordering is a clue, not proof; no fitted palette or runtime patch added.
- Updated current fork/coverage references and earlier CR-001/CR-003 ledgers to
  disclose the shared image residual. Backend canonical update is Markdown only.
- 430 affected tests, seven Chinese checks, strict affected-crate Clippy, formatting
  and correct-package worker build pass. A rejected binary-name Cargo invocation
  was corrected before rendering. Original template/snapshot hashes are unchanged.
- Final resources 10:33:54: C 286.00GiB, D 561.38GiB free, host RAM 5.42GiB available,
  WSL 1.62GiB, no running container. D temp/evidence and sequential 2GiB Docker retained.
- Full bridge compatibility and exact logo conversion remain open. Local commits
  only; no SQL, branches/worktrees, push, deployment or application behavior change.

## 2026-10-18 - Checkpoint 7a: exact generic legacy palette

- Isolated and independently reproduced the sampled six-bit octree palette with a 236-color
  budget, deepest/newest online reduction and floor centroids. All 218 captured palette entries
  and five generated native controls match byte-for-byte; no report-specific constants added.
- Added a bounded, checked implementation and eleven synthetic regressions. All 22 utility
  tests pass in capped Linux Docker. PDF output is unchanged until pixel mapping is proven.
- Direct native BitBlt reproduces the captured image, but nearest-color approximations do not.
  Generic inverse mapping, PDF integration, regenerated pair and shared-case replay remain open.
- Findings, evidence locations, resource controls and remaining gates: coverage/cr-005-logo.md.
  Existing 26 GB target cache is reused; private scratch/evidence stay on D:. No SQL or deployment.

## 2026-10-18 - Checkpoint 7b: deterministic legacy bitmap PDF conversion

- Independent native/model checks reproduce all 9,600 captured logo pixels; all 416 occupied bins
  are written. Added an original portable directional inverse map with deterministic nearest
  fallback only for cells native leaves unwritten. No private palette/LUT copied into runtime.
- Added guarded opt-in bottom-up 24-bit BI_RGB conversion. STC worker enables it; default/general
  CLI and other raster formats retain existing behavior. Page IR/source bytes remain untouched.
- 189 affected tests plus six doctests pass, including seven Chinese checks; strict workspace Clippy,
  formatting and worker build pass. Added inverse-map, header and actual PDF RGB regressions;
  native-generated synthetic controls supply independent goldens. Independent review continues.
- Detailed contracts, fallback rationale, characterized versions and limitations are recorded in
  coverage/cr-005-logo.md. Fresh clean-source PDF pair and shared-case replay remain acceptance gates.


## 2026-10-18 - Checkpoint 7c: CR-005 logo repair accepted

- Fresh original Crystal and Docker Rust exports from clean `eb4656d` at 10:59:25-10:59:47
  pass PDF integrity, three-page/date and 35-rating checks. All 9,600 decoded logo RGB pixels
  match exactly. Both reviewers inspected all six page PNGs; no new visual issue remains.
- Rust drawing streams, fonts, other images and Page IR are unchanged, preserving earlier
  glyph/path tolerances. Native output equals its previous reference. Full PDF identity is
  not claimed; known outline-equivalent CMap differences and sub-point rounding remain.
- CR-001/CR-003 Docker replays also close exact logo matching with prior native references,
  unchanged drawing/font/IR resources and all 34/52 ratings intact. No SQL executed.
- 189 affected tests plus six doctests, seven Chinese checks, strict workspace Clippy,
  formatting and worker build pass. Independent source review has no blockers.
- Updated current fork, coverage and canonical backend references. All private evidence stays
  on D:. At 11:01:40 C: had 285.08 GiB free and RAM 5.71 GiB available; no container remained.
  Existing volumes grew about 0.67 GB; no new volume, pruning, branch, worktree or deployment.

## 2026-10-18 - Checkpoint 8a: move Docker target storage to D:

- User requested relocating the Rust build cache to D: while preserving work and printing.
  Inventoried the C-backed named volume and started a read-only-source, hardlink-preserving
  copy to D:\tmpppppp123\docker-cache\rpt-rs-target with independent SHA-256 verification.
- Centralized target/temp bind mounts for Cargo, bridge printing, case artifact probes and
  the eight existing private replay/build helpers. Default target and temporary writes now
  resolve to D:, with an explicit/environment override for other machines. Printing remains
  read-only against the target. Existing positional arguments, clocks and printer DPI remain.
- Storage contracts pass in Windows PowerShell 5.1 and PowerShell 7; seven clock and five
  printer contract checks pass. Added two-CPU/2-GiB limits and one Cargo/test thread to the
  relevant development launchers. Rust source and production configuration are unchanged.
- Details and rollback/safety gates: docker-cache-storage.md. At this checkpoint the original
  volume is intact; copy verification, real build/printing and removal checks are pending.

## 2026-10-18 - Checkpoint 8b: D: cache verified and C: space reclaimed

- Completed the copy of 47,982 files / 35,251 unique inodes, preserving 12,731 extra
  hardlinks. Full independent SHA-256/size/mode/hardlink verification has zero
  mismatches. Windows rounded mtimes by less than one second; Cargo accepted the cache.
- From implementation checkpoint `c2e9221`, the actual Docker launcher builds the
  worker offline and passes 50 focused Rust tests, including seven Chinese tests.
  Storage/clock/printer contracts pass. Private helper issues (JSON clock coercion,
  slow sequential verification and PowerShell 5.1 array wrapping) are documented.
- All three frozen reports print before removal, after removal and after Docker
  compaction/restart, with byte-identical PDF/Page IR against accepted Rust outputs.
  Final qpdf checks pass; all three CR-005 raster pages match and were inspected.
- Removed only `stc-rpt-rs-target` after the copy and print gates passed. No launcher
  recreates it. The user approved UAC for detached-disk compaction; Docker restarted
  normally. Windows recovered 29,185,019,904 bytes (27.18 GiB) on C:.
- Final 12:32:59 readings: C: 313.80 GiB free, D: 535.88 GiB, RAM 5.32 GiB available.
  Source, local history, templates and prior evidence are preserved. Keep the new
  D: cache persistent; unrelated images/BuildKit still occupy Docker's C: disk.
- Full details: [storage migration](docker-cache-storage.md). Backend documentation
  updated without touching unrelated code/config edits. No SQL, new branch/worktree,
  push, deployment or change to Parent report behavior.

## 2026-10-18 - Checkpoint 9a: additional evaluation batch, first case

- Refreshed the read-only AAJ inventory; selected three new students covering 39 rows,
  20 rows and a long name. Candidate comments are empty, so that coverage stays open.
- CR-001-B01 native live/frozen and Docker Rust exports pass. All three pages reviewed;
  39 ordered ratings, image RGB and 5,060 painting glyphs agree within existing limits.
- Found and corrected an audit false positive: a one-point space contour paints no area.
  Added reusable pure glyph helpers and ten synthetic regressions, with four existing
  comparison checks. Renderer behavior is unchanged; later cases remain pending.
- Evidence and findings: [batch 01](coverage/additional-evaluation-batch-01.md). Initial
  C: 313.78 GiB, D: 535.88 GiB free, RAM 6.64 GiB. Sequential capped Docker and D: temp/cache;
  no data mutation, deployment, branch, worktree or push. Local committed checkpoint.

## 2026-10-18 - Checkpoint 9b: additional evaluation batch accepted

- All three additional students passed original live/frozen and Docker Rust exports,
  qpdf checks and review of all 16 compared page images. They cover 39 rows/three pages,
  20 rows/two pages and 35 rows/three pages. Native live/frozen raster pages agree.
- All 94 ratings, 13,272 painting glyphs and decoded image RGB values match within
  existing tolerances (max 0.06008 pt X / 0.03016 pt Y). No renderer fix was needed.
  The ten new audit tests plus four existing checks and seven Chinese PDF tests pass.
- The source-name selection is shortened by the report formula; overflowing displayed
  names and populated comments remain untested. Extraction mappings still differ;
  neither whole-page pixel equality nor full bridge coverage is claimed.
- Six distinct eligible students now have accepted comparisons; 92 remain untested.
  Updated current fork and canonical report references; private paired PDFs and index
  remain in ignored research. See [batch 01](coverage/additional-evaluation-batch-01.md).
- Final resources at 15:01:14: C: 313.56 GiB, D: 535.85 GiB free; RAM 4.96 GiB available.
  No containers remained; target cache and temporary writes stayed on D:. Only reviewed
  SELECT-only evaluation SQL ran. No data/config changes, branches, worktrees or push.

## 2026-10-18 - Checkpoint 10a: next evaluation batch, first case

- Refreshed read-only eligibility; excluded six accepted students and selected three
  new students with 16, 41 and 27 rows, each a new row count for its template.
- CR-001-B02 passes all four export stages, qpdf, replay raster equality and all-page
  review. Sixteen ratings, 2,725 painting glyphs and decoded image RGB agree within
  existing tolerances. Sparse continuation and final signatures/footer are preserved.
- Added four synthetic audit regressions for missing final content, Chinese page
  swaps, visible whitespace-mapped outlines and added blank pages. No renderer fix.
- Next two students remain pending; details in [batch 02](coverage/additional-evaluation-batch-02.md).
  C: 313.34 GiB free and RAM 5.83 GiB after first case. D: cache/temp, capped Docker,
  SELECT-only SQL, local committed checkpoint; no branch/worktree/push/deployment.

## 2026-10-18 - Checkpoint 10b: symbol ratings checked against captured data

- CR-003-B02 passes all exports and three-page review: 41 ratings, 5,568 painting
  glyphs and exact image RGB, within existing displacement limits.
- Found an audit-only omission: six source `*` ratings were displayed correctly by
  both engines but excluded by the old letter allowlist. New reusable rating checks
  retain all column tokens, compare per-page order and require source multiplicity.
- Whole-word bounds exclude a crossing Arabic header and signature parenthesis;
  four new synthetic tests cover this and shared missing/duplicate/unknown values.
  All 22 Python checks pass; both existing pairs pass the strengthened data gate.
- Third student and Chinese PDF checks remain pending. No renderer/source-data fix;
  see [batch 02](coverage/additional-evaluation-batch-02.md) for full findings.

## 2026-10-18 - Checkpoint 10c: second additional evaluation batch accepted

- Three new students pass all exports and every page review: 16 ratings/two pages,
  41/three and 27/three. All 84 captured aliases (including six `*`), 12,481 painting
  glyphs and decoded image RGB agree within existing 0.061 pt X / 0.031 pt Y limits.
- The third case preserves a sparse four-row second page and native Science page break.
  No renderer defect was found. Audit-only fixes and eight added synthetic regressions
  are documented in [batch 02](coverage/additional-evaluation-batch-02.md). All 29 focused
  checks pass (22 Python plus seven actual Chinese PDF tests in offline capped Docker).
- Verified nine distinct accepted student manifests, leaving 89 eligible candidates.
  Private PDFs, all-page galleries, final-summary and review evidence remain on D:.
  Updated current fork and canonical backend report references; prior histories preserved.
- Final 16:02:28 resources: C: 313.34 GiB free, D: 535.82 GiB, RAM 6.47 GiB available.
  No containers remain; old target volume absent. Source data/config/runtime unchanged,
  SELECT-only rehearsal, local commits only; no branch, worktree, push or deployment.

## 2026-10-18 - Checkpoint 11a: third batch exposes a real CR-001 difference

- Refreshed SELECT-only eligibility, excluded nine accepted students and selected
  36-, 50- and 35-row additional cases. Comments remain empty; CR-005 has no new
  row-count shape and uses a different longer Arabic source-name candidate.
- CR-001-B03 prints in both engines but fails parity: the page-two English heading
  is missing in Rust, shifting its body upward; Arabic name outlines also differ.
  All 36 ratings, image RGB, page/date/integrity and native replay checks pass.
- Preserved the failed pair and recorded all-page rejection. Two synthetic audit
  regressions verify detection; all 24 Python checks pass. No renderer fix claimed.
- Next two cases are pending. [Batch 03](coverage/additional-evaluation-batch-03.md)
  records findings and repair evidence. D: scratch/cache, capped sequential Docker,
  resource guards and local commits only; no branch/worktree or data/config changes.

## 2026-10-18 - Checkpoint 11b: third batch complete, two passes and one rejection

- CR-003-B03 and CR-005-B03 pass all-page comparisons: 50 and 35 ratings, 6,474 and
  4,960 painting glyphs, exact image RGB, within existing layout tolerances.
- CR-001-B03 remains rejected: missing page-two repeated heading, 21.5 pt body shift,
  and Arabic name-outline mismatches. All 36 source ratings and native replay match.
  No renderer fix claimed; frozen evidence is ready for the next focused repair.
- All 18 comparison images reviewed; all nine execution PDFs pass integrity checks.
  Thirty-one focused checks pass, including two new audit-rejection regressions and
  seven Chinese PDF tests. CR-005 covers Latin text stored in the Arabic-name field.
- Twelve distinct students compared: eleven accepted, one rejected, 86 eligible untested.
  [Batch 03](coverage/additional-evaluation-batch-03.md) records evidence and next repair.
- Final C: 313.34 GiB, D: 535.79 GiB free; RAM 6.01 GiB available, minimum sample 5.84 GiB.
  No containers or retired target volume; D: storage and caps preserved. SELECT-only SQL,
  no source-data/config/runtime changes, local commits only, no branch/worktree/push.

## 2026-10-18 - Checkpoint 12a: CR-001-B03 split header fixed

- Registered active group headers before their section walk and repeat only completed
  sections across a page break. Fixes the missing-prefix cause without duplicating a
  header moved before its first section.
- Two synthetic regressions cover repeat on/off, overflow, New Page Before/After,
  exact row positions and no duplication. Intended red failure reproduced; all 374
  layout tests pass in capped Docker. Formatting checked.
- Arabic discrepancy traced to Arial Bold `dlig` shaping; scoped repair still pending.
  Final pair is not yet regenerated or accepted. See [repair record](coverage/cr-001-b03-parity.md).
- Frozen data only; no SQL. D: cache/temp, RAM about 5.8 GiB available, C: 313 GiB free.
  Local recoverable checkpoint on existing main; no branches/worktrees/push/deployment.

## 2026-10-18 - Checkpoint 12b: Arabic ligatures repaired and candidate reviewed

- Exact native font-outline/GSUB evidence identifies optional Arabic `dlig`. Enabled
  an explicit Arabic-only policy consistently in layout measurement and PDF shaping;
  preserve logical Unicode and ordinary/Latin/Hebrew/CJK defaults. No data substitutions.
- Seven new synthetic actual-font/PDF regressions pass. Combined with pagination,
  Chinese, worker and audit suites, 542 focused checks pass. Existing pinned versions
  retained; new dependency edges verified with locked/offline Cargo.
- Candidate CR-001-B03 passes all 4,748 painted glyphs, all 36 ratings, images and
  geometry (max X/Y 0.05708/0.03016 pt). Independent all-six-page visual review accepts
  the heading, body, name and final content. Clean-source final pair/guard replays follow.
- Findings/reproduction: [CR-001-B03 repair](coverage/cr-001-b03-parity.md).
  D: cache/temp, two-CPU/2-GiB Docker; sampled RAM 5.21 GiB, C: 313.35 GiB free.
  Frozen data only, no SQL/deployment/branches/worktrees/push; local commits only.


## 2026-10-18 - Checkpoint 12c: CR-001-B03 final pair accepted

- Fresh original/Docker Rust PDFs from clean `8f9fd5e` pass integrity, all 36 ratings,
  all 4,748 painted outlines, exact image RGB, equal boxes/dates and all-six-page review.
  Page-two body residual is now -0.00004 to +0.00001 pt; max glyph X/Y 0.05708/0.03016 pt.
- Independent candidate visual review passed; final same-engine page PNGs are identical.
  CR-001-B02/003-B03/005-B03 guard replays pass and Rust PDFs remain byte-identical to
  their prior accepted exports (101 ratings, 14,159 painting glyphs). No new students.
- 542 focused checks, strict workspace clippy/formatting and final worker build pass.
  Twelve students now accepted, none rejected, 86 untested. Historical rejection and
  extraction/pixel/full-bridge limitations preserved in [repair record](coverage/cr-001-b03-parity.md).
- Final 17:04:41 resources: C: 313.14 GiB, D: 534.41 GiB free; RAM 6.24 GiB available,
  observed minimum 5.21 GiB. No running containers or retired target volume. D: cache/temp,
  frozen data/no SQL, no runtime/template/config changes; local commits only on main.

## 2026-10-18 - Checkpoint 13a: fourth evaluation batch selected

- Refreshed SELECT-only eligibility: 98 ready candidates; excluded all twelve previously
  captured students. Selected new 33-, 34- and 35-row cases, all with Arabic source names.
- Added reusable deterministic selection and six synthetic safety/coverage tests;
  all 30 Python audit/selection checks pass in capped Docker. No renderer change.
- Initial RAM 2.86 GiB held exports; recovered to 4.88 GiB. C: 312.73 GiB free; D: cache/temp.
  Actual pair/Chinese checks remain pending. [Batch 04](coverage/additional-evaluation-batch-04.md)
  records scope, evidence and selection limits. Existing main, local commits only.

## 2026-10-18 - Checkpoint 13b: CR-001-B04 comparison rejected

- All execution stages, PDF integrity, 33 ratings (17/16/0), native replay and images
  pass, but all-six-page reviews confirm a new stale-group header on Rust page two.
- The extra Arabic heading/empty column block shifts English rows down 42.2 pt and
  moves signatures onto page three. Geometry fails; case explicitly rejected.
- Retained snapshot/failed PDFs ready for focused footer/group-transition investigation;
  no root cause or renderer fix claimed. Six new selection tests and all 30 Python checks
  pass; they do not override rejection. Next two students remain pending.
- [Batch 04](coverage/additional-evaluation-batch-04.md); RAM 4.91 GiB/C: 312.74 GiB free.
  D: storage, bounded Docker, local committed checkpoint; no branches/worktrees/push.


## 2026-10-18 - Checkpoint 13c: fourth evaluation batch complete

- CR-003-B04/005-B04 accepted after both coordinator and independent all-page review:
  69 ratings and 9,731 painted glyphs match, max X/Y 0.06008/0.03016 pt, exact image RGB.
  Arabic names, boundary headings, sparse final page and signatures preserved.
- CR-001-B04 remains rejected for stale header/body/signature placement; all 33 ratings
  and name shapes match. Preserve its frozen pair for focused repair; no renderer fix.
- All nine execution PDFs pass integrity; native replay pixels and all 102 ratings match.
  Six new selection tests, 24 audit checks and seven Chinese tests pass (37 total).
  Fifteen distinct students compared: fourteen accepted, one rejected, 83 untested.
- [Batch 04](coverage/additional-evaluation-batch-04.md) records full findings/reproduction.
  Final C: 312.72 GiB/D: 534.36 GiB free, RAM 4.16 GiB available. Initial low RAM held exports
  until recovery; D: storage and caps retained, no running containers or old target volume.
  Read-only SQL and local commits only; no runtime/config/template/branch/worktree changes.


## 2026-10-18 - Checkpoint 14a: CR-001-B04 footer continuation fixed

- Retire a completed group's repeating header before its footer can start a new page;
  preserve live ancestors, multipart header continuation and all footer resolution scope.
- Three new synthetic regressions cover overflow/explicit/deferred breaks, signatures,
  multipart suppression and nested group scopes. Red test reproduced the extra page.
- All 396 focused Docker checks pass: 377 layout, seven Chinese PDF and twelve worker.
- Candidate fresh native/Rust pair matches 33 ratings and all 4,256 painted glyphs;
  42.2 pt body shift and stale heading are gone, signatures restored to page two.
  Coordinator and independent review accept all six candidate page images within unchanged
  limits. [Repair record](coverage/cr-001-b04-parity.md) retains evidence and reproduction.
- Final clean-source pair and prior-case replays pending. No SQL/template/config changes;
  local commits on main only. D: cache/temp, bounded sequential Docker; latest RAM 4.02 GiB
  available and C: 312.10 GiB free. No unrelated applications stopped.


## 2026-10-18 - Checkpoint 14b: CR-001-B04 final paired acceptance

- Fresh original Crystal/Docker Rust PDFs from clean `85ff5ce` match all 33 ratings and
  4,256 painted glyphs, max X/Y 0.05708/0.03016 pt. All six final pages reviewed; each
  reproduces the independently reviewed same-engine candidate pixels. Original native
  reference pixels and snapshot are preserved; heading/body/signature defects are closed.
- All 396 focused checks plus strict affected Clippy and workspace formatting pass.
  CR-001-B03, CR-001-B02, CR-003-B04 and CR-005-B04 replays pass every audit and their Rust
  PDFs remain byte-identical. Private timestamp formatting failure/retry is documented.
- [Full repair record](coverage/cr-001-b04-parity.md) records causes, tests, final evidence,
  residuals and scope. Coverage: fifteen accepted, none rejected, 83 eligible untested.
- Final C: 312.09 GiB/D: 534.01 GiB free, RAM 5.40 GiB available; lowest sample 4.02 GiB.
  D: cache/temp and sequential caps retained, no task containers/retired target volume.
  No SQL/template/production changes, no branches/worktrees/push; local commits only.


## 2026-10-18 - Checkpoint 15a: fifth evaluation batch prepared

- Refreshed SELECT-only inventory: 98 ready candidates; exclude all fifteen captured
  students, including repaired cases. Select three new 28/52/35-row cases with Arabic
  source names. Only the first adds a new row count; no populated comments are present.
- Reserve each selected ID before the next template, with historical coverage separate.
  Three new selection regressions pass; all 33 Python selection/audit checks pass in Docker.
- [Batch 05](coverage/additional-evaluation-batch-05.md) records selection, evidence,
  reproduction and limits. Actual PDF/Chinese/visual gates pending; renderer unchanged.
- Initial RAM 5.22 GiB/C: 312.09 GiB free; D: cache/temp and sequential resource caps.
  No SQL writes/runtime/template/config change. Local commits only, no branches/worktrees.


## 2026-10-18 - Checkpoint 15b: CR-001-B05 page-total/date-footer rejection

- Capture and both engines print; native replay pixels, all 28 ratings and decoded images
  match, with three physical pages. All-six-page review rejects printed totals/date footers.
- Native labels 1/2, 2/2, 1/1 versus Rust 1/3, 2/3, 1/3; Rust misses 38 date glyphs on each
  of pages two/three and raises legends/footers 16 pt. Body/name/signatures match.
- [Batch 05](coverage/additional-evaluation-batch-05.md) preserves the failed pair and exact
  findings. Root cause/renderer repair not claimed; investigate reset-aware totals and
  conditional footers. Other two students pending; resource caps/D: paths retained.


## 2026-10-18 - Checkpoint 15c: fifth batch complete, two accepted and one rejected

- CR-003-B05 and CR-005-B05 pass every page review, all 87 ratings and 11,383 painted
  glyphs, existing 0.061/0.031-pt limits, decoded RGB/date/boxes and native replay checks.
  All three cases print; all 115 ratings match. CR-001-B05 remains explicitly rejected
  for wrong page totals, missing date blocks and raised footers; no renderer repair claimed.
- [Batch 05](coverage/additional-evaluation-batch-05.md) records all outcomes, selection
  safeguards, exact remaining defect, private evidence/index and reproduction commands.
  All 40 focused tests pass: 33 Python checks (three new selection tests) and seven Chinese.
- Eighteen distinct students compared: seventeen accepted, one rejected, 80 untested.
  Prior accepted cases remain scoped to their captured inputs. Next repair is CR-001-B05.
- Final sample: C: 312.10 GiB/D: 533.98 GiB free, RAM 4.69 GiB available. All pre-export
  samples passed the resource guard; D: cache/temp, capped sequential Docker, no task
  containers or retired target volume. No database/template/runtime/configuration changes.
  Local commits on existing main only, no branches/worktrees/push or deployment.

## 2026-10-18 - Checkpoint 16a: CR-001-B05 page-total/footer repair

- Partition finalized pages into actual reset-delimited numbering sequences; use their
  totals for deferred footer conditions and earlier placed/embedded page-count runs.
  The retained case needs totals 2/2/1, restoring its suppressed 320-twip date band without
  an offset correction. B04 header retirement and body formula execution remain intact.
- Discard only fixups added during already-final footer emission, preventing embedded
  total substitution from rewriting matching literal text. Preserve earlier/currency fixups.
- Four new synthetic regressions reproduced reset totals and embedded literal corruption.
  All 400 Docker checks pass (381 layout, seven Chinese PDF, twelve worker); independent
  read-only source review has no blocking finding. Fresh pair/final acceptance pending.
- [Repair record](coverage/cr-001-b05-parity.md) documents cause, fix, red fixture correction,
  tests, scope and private evidence. D: cache/temp, sequential caps and resource guards;
  no SQL/template/production change, no branches/worktrees/push; local commits only.
- Fresh candidate original/Rust pair and independent all-page review now pass: 3,962
  glyphs, 28 ratings, dates/image RGB/page boxes, labels 1/2, 2/2, 1/1 and footer positions.
  Strict affected Clippy/workspace format pass. Final clean-source pair and five prior-case
  guards remain pending; candidate dirty provenance is explicitly retained.
