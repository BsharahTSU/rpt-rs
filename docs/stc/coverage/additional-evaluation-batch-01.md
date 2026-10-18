# Additional evaluation student batch 01

## Completed batch

All three new students are accepted within the existing measured layout tolerances.
Original Windows Crystal and Docker Linux Rust agree on 94 ordered ratings, all
13,272 painting glyph outlines/sizes and every decoded image RGB value. Every page
was reviewed. No renderer repair was necessary; the audit correction below is covered
by synthetic tests. Six distinct students now have accepted comparisons across the
three evaluation templates; 92 of the 98 eligible candidates remain untested.

## Scope and selection (2026-10-18)

The user requested a small batch of additional eligible students, comparing original
Windows Crystal with Docker Linux Rust one case at a time. The refreshed SELECT-only
AAJ inventory still has 98 ready candidates across CR-001/003/005. Excluding the three
already accepted students leaves 95. No publication, access or database data is changed.

Choose three distinct additional students, one per template. Private selectors and
all generated evidence live in ignored `research/stc-verification/additional-evaluation-batch-01/`.
Scratch uses `D:\tmpppppp123\rpt-rs-additional-batch-01`; the build cache remains on D:.

| Opaque case | Selection | Source rows | English/Arabic name lengths | Status |
| --- | --- | ---: | --- | --- |
| CR-001-B01 | Largest available row set; long English name breaks ties | 39 | 37 / 26 | Accepted within existing tolerances |
| CR-003-B01 | Smallest available row set | 20 | 27 / 23 | Accepted within existing tolerances |
| CR-005-B01 | Longest available English name | 35 | 38 / 31 | Accepted within existing tolerances |

Name lengths describe source fields, not a claim that the template prints the complete
field. All eligible candidates have empty course/general comments; populated-comment
coverage remains open. The private aggregate query initially used SQL Server's reserved
`RowCount` alias without brackets; the rejected SELECT was corrected before selection.

The unchanged evaluation procedure definition was read and its SHA-256 rechecked before
execution. Its reviewed branch is SELECT-only. Each case captures scoped typed data,
prints the existing native live route, then prints native and Rust from the identical
schema-bearing snapshot. Live/frozen native raster equality is checked separately.
Standard report procedures that write data are excluded from this live rehearsal.

## First case checkpoint

CR-001-B01 exported from clean local `7989b3b` using the unchanged worker/bridge. All
three PDFs pass qpdf, use three 595x841 pt pages, and have matching dates. Native live
and frozen pages are pixel-identical. Native frozen/Rust agree on all 39 ordered
ratings (17 / 17 / 5), every embedded image's decoded RGB, and all 5,060 painting glyph
outlines and sizes. Maximum glyph displacement is 0.05708 pt X / 0.03016 pt Y, within
the previously accepted 0.061 / 0.031 pt limits. Every native/Rust page was inspected;
no new clipping, wrapping, missing content, footer or logo defect was observed.

Native and Rust text extraction still differ at outline-equivalent Unicode mappings.
This is visual/layout acceptance with measured residuals, not PDF byte identity or a
new promise of equivalent extracted text. Authenticated Parent API delivery is untested.

## Audit finding and synthetic regression tests

The old private outline audit counted a Microsoft Sans Serif space as painting because
its embedded TrueType glyph contains one contour. Inspection showed that this contour
has exactly one point at (0,0) and a zero-area bounding box. Three extra line-edge native
space operators therefore produced a false visible-glyph mismatch. All other glyphs
already matched within the existing tolerances. No renderer change was needed.

`simple_glyph_outline` in [glyph_audit.py](../../../scripts/stc/glyph_audit.py) now decodes
the actual coordinates and treats zero-width/zero-height outlines as nonpainting. It
does not discard glyphs based on Unicode whitespace or fitted report-specific hashes.
Visible punctuation and marks remain subject to the same audit. Canonical outlines
ignore hint instructions; composite resolution remains the caller's responsibility.
The private copy of the existing outline extractor uses this tested helper; original
comparison evidence and scripts remain available.

`compare_outline_pages` checks page counts and per-page outline/size multiplicity, then
matches each painting glyph once within explicit X/Y tolerances. It does not infer
image, vector-path, semantic-text or all-page visual acceptance. Private pair checks
separately cover ordered ratings, decoded images, dates, page boxes and native replay.

Synthetic tests cover one-point and line-only outlines, real coordinate bounds, visible
outlines, ignored hints, malformed/composite data, Unicode aliases, Chinese glyph loss,
duplicate/substituted glyphs, wrapping/page movement, size changes, mark displacement,
empty reports and invalid numeric tolerances. Run the pure checks in capped Docker:

```text
python3 -m unittest test_glyph_audit.py test_compare_case.py
```

Run from `scripts/stc` with the repository read-only, `PYTHONDONTWRITEBYTECODE=1`, D:
temporary storage and the existing development image. There are 10 new audit checks
and four existing comparison checks. Rust renderer sources and production configuration
are unchanged. The first case/audit checkpoint was committed as `56a670e` before
running the second and third students.

## Final case results

| Case | Pages per engine | Ordered ratings per page | Painting glyphs | Max X / Y difference (pt) | Export source |
| --- | ---: | --- | ---: | --- | --- |
| CR-001-B01 | 3 | 17 / 17 / 5 | 5,060 | 0.05708 / 0.03016 | clean `7989b3b` |
| CR-003-B01 | 2 | 16 / 4 | 3,282 | 0.06008 / 0.03016 | clean `56a670e` |
| CR-005-B01 | 3 | 16 / 12 / 7 | 4,930 | 0.05894 / 0.03016 | clean `56a670e` |

Each case passed capture, original live export, original frozen export and Docker
Rust frozen export. All nine PDFs pass qpdf; page boxes/dates agree within each case.
Native live/frozen raster pages are identical, independently confirming replay for
this data. All 16 native-frozen/Rust page images were reviewed, including the two-page
sparse case, continuation headers, different ratings, signatures and repeated footers.
Every image's decoded RGB agrees, including each 9,600-pixel logo.

The first case adds five rows relative to its earlier representative, including a
third-page Science section. The second reduces the earlier 52-row, three-page sample
to 20 rows and two pages, testing different page totals and final-footer placement.
The third validates a different student and rating distribution. Its selected long
full-name field is shortened by the existing report formula, so this is not evidence
of handling an overflowing long displayed name. Populated comments and additional
pagination/content shapes remain open.

The 46 / 53 / 62 outline-equivalent Unicode mapping differences remain visible in the
raw extraction diagnostics. They do not hide missing painting glyphs: matching is
one-to-one on each page, includes marks, and uses the same explicit 0.061 pt X /
0.031 pt Y limits for every glyph. Native/Rust whole-page raster equality and PDF byte
identity are not claimed. The zero-area space correction changes audit counting only;
counts in older investigation records used their then-current audit and are not directly
comparable without rerunning it.

Validation: **21 focused checks passed** (ten new synthetic glyph-audit tests, four
existing comparison tests and seven actual Chinese PDF embedding/extraction tests).
The seven Chinese tests ran offline in capped Docker from the existing D: cache.
No unchanged full workspace build was repeated for this audit-only code change.

Docker report exports were sequential with two CPUs and a 2 GiB cap; rendering/comparison
checks used 512 MiB. Original Crystal exports ran sequentially on Windows. Temp and cache
remain on D:. Final health at 15:01:14: C: 313.56 GiB
free, D: 535.85 GiB free, host RAM 4.96 GiB available, no running containers. The lowest
recorded available RAM during these checkpoints was 4.70 GiB; the next export was guarded
against starting below 3 GiB. The retired C-backed target volume remains absent.

Private `final-summary.json` records run directories, source revisions, checks and
resource readings. `comparison.html` links each per-case comparison and original/Rust
PDF. Each run retains source/template/snapshot/binary hashes, execution evidence, qpdf
logs, raster pages, glyph details, image checks and `review/visual-review.json`.
No student selectors, names, raw PDFs, credentials or school templates are committed.

Only read-only discovery and the reviewed SELECT-only evaluation procedure ran. Parent
access/publication, production rendering and configuration were not changed. Local
commits only; no branch, worktree, push or deployment. Next work is additional content
shapes and the separately blocked standard-template/other-school/Excel/subreport and
authenticated delivery coverage, not another fix for these accepted three samples.
