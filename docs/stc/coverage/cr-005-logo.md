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