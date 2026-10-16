# CR-001 Basic-Latin spacing

## Measured cause and correction

The original Crystal PDF uses unkerned advances for the matched English text. Rust's
OpenType pair kerning shifted the A after a leading space by -0.49533 pt and the r in Wr
by -0.16456 pt. The largest error among 63 matched English body lines was 0.64045 pt.
These are actual PDF text-operator positions, not extraction bounding boxes.

The STC worker now opts into the same Basic-Latin spacing policy on both sides:
WindowsLayout::without_latin_kerning controls measurement/wrapping, and
PdfOptions::disable_latin_kerning controls painted glyph advances. General library and CLI
defaults remain unchanged. A shared UTF-8 range helper selects contiguous ASCII spans
containing a Latin letter, including adjacent spaces and punctuation. Non-ASCII characters
stay outside the feature masks. Only the kern feature is disabled; Unicode shaping,
Arabic joining, substitutions, ligatures, mark positioning and font fallback remain active.

The PDF feature ranges use explicit exclusive byte endpoints. The installed harfrust
RangeBounds constructor changes an excluded end, which could omit the final character;
the regression includes a kern pair at the end of the selected range.

## Verified replay

The private spacing replay consumes the same frozen CR-001 snapshot as the original
baseline; no SQL is executed. Across the same 63 English body lines, the largest glyph
position error falls from 0.64045 to 0.05832 pt (about 91%). The leading A error falls
to -0.00150 pt. The remaining tiny difference includes native integer-thousandth font
widths versus Rust fractional font widths.

All 63 line breaks, all 34 ordered ratings, their page allocation and the restored boundary
heading are unchanged. All 2,015 non-ASCII glyphs (815 / 910 / 290 by page), including
Arabic, retain identical text and exact X/Y positions relative to the DPI600 replay.
All three page pairs were independently inspected. Signatures/rule geometry were still
separate issues at this checkpoint; see [geometry](cr-001-geometry.md).

## Regression coverage

- Real font kern pairs versus nominal character advances, with character spacing.
- Shared UTF-8 byte ranges and their final-character boundary.
- Actual PDF option transport and byte-identical Arabic-only output.
- Arabic joining and Latin ligature glyph identities/positions.
- A wrap threshold between kerned and unkerned measured widths.
- Composed and separately positioned combining accents: glyph IDs, clusters, positions,
  widths, offsets and bidi levels, not only total width.
- Chinese simplified/traditional/mixed text and Arial fallback through an actual PDF
  round trip using the 600-DPI, unkerned compatibility profile.

Focused Docker text/PDF/worker tests and the five Chinese integration tests pass.
The ordinary shaping profile retains its default behavior. Accented Latin and other
non-ASCII scripts are deliberately outside this narrow kerning override.

Implementation: [shared ranges](../../../crates/rpt-text/src/compatibility.rs),
[measurement](../../../crates/rpt-text/src/cosmic.rs),
[PDF shaping](../../../crates/rpt-render-pdf/src/writer_krilla.rs),
[worker](../../../apps/stc-render-worker/src/main.rs).
Private coordinate evidence is in research/stc-verification/cr-001-parity/spacing/review.
No font binaries, private report data or generated PDFs are committed.
