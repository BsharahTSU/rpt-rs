//! Opt-in composition of the measured legacy bitmap palette and inverse mapping.

/// Decode and quantize a bottom-up, 24-bit, uncompressed Windows BMP to opaque RGBA8.
///
/// This compatibility path covers BITMAPINFOHEADER (or larger) with one color plane only.
/// Other storage modes return `None`, allowing callers to retain their normal BMP decoder.
/// Invalid input or checked-arithmetic failure also returns `None`. The source is never changed.
/// Generated palettes are padded to the native 256-entry DIB table with black. Every lookup
/// index is initialized deterministically; native unwritten-cell memory is not reproduced.
#[must_use]
pub fn decode_legacy_bmp_rgba(data: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
    if data.len() < 54 || &data[..2] != b"BM" {
        return None;
    }
    let header = u32::from_le_bytes(data[14..18].try_into().ok()?);
    let offset = u32::from_le_bytes(data[10..14].try_into().ok()?);
    let height = i32::from_le_bytes(data[22..26].try_into().ok()?);
    if header < 40
        || offset < 14u32.checked_add(header)?
        || height <= 0
        || u16::from_le_bytes(data[26..28].try_into().ok()?) != 1
        || u16::from_le_bytes(data[28..30].try_into().ok()?) != 24
        || u32::from_le_bytes(data[30..34].try_into().ok()?) != 0
    {
        return None;
    }
    // Prove the existing decoder's stride and allocation arithmetic on 32-bit targets too.
    let width = i32::from_le_bytes(data[18..22].try_into().ok()?);
    let width = usize::try_from(width).ok().filter(|&width| width != 0)?;
    let rows = usize::try_from(height).ok()?;
    let stride = width.checked_mul(3)?.checked_add(3)? & !3;
    width.checked_mul(rows)?.checked_mul(4)?;
    if usize::try_from(offset)
        .ok()?
        .checked_add(stride.checked_mul(rows)?)?
        > data.len()
    {
        return None;
    }
    let (mut rgba, width, height) = crate::decode_bmp_rgba(data)?;
    let mut palette = crate::legacy_bmp_palette(&rgba, width, height)?;
    palette.resize(256, [0; 3]);
    let indices = crate::legacy_palette_indices(&palette)?;
    for pixel in rgba.chunks_exact_mut(4) {
        let bin = (usize::from(pixel[0] >> 3) << 10)
            | (usize::from(pixel[1] >> 3) << 5)
            | usize::from(pixel[2] >> 3);
        pixel[..3].copy_from_slice(&palette[usize::from(indices[bin])]);
    }
    Some((rgba, width, height))
}
#[cfg(test)]
mod tests {
    use super::*;

    fn bitmap() -> Vec<u8> {
        let mut bytes = vec![0; 58];
        bytes[..2].copy_from_slice(b"BM");
        bytes[10..14].copy_from_slice(&54u32.to_le_bytes());
        bytes[14..18].copy_from_slice(&40u32.to_le_bytes());
        bytes[18..22].copy_from_slice(&1i32.to_le_bytes());
        bytes[22..26].copy_from_slice(&1i32.to_le_bytes());
        bytes[26..28].copy_from_slice(&1u16.to_le_bytes());
        bytes[28..30].copy_from_slice(&24u16.to_le_bytes());
        bytes[54..57].copy_from_slice(&[40, 80, 16]);
        bytes
    }

    #[test]
    fn rejects_truncated_or_uncharacterized_bmp_headers_without_modifying_source() {
        let original = bitmap();
        for length in 0..original.len() {
            assert!(decode_legacy_bmp_rgba(&original[..length]).is_none());
        }
        for (offset, value) in [
            (10, 53u32),
            (14, u32::MAX),
            (18, 0),
            (22, 0),
            (22, u32::MAX),
            (30, 1),
        ] {
            let mut invalid = original.clone();
            invalid[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
            assert!(decode_legacy_bmp_rgba(&invalid).is_none());
        }
        for (offset, value) in [(26, 0u16), (26, 2), (28, 32), (28, 8)] {
            let mut invalid = original.clone();
            invalid[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
            assert!(decode_legacy_bmp_rgba(&invalid).is_none());
        }
        let result = decode_legacy_bmp_rgba(&original).unwrap();
        assert_eq!((result.1, result.2), (1, 1));
        assert_eq!(original, bitmap());
    }
}
