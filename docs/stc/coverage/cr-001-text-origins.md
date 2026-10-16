# CR-001 final text origins

Follow-up: 2026-10-17. This records native measurements, numerical predictions and
the current source implementation. Fresh-pair measurements and final acceptance are
owned by the [parity record](cr-001-parity.md). The prior committed replay and its
test results do not establish parity for these additional changes.

## Three distinct causes

The final review of checkpoint `3683e20` isolated three remaining text-origin errors:

| Cause | Evidence | Implementation scope |
| --- | --- | --- |
| Trailing whitespace in centering width | Controlled field and static-text native probes | Centered LTR Field emission drops terminal U+0020; centered LTR final measurement excludes it while static glyphs remain |
| Final alignment advance uses a different rounding rule | Actual shaped-glyph positions and native alignment anchors | Worker Center/Right runs with zero character spacing use per-glyph PDF-width then twip rounding |
| First-line ascent requires a layout-path-dependent metric choice | Blanket 600-DPI ascent regresses fixed StandardText fields; controlled RTL and RTF toggles each change native Y independently | Scope selected-DPI ascent using proven native paths; default and PDF cell conversion remain 1440 DPI |

No constant X/Y translation explains these causes. They depend on text, shaped glyph
advances, alignment, font size and the selected printer resolution.

## Horizontal advance evidence

Three right-aligned Arabic runs demonstrate the alignment-width difference:

| Previous Rust advance | Native inferred alignment advance | Difference |
| ---: | ---: | ---: |
| 199.40 pt | 198.95 pt | 0.45 pt |
| 187.90 pt | 187.55 pt | 0.35 pt |
| 144.30 pt | 144.00 pt | 0.30 pt |

Their painted glyph widths already agree closely. Changing those drawn widths to
correct a nearly uniform origin shift would mix two separate measurements.

The broad analysis matched 283 nonempty runs, including 216 centered/right-aligned
runs and 4,467 nonblank glyphs, with no unmapped glyphs. It treated ligatures and
zero-advance marks as shaped glyphs. Of the candidates examined, the best-supported
alignment calculation was:

```text
pdf_width_g = round(normalized_shaped_advance_g * 1000)
em_twips_g = round(effective_point_size_g * 20)
alignment_width = sum(round(pdf_width_g * em_twips_g / 1000))
```

The effective size is the final positive-height PDF font-cell size, not the nominal
layout size. Each glyph is rounded before summation. Rounding a whole-line width,
rounding direct font units straight to twips, or summing Unicode character widths
does not reproduce this calculation.

With the whitespace treatment modeled by that analysis, the maximum predicted X
error over centered/right-aligned runs falls from 1.025 to 0.05708 pt. The 63
matched English lines predict at most 0.03332 pt. Direct twip rounding without
the integer-thousandth-em step leaves errors up to 0.163 pt and worsens some runs.
Those numbers originated as predictions from existing artifacts. The subsequent
independent horizontal replay now confirms the 0.05708-point maximum across all
4,467 matched nonblank glyphs and the 0.03332-point English maximum. Shape
inventories are preserved. Vertical policy requires the native controls below and
a new combined replay, so this horizontal result does not establish acceptance of
the combined implementation.

The prediction also records a tradeoff: one already-close centered ASCII label
moves by another half twip, increasing its maximum error from 0.01699 to 0.04199 pt.
Some other small residuals move within that margin. Do not describe every coordinate
as improved or use the predicted maximum as an acceptance claim.

## Current alignment implementation

[CosmicLayout](../../../crates/rpt-text/src/cosmic.rs) exposes
`crystal_alignment_width_twips` as an explicit compatibility measurement. It uses
the shaped buffer, retaining font fallback, ligatures, combining marks and the
configured Basic-Latin kerning policy. Ordinary `width_twips` and wrapping keep
their existing behavior.

The [STC worker](../../../apps/stc-render-worker/src/main.rs) invokes this measurement
after layout and positive-height font-cell conversion. It replaces the stored final
advance only for Center or Right alignment with zero rigid character spacing and
available Windows font metrics. Other cases retain their existing scaled advance.
The PDF writer still paints the shaped glyph advances; this change adjusts the
alignment origin rather than each glyph's drawing advance.

For centered LTR runs, the worker excludes trailing U+0020 from this final width.
Static strings retain their trailing glyphs. Right-aligned and RTL runs keep the
whole string in final measurement, avoiding a blind logical-space trim that would
also change visual leading-space placement. The separate preparation rule removes
terminal U+0020 from emitted strings only for centered LTR Field objects. See
[whitespace evidence and limits](cr-001-whitespace.md).

NBSP/tab removal is not implemented despite the native field-probe result. The
native static-text probe covers ordinary spaces only. FieldHeading whitespace
behavior and other alignment/wrapping cases were not independently probed.

## Selected printer DPI and baseline origin

The same review found uniform baseline errors by face and nominal size:

| Face and nominal size | Previous ascent | Selected 600-DPI ascent, rounded | Native inferred ascent |
| --- | ---: | ---: | ---: |
| Arial Bold, 16 pt | 290 twips | 295 | 295 |
| Microsoft Sans Serif, 10 pt | 184 | 185 | 185 |
| Arial, 11 pt | 197 | 199 | 199 |
| Times New Roman Bold, 10 pt | 183 | 180 | 180 |

The first follow-up in
[WindowsLayout](../../../crates/rpt-text/src/windows_metrics.rs) used selected-printer
ascent for every run, converted to twips and rounded once. The fresh replay rejects
that blanket policy: all 54 identified regressions are fixed StandardText fields.
The initial static/growing/RTF grouping was a correlation, not a proven branch.
The four numerical matches above establish useful cases, not a universal ascent rule.

The source selection must be scoped before combined acceptance. The original
blanket claim that first-line ascent always stays at 1440 DPI is also superseded.
`WindowsLayout::new` still selects 1440 DPI, and positive-height PDF font-cell sizing
retains its separate 1440-DPI policy. No proposed ascent selection rescales glyphs.

The original metric and spacing checkpoint results remain in
[printer metrics](cr-001-metrics.md) and [Basic-Latin spacing](cr-001-spacing.md).
The current follow-up must be measured after the changes are combined; historical
success cannot substitute for that replay.

## Controlled native ascent probes

All controls use private saved copies of the same normalized template and the same
frozen DataSet. SDK readback confirms the intended properties before export.
Original template and data hashes remain unchanged; no SQL is executed. Measurements
compare actual glyph origins from PDF operators, not font-descriptor bounding boxes.
Positive Y below means upward movement in PDF coordinates.

| Controlled change | Native result | Conclusion |
| --- | --- | --- |
| Standard centered rating field: CanGrow false to true | No glyph position changes | Growing alone is not a general printer-ascent switch |
| Same fixed standard field: reading order LTR to RTL | All 34 rating baselines move +0.05 pt Y; X unchanged | Direction independently selects a different ascent path |
| Same fixed standard field: Center to Right alignment, LTR retained | Y unchanged; X changes with alignment | Right alignment alone does not select printer ascent in this control |
| Same standard field: enable CanGrow while Right alignment is retained | All 34 rating origins remain unchanged | CanGrow plus Right alignment is not a general printer-ascent switch either |
| Separate LTR/right standard field: CanGrow true to false | Only its two runs / 36 glyphs move +0.10 pt Y | Growth has an effect in this object; the cause remains unresolved after the controls below |
| Grow/right/LTR rating field: authored height 290 to 200 twips, below its 228-twip line pitch | No glyph position changes anywhere; all 34 ratings unchanged | Height below line pitch alone does not explain the exception |
| Centered LTR RTF field: RTF to StandardText, CanGrow true retained | All 15 single-line headings / 175 glyphs move +0.05 pt Y; X unchanged; no other glyph coordinates change | Formatting independently selects a different ascent path |
| RTL/right-aligned RTF field: RTF to StandardText | No glyph position changes | RTL remains an independent path, so this case cannot isolate formatting |
| Date field: attempted StandardText to RTF | Native readback remains StandardText | Setter was ignored; exclude this attempted toggle from evidence |

The decisive centered-format control uses a 7.8-pt effective bold font. Its first
native baseline changes from 593.95 to 594.00 pt, matching the one-twip difference
between the selected-printer ascent of 168 and ordinary ascent of 167 twips.
The source text is plain single-line data interpreted through the RTF option;
the probe does not test embedded rich-text runs or arbitrary RTF syntax.

These controls reject universal CanGrow, right-alignment, CanGrow-plus-Right,
height-below-line-pitch and selected-DPI rules.
They positively establish separate RTL and RTF paths within the measured native
cases. They do not prove HTML behavior or every font/size/wrapping combination.
The bounded investigation stops with the standard LTR field's growth-dependent
two-twip change unresolved. Do not infer a language/content rule, adjust a named
object, or claim that all fields follow printer ascent. The evidence supports
scoped RTL and RTF field paths together with the measured static-text path. The
unresolved case must remain visible in the checkpoint and fresh-pair comparison.

The scoped source integration is implemented. The default
`TextLayout::field_ascent_twips` delegates to existing ascent behavior, and
`WindowsLayout` overrides it with the legacy 1440-DPI field metric. Field layout
uses selected-printer ascent only for explicit RTL or known RTF interpretation;
all other field formats use the legacy field path. Static-text placement keeps
the selected-printer metric. This introduces no CanGrow, overflow, language or
named-object heuristic. Integration and fresh combined verification remain
pending; the policy does not claim to resolve the isolated standard LTR case.

Private saved reports, PDFs and numeric comparisons are retained under
`ascent-grow-probe/`, `ascent-direction-probe/`, `ascent-format-probe/` and
`ascent-centered-format-probe/` inside the CR-001 parity evidence directory.
The additional controls are in `ascent-student-grow-probe/` and
`ascent-grow-right-probe/`, with the last height control in
`ascent-grow-height-probe/`.
The centered-format comparison is `native-centered-format-comparison.json`;
its adjacent `metadata.json` proves the one persisted format change and retained
CanGrow, reading order, alignment, rotation and authored bounds. Captured text is
not copied into this documentation.

## Evidence and remaining validation

Private numerical evidence is retained under ignored
`research/stc-verification/cr-001-parity/final/review/`, including
`remaining-residual-classification.md`, `alignment-quantization-comparison.json`,
`per-glyph-rounding-probe.json`, `english-quantization-prediction.json` and
`printer-ascent-evidence.json`. Native whitespace probes are in the sibling
`trailing-space-probe/` directory. No captured text, identifiers, private templates,
font binaries or generated reports are committed.

Required acceptance evidence is the regenerated native/Rust pair, all-page visual
review and actual text-operator comparison, with page allocation, line breaks,
rating order and default-profile/Chinese regressions checked. Record those results
in the coordinating parity checkpoint once available; this document makes no
claim that the new combined output has already passed. Horizontal origin improvement
has been measured; the controlled native ascent evidence must now be applied to a
scoped implementation and verified in the combined output. The isolated standard
LTR growth-dependent baseline difference remains an explicit open issue, even if
the proven paths pass their focused checks.


Focused verification after integration: 481 Docker tests pass across Page IR, text,
layout, render and worker packages; this includes the five actual Chinese PDF tests
and the public reference net. Strict all-target/all-feature workspace Clippy and
formatting pass. The committed-source fresh pair remains the next comparison gate.
