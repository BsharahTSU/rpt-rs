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
| CR-003-B02 | 41 | 17 | 27 / 23 | Pending |
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
