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

[windows_metrics.rs](../../../crates/rpt-text/src/windows_metrics.rs) now separates font-device pixels from PDF twips:

- `WindowsLayout::new(source)` retains the previous **1440-DPI** behavior for existing general-library callers.
- `WindowsLayout::with_printer_dpi(source, NonZeroU32)` accepts an explicit vertical printer resolution. It never queries a host printer.
- Line pitch rounds the nominal point size to printer pixels, reads the applicable VDMX metrics (or the OS/2 fallback), rounds hhea external leading at that same resolution, then converts the sum back to twips. Fractional twips are retained until downstream geometry decides how to round.
- PDF positive-height glyph sizing, nominal first-line ascent, Unicode shaping, wrapping, and font fallback retain their previous behavior. In particular the 10 pt Arial cell still selects an 8.95 pt PDF glyph em under the existing 1440-DPI conversion.

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
