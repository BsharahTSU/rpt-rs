//! Portable inverse palette for the characterized legacy Windows bitmap conversion.
//!
//! Native controls establish a five-bit RGB lookup with directional scans and retained scan
//! centers. Its continuation bounds remain zero. This leaves some cells unvisited; their native
//! bytes depend on allocation history. We reproduce the written cells and fill every remaining
//! cell with deterministic nearest-color matching instead of carrying undefined memory forward.

const SIDE: usize = 32;
const CELLS: usize = SIDE * SIDE * SIDE;
const UNWRITTEN: u32 = u32::MAX;

/// Build the legacy bitmap converter's five-bit RGB inverse palette.
///
/// The result has 32,768 entries, addressed by `(r >> 3) * 1024 + (g >> 3) * 32 + (b >> 3)`.
/// Native directional scans preserve palette order and update a visited cell only for a strictly
/// smaller squared RGB distance from its center. Unvisited cells use global nearest color, with
/// the first palette entry winning ties. Every returned index is valid for the supplied palette.
///
/// Returns `None` for an empty palette or more than 256 entries. The caller must supply any native
/// black padding explicitly; this helper neither invents palette colors nor changes pixel alpha.
#[must_use]
pub fn legacy_palette_indices(palette: &[[u8; 3]]) -> Option<Vec<u8>> {
    let mut table = InverseTable::scan(palette)?;
    table.fill_unwritten(palette);
    Some(table.indices)
}

#[derive(Debug)]
struct InverseTable {
    indices: Vec<u8>,
    // The sentinel is also the written mask; valid distances are at most 3 * 255^2.
    distances: Vec<u32>,
}

#[derive(Debug)]
struct ColorScan {
    index: u8,
    center: [usize; 3],
    axis_distance: [[u32; SIDE]; 3],
    green_start: usize,
    blue_start: usize,
}

impl ColorScan {
    fn new(index: u8, rgb: [u8; 3]) -> Self {
        let center = rgb.map(|channel| usize::from(channel >> 3));
        let axis_distance = rgb.map(|channel| {
            std::array::from_fn(|cell| {
                let delta = cell_center(cell) - i32::from(channel);
                (delta * delta) as u32
            })
        });
        Self {
            index,
            center,
            axis_distance,
            green_start: center[1],
            blue_start: center[2],
        }
    }

    fn distance(&self, red: usize, green: usize, blue: usize) -> u32 {
        self.axis_distance[0][red] + self.axis_distance[1][green] + self.axis_distance[2][blue]
    }
}

fn cell_center(cell: usize) -> i32 {
    (cell * 8 + 4) as i32
}

fn cell_index(red: usize, green: usize, blue: usize) -> usize {
    red * SIDE * SIDE + green * SIDE + blue
}

impl InverseTable {
    fn scan(palette: &[[u8; 3]]) -> Option<Self> {
        if palette.is_empty() || palette.len() > 256 {
            return None;
        }
        let mut table = Self {
            indices: vec![0; CELLS],
            distances: vec![UNWRITTEN; CELLS],
        };
        for (index, &rgb) in palette.iter().enumerate() {
            let mut scan = ColorScan::new(index as u8, rgb);
            table.red(&mut scan);
        }
        Some(table)
    }

    fn update(&mut self, scan: &ColorScan, red: usize, green: usize, blue: usize) -> bool {
        let cell = cell_index(red, green, blue);
        let distance = scan.distance(red, green, blue);
        if distance >= self.distances[cell] {
            return false;
        }
        self.distances[cell] = distance;
        self.indices[cell] = scan.index;
        true
    }

    fn blue(&mut self, scan: &mut ColorScan, red: usize, green: usize, restart: bool) -> bool {
        if restart {
            scan.blue_start = scan.center[2];
        }
        // Native starts a fresh directional scan over the whole axis. Continuations use its
        // zero cached upper bound; replacing this with SIDE - 1 changes real native colors.
        let upper = if restart { SIDE - 1 } else { 0 };
        let mut seen = false;
        for blue in scan.blue_start..=upper {
            if self.update(scan, red, green, blue) {
                if !seen && blue > scan.blue_start {
                    scan.blue_start = blue;
                }
                seen = true;
            } else if seen {
                break;
            }
        }
        for blue in (0..scan.blue_start).rev() {
            if self.update(scan, red, green, blue) {
                if !seen {
                    scan.blue_start = blue;
                }
                seen = true;
            } else if seen {
                break;
            }
        }
        seen
    }

    fn green(&mut self, scan: &mut ColorScan, red: usize, restart: bool) -> bool {
        if restart {
            scan.green_start = scan.center[1];
        }
        let upper = if restart { SIDE - 1 } else { 0 };
        let mut seen = false;
        let mut first = true;
        for green in scan.green_start..=upper {
            if self.blue(scan, red, green, first) {
                if !seen && green > scan.green_start {
                    scan.green_start = green;
                }
                seen = true;
            } else if seen {
                break;
            }
            first = false;
        }
        first = true;
        for green in (0..scan.green_start).rev() {
            if self.blue(scan, red, green, first) {
                if !seen {
                    scan.green_start = green;
                }
                seen = true;
            } else if seen {
                break;
            }
            first = false;
        }
        seen
    }

    fn red(&mut self, scan: &mut ColorScan) {
        let mut seen = false;
        let mut first = true;
        for red in scan.center[0]..SIDE {
            if self.green(scan, red, first) {
                seen = true;
            } else if seen {
                break;
            }
            first = false;
        }
        first = true;
        for red in (0..scan.center[0]).rev() {
            if self.green(scan, red, first) {
                seen = true;
            } else if seen {
                break;
            }
            first = false;
        }
    }

    fn fill_unwritten(&mut self, palette: &[[u8; 3]]) {
        for cell in 0..CELLS {
            if self.distances[cell] != UNWRITTEN {
                continue;
            }
            let center = [
                cell_center(cell / (SIDE * SIDE)),
                cell_center((cell / SIDE) % SIDE),
                cell_center(cell % SIDE),
            ];
            let mut best = UNWRITTEN;
            for (index, rgb) in palette.iter().enumerate() {
                let distance = center
                    .iter()
                    .zip(rgb)
                    .map(|(&component, &channel)| {
                        let delta = component - i32::from(channel);
                        (delta * delta) as u32
                    })
                    .sum();
                if distance < best {
                    best = distance;
                    self.indices[cell] = index as u8;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn padded(colors: &[[u8; 3]]) -> Vec<[u8; 3]> {
        let mut palette = colors.to_vec();
        palette.resize(256, [0, 0, 0]);
        palette
    }

    fn written_fingerprint(table: &InverseTable) -> (usize, u64) {
        let mut count = 0;
        let mut hash = 0xcbf2_9ce4_8422_2325_u64;
        for (cell, &distance) in table.distances.iter().enumerate() {
            if distance == UNWRITTEN {
                continue;
            }
            count += 1;
            for byte in [cell as u8, (cell >> 8) as u8, table.indices[cell]] {
                hash = (hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3);
            }
        }
        (count, hash)
    }

    #[test]
    fn rejects_invalid_palette_lengths() {
        assert!(legacy_palette_indices(&[]).is_none());
        assert!(legacy_palette_indices(&[[0, 0, 0]; 257]).is_none());
    }

    #[test]
    fn matches_native_written_cells_for_six_synthetic_palettes() {
        // Native BitBlt controls on generated palettes. Unwritten native cells are allocation
        // history, so the fingerprint records each written key and its index, not those bytes.
        let cases: &[(&[[u8; 3]], usize, u64)] = &[
            (&[[0, 0, 0], [255, 255, 255]], 12_553, 0xcdd9_0173_6efe_6481),
            (&[[255, 255, 255], [0, 0, 0]], 12_553, 0x42cc_0d51_e2dd_81de),
            (
                &[
                    [0, 0, 0],
                    [255, 0, 0],
                    [0, 255, 0],
                    [0, 0, 255],
                    [255, 255, 255],
                ],
                13_932,
                0x0889_2468_402d_2993,
            ),
            (
                &[[254, 254, 250], [254, 254, 254]],
                13_743,
                0x7830_ef3c_b91f_e43d,
            ),
            (
                &[[254, 254, 254], [254, 254, 250]],
                12_553,
                0x420c_65f1_62b8_7d16,
            ),
            (
                &[
                    [20, 80, 40],
                    [200, 40, 80],
                    [40, 60, 220],
                    [180, 210, 160],
                    [120, 120, 120],
                ],
                11_456,
                0xa8a8_e997_3f64_8a54,
            ),
        ];
        for &(colors, count, fingerprint) in cases {
            let table = InverseTable::scan(&padded(colors)).expect("valid synthetic palette");
            assert_eq!(written_fingerprint(&table), (count, fingerprint));
        }
    }

    #[test]
    fn retains_native_non_nearest_written_cell() {
        let palette = padded(&[
            [20, 80, 40],
            [200, 40, 80],
            [40, 60, 220],
            [180, 210, 160],
            [120, 120, 120],
        ]);
        let mut table = InverseTable::scan(&palette).expect("valid palette");
        let cell = cell_index(0, 0, 15); // Native chooses blue; globally nearer green is not visited.
        assert_ne!(table.distances[cell], UNWRITTEN);
        assert_eq!(table.indices[cell], 2);
        table.fill_unwritten(&palette);
        assert_eq!(
            table.indices[cell], 2,
            "fallback must preserve native written cells"
        );
    }

    #[test]
    fn preserves_existing_cell_on_equal_distance() {
        for palette in [
            [[254, 254, 250], [254, 254, 254]],
            [[254, 254, 254], [254, 254, 250]],
        ] {
            let indices = legacy_palette_indices(&palette).expect("valid palette");
            assert_eq!(indices[cell_index(31, 31, 31)], 0);
        }
    }

    #[test]
    fn fills_unwritten_cells_with_deterministic_nearest_color() {
        let palette = [[0, 0, 0], [255, 255, 255]];
        let mut table = InverseTable::scan(&palette).expect("valid palette");
        let cell = cell_index(0, 16, 31); // Center (4,132,252) is nearer white.
        assert_eq!(table.distances[cell], UNWRITTEN);
        table.fill_unwritten(&palette);
        assert_eq!(table.indices[cell], 1);
        assert_eq!(table.indices, legacy_palette_indices(&palette).unwrap());
    }

    #[test]
    fn keeps_every_index_in_range_including_edges_and_single_color() {
        for palette in [vec![[17, 83, 219]], vec![[0, 0, 0], [255, 255, 255]]] {
            let indices = legacy_palette_indices(&palette).expect("valid palette");
            assert_eq!(indices.len(), CELLS);
            assert!(indices
                .iter()
                .all(|&index| usize::from(index) < palette.len()));
            if palette.len() == 1 {
                assert!(indices.iter().all(|&index| index == 0));
            }
            assert_eq!(indices, legacy_palette_indices(&palette).unwrap());
        }
    }
}
