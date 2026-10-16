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

The corrected intermediate replay preserves three 595 x 841 pt pages and all 34 ordered
ratings, distributed 17 / 17 / 0. Actual rating baselines agree within 0.00004 pt numerical
serialization precision. The Songs and Recitals heading appears at page 1's bottom and
repeats on page 2. All 63 matched English body lines keep their breaks. Maximum English
glyph deviation falls from 0.64045 to 0.05832 pt. All 2,015 non-ASCII glyphs retain their
text and positions across the spacing correction alone. Subsequent actual glyph-outline
review identified and repaired separate Arabic punctuation and mark-placement defects.

At checkpoint `3683e20`, the full regression sweep covered 1,630 passing Rust tests:
1,590 workspace tests plus the 40 just-passed unchanged reader parity tests. Zero
failed or ignored; included five Chinese PDF tests, 154-report JSON baselines, KDL
and the nine-fixture / 15-page PDF reference net. No live database was configured.

Independent review then found the final text-origin issues described above. The
intermediate horizontal fix is measured across all 4,467 nonblank glyphs and all
216 centered/right-aligned runs: maximum X difference is 0.05708 pt, down from
1.025 pt. All 63 English line breaks remain, with maximum glyph-X error 0.03332 pt.
The existing Chinese PDF profile now exercises the final alignment measurement
through actual PDF generation/extraction as well as printer/Arabic compatibility.

The first universal printer-ascent attempt regressed 54 field runs and was rejected.
Controlled native copies prove separate RTL and RTF paths; growth, alignment, their
conjunction and a short box do not provide a universal rule. Synthetic ASCII in the
remaining footer removes its growth-dependent difference. An independent Arabic
rating control reproduces the interaction across 34 runs / 170 glyphs, while its
Latin control does not move. The final rule inspects actual shaped RTL glyph levels
for growing StandardText fields; it preserves fixed fields, English/CJK metrics,
unknown formats, declared direction and all glyph advances. No report/object/font
constant is used. Details and rejected hypotheses are recorded in
[final text origins](cr-001-text-origins.md). Final verification of this last repair follows.

The scoped integration passes 481 focused Docker tests across Page IR, text, layout, render and worker packages, including all five Chinese tests and the public PDF reference net. Strict all-target/all-feature workspace Clippy and formatting pass. The minimal-feature text suite previously passed 12 tests. Regenerated committed-source
PDFs and all-page review follow at the next checkpoint. CR-001 acceptance remains open.

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

Remaining verification runs sequentially with Docker limits of 2 GiB RAM, no additional
container swap, two CPUs, one Cargo build job and one Rust test thread. Temporary files
use the user-provided D:\tmpppppp123\rpt-rs-cr001 directory, mounted as /tmp; private
report evidence remains on D: beneath ignored research/. Existing Docker image/target
volumes are reused rather than duplicated. Those volumes still reside in Docker's
managed storage; this work does not claim to relocate the Docker disk image.

The private bounded-Cargo runner checks at least 30 GiB free on C:, 10 GiB on D:, and
2 GiB available host RAM before starting. Resources are checked between verification
stages. No broad Docker prune, volume deletion or user-process termination is used.