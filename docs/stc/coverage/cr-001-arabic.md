# CR-001 Arabic reading-order evidence

Investigation: 2026-10-17. This record separates native stored direction from Unicode
shaping and punctuation placement. It documents the confirmed decoder defect; the
implementation and final PDF comparison must record their own validation.

## Symptom and native metadata

The final comparison found a static Arabic note's terminal period on opposite sides
in native Crystal and Rust. The affected object is Text44 in GroupFooterSection15.
The pre-fix model says LeftToRight, and the corresponding page operation has rtl=false.
This note has one paragraph, horizontal-center alignment and no text rotation.

The original report was loaded read-only through the 32-bit Crystal Engine/RAS.
The actual ReportDefModel SDK enum defines:

| SDK enum | Value |
| --- | ---: |
| crReadingOrderLeftToRight | 0 |
| crReadingOrderRightToLeft | 1 |

Both native TextObject.ReadingOrder and its sole Paragraph.ReadingOrder return **1**.
The paragraph alignment is SDK value 2 (horizontal center). Original template hashes
were checked before and after inspection and remained unchanged.

Native metadata therefore explicitly requests RTL. This case provides no evidence
that Crystal automatically overrides an authored LTR setting because the text contains
Arabic. The missing stored direction must be repaired before adding any such heuristic.

## Canonical stored-field proof

A small native metadata-only probe saved two private copies: the loaded report unchanged
and a clone with the text object's and its paragraph's ReadingOrder set to the actual
SDK LeftToRight enum. The ReportObjectController applied the cloned object. Both saved
copies were reopened through native RAS:

| Private variant | Text-object ReadingOrder | Paragraph ReadingOrder |
| --- | ---: | ---: |
| RTL | 1 | 1 |
| LTR | 0 | 0 |

The original report hash remained unchanged. No SQL statement, report refresh, DataSet
query or PDF export ran. The existing CLI then decoded these copies in two serial
read-only Docker containers capped at 512 MiB and one CPU, with no compilation.

Among 78 `0x00c0 TextObjectFormat` paragraph records, exactly one changed. Its entire
demasked field-byte sequence is:

```text
RTL  0000000000000000000000000200000001000001000001
LTR  0000000000000000000000000200000001000001000000
```

The final VarU16, after the U32BE line-spacing value, changes from 1 to 0. For this
record it occupies joined byte offset 22; the offset is evidence for this pair, not
a fixed-position parsing contract. Alignment and line-spacing fields are unchanged.
All 75 `0x00a5 TextObjectContainer` field-byte sequences are identical between copies.

This proves that the terminal paragraph-format slot previously named `_u0` in
`crates/rpt-reader/src/field_table/tables/layout.rs` is `reading_order`.
The existing `ReadingOrder::from_code` enum mapping is already correct. The
pre-fix `open_paragraph` handler in `crates/rpt-reader/src/build_model/report_def/walk.rs`
consumed indents, line spacing and alignment but dropped this final value.
`TextObject.reading_order` therefore retained its default LeftToRight in this case.
The separate StringFieldFormat decoder handles field objects and does not repair
static text-object paragraphs.

## Implementation boundary

The reader now decodes the proven terminal VarU16 in the normal paragraph record
sequence with the existing SDK enum conversion. `Paragraph.reading_order` is an
optional override: decoded values are retained, an absent/truncated tail stays `None`,
and unknown ordinal values remain `ReadingOrder::Other(value)`. A missing tail does
not silently become an explicitly authored LTR value.

For older consumers of the single object-level property, the first paragraph
record, when it contains an explicit direction, supplies `TextObject.reading_order` (and a field heading's default direction).
This is a compatibility projection; the one-paragraph native probe does not establish
a native aggregate direction for mixed paragraphs. A later paragraph cannot overwrite
the earlier paragraph's direction. The model retains each text paragraph's setting,
and the layout resolves it per paragraph, then carries it onto every wrapped line and
PDF page operation. Explicit center/left/right alignment remains authored; default
alignment follows the resolved paragraph direction.

Existing constructed models and older serialized paragraphs have `None` and continue
to inherit the text object's direction. Synthetic regressions cover LTR/RTL, a widened
unknown ordinal, a missing direction tail, independence from line spacing, first-paragraph
projection/reset, field-heading promotion, mixed directions through line wrapping,
authored center alignment, inherited old-model output, and serde compatibility.

The KDL export remains under its existing policy: it emits the object-level direction
but its detailed paragraph projection does not currently export direction overrides
(or paragraph indentation). Mixed-direction KDL round trips are outside this fix.
Field headings still use their existing flattened text model and one default direction;
independent directions inside a multi-paragraph field heading are likewise not represented.
No direction is inferred from language, report/object names or student identifiers.

The native reading-order proof does not settle mark placement. The independent
investigation below establishes the separate legacy PDF placement quirk.

## Arabic mark offsets: native proof and opt-in compatibility

Actual glyph outlines, glyph IDs and PDF operators revealed that the Arabic shadda
marks were painted on opposite sides of their normal shaping origins. This is a real
positioning difference, independent of the engines' different ToUnicode mappings.

A Windows Uniscribe ScriptShape/ScriptPlace probe used the same Arial face and 9.85-pt
size as the four affected footer labels. At the native 1440 logical grid (197 units
per em), its four positive vertical offsets were 31, 28, 22 and 41 units: +1.55,
+1.40, +1.10 and +2.05 pt. The native Crystal PDF paints the corresponding offsets
with the opposite sign. At the face's 2048 design units per em, Uniscribe returned
316, 288, 226 and 425 units, matching the HarfRust/Rust design-space results. This
separates the sign quirk from the smaller printer-grid rounding difference.

Krilla's ordinary positive-up shaping-offset convention is correct. The new
`PdfOptions.legacy_arabic_mark_offsets` is false by default and enabled by the STC
worker. It negates only vertical offsets whose affected logical cluster resolves to
Arabic. A shared RTL font run can contain Hebrew; its marks must retain normal offsets. Glyph IDs, clusters, horizontal offsets, advances,
ligatures and joining remain unchanged. Vertical advances are not inverted: the
native evidence establishes offset reversal only.

Regressions exercise actual positioned and stacked Arabic marks, unchanged Hebrew
marks within a shared Arabic/Hebrew RTL face run, ordinary Latin combining marks,
unchanged glyph/advance/cluster data, and actual serialized PDF effects. The existing
Chinese PDF profile also enables this option to verify it leaves CJK text intact.
No report identifier, literal label or student selector is used by the implementation.

The measured sign correction does not prove a universal integer-grid rounding rule.
The separate 0.10-pt footer baseline difference is resolved by the verified
content-plus-growth metric path in [final text origins](cr-001-text-origins.md).
The maximum remaining mark Y rounding is 0.03016 pt across 12 shadda marks; all
physical outlines match. Final actual-PDF measurements and tolerances are recorded
in [the comparison](cr-001-parity.md).
Focused Docker checks passed: 325 reader, 354 layout, 64 model, 79 PDF unit and
one Arabic PDF integration test. A positioned non-precomposable Latin capital accent
verifies that nonzero offsets outside Arabic stay unchanged; mixed Hebrew clusters
also retain their glyphs, offsets and advances. The initial lowercase/precomposable
negative fixtures had zero offsets, so they were not accepted as proof of this guard.

The strict public JSON audit decoded all 154 reports before refreshing anything.
Exactly 106 baselines changed (101 general fixtures, five Meridian): 364 new paragraph
`reading_order` values, all `LeftToRight`. There were zero object/heading projections,
removed lines or unrelated changes. This is newly retained stored metadata, not a
visual change to those public reports. The JSON corpus and existing KDL contract are
also checked by the final workspace sweep. Fresh-pair results follow in the comparison
record; raw audit evidence remains private.

## Private reproducible evidence

All artifacts remain beneath ignored `research/stc-verification/cr-001-parity`:

- `inspect-native-arabic.ps1` and `native-arabic-metadata.log`: SDK object, paragraph
  and enum inspection, including unchanged-template confirmation.
- `probe-native-reading-order.ps1` and `native-arabic-direction-pair.log`: copied-report
  RTL/LTR setter probe and native reopen results.
- `arabic-direction-probe/reading-order-rtl.rpt` and `reading-order-ltr.rpt`: private
  canonical pair; never commit these reports.
- `arabic-direction-probe/pair-0x00c0.json` and `pair-0x00a5.json`: existing-CLI raw
  record dumps used for the byte comparison.
- `review/arabic-investigation/uniscribe-offsets.json` and
  `mark-offset-sign-evidence.json`: independent Windows shaping and native/Rust
  paint-origin evidence. `review/krilla-y-offset.txt` records the cached primary
  implementation's standard offset convention.

No private report text, student payload, rendered PDF or template belongs in Git.
