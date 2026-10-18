# Parent Crystal PDF coverage

## Scope and acceptance

Inventory every current Parent App Crystal report path and configuration branch. Discover eligible
student/report/term slots from SQL without changing publication or access rules. Select representative
cases per template/recipe, including meaningful language and pagination variants. Run cases separately,
with an original Windows Crystal baseline and a Docker Linux Rust candidate.

Do not execute an unreviewed procedure. In particular, the documented legacy GOPS marks loader can
update report selectors. Confirm deployed procedure behavior before calling it. Database discovery
uses metadata/SELECT queries only. Record unavailable databases/templates and absent eligible rows as
coverage gaps, not renderer failures or passes.

## Per-case gates

1. `discovered`: current Parent route/catalog/template branch established from source.
2. `eligible`: matching student course, school/class options, report/term metadata and publication
   satisfy the current backend rules; record parent ownership/HideMarks policy separately.
3. `native-pdf`: original bridge returned success and a non-empty structurally valid PDF.
4. `rust-pdf`: Docker bridge returned success without forbidden fidelity diagnostics and a valid PDF.
5. `content`: same template, selectors, parameters and data established; compare extracted text/values,
   page sizes/counts, images, dates, repeated groups and footer conditions.
6. `visual`: inspect rendered pages individually; record each defect. A matching page count alone is insufficient.
7. `accepted`: all applicable checks pass with documented tolerances. Pixel identity is a separate stricter claim.

An offline eligibility/export check never establishes authenticated API ownership or UI behavior.
Do not label native and Rust as using identical data unless a captured-data replay or a verified stable
snapshot establishes that. Flag independent live queries as a comparison limitation.

## Evidence and reproducibility

Shared Markdown uses opaque case IDs. Private manifests under ignored `research/stc-verification/`
map cases to database/student selectors, RPT path/hash, recipe, parameter types, culture/date format,
font inputs, engine commits/versions, timestamps, status and artifact hashes. Never serialize passwords
into a tracked manifest. Remove temporary credential-bearing requests in `finally` blocks.

Run a bounded case, verify it, update the ledger, then proceed. Keep failed output diagnostic logs local.
New renderer fixes require a synthetic regression and a rerun of the affected case; do not repeat an
unchanged full suite on every case. Chinese regression coverage remains part of renderer validation.

## Inventory status

Source and SQL discovery are complete for the documented local scope: 38 AAJ case groups, three
ready evaluation groups and 98 candidate students. Other local copies are metadata-only evidence;
external Gradebook and authenticated API/UI delivery remain open. Read the [surface inventory](parent-surfaces.md),
[SQL findings](sql-eligibility.md), [isolated runner](case-runner.md), and [clock fix](clock-parity.md).
[Initial actual comparisons](initial-comparisons.md) preserve the original three exports and their then-open defects; the subsequent repair/acceptance records supersede those defects. After the [CR-001-B05 repair](cr-001-b05-parity.md), eighteen students have accepted comparisons, none is currently rejected, and 80 eligible students remain SQL-only evidence.


## Current comparison checkpoint: CR-001-B05 repaired and accepted (2026-10-18)

The [repair](cr-001-b05-parity.md) scopes page totals to actual numbering sequences and
restores conditional date sections without an offset correction. Fresh original/Docker
Rust PDFs from clean `0e6476a` print 1 of 2, 2 of 2, 1 of 1 and match all 3,962 painted
glyphs, 28 ratings, date/footer positions and decoded RGB. All six final pages reviewed.
Four new regressions, 400 focused checks and strict affected lint/format pass; five prior
accepted Rust PDFs remain byte-identical. Coverage: eighteen accepted, none rejected,
80 untested. D: cache/temp; final C: 311.40 GiB free and RAM 5.96 GiB available. No SQL,
template or production configuration changes. Another bounded student batch is next.

## Previous comparison checkpoint: batch 05 has two passes and one rejection (2026-10-18)

[Batch 05](additional-evaluation-batch-05.md) compares three new 28-, 52- and 35-row
students sequentially. CR-003/005-B05 pass independent all-page, glyph/data/image/date
checks; CR-001-B05 is rejected for wrong reset-aware printed page totals, missing dates
on pages two/three and legends/footers raised 16 pt. Body, Arabic names and signatures match.
Root cause and renderer repair are not claimed. All 115 ratings match; 40 focused checks
pass, including three new selection regressions and seven Chinese PDF tests.
Coverage: seventeen accepted, one rejected, 80 untested. D: cache/temp; final C: 312.10 GiB
free and RAM 4.69 GiB available. Repair CR-001-B05 from the retained frozen pair next.

## Previous comparison checkpoint: CR-001-B04 repaired and accepted (2026-10-18)

The [repair](cr-001-b04-parity.md) retires the completed group's repeat entry before
footer pagination while preserving live ancestors and footer resolution scope. Fresh
native/Docker Rust PDFs from clean `85ff5ce` match all 4,256 painted glyphs and 33 ratings
within existing limits; the stale heading/body shift is gone and signatures are on page two.
All six final pages reviewed. All 396 focused checks pass, including three new synthetic
regressions and seven Chinese PDF tests; four prior accepted Rust PDFs remain byte-identical.
Coverage: fifteen accepted, none rejected, 83 untested. D: cache/temp; final C: 312.09 GiB
free and RAM 5.40 GiB available. No SQL/template/production configuration changes.

## Previous comparison checkpoint: batch 04 has two passes and one rejection (2026-10-18)

[Batch 04](additional-evaluation-batch-04.md) compares three new 33-, 34- and 35-row
students sequentially. CR-003/005-B04 pass all-page and glyph/data/image checks;
CR-001-B04 is rejected for a stale Arabic heading/empty column block on page two,
a 42.2 pt body shift and signatures moved onto page three. Frozen evidence is retained
for focused repair; no renderer fix is claimed. All 102 ratings match, including the
rejected case. Six new selection guards plus existing audits and Chinese total 37 checks.
Fifteen students compared: fourteen accepted, one rejected, 83 eligible untested.
D: cache/temp, final C: 312.72 GiB free and RAM 4.16 GiB available. Local commits only.

## Previous comparison checkpoint: CR-001-B03 repaired and accepted (2026-10-18)

The [repair](cr-001-b03-parity.md) fixes multipart group-header continuation and opts
legacy Arabic shaping into font-defined discretionary ligatures in both measurement
and PDF drawing. Fresh original/Rust PDFs from clean `8f9fd5e` match all 36 ratings and
4,748 painted glyphs, with max X/Y 0.05708 / 0.03016 pt and exact image RGB. All six
final page images were reviewed. No acceptance limits were widened.
542 focused checks pass (nine new renderer regressions included), as do strict workspace
lint/format checks. Three prior accepted Rust PDFs remain byte-identical after replay.
Coverage at that checkpoint: twelve accepted, none rejected, 86 untested. C: 313.14 GiB free,
RAM 6.24 GiB available; D: cache/temp, bounded Docker and local commits only.

## Previous comparison checkpoint: third batch had two passes and one rejection (2026-10-18)

[Batch 03](additional-evaluation-batch-03.md) tests new 36-, 50- and 35-row students.
CR-003-B03 and CR-005-B03 pass all-page review, captured-rating and glyph/image checks.
CR-001-B03 prints all 36 ratings but Rust omits a repeated page-two group heading,
shifts following rows upward by 21.5 pt and differs in Arabic name outlines. This is a
confirmed failed comparison, preserved for repair; no renderer fix is claimed.
Thirty-one audit/Chinese checks pass, including two new synthetic rejection regressions.
Twelve distinct students have been compared: eleven accepted, one rejected, 86 untested.
C: has 313.34 GiB free, RAM 6.01 GiB available; D: cache/temp and bounded Docker remain.

## Previous comparison checkpoint: second batch accepted (2026-10-18)

[Batch 02](additional-evaluation-batch-02.md) adds new 16-, 41- and 27-row students.
All 84 ratings, 12,481 painting glyphs and decoded image RGB values agree within the
existing limits; all 16 compared page images were reviewed. The six valid asterisk
ratings in CR-003 exposed a letter-only audit omission, now fixed by preserving all
column tokens and checking captured-source multiplicity. No renderer change was needed.
Eight new synthetic regressions bring this batch's focused checks to 29 including seven
Chinese PDF tests. Nine students are accepted; 89 eligible candidates remain untested.
D: cache/temp and capped sequential Docker kept C: free at 313.34 GiB; final RAM was
6.47 GiB available. Populated comments and overflowing displayed names remain open.

## Previous comparison checkpoint: first three additional students accepted (2026-10-18)

[Batch 01](additional-evaluation-batch-01.md) adds one student per template, covering
39 rows, a sparse 20-row/two-page report, and a different long-source-name candidate.
All 94 ordered ratings, 13,272 painting glyphs and decoded image pixels agree within
existing tolerances; all 16 compared page images were reviewed. Ten new audit tests,
four existing comparison tests and seven Chinese PDF tests pass. A zero-area space
contour false positive in the audit was corrected; no renderer change was required.
Six distinct students are now accepted, leaving 92 eligible candidates untested.
Populated comments and overflowing displayed names are not covered by this batch.

## Previous comparison checkpoint: CR-005 logo repaired (2026-10-18)

The [generic logo-color fix](cr-005-logo.md) is committed in `eb4656d`. Fresh original
Crystal and Docker Rust PDFs from that clean source pass integrity and all-page review.
All 9,600 decoded logo RGB pixels match exactly. Drawing streams, fonts, other images and
Page IR remain identical to accepted pre-fix Rust output. The three pages retain all 35
ratings (16 / 12 / 7), 59 English lines and 4,918 glyphs within existing layout tolerances.

CR-001/CR-003 frozen replays also match native logo RGB exactly, preserve prior drawing/font
resources and IR, and retain their 34/52 ratings. Their original accepted native references
were reused; only CR-005 received newly generated outputs from both engines this follow-up.

189 affected tests plus six doctests pass, including seven Chinese checks. Strict workspace
Clippy, formatting, worker build and independent source/PDF review pass. The opted-in STC
BMP path uses measured palette/lookup rules with deterministic fallback for native-unwritten
cells. Default output and other raster formats are preserved. No full-bridge or whole-page
pixel-identity claim follows from these captured cases.

## Previous repair checkpoint: CR-003 (2026-10-17)

[CR-003 acceptance](cr-003-parity.md) records the fresh native/Rust pair from clean
`899ca64` and [native findings/fixes](cr-003-whitespace.md). All 6,449 glyphs,
72 English lines and 52 ordered ratings match; all six page PNGs pass independent
review. The captured case is accepted below 0.061 pt X / 0.031 pt mark Y, with
ordinary baselines within 0.00005 pt and no row drift. The CR-001 replay is byte-identical.

479 focused Docker tests and 13 minimal text tests pass, including six Chinese
tests and synthetic hinted-font fitting/guard tests. Strict Clippy/formatting and
worker build pass. Original template/data remain unchanged; no SQL or deployment.
The first candidate's regressions and their measured corrections remain documented.

## Previous repair checkpoint: CR-001 (2026-10-17)

The [CR-001 repair and comparison record](cr-001-parity.md) is the current acceptance ledger.
The original case is replayed from captured data, with Windows Crystal as the reference and Linux
Docker as the Rust renderer. The repair is implemented through clean commit `be022bd`; both
PDFs have been regenerated and pass integrity, page/date and rating checks. Independent final
all-page/operator review passes without blockers. The captured CR-001 case is accepted within
the measured tolerances below; this status does not extend to other students or templates.

| Repair area | Detailed evidence and scope |
| --- | --- |
| Saved printer DPI, multiline pitch and scoped first ascent | [Printer metrics](cr-001-metrics.md); printer ascent for static/explicit RTL/known RTF and growing StandardText with actual shaped RTL; fixed/English, library-default and PDF-cell guards remain |
| Matching Basic-Latin measurement and drawing | [Spacing](cr-001-spacing.md); opt-in unkerned ASCII ranges, with advanced shaping preserved |
| Signature borders, standalone rules and rounded frames | [Geometry](cr-001-geometry.md); measured guards and per-fragment half-stroke handling |
| Explicit text-paragraph RTL, including mixed paragraphs | [Arabic direction](cr-001-arabic.md); decode stored metadata, preserve old-model inheritance |
| Native Arabic mark-offset convention | [Current comparison record](cr-001-parity.md); opt-in PDF compatibility, ordinary positioning unchanged |
| Trailing-space centering and emitted text | [Whitespace](cr-001-whitespace.md); narrow centered-LTR field trim, with static trailing glyphs retained |
| Final shaped alignment width | [Text origins](cr-001-text-origins.md); integer PDF-width then per-glyph twip rounding for centered/right zero-spacing runs |

Current focused Docker verification passes 486 tests plus 12 minimal-feature text tests,
including five Chinese PDF tests, strict workspace Clippy and formatting. The prior full
workspace run of 1,630 tests is recorded separately; it is not a new full run for these changes.
The fresh pair has three 595 x 841 pt pages, the same date and 34 ordered ratings allocated
17 / 17 / 0. Independent final measurements match all 4,467 glyph outlines and sizes in 283 runs,
including 216 aligned runs, with maximum differences 0.05708 pt X and 0.03016 pt Y at existing
marks. Ordinary baselines are within 0.00005 pt; both formerly displaced footer runs match.
All 63 English line breaks are preserved. The CR-001 tolerance is below 0.06 pt X / 0.031 pt Y;
the remaining curve-endpoint difference is at most 0.05005 pt, and 42 CMap differences map to
identical outlines. The coordinator and independent reviewer inspected all six fresh page PNGs;
fresh per-engine pixels, glyph data and geometry reproduce the reviewed candidate, as does Rust IR.
This is accepted captured-case parity with documented residuals, not pixel identity.

CR-003 retains its recorded text/layout acceptance. CR-005 passes text/layout and exact decoded-logo matching above; CR-001/003 replays close their same image residual.
After the [CR-001-B05 repair](cr-001-b05-parity.md), eighteen students have accepted comparisons, none is currently rejected, and 80 eligible students remain SQL-only evidence. Unpublished standard/second-term cases,
missing authoritative databases/templates, external Gradebook, authenticated API/UI delivery,
BIFF/XLS and broad legacy/subreport coverage retain the gaps recorded in
[initial comparisons](initial-comparisons.md), [SQL findings](sql-eligibility.md) and
the [surface inventory](parent-surfaces.md). Do not infer their success from the three captured text/layout comparisons.

Keep the current resource controls in the CR-005 record: sequential bounded containers, one Cargo
build/test thread, private evidence and additional temporary files on the designated D: paths.
No new branches, worktrees, hosted fork or deployment are part of this checkpoint.
