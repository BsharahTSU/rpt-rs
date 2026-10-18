//! Sampled RGB octree used by the legacy bitmap export path.
//!
//! The traversal, reduction order and integer centroids follow independently generated native
//! synthetic-image controls. The implementation stores only the current tree and reuses freed
//! slots, so its working memory depends on the palette budget rather than the image dimensions.

const DEPTH: usize = 6;
const PALETTE_LIMIT: usize = 236;

/// Generate the legacy export palette from tightly packed, top-down, opaque RGBA8 pixels.
///
/// Samples start at the bottom row, visit every third row and cycle the starting column through
/// 0, 1 and 2. A six-level RGB octree is reduced online to at most 236 colors. Palette entries are
/// the floor of each leaf's full-byte RGB means, emitted in RGB-bit (Morton) order.
///
/// Returns `None` for zero or overflowing dimensions, an inexact buffer length, nonopaque pixels,
/// or an accumulation that cannot be represented by `u64`. There is no image-size cutoff.
/// BMP row padding and storage direction must already have been removed by the decoder.
#[must_use]
pub fn legacy_bmp_palette(rgba: &[u8], width: u32, height: u32) -> Option<Vec<[u8; 3]>> {
    let width = usize::try_from(width).ok()?;
    let height = usize::try_from(height).ok()?;
    if width == 0 || height == 0 {
        return None;
    }
    let length = width.checked_mul(height)?.checked_mul(4)?;
    if rgba.len() != length || rgba.chunks_exact(4).any(|pixel| pixel[3] != 255) {
        return None;
    }

    let mut tree = Octree::new();
    for (sample_row, y) in (0..height).rev().step_by(3).enumerate() {
        for x in (sample_row % 3..width).step_by(3) {
            // Exact dimensions and length above prove both the offset and the RGB slice fit.
            let offset = (y * width + x) * 4;
            tree.insert([rgba[offset], rgba[offset + 1], rgba[offset + 2]])?;
        }
    }
    tree.palette()
}

#[derive(Debug)]
struct Node {
    children: [Option<usize>; 8],
    next_reducible: Option<usize>,
    level: usize,
    leaf: bool,
    count: u64,
    sums: [u64; 3],
}

impl Node {
    fn new(level: usize) -> Self {
        Self {
            children: [None; 8],
            next_reducible: None,
            level,
            leaf: level == DEPTH,
            count: 0,
            sums: [0; 3],
        }
    }
}

#[derive(Debug)]
struct Octree {
    nodes: Vec<Node>,
    free: Vec<usize>,
    // Each head is the most recently created internal node at that depth.
    reducible: [Option<usize>; DEPTH],
    leaves: usize,
}

impl Octree {
    fn new() -> Self {
        let mut tree = Self {
            nodes: Vec::new(),
            free: Vec::new(),
            reducible: [None; DEPTH],
            leaves: 0,
        };
        tree.allocate(0);
        tree
    }

    fn allocate(&mut self, level: usize) -> usize {
        let node = Node::new(level);
        let index = if let Some(index) = self.free.pop() {
            self.nodes[index] = node;
            index
        } else {
            self.nodes.push(node);
            self.nodes.len() - 1
        };
        if level == DEPTH {
            self.leaves += 1;
        } else {
            self.nodes[index].next_reducible = self.reducible[level];
            self.reducible[level] = Some(index);
        }
        index
    }

    fn insert(&mut self, rgb: [u8; 3]) -> Option<()> {
        let mut index = 0;
        while !self.nodes[index].leaf {
            let level = self.nodes[index].level;
            let shift = 7 - level;
            let child = usize::from(
                (((rgb[0] >> shift) & 1) << 2)
                    | (((rgb[1] >> shift) & 1) << 1)
                    | ((rgb[2] >> shift) & 1),
            );
            index = if let Some(next) = self.nodes[index].children[child] {
                next
            } else {
                let next = self.allocate(level + 1);
                self.nodes[index].children[child] = Some(next);
                next
            };
        }
        let node = &mut self.nodes[index];
        node.count = node.count.checked_add(1)?;
        for (sum, channel) in node.sums.iter_mut().zip(rgb) {
            *sum = sum.checked_add(u64::from(channel))?;
        }
        while self.leaves > PALETTE_LIMIT {
            self.reduce()?;
        }
        Some(())
    }

    fn reduce(&mut self) -> Option<()> {
        let level = self.reducible.iter().rposition(Option::is_some)?;
        let index = self.reducible[level]?;
        self.reducible[level] = self.nodes[index].next_reducible;
        let children = std::mem::take(&mut self.nodes[index].children);
        let mut count = 0u64;
        let mut sums = [0u64; 3];
        for child in children.into_iter().flatten() {
            let node = &self.nodes[child];
            // Choosing the deepest remaining reducer guarantees every child is a leaf. A freed
            // child consequently cannot remain in a reducible list when its slot is reused.
            debug_assert!(node.leaf);
            count = count.checked_add(node.count)?;
            for (sum, channel) in sums.iter_mut().zip(node.sums) {
                *sum = sum.checked_add(channel)?;
            }
            self.free.push(child);
            self.leaves -= 1;
        }
        let node = &mut self.nodes[index];
        node.next_reducible = None;
        node.leaf = true;
        node.count = count;
        node.sums = sums;
        self.leaves += 1;
        Some(())
    }

    fn palette(&self) -> Option<Vec<[u8; 3]>> {
        let mut colors = Vec::with_capacity(self.leaves);
        self.visit(0, &mut colors)?;
        Some(colors)
    }

    fn visit(&self, index: usize, colors: &mut Vec<[u8; 3]>) -> Option<()> {
        let node = &self.nodes[index];
        if node.leaf {
            let mut color = [0; 3];
            for (channel, sum) in color.iter_mut().zip(node.sums) {
                *channel = u8::try_from(sum.checked_div(node.count)?).ok()?;
            }
            colors.push(color);
        } else {
            for child in node.children.into_iter().flatten() {
                self.visit(child, colors)?;
            }
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgba(width: u32, height: u32, mut pixel: impl FnMut(u32, u32) -> [u8; 3]) -> Vec<u8> {
        let mut pixels = Vec::new();
        for y in 0..height {
            for x in 0..width {
                pixels.extend_from_slice(&pixel(x, y));
                pixels.push(255);
            }
        }
        pixels
    }

    fn random_rgba(width: u32, height: u32, mut seed: u32) -> Vec<u8> {
        rgba(width, height, |_, _| {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            [(seed >> 24) as u8, (seed >> 16) as u8, (seed >> 8) as u8]
        })
    }

    fn fnv64(colors: &[[u8; 3]]) -> u64 {
        colors
            .iter()
            .flatten()
            .fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
                (hash ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3)
            })
    }

    fn assert_native_control(pixels: &[u8], width: u32, height: u32, count: usize, hash: u64) {
        let colors = legacy_bmp_palette(pixels, width, height).unwrap();
        assert_eq!(colors.len(), count);
        assert_eq!(fnv64(&colors), hash);
    }

    // These palette fingerprints were calculated from native-produced RGB palette bytes, not
    // from this implementation. Inputs are generated synthetic pixels with no report data.
    #[test]
    fn matches_native_single_and_phased_sampling_controls() {
        assert_native_control(&[13, 79, 181, 255], 1, 1, 1, 0xf26b_e818_7a0a_4ac0);
        let pixels = rgba(11, 13, |x, y| {
            [
                ((x * 23 + y * 7) % 256) as u8,
                ((x * 5 + y * 41) % 256) as u8,
                ((x * 37 + y * 11) % 256) as u8,
            ]
        });
        assert_native_control(&pixels, 11, 13, 19, 0x19b3_a903_26cf_614c);
    }

    #[test]
    fn matches_native_gray_centroids() {
        let pixels = rgba(257, 13, |x, _| [x as u8; 3]);
        assert_native_control(&pixels, 257, 13, 64, 0xe7e5_b85a_3046_6de5);
    }

    #[test]
    fn matches_native_online_reduction_for_odd_and_even_widths() {
        assert_native_control(
            &random_rgba(113, 97, 0x15_3492),
            113,
            97,
            236,
            0x2bc6_8306_edf7_c27c,
        );
        assert_native_control(
            &random_rgba(128, 128, 0x71_f34a),
            128,
            128,
            236,
            0x0d86_f834_c6bf_4bcf,
        );
    }

    #[test]
    fn samples_bottom_up_with_a_rotating_column_phase() {
        let pixels = rgba(4, 7, |x, y| match (x, y) {
            (0, 6) => [255, 0, 0],
            (3, 6) => [0, 255, 0],
            (1, 3) => [0, 0, 255],
            (2, 0) => [0, 0, 0],
            _ => [255, 255, 255], // Every white pixel must be skipped.
        });
        assert_eq!(
            legacy_bmp_palette(&pixels, 4, 7).unwrap(),
            [[0, 0, 0], [0, 0, 255], [0, 255, 0], [255, 0, 0]]
        );
    }

    #[test]
    fn narrow_rows_with_no_sample_do_not_reset_the_phase() {
        let pixels = rgba(1, 10, |_, y| if y == 9 { [8; 3] } else { [12; 3] });
        assert_eq!(
            legacy_bmp_palette(&pixels, 1, 10).unwrap(),
            [[8; 3], [12; 3]]
        );
    }

    #[test]
    fn six_bit_leaves_keep_weighted_full_byte_means() {
        let pixels = rgba(7, 1, |x, _| match x {
            0 => [8, 12, 16],
            3 | 6 => [11, 15, 19],
            _ => [255; 3],
        });
        assert_eq!(legacy_bmp_palette(&pixels, 7, 1).unwrap(), [[10, 14, 18]]);
    }

    #[test]
    fn reduction_uses_newest_deepest_node_and_preserves_later_samples() {
        let mut tree = Octree::new();
        for color in [[0, 0, 0], [0, 0, 4], [0, 8, 0], [0, 8, 4]] {
            tree.insert(color).unwrap();
        }
        tree.reduce().unwrap();
        assert_eq!(tree.palette().unwrap(), [[0, 0, 0], [0, 0, 4], [0, 8, 2]]);
        tree.insert([0, 8, 7]).unwrap();
        assert_eq!(tree.palette().unwrap(), [[0, 0, 0], [0, 0, 4], [0, 8, 3]]);
        tree.reduce().unwrap();
        assert_eq!(tree.palette().unwrap(), [[0, 0, 2], [0, 8, 3]]);
    }

    #[test]
    fn reuses_pruned_nodes_in_bounded_memory() {
        let mut tree = Octree::new();
        let mut seed = 0x71_f34au32;
        for _ in 0..100_000 {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            tree.insert([(seed >> 24) as u8, (seed >> 16) as u8, (seed >> 8) as u8])
                .unwrap();
            assert!(tree.leaves <= PALETTE_LIMIT);
            // Every live node lies on one of at most 237 paths of length six, including the
            // one insertion allowed before reduction. Freed slots are reused before growing.
            assert!(tree.nodes.len() <= 1 + (PALETTE_LIMIT + 1) * DEPTH);
        }
        assert!(!tree.free.is_empty());
        assert_eq!(tree.palette().unwrap().len(), tree.leaves);
    }

    #[test]
    fn accumulates_above_u32_and_checks_u64_overflow() {
        let mut tree = Octree::new();
        tree.insert([255; 3]).unwrap();
        let leaf = tree.nodes.iter().position(|node| node.leaf).unwrap();
        let count = u64::from(u32::MAX) + 3;
        tree.nodes[leaf].count = count;
        tree.nodes[leaf].sums = [count * 255; 3];
        tree.insert([255; 3]).unwrap();
        assert_eq!(tree.palette().unwrap(), [[255; 3]]);
        tree.nodes[leaf].sums = [u64::MAX; 3];
        assert!(tree.insert([255; 3]).is_none());
        tree.nodes[leaf].count = u64::MAX;
        assert!(tree.insert([255; 3]).is_none());
    }

    #[test]
    fn decoded_bmp_padding_does_not_contribute_colors() {
        // Five 24-bit pixels occupy 15 bytes plus one nonzero pad byte in each stored row.
        let width = 5usize;
        let height = 7usize;
        let stride = 16usize;
        let mut bmp = vec![0xcc; 54 + stride * height];
        bmp[..54].fill(0);
        bmp[..2].copy_from_slice(b"BM");
        bmp[10..14].copy_from_slice(&54u32.to_le_bytes());
        bmp[14..18].copy_from_slice(&40u32.to_le_bytes());
        bmp[18..22].copy_from_slice(&(width as i32).to_le_bytes());
        bmp[22..26].copy_from_slice(&(height as i32).to_le_bytes());
        bmp[26..28].copy_from_slice(&1u16.to_le_bytes());
        bmp[28..30].copy_from_slice(&24u16.to_le_bytes());
        for row in bmp[54..].chunks_exact_mut(stride) {
            for pixel in row[..width * 3].chunks_exact_mut(3) {
                pixel.copy_from_slice(&[181, 79, 13]);
            }
        }
        let (pixels, width, height) = crate::decode_bmp_rgba(&bmp).unwrap();
        assert_eq!(
            legacy_bmp_palette(&pixels, width, height).unwrap(),
            [[13, 79, 181]]
        );
    }

    #[test]
    fn rejects_invalid_dimensions_lengths_and_alpha() {
        assert!(legacy_bmp_palette(&[], 0, 1).is_none());
        assert!(legacy_bmp_palette(&[], 1, 0).is_none());
        assert!(legacy_bmp_palette(&[], u32::MAX, u32::MAX).is_none());
        let pixels = rgba(3, 2, |_, _| [13, 79, 181]);
        for end in 0..pixels.len() {
            assert!(legacy_bmp_palette(&pixels[..end], 3, 2).is_none());
        }
        let mut invalid = pixels.clone();
        invalid.push(255);
        assert!(legacy_bmp_palette(&invalid, 3, 2).is_none());
        // Even unsampled pixels must satisfy this opaque-BMP API's input contract.
        let mut invalid = pixels;
        invalid[3] = 254;
        assert!(legacy_bmp_palette(&invalid, 3, 2).is_none());
    }
}
