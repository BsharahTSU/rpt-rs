# CR-001 repair and final comparison

CR-001 is the first evaluation case in the local Parent Crystal coverage inventory.
This work changes the local Rust fork only. Original Crystal, Parent application code,
report publication, database data and the original RPT remain unchanged. All case rendering
uses the previously captured, schema-bearing DataSet; no SQL is executed during this repair.

## Findings and fixes

| Difference | Verified cause | Fix |
| --- | --- | --- |
| Missing final subgroup heading on page 1; growing row drift | Printer line pitch measured at 1440 DPI instead of stored 600 DPI | Decode saved X/Y resolution; use printer-pixel pitch while preserving PDF glyph cells |
| Small English letter-spacing differences | Rust applied pair kerning; native uses unkerned advances | Shared opt-in Basic-Latin policy for measurement, wrapping and PDF shaping |
| Signature rules 1.5 pt too high | Native text/field borders sit outside selected object edges | Apply 30-twip outward padding to ordinary SingleLine frames; preserve isolated edge endpoints |
| Standalone rules 0.25 pt off at 0.5 pt thickness | Native coordinates anchor stroke edges, while PDF paths anchor centers | Offset the path by half its width on the perpendicular axis |
| Terminal Arabic period on the wrong side | Reader dropped the stored paragraph reading-order field | Preserve optional per-paragraph direction and carry it through wrapping to PDF operations |
| Arabic shadda marks vertically mirrored | Native Crystal reverses ScriptPlace vertical offsets | Opt-in Arabic-cluster offset sign compatibility, preserving other scripts and all glyph advances |
| Centered Arabic field about 1.025 pt left of Crystal | Rust includes a trailing ASCII space in its centering width; native omits it | Opt-in centered-LTR field preparation; preserve leading/internal spaces and unverified field directions |
| First-line offsets in static, RTL and RTF text up to 0.25 pt | Native uses a separate printer-ascent path for these objects | Use selected-printer ascent for measured paths; preserve prior nominal ascent for other fields and preserve PDF cell sizing |
| Two repeated growing-field baselines 0.10 pt from Crystal | Native uses printer ascent when a growing plain LTR field actually shapes RTL glyphs | Inspect shaped glyph levels for the complete resolved field; preserve fixed, English/CJK and unknown-format paths |
| Remaining centered/right text-origin offsets | Native rounds each shaped advance through PDF thousandths and whole twips before summing | Measure alignment after effective font selection; nominal wrapping and painted advances stay unchanged |
| Rounded outer frames 0.25 / 0.50 pt outside native centers | Native box bounds contain the full stroke | Inset ordinary unfilled circular rounded frames by half the stroke width, including spanning fragments |

Detailed evidence, numeric probes, implementation contracts and regression coverage:
[printer metrics](cr-001-metrics.md), [spacing](cr-001-spacing.md),
[geometry](cr-001-geometry.md), [Arabic direction and marks](cr-001-arabic.md),
[trailing-space alignment](cr-001-whitespace.md), [final text origins](cr-001-text-origins.md).

The earlier large baseline estimate from text-extraction bounding boxes was partly a
font-descriptor artifact. Actual first rating baselines already matched. The repair does
not apply a blanket text translation or alter group keep-together semantics.

## Verification

**Accepted for this captured CR-001 case within the measured tolerances below.**
Independent final operator and all-page visual reviews pass; no unresolved layout
defect remains in this pair. This is not a claim of byte or pixel identity.

Both original Windows Crystal and Linux Docker Rust were freshly regenerated at
01:38:46-01:39:09 local on 2026-10-17 from clean source commit `be022bd` and the same
captured schema-bearing DataSet. Original template, native bridge and portable
adapter hashes are unchanged. Both PDFs pass integrity checks and contain three
595 x 841 pt pages, the same print date and all 34 ordered ratings (17 / 17 / 0).
The page-boundary subgroup heading is restored and repeated correctly; all 63
English body line breaks remain. The coordinator reviewed all six final page images.

The final fresh pair's independent actual-PDF audit matches all 4,467 nonblank glyph
outlines and sizes, 283 nonempty runs and 216 centered/right-aligned runs. Exactly
two footer runs / 36 glyphs (32 nonblank) moved by -0.10 pt in the last correction;
their baselines now match native exactly. No other text, X coordinate, font, width,
line allocation or geometry changed in that correction. Independent final review
inspected all six PNGs and confirmed the regenerated glyph/outline data, Page IR,
run geometry and horizontal paths are identical to the verified candidate. Each
fresh engine PNG is also pixel-identical to its corresponding candidate PNG; this
does not mean the two different engines produce identical pixels.

Measured residuals are explicitly retained rather than described as exact identity:

| Measurement | Maximum final-pair difference |
| --- | ---: |
| All matched glyph X origins | 0.05708 pt |
| English glyph X origins across 63 lines | 0.03332 pt |
| Ordinary text baselines | 0.00005 pt |
| Rating baselines | 0.00004 pt |
| Arabic shadda Y rounding (12 marks) | 0.03016 pt |
| Rounded-frame stroke bounds (four frames) | Exact |
| Rounded-curve endpoint/control-point rounding | Approximately 0.05005 pt |

This captured-case comparison uses a documented envelope below 0.06 pt horizontally,
0.031 pt for mark vertical rounding and 0.051 pt for the rounded-curve endpoints.
There are no remaining unexplained ordinary baseline offsets. These are measured
case tolerances, not a universal promise for other reports, fonts or printer profiles.
PDF bytes and raster pixels differ. Forty-two non-ASCII glyph mappings differ between
font ToUnicode tables despite matching outlines; raw extracted-word equality is
therefore false. Content, glyph geometry and visual checks are recorded separately.

Latest source validation passes 486 focused Docker tests: 19 Page IR, 29 text,
370 layout, 56 render and 12 worker tests. These include all five Chinese PDF tests,
actual Arabic/mixed/CJK shaping guards and the public PDF reference net. Twelve
minimal-feature text tests also pass. Strict all-target/all-feature workspace
Clippy, formatting and the worker build pass. Independent source review found no
blocking issue. No public PDF baseline was changed for the final text-origin fixes.

The earlier full regression checkpoint `3683e20` passed 1,630 tests: 1,590 workspace
tests plus the 40 just-passed unchanged reader parity tests, including 154-report
JSON baselines, KDL and the nine-fixture / 15-page PDF reference net. Those counts
belong to that earlier checkpoint and are not added to the latest focused count.
No live database was configured or executed during this repair.

The first universal printer-ascent attempt regressed 54 field runs and was rejected.
Controlled native copies distinguish explicit RTL, known RTF and growing plain fields
with actual shaped RTL content. ASCII and independent Arabic controls establish the
last interaction. Growth, alignment and short bounds alone do not select the metric.
Fixed fields, growing English/CJK content and unknown formats retain their prior
paths. See [final text origins](cr-001-text-origins.md) for evidence and rejected rules.

## Reproduction and evidence handling

Use the Docker development images and the case-runner contract documented in
[case-runner.md](case-runner.md). The case's original snapshot and its manifest remain in
ignored research/stc-verification/parent-cases/cr-001. Private iteration scripts, template
numeric dumps, native controlled probes, PDF operator measurements, responses, PDF integrity
logs and all-page PNGs are under research/stc-verification/cr-001-parity.

The worker uses explicit STC_RPT_PRINTER_DPI when set, then stored Y, stored X, and a
deterministic 600-DPI fallback. Invoke-DockerBridge.ps1 -PrinterDpi is optional; leave it
unset to honor the RPT. The native engine still requires Windows. Rust runs in Linux Docker
with the existing Windows fonts mounted read-only and networking disabled for frozen replay.

The final comparison is scoped to this captured case. Matching report values, page allocation
and measured geometry do not imply byte-identical PDFs or identical font rasterization.
The engines encode font descriptors/widths and some Arabic text mappings differently.
General Excel/legacy/subreport compatibility and the broader case inventory remain separate
work. Other border styles, diagonal/rotated/tight-horizontal/shadowed/filled geometry and different printer
profiles need their own evidence; this change does not claim those are repaired.

## Resource controls

The user requested protection of the system drive and highlighted 97% host memory usage.
At the resource checkpoint, C: had 294.13 GiB free and D: 561.63 GiB free; Windows had
only 0.70 GiB available RAM. No Docker container was running, but WSL held about 7 GiB
of filesystem cache. Releasing that unused Linux cache recovered about 4 GiB of host RAM
without deleting files, stopping applications or changing global WSL settings.

Verification ran sequentially with Docker limits of 2 GiB RAM, no additional
container swap, two CPUs, one Cargo build job and one Rust test thread. Temporary files
use the user-provided D:\tmpppppp123\rpt-rs-cr001 directory, mounted as /tmp; private
report evidence remains on D: beneath ignored research/. Existing Docker image/target
volumes are reused rather than duplicated. Those volumes still reside in Docker's
managed storage; this work does not claim to relocate the Docker disk image.

The private bounded-Cargo runner checks at least 30 GiB free on C:, 10 GiB on D:, and
2 GiB available host RAM before starting. Resources are checked between verification
stages. No broad Docker prune, volume deletion or user-process termination is used.

Final resource checkpoint at 01:40:15 local: C: 290.38 GiB free, D: 561.49 GiB free,
6.88 GiB available host RAM and 0.97 GiB WSL working set; no Docker container remained.
The initial C: reading was 294.13 GiB. Reused Docker compilation storage accounts for
continued C: use; temporary case files and all private evidence use D:.


## Final evidence and local commits

Implementation checkpoints are `45266db` (printer pitch), `04e071f` (Latin spacing),
`b97edfb` (borders/rules), `09debec` (rounded frames), `3683e20` (paragraph RTL and
Arabic mark convention), `33f1998` (whitespace/alignment/scoped ascent) and `be022bd`
(growing shaped-RTL ascent). All are local on the existing branch. No branches,
worktrees, hosted fork, push or deployment were created.

Private final outputs are `final/native.pdf`, `final/rust.pdf`, `final/comparison.html`
and six page PNGs beneath the parity evidence directory. `final/evidence.json` records
the clean source commit, snapshot hash, timestamps and no-SQL replay;
`final/engine-hashes.json` records PDF/template/worker/bridge/image hashes.
`final/review/final-review.md` and `final-review-summary.json` hold the independent
review, with underlying glyph, outline, run, mark and geometry measurements.
Earlier pairs are preserved under `final-before-text-origins/` and
`final-before-growing-content/`; the isolated final change is in `growing-content/`.
These private evidence files remain ignored by Git.
