# CR-005 comparison and regression coverage

## Current status

**Captured-case text/layout and exact decoded-logo color matching pass.** The
[logo repair](cr-005-logo.md), committed in `eb4656d`, reproduces all 9,600 logo RGB
pixels while preserving drawing streams, fonts, other images and Page IR. Both PDFs
were freshly generated from that clean source and all six page PNGs reviewed. Existing
layout tolerances remain; this is not whole-page pixel identity or full bridge coverage.

The findings below preserve the earlier text/layout checkpoint and its then-open
image investigation. The subsequent repair supersedes those image limitations for
these captured cases; see the final follow-up at the end of this record.

The unchanged original template and frozen schema-bearing DataSet are replayed
through Windows Crystal and Linux Docker Rust. No SQL is executed. Parent behavior,
configuration, publication, report inputs and deployment remain unchanged. Work uses
the existing branch and local commits only.

## Findings

The fresh baseline from clean `bf34df6` contains three 595 x 841 pt pages per
engine, the same print date and 35 ordered ratings allocated 16 / 12 / 7.
Coordinator and independent review cover all six rendered page PNGs. Continued
headers, the group starting page three, signatures and final-page date agree.

The full operator/embedded-outline audit excludes no glyph or text run:

| Check | Result / maximum difference |
| --- | --- |
| Nonblank glyph outlines and sizes | 4,918 agree: 2,108 / 1,546 / 1,264 |
| Text runs / aligned runs | 301 / 218, all mapped |
| English line content and breaks | 59 agree: 25 / 20 / 14 |
| Ordered ratings | 35 agree: 16 / 12 / 7 |
| All glyph X origins | 0.05894 pt |
| English glyph X origins | 0.02748 pt |
| Ordinary text baselines | 0.00005 pt |
| Rating baselines | 0.00004 pt |
| Arabic mark Y rounding | 0.03016 pt |
| Six stroked rounded frame bounds | Exact |
| Five filled header bounds | 0.00001 pt |
| 118 horizontal path endpoints | 0.05004 pt |

The text/layout envelope is below 0.06 pt X, 0.031 pt mark Y and 0.051 pt path
rounding. Sixty-two Unicode mapping differences pair identical embedded glyph
outlines/sizes. Raw extracted-word equality remains false; matching visible text
and geometry do not imply PDF byte identity or identical Unicode extraction.

## Shared logo residual

Decoded logo pixels differ in all three captured cases, including the earlier
CR-001 and CR-003 accepted comparisons. This extends their residual inventory;
their text/layout results remain valid. It must not be hidden by an image mask or
folded into the text tolerances.

Read-only compound-file inspection establishes that the selected `Ole10Native`
image is a 96 x 100, 24-bit BMP. Rust's decoded PDF RGB pixels exactly equal its
source pixels. The embedding has no alternate CONTENTS/OlePres representation;
the other two embeddings contain unrelated 70 x 33 bitmaps. The original template
hash remains unchanged.

Crystal exports this image with a 256-entry indexed palette containing 219 distinct
palette values and 185 used colors, reducing 2,358 source colors. Each original RGB
color maps consistently to one native color; no position-dependent dithering is
observed. This establishes lossy export-time color reduction, not a Rust image
selection/decoding failure. The palette-generation algorithm is not established.

| Per-channel decoded RGB difference (0-255) | R | G | B |
| --- | ---: | ---: | ---: |
| Mean absolute difference | 1.36135 | 1.31750 | 4.24479 |
| Maximum difference | 28 | 32 | 38 |

Native's dominant background is (254,254,250), while the original/Rust dominant
background is (255,255,255). The difference is small at report scale, but exact
logo-color matching remains unresolved. No report-specific palette, approximate
quantizer or alteration of the original image is committed. Overall exact matching
is not marked complete.

## Regression coverage

Two new synthetic tests cover the shared behavior now verified by CR-005:

- Combined continuation and forced next-group pagination: two repeated header
  sections, unique ordered row identities, three pages with 4 / 1 / 2 rows,
  finalized Page N of M, stable footer position and final-page-only date label.
  This catches dropped/duplicated rows, repeated forced breaks and phantom pages.
- Simplified and Traditional Chinese terminal U+0020/LRM handling: narrow and
  full-line widths, leading spaces, three tail orders, unchanged visible line
  breaks and exact preservation of all logical characters. It guards the previous
  fitting fix outside ASCII without storing school data or proprietary fonts.

No private template, values or PDF becomes a committed fixture. Existing Chinese
PDF embedding/extraction tests continue to exercise the actual PDF path. Focused
validation results and final regeneration provenance follow in the next checkpoint.

## Evidence and reproduction

Use [the isolated case contract](case-runner.md). Ignored evidence lives under
`research/stc-verification/cr-005-parity/` on D:. `baseline/` contains freshly
regenerated native/Rust PDFs, IR, PNGs, comparison and full review. `native-metrics/`
contains original embedding inventories, decoded-pixel mapping and logo findings.
The original frozen input remains in `research/stc-verification/parent-cases/cr-005/`.
Private manifests retain template/snapshot hashes, selectors and engine identities.

Baseline regeneration ran 2026-10-18 10:21:14-10:21:43 local from clean `bf34df6`.
Both original and Rust were regenerated; the original template and snapshot were
not modified. No live connection is needed for replay.

## Resources and remaining scope

Temporary files use `D:\tmpppppp123\rpt-rs-cr005`; ignored evidence also stays on D:.
Docker jobs run sequentially with 2 GiB RAM, no additional swap, two CPUs, one Cargo
build job and one test thread. The runner checks C: >=30 GiB, D: >=10 GiB and host
available RAM >=2 GiB before heavy work. Existing images/volumes are reused; their
managed storage is not claimed to have moved off C:.

Initial readings: C: 285.87 GiB free, D: 561.41 GiB free, available host RAM 5.66 GiB
with Docker stopped. Resource checkpoints are retained privately. No pruning,
volume removal, process termination or global WSL setting change is part of this work.

Exact logo quantization needs a separately proven generic implementation and
synthetic native controls before it can be claimed compatible. Other students,
unpublished standard reports, template-owned SQL, BIFF/XLS, broad subreport/legacy
coverage and authenticated Parent API/UI delivery remain separate open gates.

## Verified tests-only checkpoint

All 430 affected Docker tests pass: 372 layout and 58 render, including seven
Chinese tests and six render documentation tests. The two added regressions also
passed in isolation; they are not added again to that total. The existing public
PDF reference tests pass without baseline updates. Strict all-target/all-feature
Clippy for both affected crates and workspace format checking pass. Independent
review of both new tests found no blocking issue.

The resource guard stopped linting before it started when host available RAM
reached 1.60 GiB. With no running containers, only idle Linux filesystem cache was
released; no files or applications were removed. Available RAM recovered to
3.94 GiB after linting, with C: 286.02 GiB free. This checkpoint changes tests and
Markdown only. The worker still uses the already verified shared renderer fixes.

## Final regeneration and independent comparison

Both engines were freshly regenerated from clean local `3a28dae` on 2026-10-18 at
10:31:55-10:32:18. Native Windows Crystal and Docker Linux Rust each produced a valid
three-page 595 x 841 pt PDF with the same date and 35 ordered ratings. Original
snapshot/template hashes are unchanged. Requests were removed; no SQL was executed.

Coordinator and independent reviewer inspected all six final PNGs. The complete
outline/operator/path audit reproduces the measurements above without exclusions.
Each final PDF, the Rust IR and all six PNGs are byte-identical to their respective
baseline outputs. The worker binary hash also equals the prior CR-003 worker, as
expected for tests/Markdown-only changes; CR-001/CR-003 were not redundantly rendered.
This establishes the captured CR-005 text/layout gate, with exact logo-color matching
still open. The standard quantizers tried in private (Pillow median-cut, maximum
coverage, fast octree and Windows GDI+ optimal-palette conversion) did not reproduce
Crystal's decoded image exactly. Palette ordering suggests an octree family, but
does not prove a compatible algorithm. No speculative production change follows.

Private final evidence under `research/stc-verification/cr-005-parity/final/`:

- `native.pdf`, `rust.pdf`, `comparison.html`, `comparison.json`, six PNGs and Rust IR.
- `evidence.json`: clean source, timestamps, unchanged frozen snapshot, no-SQL replay.
- `engine-hashes.json`: original template, PDF, worker, bridges, image and font hashes.
- `review/final-review.md`, `final-review-summary.json`, `baseline-identity.json`,
  full glyph/run/path measurements and `image-residual-summary.json`.
- `resources.json`: final C:/D:/RAM and empty running-container inventory.

The final worker build passes using `cargo build --locked -p stc-render-worker`.
An initial invocation mistakenly used its binary name `stc-rpt-worker` as the Cargo
package; Cargo rejected that before building or generating output. The corrected
package build and subsequent regeneration passed; both logs remain private.

At 10:33:54 local, C: had 286.00 GiB free, D: 561.38 GiB free, available host RAM
was 5.42 GiB and WSL working set was 1.62 GiB; no container remained. Temporary and
evidence files remain on D:. The tests/findings checkpoint is `3a28dae`; a separate
local Markdown checkpoint records final review and updates the current fork/backend
references. No branches, worktrees, pushes, original-input changes or deployments.


## Final fresh pair and shared-case replay (2026-10-18)

Both CR-005 exports ran from clean local `eb4656d` at 10:59:25-10:59:47 using the
same unchanged template and frozen snapshot, without SQL. Both are valid three-page
595x841 pt PDFs with the same date and all 35 ordered ratings (16 / 12 / 7).
All 9,600 decoded logo RGB pixels now match exactly. The native PDF is unchanged
from the earlier reference; Rust drawing streams, embedded font resources, other
raster pixels and Page IR are unchanged. Thus the previous complete glyph/run/path
comparison remains applicable: max X 0.05894 pt, ordinary Y 0.00005 pt and mark Y
0.03016 pt. The known 62 CMap aliases preserve identical outlines; coordinate-free
text extraction still differs and is not used as the visual acceptance criterion.
Coordinator and independent reviewer inspected every native/Rust page PNG and logo.
No new wrapping, clipping, missing elements or alignment issue was observed.

CR-001 and CR-003 also received fresh Docker Rust replays from the same commit at
their recorded historical dates, compared with their accepted native references.
Both now match all logo RGB pixels; drawing streams/fonts/IR/other images remain
unchanged and integrity, dates, page boxes and 34/52 ratings pass. This closes the
shared image residual without implying coverage of other students/templates.

Private final artifacts: `research/stc-verification/cr-005-logo/final/` contains the
paired PDFs, all page PNGs, side-by-side HTML, evidence manifest, image audit and
independent review. Sibling `replay-cr001/` and `replay-cr003/` contain shared-case
replays. These artifacts and private source/control data are untracked.

Source validation: 189 affected tests plus six doctests, including seven Chinese
checks; strict workspace Clippy, formatting and worker build all pass. Independent
source review has no blocker. Tests use synthetic data only. At 11:01:40 C: had
285.08 GiB free, D: 561.37 GiB free, host RAM 5.71 GiB available and WSL 1.31 GiB.
No container remained. Existing Docker volumes total 27.31 GB after compilation
(previously 26.64 GB); no new volume or pruning was performed. Temporary work uses
D:\tmpppppp123\rpt-rs-cr005-logo, and existing Docker storage may still consume C:.

Only local commits on the existing branch were created. No branch, worktree, push,
SQL, original-input modification or application deployment occurred. Remaining work
is broader report coverage, unsupported storage/report-color-depth modes, BIFF/XLS,
subreports and authenticated Parent delivery, as recorded in the fork status.
