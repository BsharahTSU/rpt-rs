# CR-001 native border and line geometry

Investigation: 2026-10-16. This record describes measurements and the generic correction they support. Root-owned layout code and its verification record determine whether the correction has been integrated.

## Method and scope

All native probes used the existing CR-001 frozen DataSet. No SQL command was executed. The original RPT was loaded read-only, selected objects were changed in memory, and the modified reports were saved only beneath ignored research/stc-verification/cr-001-parity. Each probe verified that the original template SHA-256 remained unchanged.

The probes changed border colors, selected sides and object height to distinguish geometry from content. Native PDF path operators were decoded with pypdf in Docker, preserving transformation matrices and stroke width. Measurements below use PDF points, with Y increasing upward. They describe path centerlines, not raster antialiasing edges.

Private reproducible evidence:

- inspect-native-borders.ps1: Engine/RAS object and border metadata.
- probe-native-borders.ps1: copied-report generator and frozen native export.
- probe-border-geometry.py: colored PDF path extraction.
- border-probe/: full single borders on two text objects and one field.
- border-probe-thick/: standalone horizontal/vertical thickness increased from 10 to 40 twips.
- border-probe-double/: full DoubleLine borders.
- border-probe-edges/: top-only, left-only and right-only borders.
- Each probe directory contains its copied RPT, native PDF and colored-geometry.json. Original bottom-only geometry is in dpi600/review/horizontal-rules.json.

No generated PDF, frozen data, copied private RPT or student payload belongs in Git. No source files were edited by this investigation other than this sanitized Markdown record.

## Text and field borders

Native SingleLine borders use a 1-point stroke whose center is **1.5 points outside the authored object edge**. This holds for full borders and isolated sides, for blank signature text and for a field containing report data. The measured 1.5-point center offset is consistent with 1-point clearance plus half the stroke; the RAS border exposes no separate configurable gap member.

| Probe | Authored size | Native centerline size | Result |
| --- | --- | --- | --- |
| Blank text, height 325 twips | 250.50 × 16.25 pt | 253.50 × 19.25 pt | 1.50 pt outward on every selected side |
| Blank text, height changed to 125 twips | 250.50 × 6.25 pt | 253.50 × 9.25 pt | Same offset; independent of text height |
| Report field, height 303 twips | 185.75 × 15.15 pt | 188.75 × 18.15 pt | Same rule for a data field |

The first signature object's authored left is 86 twips; the page X origin is 34.55 points. Its object X interval is therefore [38.85, 289.35]. A full border uses [37.35, 290.85], extending 1.50 points on both sides.

**Only selected sides extend the border rectangle.** The original bottom-only signature rule retains X endpoints [38.85, 289.35], while its center is 1.50 points below the object bottom. Controlled isolated-side measurements confirm the other directions:

| Selected side | Native centerline | Unchanged endpoints |
| --- | --- | --- |
| Top only | Y = 769.65, object top = 768.15 | X = [38.85, 289.35] |
| Left only | X = 318.05, object left = 319.55 | Y = [761.90, 768.15] |
| Right only, field | X = 316.05, object right = 314.55 | Y = [661.30, 676.45] |
| Bottom only, original signature | Y = 750.40, object bottom = 751.90 | X = [38.85, 289.35] |

A generic SingleLine implementation can therefore form a drawing rectangle from the actual placed object rectangle: subtract 30 twips from left/top only when their respective edges are enabled, and add 30 twips to right/bottom only when enabled. Draw each selected side against that adjusted rectangle. This preserves isolated-rule endpoints and joins full-border corners. It must not depend on report names, object names, student IDs, language or blank-text detection.

The existing reader's border model correctly records each side's line style. There is no stored extra 30-twip dimension to decode. The gap belongs to rendering geometry. The relevant implementation is crates/rpt-layout/src/place.rs, push_border; changing the object model bounds would incorrectly change the stored-facts JSON and unrelated layout.

## DoubleLine

A full native DoubleLine border emits two 1-point strokes. The inner stroke is at the same 1.50-point outward offset as SingleLine; the outer stroke is at 3.50 points outward, giving 2.00 points between centers.

For the first probe text, the inner rectangle is X=[37.35,290.85], Y=[750.40,769.65]; the outer is X=[35.35,292.85], Y=[748.40,771.65]. The field probe follows the same rule.

The current PDF writer maps both Single and Double styles to an undashed single path. Explicitly emitting the two measured Single strokes at layout time avoids claiming double-border support from that style token alone. Full uniform DoubleLine borders were verified; mixed Single/Double corner joins and isolated DoubleLine endpoints were not separately probed.

## Standalone line objects

Authored horizontal and vertical line coordinates locate the **top/left edge of the stroke**, while PDF strokes are centered on their path. Native output consequently shifts a horizontal path down by half its width and a vertical path right by half its width. Endpoints along the line's length remain unchanged.

| Line probe | 0.5-point width | 2-point width | Change |
| --- | --- | --- | --- |
| Horizontal center Y | 71.35 | 70.60 | Down 0.75 point; exactly half the width increase |
| Vertical center X | 338.60 | 339.35 | Right 0.75 point; exactly half the width increase |

The horizontal X interval remains [38.75,570.35]. The vertical Y interval remains [603.65,621.20]. The authored vertical X is 338.35, confirming the initial 0.25-point offset. This is generic stroke anchoring, independent of font metrics or section placement.

In page coordinates with Y increasing downward: horizontal lines add width/2 to both Y coordinates; vertical lines add width/2 to both X coordinates. A zero-width hairline retains zero offset. The relevant source is emit_line and line_endpoints in crates/rpt-layout/src/place.rs. Section-spanning endpoints must preserve this perpendicular offset when their terminal position is updated.

## Boundaries

Measurements establish SingleLine borders on text/fields, full uniform DoubleLine borders, and axis-aligned standalone lines at 0.5 and 2 points. The report carries saved 600-DPI printer metrics. Dashed/dotted borders, tight-horizontal borders, drop shadows, background-fill expansion, rotated text, diagonal lines and mixed DoubleLine corners were not measured here. Do not broaden parity claims to those cases without further evidence.

These probes explain the CR-001 signature-rule difference of 1.50 points and the standalone 0.5-point rule difference of 0.25 points. They do not establish complete report parity or authorize changes to publication settings, data or templates.

## Implemented scope

The CR-001 repair changes axis-aligned standalone line anchoring and text/field frames
whose selected borders are all SingleLine. Each selected side moves 30 twips outward;
isolated rules retain their authored endpoint extent. Mixed or other-style frames keep
their previous geometry. The DoubleLine measurements above are future implementation
evidence, not a claim that DoubleLine rendering is repaired by this checkpoint.

Synthetic regressions cover text and data-field borders, full and bottom-only frames,
two independent object heights, horizontal/vertical rules at zero, 0.5 and 2-point widths,
and the guard preserving mixed-style geometry. No fixture contains private report data.

## PDF content-stream baseline validation

The existing Docker workspace test stopped at the expected PDF operator baselines after
the geometry correction. Before refreshing them, the complete failure diff was checked:
all 568 added/removed lines were coordinate operands for `m` or `l`; no other operator
changed. The baselines were refreshed with `RPT_BLESS=1 cargo test --locked -p rpt-render
--test pdf_baselines` in `stc-rpt-rs:dev`, then independently compared line by line with
Git HEAD. Exactly two of the nine fixture files changed:

| Public fixture | Changed operator lines | Verified geometry |
| --- | ---: | --- |
| `benbrahim777/Top5USA_piechart` | 2 | One horizontal rule moved +0.50 pt in page-down Y, half its stored 20-twip / 1-point thickness. Its X endpoints did not change. |
| `benbrahim777/USA Orders, Percentages` | 282 | 47 top-border segments moved -1.50 pt and 94 bottom-border segments moved +1.50 pt in page-down Y. All X endpoints stayed fixed, matching the source fields' bottom-only and top/bottom-only SingleLine borders. |

These are 142 segments, each represented by one `m` and one `l`. Every other line in
both normalized PDF listings remained identical, including text, fonts, image operations,
colors, stroke widths, transformations and page counts. The other seven baselines were
unchanged. The private audit is `cr-001-parity/pdf-baseline-coordinate-audit.json` beneath
the ignored verification directory.

An unblessed Docker rerun passed all nine PDF fixtures across 15 pages, with zero skips.
Logs are `cr-001-parity/pdf-baselines-bless.log` and `pdf-baselines-verify.log`. This check
establishes the intentional public PDF regression changes; it does not replace the
separate frozen CR-001 native-versus-Rust comparison or claim complete workspace success.

Independent review also identified that a side-style guard alone would change unprobed
special frames. The implementation therefore preserves prior geometry for tight-horizontal,
shadowed, background-filled and rotated text/field frames. A synthetic regression covers
each exclusion (including both quarter-turn rotations). CR-001's signature objects have
none of those flags; the measured correction remains applicable.

Standalone offsets are likewise restricted to authored lines with exactly one positive
dimension and the other dimension zero. Two-dimensional and zero-length boxes retain their
previous geometry, with an explicit regression for each orientation and the degenerate case.
Every CR-001 standalone line meets the verified axis-aligned contract.

## Rounded box stroke anchoring

A follow-up on 2026-10-17 isolated the remaining ordinary rounded-box offset. The
existing final native/Rust PDFs were inspected with host Python/pypdf. One additional
native probe changed a representative outer box from 10-twip (0.5-point) to 40-twip
(2-point) thickness and colored its stroke for identification. It reused the frozen
DataSet, saved only a private report copy, and verified the original RPT hash before
and after export. No SQL, Cargo or Docker execution was needed for this investigation.

For the original 0.5-point box, the native page-one path-center rectangle is
[38.55, 75.00, 569.80, 621.80] versus Rust [38.30, 74.75, 570.05, 622.05], in
[left, bottom, right, top] PDF points. The thicker native probe yields
[39.30, 75.75, 569.05, 621.05]. Each edge therefore moves inward by precisely
0.75 point when the stroke grows by 1.50 points. Both page-two instances follow
the same rule. A second existing box with a 1-point stroke independently shows
a 0.50-point inset on all sides.

The supported generic rule is **centerline rectangle = final placed outer rectangle
inset by half the stroke width**: add half-width to left/top in page-down coordinates
and subtract the full stroke width from width/height. Resolve section-spanning and
page-continuation bounds before applying the inset. Preserve authored model bounds;
this is a paint-coordinate correction. Ordinary unfilled stroked rounded boxes were
measured. The unstroked filled rounded box already has matching native/Rust bounds
and must not receive this stroke inset. Filled-and-stroked, shadowed, unusually narrow
or square-corner boxes were not separately probed.

The corner diameter is independent of stroke thickness. The stored 225-twip diameter
produces alternating 112/113-twip distances to arc endpoints in the native output at
both tested thicknesses. The current layout truncates it to a uniform 112-twip radius.
For example, after the correct inset, the native bottom straight segment runs
X=[44.20, 564.20] at Y=75.00, while a uniform 112-twip radius would give
X=[44.15, 564.20]. Thus the inset resolves the 0.25-point centerline offset but alone
does not remove the separate 0.05-point odd-diameter endpoint difference.

All measured native 225-twip corner paths fit the following integer-twip construction.
Let r=floor(diameter/2)=112, R=ceil(diameter/2)=113 and k=62. In page-down coordinates
the top-right arc begins at (right-R, top), has controls (right-R+k, top) and
(right, top+r-k), and ends at (right, top+r). Subsequent corners rotate this pattern;
the top-left starting point is (left+r, top) and the bottom straight endpoints are
(left+R, bottom) and (right-r, bottom). The observed k equals round(r*KAPPA). The
existing fill-only 150-twip corner also fits r=R=75 and k=41. This is evidence for
the measured diameters, not a claim that arbitrary elliptical, clipped or degenerate
corners have been characterized.

Private evidence beneath the ignored verification directory:

- `cr-001-parity/inspect-rounded-boxes.py`: transformed curved-path extraction.
- `cr-001-parity/final/native-rounded-geometry.json` and `rust-rounded-geometry.json`:
  existing final PDF path coordinates, with no extracted text.
- `cr-001-parity/probe-native-box.ps1`: one controlled copied-report thickness probe.
- `cr-001-parity/box-probe-thick/native-rounded-geometry.json`: resulting native path
  coordinates; the private directory also retains the copied RPT and small probe PDF.

This section records findings only. Root-owned implementation and regression evidence
must state which box and corner corrections are actually integrated.

## Rounded-frame integration checkpoint

The layout now insets only circular rounded boxes with an ordinary SingleLine stroke,
no fill/shadow/tight/rotation, and authored dimensions greater than the stroke width.
Excluded profiles retain their previous bounds exactly. Span metadata carries the inset
through final-section endpoints, page-bottom closure and continuation tops; authored
layout extents, row positions and model data remain unchanged. Odd-diameter corner
rounding is unchanged and retains the separately measured maximum 0.05-point endpoint
residual; this checkpoint does not claim pixel-identical corner curves.

Docker verification passed: 351 layout tests, 12 worker tests, all five Chinese PDF
tests, all nine public PDF fixtures / 15 pages, strict workspace Clippy and formatting.
No additional PDF reference changes were required. Rebuilt the worker with one build
job in a 2-GiB-limited container. Detailed glyph review also found two pre-existing
Arabic placement differences (sentence-final punctuation and a footer shadda); those
are separate from the completed box inset. Their subsequent measured fixes and
final bounds are recorded in [the comparison](cr-001-parity.md).
