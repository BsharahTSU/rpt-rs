# Additional evaluation student batch 02

## Scope and selection (2026-10-18)

Test three additional eligible students sequentially, one each for CR-001/003/005.
The refreshed SELECT-only AAJ inventory has 98 ready candidates; excluding the six
previously accepted distinct students leaves 92 before this batch. The three selected
students are distinct and new. Their row counts differ from both earlier samples of
the same template. No publication, access, database data or production config changes.

| Opaque case | Source rows | Subjects | English/Arabic source name lengths | Status |
| --- | ---: | ---: | --- | --- |
| CR-001-B02 | 16 | 8 | 30 / 23 | Accepted within existing tolerances |
| CR-003-B02 | 41 | 17 | 27 / 23 | Accepted within existing tolerances |
| CR-005-B02 | 27 | 9 | 30 / 23 | Pending |

Selection prefers an untested row count per template, with longer Arabic then English
source names breaking ties. The source-name field is shortened by the template formula;
this does not establish overflowing displayed-name coverage. Candidate comments remain
empty. The unchanged evaluation procedure definition was hashed and checked against the
reviewed SELECT-only version before execution. Mutating standard-report procedures are
excluded from this live rehearsal. Discovery does not enable any blocked report group.

Private selectors, feature aggregates, plans, snapshots and PDFs remain in ignored
`research/stc-verification/additional-evaluation-batch-02/`. Scratch is
`D:\tmpppppp123\rpt-rs-additional-batch-02`; target cache and temporary mounts stay on D:.
The first selection attempt failed while assembling exclusion paths in PowerShell;
parenthesizing each path corrected it before selection or export. Six prior students
were then explicitly verified as distinct and excluded.

## First case checkpoint

CR-001-B02 exported from clean `e5e6263`: capture, native-live, native-frozen and Docker
Rust-frozen all passed. All three PDFs pass qpdf with two pages and equal page boxes
and print dates. Native live/frozen raster pages are identical. Native frozen/Rust
agree on 16 ordered ratings (14 / 2), all decoded image RGB and 2,725 painting glyphs.
Maximum X/Y displacement is 0.05552 / 0.03016 pt, within existing 0.061 / 0.031 pt limits.
All four compared page images were inspected: the sparse English-only content,
page-one Grammar heading, continuation, final signatures and footer are preserved.
No renderer repair was needed. Thirty outline-equivalent Unicode mappings differ;
text-extraction identity and whole-page pixel identity are not claimed.

Four new synthetic audit tests guard sparse continuation/final-footer completeness,
Chinese glyphs swapped between equal-count pages, visible outlines carrying whitespace
or format-character mappings, and extra blank pages. They use independent fixtures;
no student data or report-specific hashes enter tests. There are now 14 glyph-audit
checks and four existing comparison checks; all 18 passed in capped Docker. Chinese PDF tests will run after the batch.
Renderer sources are unchanged. Later cases remain pending at this checkpoint.

Docker exports use two CPUs and 2 GiB; comparison containers use 512 MiB, sequentially.
A 3 GiB available-RAM guard runs before each case. Initial C: free 313.34 GiB,
D: 535.85 GiB, RAM 5.91 GiB; after the first case C: 313.34 GiB, RAM 5.83 GiB.
Local commits only, on the existing branch; no worktree, branch, push or deployment.

## Second case and rating-audit correction

CR-003-B02 exports from clean `154a587` pass integrity and all-page review: three pages,
41 ordered ratings (17 / 19 / 5), 5,568 painting glyphs and exact decoded image RGB.
Maximum displacement is 0.06008 pt X / 0.03016 pt Y. Eighty-five outline-equivalent
Unicode mappings differ. All six compared page images preserve wrapped content,
Arabic/English/Math/Science grouping, continuation headers, signatures and footers.
Original live/frozen raster equality also passes; no renderer change is required.

The prior private rating audit used a letter allowlist (O/M/C/S/N). It reported only
35 ratings because six source rows have MarkCode 99 / MarkAlias `*`. Both engines
already displayed all six correctly. The new reusable [rating audit](../../../scripts/stc/rating_audit.py)
keeps every token wholly inside the explicitly supplied rating-column bounds, including
symbols and unexpected values. It compares per-page ordered sequences between engines
and both value multisets against the captured snapshot. Thus identical omissions in
both engines cannot pass. Snapshot order itself is not assumed to be display order.

The first attempt to remove the allowlist used only word-start bounds and included an
Arabic column heading and a signature parenthesis. Requiring the entire word box inside
the column excludes these crossing tokens. Regression fixtures include both cases.
The snapshot gate caught this issue before acceptance; it was not a PDF rendering defect.
Bounds are explicit for the reviewed templates (300 < xMin, xMax < 320, yMin < 760 pt),
not a universal report detector. A wider future alias will fail the snapshot gate and
require reviewed bounds rather than disappearing silently.

Four additional synthetic tests cover symbol/unknown extraction, heading/signature/legend
exclusion, reordered snapshot rows, missing/duplicated/unexpected values shared by both
engines, page/order movement and empty data. All 22 Python checks pass in capped Docker
(14 glyph, four comparison, four rating). Only the affected data audit was rerun on the
existing two PDF pairs; both now match every captured alias. Original PDFs are unchanged.
