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
[Initial actual comparisons](initial-comparisons.md) now record three passing native/Rust representative exports, identical native live/replay images, matching ordered ratings and the remaining visual defects. The other 95 eligible students remain SQL-only evidence.


## Current repair checkpoint: CR-001 (2026-10-17)

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

CR-003 and CR-005 remain open and need their own comparisons after shared renderer fixes.
The other 95 eligible students remain SQL-only evidence. Unpublished standard/second-term cases,
missing authoritative databases/templates, external Gradebook, authenticated API/UI delivery,
BIFF/XLS and broad legacy/subreport coverage retain the gaps recorded in
[initial comparisons](initial-comparisons.md), [SQL findings](sql-eligibility.md) and
the [surface inventory](parent-surfaces.md). Do not infer their success from CR-001.

Keep the current resource controls in the CR-001 record: sequential bounded containers, one Cargo
build/test thread, private evidence and additional temporary files on the designated D: paths.
No new branches, worktrees, hosted fork or deployment are part of this checkpoint.
