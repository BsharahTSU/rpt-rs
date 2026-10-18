# Additional evaluation student batch 02

## Completed batch

All three new students are accepted within the existing 0.061 pt X / 0.031 pt Y
layout limits. The engines agree on all 84 rating cells, 12,481 painting glyphs and
every decoded image RGB value. Every page was inspected. No renderer fix was needed;
the rating-audit omission described below was corrected and regression-tested.
Nine distinct students across three templates now have accepted comparisons;
89 of the current 98 eligible candidates remain untested.

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
| CR-005-B02 | 27 | 9 | 30 / 23 | Accepted within existing tolerances |

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
checks and four existing comparison checks; all 18 passed in capped Docker. Chinese PDF tests were pending at this first checkpoint; final results follow below.
Renderer sources are unchanged. Later cases were pending at this first checkpoint.

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

## Final results and validation

| Case | Pages each | Ordered ratings per page | Painting glyphs | Max X / Y delta (pt) | Export source |
| --- | ---: | --- | ---: | --- | --- |
| CR-001-B02 | 2 | 14 / 2 | 2,725 | 0.05552 / 0.03016 | clean `e5e6263` |
| CR-003-B02 | 3 | 17 / 19 / 5 | 5,568 | 0.06008 / 0.03016 | clean `154a587` |
| CR-005-B02 | 3 | 16 / 4 / 7 | 4,188 | 0.05894 / 0.03016 | clean `29d2d1e` |

All nine execution PDFs pass qpdf. Capture, native-live, native-frozen and rust-frozen
stages passed explicitly for every case; all have matching page boxes/dates. Native
live/frozen raster images are identical. All 16 native-frozen/Rust page images were
inspected. The third case preserves a sparse four-row Mathematics continuation on
page two and the native page break before Science on page three. It tests a different
content shape from its earlier 35-row cases without removing the original blank space.
Final signatures, bilingual notes, repeated footers and every 9,600-pixel logo agree.

The data gate now matches every source MarkAlias, including CR-003's six asterisks,
and checks per-page order between engines. The existing outline audit matches each
painting glyph exactly once on its page by outline/size within explicit tolerances.
The 30 / 85 / 56 outline-equivalent Unicode mapping differences remain recorded;
equivalent extracted text, PDF byte identity and whole-page pixel identity are not claimed.
Image checks, glyph checks, source-value checks and visual review remain separate gates.

**29 focused checks pass:** 14 glyph-audit, four comparison, four rating-audit and
seven actual Chinese PDF embedding/extraction tests. Eight synthetic tests were added
in this batch. The Chinese tests ran offline in capped Docker using the existing D:
cache. Renderer code is unchanged, so no full workspace rebuild was repeated.
The first tests checkpoint is `154a587`; the rating-audit fix is `29d2d1e`.

Reproduce the pure checks from `scripts/stc` in the development container:

```text
python3 -m unittest test_glyph_audit.py test_compare_case.py test_rating_audit.py
```

Use read-only source, a D: temp mount, `PYTHONDONTWRITEBYTECODE=1`, network disabled,
512 MiB and two CPUs. Run the actual Chinese tests with the existing Docker launcher:

```powershell
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @('test','--locked','--offline','-p','rpt-render','--test','chinese')
```

Private `verify-case.ps1`, `outline-review.py` and `audit-pair.py` in the batch evidence
root record this exact audit pipeline. `final-summary.json` records sources, run paths,
ratings, glyphs, image/replay results and resource readings; `comparison.html` links all
three paired PDFs and per-case page galleries. Each run retains input/artifact hashes,
execution evidence, qpdf/bbox/PNG outputs and `review/visual-review.json`. The execution
record's original `visual: pending` remains historical; the separate completed review
and final summary record acceptance. No private inputs, names, selectors or PDFs are tracked.

Final resource checkpoint at 16:02:28: C: **313.34 GiB free**, D: **535.82 GiB free**,
host RAM **6.47 GiB available**, no running containers, retired C-backed target volume
absent. C: free space was 313.34 GiB at both first and final readings. Available RAM at
recorded checkpoints stayed at or above 5.83 GiB; these samples do not claim a continuous
peak measurement. Export containers retained the two-CPU/2-GiB limits and D: target/temp.

All nine accepted manifests were checked for distinct students. The additional 89 ready
students, populated comments, overflowing displayed names, unavailable standard/second-term
and other-school cases, Excel, broader subreports/legacy callers and authenticated API/UI
delivery remain outside this acceptance. Only SELECT discovery and the reviewed SELECT-only
evaluation procedure ran. No report flags, source data, production config or Parent runtime
changed. Local commits only; no new branch, worktree, push or deployment.
