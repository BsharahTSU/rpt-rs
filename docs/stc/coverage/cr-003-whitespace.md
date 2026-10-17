# CR-003 whitespace fitting and centered RTL fields

## Status and scope

This records two independently controlled native Crystal behaviors identified during
CR-003 comparison on 2026-10-17. The source fix and synthetic regressions have been
reviewed; final Docker validation and fresh paired-PDF acceptance are recorded by
the coordinator in the [CR-003 repair ledger](cr-003-parity.md). This document does not establish
broader template coverage, Excel support, live SQL behavior or Parent API delivery.

The original template and frozen schema-bearing DataSet were hashed before and
after the probes and remained unchanged. Native controls used Windows Crystal;
Rust builds, tests and candidate exports use Linux Docker. Private copies, PDFs,
operator measurements and raw field values remain under ignored
`research/stc-verification/cr-003-parity/`. Temporary files use
`D:\tmpppppp123\rpt-rs-cr003`. No SQL was executed.

## Defect 1: a visible LTR prefix did not consume wrapping width

CR-003 had one English question whose Rust first line included an additional word.
Both engines produced two lines, so a page-count check alone could not detect the
error. The authored formula prepends and appends one ASCII space to the decoded
field content. The frozen input has no leading space itself.

Read-only native RAS inspection identifies the affected field as `Question2`:

| Property | Native value |
| --- | --- |
| Horizontal alignment / reading order | Left / LTR |
| Text format / CanGrow | RTF / true |
| Bounds | Left 115, top 30, width 4795, height 290 twips |
| Character spacing | 0 |
| First-line / left / right indents | 0 / 0 / 0 |
| Line spacing / type | 1 / 0 |
| Word wrap / maximum lines | Enabled / 0 |

Two private report copies changed only the formula's leading-space literal to zero
or two spaces. The trailing suffix remained intact. Saved formula source was read
back and verified before export; the original report was never saved or modified.

| Native control | Observed break |
| --- | --- |
| Zero leading spaces | The boundary word fits on the first line |
| One leading space, original report | The boundary word moves to the second line |
| Two leading spaces | The boundary word remains on the second line |

This directly establishes that the LTR leading spaces consume fitting width.
Previously `WindowsLayout::wrap` removed leading U+0020 before fitting and restored
it afterward for every paragraph. That compatibility behavior is still needed for
the existing RTL case, but applying it to LTR text caused this defect.

The fix passes the complete LTR paragraph to the shaper. The existing RTL prefix
path remains, selected using the shaper's paragraph base direction (`LayoutRun.rtl`).
This intentionally differs from the presence of any RTL glyph: English followed by
Arabic still has an LTR base direction and must count its prefix. The separate
content-aware growing-field ascent rule continues to inspect actual RTL glyph
levels; its purpose and behavior do not change.

Whitespace-only input retains its existing output. The initial prefix-only candidate
was rejected: it corrected the target sentence but wrapped another English line too
early and created a terminal LRM-only line. Native hinted widths and suffix handling
below complete the fitting rule. No data replacement or arbitrary width offset is used.

### Printer hinted advances, not design-width rounding

Native GDI probes measured printable ASCII with Arial regular and bold at 75, 83
and 92 pixels per em. All 570 individual glyph advances exactly match the font's
`hdmx` records; whole-string sums also agree. At nominal 10 pt and 600 DPI, the
selected 83-ppem record distinguishes the close fit boundaries:

| Line boundary | Native GDI / hdmx width | Available width | Result |
| --- | ---: | ---: | --- |
| Earlier sentence's final short word | 4792.8 twips | 4795 | Fits |
| Target sentence's next word | 4802.4 twips | 4795 | Wraps |

Raw design widths are 4802.44 and 4813.48 twips, making both fail. Naively rounding
design advances to device pixels makes both fit. Neither approximation reproduces
the native decision; a general tolerance would hide the cause.

For unkerned printable ASCII only, the Windows layout now selects the exact
font-specific `hdmx` record at rounded nominal ppem. It clones the already shaped
paragraph, replaces its fitting advances, and reuses Cosmic's existing Unicode
word/glyph line-layout algorithm. `ShapeWord::width` recomputes from those advances;
there is no stale width cache. Glyph selection, shaping and logical reconstruction
remain shared. Painted PDF advances, alignment metrics and public nominal width
measurement do not change.

Absent ppem records, unsupported text, non-opt-in layouts and malformed/incomplete
tables retain the nominal path. There is no font/report name check, interpolation,
new dependency or installed-printer query. The parser validates table version,
record sizes, glyph bounds and complete table length. This measured policy is not
a claim that all fonts or scripts have portable hinted metrics.

### Terminal direction marker and spaces

One input ends with U+0020 followed by LRM (U+200E), then the formula adds U+0020.
The rejected candidate wrapped the marker and space to another line, increasing
row height and shifting later content by 141 twips (7.05 pt).
A private native control removed only the raw terminal space/LRM from exactly one
cell; all 6,449 visible glyphs and their positions across all three pages stayed
identical. Schema, other cells and original inputs were verified unchanged.

LTR fitting now temporarily separates only the terminal U+0020/LRM suffix and
restores it to the final content line after wrapping. It preserves logical content
and emitted spaces. Other controls, NBSP, combining marks and RTL suffix behavior
are not generalized as invisible; zero advance alone does not prove absence of ink.

Implementation: [Windows layout](../../../crates/rpt-text/src/windows_metrics.rs)
and [Cosmic paragraph direction](../../../crates/rpt-text/src/cosmic.rs).

## Defect 2: terminal spaces displaced centered RTL headings

Two Arabic headings were shifted by approximately 1.10 pt in the Rust PDF. Both
source values end with one logical U+0020. The native field `SubjectNameA1` is
centered, RTL, RTF, not growing, with zero character spacing and zero indents. Its
bounds are left 6150, top 61, width 4495 and height 250 twips.

Three private DataSet variants changed only the terminal spaces of those two values,
covering exactly three source rows, to zero, one or two U+0020 characters. The XML
roundtrip preserved schema, table/row/column shape and every other cell, including
binary values. The original report and frozen snapshot remained unchanged.

All three native PDFs had identical visible glyph sequences, sizes, advances and
X/Y origins for both headings. They also matched the unchanged baseline exactly:

| Affected run | Nonblank glyphs | First X | Last glyph end X | Baseline Y |
| --- | ---: | ---: | ---: | ---: |
| First heading | 16 | 430.75 pt | 478.0336 pt | 760.5 pt |
| Second heading | 12 | 435.90 pt | 472.9422 pt | 469.6 pt |

Native Crystal emitted the internal space but no logical terminal space in any
variant. Merely excluding the terminal space from the alignment width while still
painting it would leave the RTL visible text displaced.

The fix extends the existing centered-field terminal-U+0020 preparation to known
RTL as well as LTR directions. It prepares each complete paragraph before fitting
and each wrapped line before measurement/emission. The late page-count and repeated
currency-symbol rewrites apply the same preparation to centered fields, preventing
a later rewrite from restoring the terminal spaces.

Other alignments and static text keep their existing preparation behavior. The
initial field plan continues to preserve unknown direction values. The existing
late numeric-field fixups store only the emitted Boolean direction, so their older
unknown-direction behavior is not newly guaranteed by this repair. NBSP, tabs,
leading spaces and internal spaces are not part of the terminal-U+0020 rule.

Implementation: [pagination](../../../crates/rpt-layout/src/paginate.rs),
[page-count rewrite](../../../crates/rpt-layout/src/lib.rs), and
[currency rewrite](../../../crates/rpt-layout/src/currency.rs).

## Related defect: mixed-direction reconstruction lost characters

The new LTR/Arabic regression exposed an older `CosmicLayout::wrap` defect:
`alpha مرحبا` could become `alpha م` even without a constrained width. Reconstruction
used only the first and last visual glyphs' byte ranges, which do not necessarily
bound the logical text when bidi reordering puts another cluster between them.

The fix takes the minimum start and maximum end across every glyph cluster in the
visual line, then slices the original UTF-8 text using that interval. Shaping,
bidi levels, font selection, advances and break decisions do not change. The
existing empty-line fallback remains. Synthetic tests cover both paragraph
directions, Arabic ligature/combining clusters, Hebrew/Arabic mixed text and a
constrained mixed-language line boundary. A separator retained by the shaper at
the end of a mixed-direction line is deliberately retained, not trimmed globally.
Independent source review found no blocking issue with this reconstruction.

This is a general correctness fix discovered by the new compatibility regression;
it is not asserted to be the cause of either observed CR-003 difference. Actual
CR-001 and CR-003 replay checks guard the shared renderer changes.
## Regression coverage and acceptance gates

Synthetic tests cover LTR prefix fitting, retained RTL prefix behavior, mixed
English/Arabic paragraph direction, whitespace-only input, device-table validation,
terminal LRM/space fitting, centered fields in both
directions, unchanged other alignments, unknown initial direction, static RTL text,
page-count and currency rewrites. Chinese coverage adds Simplified and Traditional
text with leading spaces: the prefix consumes first-line capacity while all spaces
and Han characters survive. Existing Chinese PDF extraction/fallback tests remain
part of the rendering checks.

Relevant tests live in [text metrics](../../../crates/rpt-text/src/windows_metrics.rs),
[shaper direction](../../../crates/rpt-text/src/cosmic.rs),
[layout whitespace](../../../crates/rpt-layout/src/tests/trailing_spaces.rs), and
[Chinese rendering](../../../crates/rpt-render/tests/chinese.rs), and
[synthetic device-font integration](../../../crates/rpt-text/src/cosmic_device_tests.rs).

Acceptance requires Docker tests and lint, regenerated original/Rust CR-003 PDFs,
all-page visual and operator checks, plus the existing CR-001 comparison guard.
The coordinator records exact test counts, source commit, final tolerances and
resource headroom in the current repair ledger. Native controls are proof of the
specific behaviors above, not substitutes for final output verification.

## Private evidence index

All paths below are relative to ignored
`research/stc-verification/cr-003-parity/native-metrics/`:

- `findings.md`, `metadata-evidence.json`: investigation and unchanged-template evidence.
- `prefix-formulas.json`, `prefix-metadata.json`, `prefix-evidence.json`,
  `prefix-wrap-metrics.json`, `prefix0.pdf`, `prefix2.pdf`: leading-space controls.
- `heading-trailing-evidence.json`, `heading-trailing-metrics.json`,
  `heading-trailing{0,1,2}.xml/.pdf`: centered RTL controls and data invariants.
- `audit-controls.py`: PDF operator/position and line-break measurements.
- `gdi-hdmx-width-probe.py/.json`: 570 actual GDI-to-device-table comparisons and fit thresholds.
- `tail-control-evidence.json`, `tail-control-metrics.json`, `tail-control.pdf`: one-cell
  terminal-control removal and unchanged visible output.
- `cosmic-shape.rs`: the installed dependency source inspected for fresh width computation.

Exploratory Engine name-index lookups returned null metadata; those nulls were not
interpreted as report settings. Explicit RAS properties and emitted PDF operators
supply the evidence above. No report-specific text match or hard-coded displacement was needed. The final
printer fitting policy uses measured font-provided device widths as documented above.
