//! Exercise the real device-width wrapping path without platform or proprietary fonts.
//! A synthetic hdmx table is added in memory to the already bundled open font.
use super::*;

const FONT: &[u8] = include_bytes!("../fonts/LiberationSans-Regular.ttf");
const PPEM: u8 = 83;
const DPI: u32 = 600;

fn u16_at(bytes: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes(bytes[offset..offset + 2].try_into().unwrap())
}

fn u32_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn checksum(bytes: &[u8]) -> u32 {
    bytes.chunks(4).fold(0_u32, |sum, chunk| {
        let mut word = [0; 4];
        word[..chunk.len()].copy_from_slice(chunk);
        sum.wrapping_add(u32::from_be_bytes(word))
    })
}

fn synthetic_advances() -> Vec<u8> {
    let face = ttf_parser::Face::parse(FONT, 0).unwrap();
    // Different glyph IDs intentionally receive different advances. A lookup by
    // character code rather than the shaped glyph ID must not pass these tests.
    (0..face.number_of_glyphs())
        .map(|glyph| 5 + (glyph % 11) as u8)
        .collect()
}

fn hdmx_table() -> Vec<u8> {
    let advances = synthetic_advances();
    let record_size = (advances.len() + 2).next_multiple_of(4);
    let mut table = vec![0; 8 + record_size];
    table[2..4].copy_from_slice(&1_u16.to_be_bytes());
    table[4..8].copy_from_slice(&(record_size as u32).to_be_bytes());
    table[8] = PPEM;
    table[9] = *advances.iter().max().unwrap();
    table[10..10 + advances.len()].copy_from_slice(&advances);
    table
}

fn font_with_table(hdmx: Option<Vec<u8>>) -> Vec<u8> {
    let mut tables: Vec<([u8; 4], Vec<u8>)> = (0..usize::from(u16_at(FONT, 4)))
        .filter_map(|index| {
            let directory = 12 + index * 16;
            let tag = FONT[directory..directory + 4].try_into().unwrap();
            if tag == *b"hdmx" {
                return None;
            }
            let offset = u32_at(FONT, directory + 8) as usize;
            let length = u32_at(FONT, directory + 12) as usize;
            let mut bytes = FONT[offset..offset + length].to_vec();
            if tag == *b"head" {
                bytes[8..12].fill(0);
            }
            Some((tag, bytes))
        })
        .collect();
    if let Some(table) = hdmx {
        tables.push((*b"hdmx", table));
    }
    tables.sort_by_key(|(tag, _)| *tag);
    let count = tables.len() as u16;
    let exponent = u16::BITS - 1 - count.leading_zeros();
    let search_range = (1_u16 << exponent) * 16;
    let mut result = vec![0; 12 + tables.len() * 16];
    result[..4].copy_from_slice(&FONT[..4]);
    result[4..6].copy_from_slice(&count.to_be_bytes());
    result[6..8].copy_from_slice(&search_range.to_be_bytes());
    result[8..10].copy_from_slice(&(exponent as u16).to_be_bytes());
    result[10..12].copy_from_slice(&(count * 16 - search_range).to_be_bytes());
    let mut head = None;
    for (index, (tag, bytes)) in tables.iter().enumerate() {
        let directory = 12 + index * 16;
        let offset = result.len();
        result[directory..directory + 4].copy_from_slice(tag);
        result[directory + 4..directory + 8].copy_from_slice(&checksum(bytes).to_be_bytes());
        result[directory + 8..directory + 12].copy_from_slice(&(offset as u32).to_be_bytes());
        result[directory + 12..directory + 16].copy_from_slice(&(bytes.len() as u32).to_be_bytes());
        if tag == b"head" {
            head = Some(offset);
        }
        result.extend_from_slice(bytes);
        result.resize(result.len().next_multiple_of(4), 0);
    }
    let adjustment = 0xb1b0_afba_u32.wrapping_sub(checksum(&result));
    let head = head.unwrap();
    result[head + 8..head + 12].copy_from_slice(&adjustment.to_be_bytes());
    assert!(ttf_parser::Face::parse(&result, 0).is_ok());
    result
}

fn layout(hdmx: Option<Vec<u8>>, unkerned: bool) -> CosmicLayout {
    let mut layout = CosmicLayout::new(FontProvider::bundled());
    {
        let mut system = layout.font_system.borrow_mut();
        let old_faces: Vec<_> = system.db().faces().map(|face| face.id).collect();
        for id in old_faces {
            system.db_mut().remove_face(id);
        }
        system.db_mut().load_font_data(font_with_table(hdmx));
        assert_eq!(system.db().faces().count(), 1);
    }
    if unkerned {
        layout = layout.without_latin_kerning();
    }
    layout
}

fn spec() -> FontSpec {
    FontSpec {
        family: "Liberation Sans".into(),
        size_pt: 10.0,
        ..Default::default()
    }
}

fn hinted_width(text: &str) -> f64 {
    let face = ttf_parser::Face::parse(FONT, 0).unwrap();
    let advances = synthetic_advances();
    text.chars()
        .map(|ch| f64::from(advances[usize::from(face.glyph_index(ch).unwrap().0)]))
        .sum::<f64>()
        * 1440.0
        / f64::from(DPI)
}

#[test]
fn hinted_fit_uses_shaped_glyph_ids_and_the_existing_word_breaker() {
    let layout = layout(Some(hdmx_table()), true);
    let font = spec();
    // The epsilon only avoids f32 arithmetic at the exact synthetic boundary.
    let width = hinted_width("alpha beta") + 0.01;
    assert_eq!(
        layout.wrap_with_device_widths("alpha beta gamma", width, &font, DPI),
        Some(vec!["alpha beta".into(), "gamma".into()])
    );
    assert_ne!(
        layout.wrap("alpha beta gamma", width, &font),
        ["alpha beta", "gamma"],
        "the record must actually change fitting rather than fall back silently"
    );
}

#[test]
fn hinted_fit_does_not_change_nominal_metrics_or_cached_shaping() {
    let layout = layout(Some(hdmx_table()), true);
    let font = spec();
    let text = "alpha beta gamma";
    let width = hinted_width("alpha beta") + 0.01;
    let nominal_width = layout.width_twips(text, &font);
    let nominal_wrap = layout.wrap(text, width, &font);
    let shaped_before = layout.shaped(text, &font, None);
    let original_advances: Vec<_> = shaped_before
        .layout_runs()
        .flat_map(|run| run.glyphs.iter())
        .map(|glyph| (glyph.glyph_id, glyph.w))
        .collect();
    let first = layout.wrap_with_device_widths(text, width, &font, DPI);
    assert!(first.is_some());
    assert_eq!(
        layout.wrap_with_device_widths(text, width, &font, DPI),
        first
    );
    assert_eq!(layout.width_twips(text, &font), nominal_width);
    assert_eq!(layout.wrap(text, width, &font), nominal_wrap);
    let shaped_after = layout.shaped(text, &font, None);
    let later_advances: Vec<_> = shaped_after
        .layout_runs()
        .flat_map(|run| run.glyphs.iter())
        .map(|glyph| (glyph.glyph_id, glyph.w))
        .collect();
    assert_eq!(later_advances, original_advances);
}

#[test]
fn hinted_fit_requires_opt_in_printable_ascii_and_exact_device_size() {
    let font = spec();
    let ordinary = layout(Some(hdmx_table()), false);
    assert_eq!(
        ordinary.wrap_with_device_widths("alpha", 1000.0, &font, DPI),
        None
    );
    let layout = layout(Some(hdmx_table()), true);
    for text in [
        "",
        "alpha\t beta",
        "alpha\n beta",
        "alpha 中文",
        "alpha مرحبا",
        "alpha\u{200e}",
    ] {
        assert_eq!(
            layout.wrap_with_device_widths(text, 1000.0, &font, DPI),
            None,
            "{text:?}"
        );
    }
    for dpi in [0, 96, 1440, 9600] {
        assert_eq!(
            layout.wrap_with_device_widths("alpha", 1000.0, &font, dpi),
            None
        );
    }
    let other_size = FontSpec {
        size_pt: 11.0,
        ..font
    };
    assert_eq!(
        layout.wrap_with_device_widths("alpha", 1000.0, &other_size, DPI),
        None
    );
}

#[test]
fn absent_or_malformed_hdmx_keeps_nominal_fitting_available() {
    let mut bad_version = hdmx_table();
    bad_version[1] = 1;
    let mut truncated = hdmx_table();
    truncated.pop();
    let mut short_record = hdmx_table();
    short_record[4..8].copy_from_slice(&4_u32.to_be_bytes());
    for table in [None, Some(bad_version), Some(truncated), Some(short_record)] {
        let layout = layout(table, true);
        let font = spec();
        assert_eq!(
            layout.wrap_with_device_widths("alpha beta", 1000.0, &font, DPI),
            None
        );
        assert_eq!(layout.wrap("alpha beta", 100000.0, &font), ["alpha beta"]);
    }
}
