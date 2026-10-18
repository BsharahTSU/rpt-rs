//! Synthetic shaping fixtures built from the bundled open DejaVu font in memory.

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

fn words(values: &[u16]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_be_bytes()).collect()
}

/// A font with two intentionally synthetic discretionary ligatures: two Arabic
/// alefs become an alef-with-madda glyph; two Latin `f`s become an ampersand glyph.
/// Both share the same dlig lookup. This makes Arabic masking, default-off behavior,
/// cluster preservation and changed measured widths testable without host fonts.
/// No real report or name is encoded in this fixture.
///
/// # Panics
///
/// Panics if the bundled DejaVu fixture lacks the expected characters/tables or
/// the rebuilt test font does not parse. Those are test-fixture invariants.
#[must_use]
pub fn discretionary_ligature_font() -> Vec<u8> {
    const FONT: &[u8] = include_bytes!("../../rpt-text/fonts/DejaVuSans.ttf");
    let face = ttf_parser::Face::parse(FONT, 0).unwrap();
    let glyph = |c| face.glyph_index(c).unwrap().0;
    let mut pairs = [(glyph('ا'), glyph('آ')), (glyph('f'), glyph('&'))];
    pairs.sort_unstable_by_key(|(input, _)| *input);
    // LigatureSubst format 1; two sets, each containing one two-component ligature.
    let mut subtable = words(&[1, 30, 2, 10, 20]);
    for (input, output) in pairs {
        subtable.extend(words(&[1, 4, output, 2, input]));
    }
    subtable.extend(words(&[1, 2, pairs[0].0, pairs[1].0]));
    let mut lookup = words(&[1, 4, 4, 0, 1, 8]);
    lookup.extend(subtable);
    let mut scripts = words(&[3]);
    for tag in [b"DFLT", b"arab", b"latn"] {
        scripts.extend(tag);
        scripts.extend(words(&[20]));
    }
    scripts.extend(words(&[4, 0, 0, 0xffff, 1, 0]));
    let mut features = words(&[1]);
    features.extend(b"dlig");
    features.extend(words(&[8, 0, 1, 0]));
    let mut gsub = words(&[1, 0, 10, 42, 56]);
    gsub.extend(scripts);
    gsub.extend(features);
    gsub.extend(lookup);
    let mut tables: Vec<([u8; 4], Vec<u8>)> = (0..usize::from(u16_at(FONT, 4)))
        .map(|index| {
            let directory = 12 + index * 16;
            let tag = FONT[directory..directory + 4].try_into().unwrap();
            let offset = u32_at(FONT, directory + 8) as usize;
            let length = u32_at(FONT, directory + 12) as usize;
            let mut bytes = if tag == *b"GSUB" {
                gsub.clone()
            } else {
                FONT[offset..offset + length].to_vec()
            };
            if tag == *b"head" {
                bytes[8..12].fill(0);
            }
            (tag, bytes)
        })
        .collect();
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
