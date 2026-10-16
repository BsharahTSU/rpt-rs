# CR-001 trailing-space alignment

Follow-up: 2026-10-17. The native probes below are complete. The current source
implementation is narrower than the probe findings; fresh-pair acceptance for the
combined text-origin changes is recorded in the [parity record](cr-001-parity.md).

## Finding

The fresh pair at commit `3683e20` corrected the previous pagination, spacing,
geometry and Arabic-direction/mark defects. Independent all-glyph review found a
separate 1.025-pt horizontal shift in one centered Arabic field.

The field formula reads a subject label and replaces legacy parenthesis tokens;
it does not trim the result. The
captured value ends in one U+0020. The Rust page operation retains that space and
reports 530 twips (26.5 pt) of advance at the effective 7.8-pt size. Centering that
advance starts the visible glyphs at X440.175 pt. Native starts them at X441.20 pt.
The nine nonblank glyphs have the same outlines; their total widths differ by only
0.01123 pt. This is whitespace alignment, not rasterization or a missing ligature.

The broader CR-001 IR contains 69 nonempty field runs ending in ASCII spaces:
34 left-aligned, 34 right-aligned and this one centered run. Changing only a named
heading would hide the generic field-rendering defect and is not the implementation.

## Native controlled probe

Five private schema-bearing DataSet variants changed only the affected label:
zero, one or two trailing U+0020 characters, one U+00A0, or one U+0009. Each was
exported by the original Windows Crystal bridge without SQL. All five native
outputs paint the same nine nonblank glyphs at X441.20-465.5828 pt, Y406.05 pt,
with 24.3828 pt visible width. Only the internal space is emitted, with 2.1684 pt
advance; none of the tested trailing whitespace is painted. The original snapshot
and RPT hashes remain unchanged.

Private input copies, requests/cleanup evidence, native exports and measurements
are under ignored `research/stc-verification/cr-001-parity/trailing-space-probe`.
The earlier independent proof is retained with the pre-fix final comparison.
Saved XML was reopened to confirm that all five suffix variants survived serialization.
All target glyph sequences match in text, codes, X/Y positions, advances and sizes.
Native metadata explicitly reports LTR reading order, centered alignment, standard
text, word wrap enabled and zero character spacing.

## Static text distinguishes measurement from emission

One additional private RPT copy contained equal-width static text objects with
synthetic `SPACEPROBE` and `SPACEPROBE  ` values. Reopening the copy confirmed
lengths 10 and 12, centered paragraph alignment and explicit LTR reading order.

Both objects have 250.50-point width. Their native nonblank text occupies the
same relative X interval [89.85, 160.5601] pt, with 70.7101-point advance span.
Thus native static text also excludes trailing spaces from centering measurement.
However, it still emits the two trailing space glyphs after the visible text,
each with 2.8078-point advance. Static text must retain those glyphs even when
its alignment measurement ignores them.

The original RPT and frozen DataSet SHA-256 values were unchanged after all six
native exports. No database query, refresh or write was performed.

## Current implementation boundary

The worker opts into `WindowsLayout::without_trailing_spaces`. The default
`TextLayout::prepare_line` is an identity operation; the opted-in implementation
removes only trailing **U+0020** from a complete display line. Layout invokes it
only for **centered LTR Field objects**, before wrapping and after each resulting
line. Empty prepared lines retain their line position. Dataset values and the
stored report model are unchanged.

This emission preparation does not alter left/right fields, centered RTL fields,
static Text, or FieldHeading text. It does not remove NBSP, tabs or other Unicode
whitespace, despite the broader field-probe evidence. Internal/leading spaces and
internal measurement fragments, including segments around tabs, keep their meaning.

Separately, the worker's final alignment-width calculation excludes trailing U+0020
for centered LTR runs while retaining static text's original emitted string. That
measurement applies only when rigid character spacing is zero. See
[final text origins](cr-001-text-origins.md) for the shaped-glyph rounding policy.
Right-aligned and RTL runs retain their complete strings for that measurement.

FieldHeading objects were not independently probed. Static NBSP/tab, other Unicode
whitespace, other alignments and multiline wrapping are outside the native evidence
collected here. Regression coverage must retain those boundaries rather than infer
a universal whitespace normalization rule.

Implementation: [line preparation contract](../../../crates/rpt-pages/src/text.rs),
[Windows compatibility policy](../../../crates/rpt-text/src/windows_metrics.rs),
[layout gates](../../../crates/rpt-layout/src/paginate.rs), and
[final worker measurement](../../../apps/stc-render-worker/src/main.rs).

Private `metrics.json`, `static-metrics.json` and `findings.md` are alongside the
probe inputs and exports. Probe scripts and hash-verification logs are in the
parent directory. Private text, captured data, IDs, templates and PDFs are not committed.

## Horizontal replay and separate vertical follow-up

After combining this narrow whitespace policy with final shaped-glyph alignment
rounding, independent PDF-operator comparison confirms a maximum X difference of
0.05708 pt across 4,467 matched nonblank glyphs and 0.03332 pt across the 63 matched
English lines. Shape inventories remain unchanged. These are measured horizontal
results; they do not establish vertical or complete-report acceptance.

The separate first-ascent investigation in [final text origins](cr-001-text-origins.md)
uses copied-template native controls to distinguish reading order, alignment,
CanGrow and text interpretation. Those probes do not expand the whitespace rules
above. Final combined replay and acceptance remain in the coordinating parity record.
