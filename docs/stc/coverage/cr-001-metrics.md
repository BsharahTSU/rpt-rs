# CR-001 printer metrics investigation

Date: 2026-10-16. Scope: the existing frozen CR-001 comparison, with no SQL execution or changes to the private template. Numeric measurements below describe layout only; school reports, student data, and font binaries stay outside Git.

## Evidence and diagnosis

The baseline comparison lives in ignored `research/stc-verification/parent-cases/cr-001/run-20261016-225147-fa0b19cd/`. Native live-data and native frozen-data page PNG hashes were identical on all three pages. This establishes that the frozen native PDF is a suitable comparison for this case; it does not establish Parent API or general report coverage.

The first central rating baseline is **266.75 pt from the top of the page in both PDFs**, as measured from PDF text positioning operators. The next two native baselines are 294.10 and 321.45 pt; Rust's are 294.30 and 321.85 pt. Each of these two-line English rows adds **0.20 pt of real drift**. The first group-heading baseline also agrees at 210.50 pt. Therefore a blanket baseline translation would damage positions that already agree.

Poppler bounding boxes initially suggest a larger constant vertical offset, but the embedded font descriptors explain it: native Arial declares ascent 905/1000 em, while Rust declares 728.02734/1000 em. At the actual 8.95 pt glyph size their difference is 1.5839 pt. Bounding-box tops are consequently unsuitable evidence for shifting all glyphs. Compare actual text positioning and raster output as well.

Actual native multiline pitch is **11.4 pt** for both English and Arabic. Before this correction Rust uses 11.5 pt for the English fallback face and 11.3 pt for the Arabic face. The opposite error directions identify device-pixel rounding, rather than a common scale or page translation.

| Numeric face metrics at nominal 10 pt | 1440-DPI calculation used before | 600-DPI printer calculation |
| --- | --- | --- |
| Arial regular, English fallback | em 200; VDMX ascent 179, descent 44; leading 7 = 230 twips | em 83; ascent 74, descent 18; leading 3 = 95 pixels = **228 twips** |
| Microsoft Sans Serif, Arabic | em 200; ascent 184, descent 42; leading 0 = 226 twips | em 83; ascent 77, descent 18; leading 0 = 95 pixels = **228 twips** |

Arial uses 2048 units/em, OS/2 Windows ascent/descent 1854/434 and hhea line gap 67. Microsoft Sans Serif uses 2048 units/em, Windows ascent/descent 1888/430 and line gap 0. Direct read-only GDI `GetTextMetrics` probes confirmed the original 1440-DPI values; they are valid font metrics at that resolution. Using them as the printer's line pitch is the error.

## The template stores 600 DPI

The public decoded `PrintOptions` model previously omitted printer resolution, and the private template has empty current/saved printer names. Those empty names do **not** mean the file lacks resolution.

Read-only Docker inspection with `rpt dump <private-template.rpt> --type 0x0007` found one schema `0x0700` PageDevmode record. Its exact 24-byte reading includes:

| Stored member | Value | Joined record offset |
| --- | ---: | ---: |
| `dm_fields` | `0x0000ff03` | `0x0000` |
| `print_quality` | **600** | `0x000c` |
| `y_resolution` | **600** | `0x0012` |

The reader field table consumed the entire record without unknown bytes. The baseline host's default printer also reported 600×600 DPI, but the **stored template values** provide the stronger, portable evidence. This finding must not become an implicit query of the current Linux/Windows host printer. In other reports, `print_quality` may encode a signed quality enum rather than positive DPI; it must not be treated unconditionally as a resolution.

Sources: [DEVMODE field table](../../../crates/rpt-reader/src/field_table/tables/print_options.rs), [PrintOptions builder](../../../crates/rpt-reader/src/build_model/print_options.rs), and the ignored numeric model/native section inspection under `research/stc-verification/cr-001-parity/`.

## Focused library correction

This section records the initial printer-pitch checkpoint. Its first-line-ascent
contract is superseded by the dated follow-up below; the original measurements
and passing-checkpoint results remain part of the history.

[windows_metrics.rs](../../../crates/rpt-text/src/windows_metrics.rs) separates font-device pixels from PDF twips:

- `WindowsLayout::new(source)` retains the previous **1440-DPI** behavior for existing general-library callers.
- `WindowsLayout::with_printer_dpi(source, NonZeroU32)` accepts an explicit vertical printer resolution. It never queries a host printer.
- Line pitch rounds the nominal point size to printer pixels, reads the applicable VDMX metrics (or the OS/2 fallback), rounds hhea external leading at that same resolution, then converts the sum back to twips. Fractional twips are retained until downstream geometry decides how to round.
- At this initial checkpoint, PDF positive-height glyph sizing, nominal first-line ascent, Unicode shaping, wrapping, and font fallback retained their previous behavior. In particular the 10 pt Arial cell still selected an 8.95 pt PDF glyph em under the existing 1440-DPI conversion. The follow-up below changes the selected-DPI ascent contract while retaining the separate PDF glyph-cell conversion.

The caller is responsible for resolving an explicit override, a valid stored report resolution, and a deterministic deployment fallback. The STC worker integration and its configuration tests are owned by the coordinating change; this library API by itself does not select a new default for all consumers.

Synthetic tests use numeric metric tables, not private RPTs or proprietary font files. They cover opposite English/Arabic pitch corrections to 228 twips, preservation of the PDF glyph-cell metrics, a fractional 237.6-twip pitch, OS/2 fallback outside VDMX coverage, preservation of first-line ascent, wrap decisions, width, and the default constructor behavior, and the existing f32 half-twip rounding (10.025 pt selects a 201-twip nominal em at 1440 DPI). Run through Docker with `cargo test --locked -p rpt-text`; also check `--no-default-features` because the font metrics API can be used without the shaping feature.

## Limits and remaining verification

This analysis does not claim visual parity by itself. The coordinating replay must regenerate both PDFs from the frozen data, inspect all pages, and verify rating order, page-boundary headings, line breaks, and raster differences. The pre-fix first English baseline also differs by 0.05 pt, and horizontal width/word-fit differences may have separate causes. Do not use the font-descriptor bounding-box offset as a substitute for those checks.

The native case has an orphan group heading near the bottom of page 1. Source inspection permits such headings when the group's KeepGroupTogether flag is false, as it is for this template. Cumulative row-height drift can move that heading onto the next page; avoid changing generic pagination semantics before checking the corrected metrics.

Earlier visual-only claims that other labels were missing were rechecked against text and raster pixels and withdrawn. Only the page-1 orphan heading difference was confirmed. This note records the pre-fix diagnosis and API contract; regenerated acceptance evidence belongs in the coordinating coverage checkpoint.

## Integrated checkpoint and first replay

The model now retains positive signed DEVMODE resolutions independently as
saved_printer_dpi_y / saved_printer_dpi_x. Missing/nonpositive quality codes stay absent;
older JSON models still deserialize. The STC worker selects an explicit STC_RPT_PRINTER_DPI,
then stored Y, stored X, and finally a deterministic 600-DPI fallback. The optional Docker
launcher -PrinterDpi passes the explicit override; omitting it preserves the template choice.
Explicit/saved values outside 72..9600 DPI are rejected by the worker. The general library's
default remains unchanged.

The first corrected replay is private cr-001-parity/dpi600, using the original normalized
snapshot without SQL. Both PDFs pass qpdf, have three 595 x 841 pt pages, the same date,
and the same 34 ordered ratings (17 / 17 / 0). Independent PDF-operator verification found
all rating baselines match within 0.00004 pt numerical serialization precision. The missing
page-1 Songs and Recitals heading is restored and repeats on page 2. All 63 matched English
body lines preserve their breaks; remaining baseline differences there are at most 0.05 pt.
All three page pairs were visually inspected. This is not pixel identity.

Regression checks passed in Docker: 845 tests across the affected model/reader/KDL/layout/text/
worker suites, 10 text tests without default features, and five Chinese integration tests.
The added Chinese case performs an actual PDF text round trip with the 600-DPI profile.
Five launcher resolution checks pass in PowerShell 7 and 5.1; seven existing clock checks pass.
The synthetic pagination fixture checks a boundary heading, its continuation repeat, all four
two-line bilingual rows, and preservation of the first baseline. No pagination policy changed.

A separate horizontal residual is now established: native Basic-Latin glyph advances are
unkerned, while Rust applies pair kerning (up to 0.64045 pt in the matched English body).
That investigation is a separate follow-up; the printer-metrics checkpoint does not claim it
is fixed. PDF descriptor differences also continue to affect text-extraction bounding boxes.

## Follow-up: first-line ascent depends on the native layout path

On 2026-10-17, independent final-PDF review found small uniform baseline errors
within specific font/size groups, rather than cumulative line-pitch drift. Actual
font VDMX tables and native PDF text positions identify selected printer DPI
as a missing first-line-ascent input for particular native layout paths:

| Nominal face and size | Previous 1440-DPI ascent | 600-DPI ascent in twips | Native inferred ascent |
| --- | ---: | ---: | ---: |
| Arial Bold, 16 pt | 290 | 295.2, rounded to 295 | 295 |
| Microsoft Sans Serif, 10 pt | 184 | 184.8, rounded to 185 | 185 |
| Arial, 11 pt | 197 | 199.2, rounded to 199 | 199 |
| Times New Roman Bold, 10 pt | 183 | 180 | 180 |

These four matches supersede the earlier blanket statement that first-line ascent
always stays at 1440 DPI. They do not establish one rule for every object kind.
The initial follow-up routed ascent through
`device_metrics(font, false, selected_dpi)`, converted printer pixels to twips and
rounded to a whole twip. The fresh replay then disproved applying that choice to
every field: all 54 identified regressions were fixed StandardText fields.
That correlation did not establish CanGrow or field interpretation as the cause.
Native copied-template controls now distinguish several paths:

- Changing a short standard-text rating field from fixed height to CanGrow leaves
  all glyph positions unchanged. CanGrow is not a universal printer-ascent switch.
- Changing only the same field's reading order from LTR to RTL shifts all 34 rating
  baselines by +0.05 pt in PDF Y and leaves X unchanged. Changing only its alignment
  to Right changes X but leaves Y unchanged. Reading order independently selects a
  different ascent path; right alignment alone does not explain it.
- Enabling CanGrow on that right-aligned standard field also leaves all 34 ratings
  at the same X/Y positions. The CanGrow-plus-Right conjunction is likewise not a
  general switch. Separately, disabling CanGrow on another standard LTR/right field
  moves only its two runs / 36 glyphs by +0.10 pt in PDF Y. This exception requires
  more than the CanGrow/right-alignment flags to explain it. Reducing the first
  grow/right field's authored height from 290 to 200 twips, below its 228-twip line
  pitch, still moves no glyphs. Height below line pitch alone also fails to explain
  the exception.
- Changing a centered LTR string field from RTF to StandardText, with CanGrow still
  true, moves all 15 short single-line heading baselines by +0.05 pt in PDF Y.
  Exactly 175 glyphs change Y, with zero X change and no other coordinate changes
  anywhere in the three-page export. The first baseline is 593.95 then 594.00 pt.
  This matches printer ascent 168 versus ordinary ascent 167 twips and establishes
  a separate formatted-text path.
- An RTF-to-standard toggle on an RTL/right-aligned string field causes no glyph
  movement. Its existing RTL path prevents this control from isolating formatting.
  A date field's attempted StandardText-to-RTF setter does not persist in native
  readback, so that attempted toggle is excluded from conclusions.

The RTF result does not prove HTML behavior, handling of arbitrary rich-text markup,
or every font/wrapping combination. Checkpoint `33f1998` stopped with the standard
nominal-LTR field's growth-dependent two-twip change unresolved. A universal
CanGrow, Right, their conjunction, or height-below-pitch heuristic remains
unsupported. These rejected explanations are preserved; the content controls
below establish the missing interaction.

The scoped integration at that checkpoint is implemented:
`TextLayout::field_ascent_twips` delegates
to the existing ascent by default, while `WindowsLayout` supplies the legacy
1440-DPI field ascent. At that point, field layout selected printer ascent only
for explicit RTL or known RTF interpretation, while other field formats retained
1440 DPI. Static text retained the selected-printer path. That blanket treatment
of other nominal-LTR fields is superseded by the following measured content case.

## Follow-up: actual RTL shaping interacts with growing standard fields

The first private content control preserves the existing three-component formula
branch and replaces its input with synthetic ASCII `TEST SAMPLE STUDENT`. The
same nominal Arial Bold 11-pt, right-aligned, explicit-LTR StandardText field now
has identical native fixed/growing baselines: 26.15 and 10.15 pt on the two pages.
Both use effective Arial Bold 9.65 pt, 19 glyphs per run and X extent
400.85..516.65965 pt. With captured Arabic content, growth enabled gives 26.05
and 10.05 pt, while growth disabled gives 26.15 and 10.15 pt. Content alone
therefore removes the two-twip growth-dependent difference.

An independent second field receives synthetic Arabic `مرحبا` in its existing
rating column, keeping centered alignment, explicit LTR, StandardText, nominal
Arial regular 10 pt and authored bounds. Enabling growth shifts all 34 runs /
170 glyphs by +0.05 pt in PDF Y. The effective 8.95-pt font outlines, X positions
and page allocation (17 / 17 / 0 runs) remain identical. No other glyph coordinate
changes. The earlier Latin fixed/growing control for that same field changes no
glyph positions. Both synthetic values fit without an actual wrapping change.

The interaction is confirmed across distinct fields, alignments, font weights,
sizes and sections. Nominal LTR metadata can still produce shaped RTL glyph runs.
The source extension is implemented: growing **StandardText** fields select
printer ascent when the shaping backend finds actual RTL glyph bidi levels.
Explicit RTL and known RTF retain their already established printer paths.
Fixed nominal-LTR StandardText and growing English fields retain the legacy
1440-DPI field metric. There are no report/object names, font constants, language
labels, Unicode-block heuristics or constant position corrections in the rule.

The content controls establish native behavior; final source tests and fresh PDF
verification for this extension remain pending. Fixed/English guards and default
profiles must be checked along with the corrected Arabic case. They are not
evidence for every complex script or multiline configuration.

`WindowsLayout::new` still selects 1440 DPI, preserving the general constructor's
default behavior. The positive-height PDF glyph-cell conversion remains the separate
1440-DPI policy used after layout. The selected-DPI ascent change does not introduce
a new line-pitch formula or a new wrapping algorithm. Font resolution and the existing
fallback path remain in place.

The initial replay and test counts above are historical results for their checkpoint,
not validation of this follow-up. Fresh output must still verify page allocation,
line breaks and actual text origins after combining ascent with the horizontal
measurement changes. See [final text origins](cr-001-text-origins.md) and the
[parity record](cr-001-parity.md); no final parity result is asserted here.

Numeric evidence is in ignored `cr-001-parity/final/review/` under
`printer-ascent-evidence.json` and `remaining-residual-classification.md`.
The controlled exports, saved-property readback and glyph comparisons are retained
under sibling `ascent-grow-probe/`, `ascent-direction-probe/`, `ascent-format-probe/`
and `ascent-centered-format-probe/`; later growth controls are in
`ascent-student-grow-probe/`, `ascent-grow-right-probe/` and
`ascent-grow-height-probe/`.
The subsequent content controls are in `ascent-content-probe/` and
`ascent-rating-content-probe/`, including metadata and independent numeric glyph
comparisons. They change only the relevant columns in private schema-bearing
snapshot copies; all other cells remain equal. These probes use read-only original
or private copied templates. Original template and snapshot hashes remain unchanged;
no SQL was executed.
Only sanitized metrics are recorded in this document; font binaries and report data
remain private.

Historical focused verification for checkpoint `33f1998`: 481 Docker tests pass across Page IR, text,
layout, render and worker packages; this includes the five actual Chinese PDF tests
and the public reference net. Strict all-target/all-feature workspace Clippy and
formatting pass. These results predate the content-plus-growth extension and do not
validate its source or output. Final verification for that extension is pending.

Final source verification for the content-plus-growth extension passes 486 focused
Docker tests (19 Page IR, 29 text, 370 layout, 56 render, 12 worker), including all
five Chinese PDF tests and the public PDF reference net. Twelve minimal-feature text
tests also pass. Strict all-target/all-feature workspace Clippy and formatting pass;
the worker is rebuilt. Independent source review found no blocking issue. Fresh
actual-PDF verification is the remaining gate at this implementation checkpoint.