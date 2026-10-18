# CR-005 legacy bitmap color conversion

## Scope and current checkpoint (2026-10-18)

CR-005 text/layout is already accepted within the tolerances in [its comparison](cr-005-parity.md).
This follow-up addresses the remaining shared CR-001/003/005 logo colors. It must preserve the
original report and frozen data, and must not introduce report hashes, copied palettes or a
replacement logo into runtime code. The default renderer should preserve source colors; the
measured legacy conversion belongs to the explicitly selected STC compatibility path.

The first checkpoint implements and tests palette generation only. It is not yet enabled in PDF
output. Pixel lookup and final regenerated PDF acceptance remain open at this checkpoint.

## Native behavior established

The captured asset is an opaque bottom-up 24-bit BI_RGB bitmap. Decoding it in Rust already
preserves every stored pixel. The installed native export instead produces an indexed image.
Read-only inspection and direct native synthetic-image controls establish these palette rules:

1. Sample stored rows from the bottom upward at a stride of three rows. Cycle the starting
   column through 0, 1 and 2; visit every third pixel in each sampled row. Row padding is ignored.
2. Insert full RGB values in a six-level octree, using the high six bits of each component for
   the tree path. Keep at most 236 leaves by reducing after each insertion.
3. Reduce the most recently created internal node at the deepest remaining level. Accumulate
   its child counts and full-byte component sums; later samples keep contributing to that leaf.
4. Visit children in RGB-bit order and emit floor(sum/count) for each component.

The independent implementation reproduces every byte of the captured 218-entry palette.
Native controls also confirm generated solid, phased-color, grayscale, odd-width random and
square random images. Both random controls exercise online reduction to 236 entries.
No private image, native library, disassembly or report-specific palette is committed.

Palette generation alone does not prove image matching. Native conversion uses an 8-bit DIB
and GDI BitBlt; direct controls with the derived palette reproduce all 9,600 captured RGB pixels.
A simple nearest-color lookup, including a five-bit bin-center approximation, is insufficient.
The generic portable inverse mapping is still being verified; do not claim exact PDF color parity
or enable an approximation based on these palette results.

## Implementation and regression controls

`rpt-render-util::legacy_bmp_palette` takes validated tightly packed top-down opaque RGBA pixels.
It uses checked u64 counts/sums and reuses freed arena slots, keeping tree memory bounded by the
palette budget. Invalid dimensions, length, nonopaque pixels and arithmetic overflow return None.
There is no arbitrary image-size limit. Decoder scope and PDF opt-in are separate pending work.

Eleven new synthetic tests cover the five native palette fingerprints, sampling phase, narrow
rows, weighted full-byte centroids, newest/deepest reduction and later accumulation, allocation
reuse over 100,000 inserts, checked arithmetic, nonzero BMP row padding and invalid inputs.
The native fingerprints were recorded independently before the Rust implementation was built.
Docker Linux `cargo test --locked -p rpt-render-util` passes all 22 tests at this checkpoint.

## Evidence and resource handling

Private controls and logs: ignored `research/stc-verification/cr-005-logo/`, plus independently
recorded native controls under the previous comparison's `native-metrics/` and `logo-code-review/`.
Scratch: `D:\tmpppppp123\rpt-rs-cr005-logo`. Rust jobs remain sequential, two CPUs, 2 GiB memory
and a single Cargo/test thread. Existing Docker target/registry volumes are reused.
The user's screenshot shows the target volume at approximately 26 GB and overall Docker disk
use near 74.5 GB. These volumes are not claimed to live on D:; only bind-mounted scratch/evidence
are on D:. No cache pruning, global setting change, new volume, branch or worktree was performed.
At the palette checkpoint C: had about 285.6 GiB free and available host RAM remained above the
2 GiB build guard. Native probes and palette controls use synthetic inputs or frozen artifacts;
no SQL, original-template modification, application switch or deployment is involved.

## Remaining gates

- Prove generic pixel mapping against independent native controls, including palette ties and
  small/synthetic images; preserve rejected approaches as findings.
- Add guarded opt-in BMP conversion, default-path and unsupported-format regressions, and PDF
  embedding verification. Run affected suites including Chinese rendering.
- Regenerate both CR-005 PDFs from a clean committed implementation; check decoded image pixels,
  text/geometry and every rendered page. Replay the shared change against CR-001 and CR-003.
- Record measured acceptance and any remaining limits in current fork and backend references.
## Checkpoint 7b: portable pixel mapping and PDF integration

The inverse mapping is now independently characterized against native BitBlt. It uses 32 bins
per RGB axis, addressed in RGB order, with cell centers at 8*q+4. Directional scans retain their
starting positions and preserve the existing winner unless a strictly smaller squared distance
is found. The measured continuation bounds are zero; replacing this behavior with global nearest
color changes valid captured pixels. The implementation is an original coordinate algorithm,
verified against a separate state model and native controls; no proprietary code is distributed.

Six generated palette controls agree on every written native cell. All 416 bins used by the
captured logo are written, producing all 9,600 native RGB pixels exactly in the independent model.
Generated palettes are padded to the native 256-entry DIB table with black. Native cells not
visited by the scan can contain allocation-history values: the portable implementation instead
fills them with deterministic global-nearest matching at the bin center, first entry winning ties.
This fallback does not affect any captured logo pixel. It intentionally does not promise to
reproduce native undefined behavior for arbitrary images.

Implementation:

- `legacy_palette_indices` uses fixed 32^3 tables (about 160 KiB) with initialized entries.
- `decode_legacy_bmp_rgba` gates the composition on valid one-plane, bottom-up 24-bit BI_RGB BMPs,
  validates dimension/stride/output arithmetic, and leaves other BMP storage modes to the normal
  decoder. No original input bytes or Page IR assets are changed.
- `PdfOptions::legacy_bmp_quantization` defaults false. The experimental STC worker enables it;
  the general CLI retains false. PNG, JPEG and GIF paths are unchanged. One per-document image
  cache safely reuses the converted image across placements/pages.
- The profile does not currently parse Crystal's report-level retain-original-color-depth setting.
  Other storage modes and engine versions remain outside this measured compatibility contract.
  The characterization used Crystal 13.0.12.1494 and Windows GDI 10.0.26100.9444.

Regression validation before the final private export:

- 189 affected tests plus six doctests pass in capped Docker Linux: rendering, PDF, utilities,
  test support and worker. All seven Chinese checks pass; the printer profile explicitly enables
  the new bitmap option. Strict workspace Clippy and workspace formatting pass, worker builds.
- Six inverse-map tests cover six independent native fingerprints, nonnearest written cells,
  ties, deterministic holes, index bounds and invalid palettes. A checked-header test covers
  truncated/unsupported BMP inputs. Eleven palette regressions remain passing.
- Two PDF integration tests decode embedded RGB bytes from a generated 7x7 image: native color
  fingerprint, default original pixels, repeated-image reuse, unchanged drawing operators,
  explicit false/default equivalence, and top-down/32-bit fallback. Direct native quantization
  and GDI independently confirmed that synthetic fixture before its fingerprint was committed.
- JPEG output remains byte-identical with the option enabled. Six image-inspection helper tests
  cover raw/Flate RGB, malformed streams, unsupported filters/transforms, nested dictionary keys,
  duplicate/indirect properties and bounded inflation; no new package dependency was added.
- The first workspace Clippy pass found a full CLI options initializer missing the new default;
  it was repaired with explicit false, and the complete strict rerun passes.

Fresh original/Docker PDF generation and shared-case replay are the next acceptance gates.
