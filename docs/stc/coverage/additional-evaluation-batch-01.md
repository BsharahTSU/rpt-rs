# Additional evaluation student batch 01

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
| CR-003-B01 | Smallest available row set | 20 | 27 / 23 | Pending export/comparison |
| CR-005-B01 | Longest available English name | 35 | 38 / 31 | Pending export/comparison |

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
are unchanged. Remaining batch cases are executed only after this first checkpoint.
