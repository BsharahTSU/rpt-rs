# CR-001 repair and final comparison

CR-001 is the first evaluation case in the local Parent Crystal coverage inventory.
This work changes the local Rust fork only. Original Crystal, Parent application code,
report publication, database data and the original RPT remain unchanged. All case rendering
uses the previously captured, schema-bearing DataSet; no SQL is executed during this repair.

## Findings and fixes

| Difference | Verified cause | Fix |
| --- | --- | --- |
| Missing final subgroup heading on page 1; growing row drift | Printer line pitch measured at 1440 DPI instead of stored 600 DPI | Decode saved X/Y resolution; use printer-pixel pitch while preserving glyph cells and first baselines |
| Small English letter-spacing differences | Rust applied pair kerning; native uses unkerned advances | Shared opt-in Basic-Latin policy for measurement, wrapping and PDF shaping |
| Signature rules 1.5 pt too high | Native text/field borders sit outside selected object edges | Apply 30-twip outward padding to ordinary SingleLine frames; preserve isolated edge endpoints |
| Standalone rules 0.25 pt off at 0.5 pt thickness | Native coordinates anchor stroke edges, while PDF paths anchor centers | Offset the path by half its width on the perpendicular axis |
| Terminal Arabic period on the wrong side | Reader dropped the stored paragraph reading-order field | Preserve optional per-paragraph direction and carry it through wrapping to PDF operations |
| Arabic shadda marks vertically mirrored | Native Crystal reverses ScriptPlace vertical offsets | Opt-in Arabic-cluster offset sign compatibility, preserving other scripts and all glyph advances |
| Rounded outer frames 0.25 / 0.50 pt outside native centers | Native box bounds contain the full stroke | Inset ordinary unfilled circular rounded frames by half the stroke width, including spanning fragments |

Detailed evidence, numeric probes, implementation contracts and regression coverage:
[printer metrics](cr-001-metrics.md), [spacing](cr-001-spacing.md),
[geometry](cr-001-geometry.md), [Arabic direction and marks](cr-001-arabic.md).

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

Final regression coverage is 1,630 passing Rust tests: the final workspace sweep
passed 1,590 tests and reused the 40 unchanged decoder parity tests that had just passed
in the focused reader run. Zero tests failed or were ignored. This includes all five
Chinese PDF tests, Arabic/mixed-script tests, 154-report JSON baselines, KDL smoke checks
and the nine-fixture / 15-page PDF reference net. Live database tests were not configured;
this repair performs no SQL. Strict workspace Clippy (all targets/features, warnings denied) and formatting pass.
Fresh-pair acceptance follows below.

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