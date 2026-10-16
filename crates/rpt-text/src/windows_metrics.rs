//! Windows GDI font-cell metrics used by the legacy Crystal PDF export path.
//! VDMX records describe hinted pixel heights; at 1440 logical DPI one pixel is a twip.
//! https://learn.microsoft.com/en-us/typography/opentype/spec/vdmx
use crate::FontDb;
use rpt_pages::FontSpec;
#[cfg(feature = "cosmic")]
use rpt_pages::TextLayout;

/// Hinted Windows cell metrics, expressed in twips.
#[derive(Debug, Clone, Copy)]
pub struct WindowsFontMetrics {
    /// Glyph em size selected by the cell height.
    pub em_twips: i32,
    /// Baseline distance from the top of the font cell.
    pub ascent: i32,
    /// Positive distance below the baseline.
    pub descent: i32,
    /// External line leading.
    pub leading: i32,
}
fn u16_at(bytes: &[u8], i: usize) -> Option<u16> {
    Some(u16::from_be_bytes(
        bytes.get(i..i.checked_add(2)?)?.try_into().ok()?,
    ))
}
fn i16_at(bytes: &[u8], i: usize) -> Option<i16> {
    u16_at(bytes, i).map(|v| v as i16)
}
fn vdmx_records(bytes: &[u8]) -> Option<Vec<(i32, i32, i32)>> {
    let ratios = usize::from(u16_at(bytes, 4)?);
    for i in 0..ratios {
        let r = bytes.get(6 + 4 * i..10 + 4 * i)?;
        // Unicode coverage, square pixels (or the catch-all ratio).
        if r[0] != 1 || !(r[1] == 0 || (r[1] >= r[2] && r[1] <= r[3])) {
            continue;
        }
        let offset = usize::from(u16_at(bytes, 6 + ratios * 4 + i * 2)?);
        let count = usize::from(u16_at(bytes, offset)?);
        let mut entries = Vec::with_capacity(count);
        for j in 0..count {
            let at = offset + 4 + j * 6;
            entries.push((
                i32::from(u16_at(bytes, at)?),
                i32::from(i16_at(bytes, at + 2)?),
                -i32::from(i16_at(bytes, at + 4)?),
            ));
        }
        return Some(entries);
    }
    None
}
fn select_cell(records: &[(i32, i32, i32)], height: i32) -> Option<(i32, i32, i32)> {
    let mut previous = None;
    for &(em, ascent, descent) in records {
        let cell = ascent + descent;
        if cell == height {
            return Some((em, ascent, descent));
        }
        if cell > height {
            return previous;
        }
        previous = Some((em, ascent, descent));
    }
    None // outside VDMX coverage: use linear OS/2 metrics
}
impl FontDb {
    /// `cell_height` selects positive lfHeight (legacy PDF glyph size) versus
    /// negative lfHeight (nominal layout size). No Windows API/runtime is needed.
    pub fn windows_metrics(
        &self,
        font: &FontSpec,
        cell_height: bool,
    ) -> Option<WindowsFontMetrics> {
        let id = self.query(font)?;
        self.with_face_data(id, |bytes, index| {
            let face = ttf_parser::Face::parse(bytes, index).ok()?;
            let raw = face.raw_face();
            let os2 = raw.table(ttf_parser::Tag::from_bytes(b"OS/2"))?;
            let hhea = raw.table(ttf_parser::Tag::from_bytes(b"hhea"))?;
            let units = f64::from(face.units_per_em());
            let win_a = f64::from(u16_at(os2, 74)?);
            let win_d = f64::from(u16_at(os2, 76)?);
            if win_a + win_d <= 0.0 {
                return None;
            }
            let wanted = (font.size_pt * 20.0).round().max(1.0) as i32;
            let table = raw
                .table(ttf_parser::Tag::from_bytes(b"VDMX"))
                .and_then(vdmx_records);
            let hinted = table.as_ref().and_then(|r| {
                if cell_height {
                    select_cell(r, wanted)
                } else {
                    r.iter().copied().find(|r| r.0 == wanted)
                }
            });
            let (em, ascent, descent) = hinted.unwrap_or_else(|| {
                let em = if cell_height {
                    (f64::from(wanted) * units / (win_a + win_d)).round() as i32
                } else {
                    wanted
                };
                (
                    em,
                    (win_a * f64::from(em) / units).round() as i32,
                    (win_d * f64::from(em) / units).round() as i32,
                )
            });
            let leading = (f64::from(i16_at(hhea, 8)?) * f64::from(em) / units).round() as i32;
            Some(WindowsFontMetrics {
                em_twips: em,
                ascent,
                descent,
                leading: leading.max(0),
            })
        })?
    }
}

/// Nominal wrapping uses the normal Unicode shaper; vertical GDI metrics preserve
/// the existing Crystal row heights and baselines when the same fonts are mounted.
#[cfg(feature = "cosmic")]
#[derive(Debug)]
pub struct WindowsLayout {
    inner: crate::CosmicLayout,
    fonts: FontDb,
}
#[cfg(feature = "cosmic")]
impl WindowsLayout {
    /// Load nominal layout metrics and the selected face library.
    pub fn new(source: crate::FontSource) -> Self {
        Self {
            inner: crate::CosmicLayout::new(crate::FontProvider::from_source(source)),
            fonts: source.load(),
        }
    }
}
#[cfg(feature = "cosmic")]
impl TextLayout for WindowsLayout {
    fn width_twips(&self, text: &str, font: &FontSpec) -> f64 {
        self.inner.width_twips(text, font)
    }
    fn wrap(&self, text: &str, width: f64, font: &FontSpec) -> Vec<String> {
        // Crystal retains leading paragraph spaces for drawing, but excludes them
        // from the word-fit decision. This matters near an RTL line boundary.
        let body = text.trim_start_matches(' ');
        let prefix = &text[..text.len() - body.len()];
        let mut lines = self.inner.wrap(body, width, font);
        if let Some(first) = lines.first_mut() {
            first.insert_str(0, prefix);
        }
        lines
    }
    fn line_height_twips(&self, font: &FontSpec) -> f64 {
        self.fonts
            .windows_metrics(font, false)
            .map(|m| f64::from(m.ascent + m.descent + m.leading))
            .unwrap_or_else(|| self.inner.line_height_twips(font))
    }
    fn ascent_twips(&self, font: &FontSpec) -> f64 {
        self.fonts
            .windows_metrics(font, false)
            .map(|m| f64::from(m.ascent))
            .unwrap_or_else(|| self.inner.ascent_twips(font))
    }
    fn substituted_chars(&self, text: &str, font: &FontSpec) -> String {
        self.inner.substituted_chars(text, font)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cell_height_uses_first_exact_match_or_previous_height() {
        let rows = [
            (175, 163, 37),
            (176, 164, 37),
            (177, 164, 37),
            (178, 165, 38),
        ];
        assert_eq!(select_cell(&rows, 200), Some(rows[0]));
        assert_eq!(select_cell(&rows, 201), Some(rows[1]));
        assert_eq!(select_cell(&rows, 202), Some(rows[2]));
        assert_eq!(select_cell(&rows, 199), None);
        assert_eq!(select_cell(&rows, 300), None);
    }
    #[test]
    fn truncated_vdmx_is_rejected() {
        let bytes = [
            0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 12, 0, 1, 8, 8, 0, 8, 0, 7, 255, 254,
        ];
        assert_eq!(vdmx_records(&bytes), Some(vec![(8, 7, 2)]));
        for n in 0..bytes.len() {
            assert!(vdmx_records(&bytes[..n]).is_none());
        }
    }
}

#[cfg(all(test, feature = "cosmic"))]
#[test]
fn leading_spaces_are_preserved_without_changing_word_fit() {
    let l = WindowsLayout::new(crate::FontSource::Bundled);
    let f = FontSpec {
        family: "Liberation Sans".into(),
        size_pt: 10.0,
        ..Default::default()
    };
    let width = l.width_twips("alpha beta", &f) + 0.01;
    assert_eq!(
        l.wrap("  alpha beta gamma", width, &f),
        vec!["  alpha beta", "gamma"]
    );
}
