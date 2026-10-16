//! Windows GDI font-cell metrics used by the legacy Crystal PDF export path.
//! VDMX records describe hinted pixel heights; at 1440 logical DPI one pixel is a twip.
//! https://learn.microsoft.com/en-us/typography/opentype/spec/vdmx
use crate::FontDb;
use rpt_pages::FontSpec;
#[cfg(feature = "cosmic")]
use rpt_pages::TextLayout;
use std::num::NonZeroU32;

const TWIPS_PER_INCH: u32 = 1440;

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

/// Hinted metrics in device pixels. PDF glyph-cell metrics use 1440 DPI; printer
/// line pitch may use a different resolution and is converted back to twips.
#[derive(Debug, Clone, Copy)]
struct DeviceMetrics {
    em: i32,
    ascent: i32,
    descent: i32,
    leading: i32,
}
impl DeviceMetrics {
    #[cfg(any(feature = "cosmic", test))]
    fn line_height_twips(self, dpi: NonZeroU32) -> f64 {
        f64::from(self.ascent + self.descent + self.leading) * f64::from(TWIPS_PER_INCH)
            / f64::from(dpi.get())
    }
}

struct MetricTables<'a> {
    units: f64,
    win_ascent: f64,
    win_descent: f64,
    line_gap: f64,
    vdmx: Option<&'a [(i32, i32, i32)]>,
}
impl MetricTables<'_> {
    fn at_dpi(&self, size_pt: f32, cell_height: bool, dpi: NonZeroU32) -> DeviceMetrics {
        // Preserve the original f32 point-to-twip rounding when dpi is 1440.
        let wanted = (size_pt * (dpi.get() as f32 / 72.0)).round().max(1.0) as i32;
        let hinted = self.vdmx.and_then(|rows| {
            if cell_height {
                select_cell(rows, wanted)
            } else {
                rows.iter().copied().find(|row| row.0 == wanted)
            }
        });
        let (em, ascent, descent) = hinted.unwrap_or_else(|| {
            let em = if cell_height {
                (f64::from(wanted) * self.units / (self.win_ascent + self.win_descent)).round()
                    as i32
            } else {
                wanted
            };
            (
                em,
                (self.win_ascent * f64::from(em) / self.units).round() as i32,
                (self.win_descent * f64::from(em) / self.units).round() as i32,
            )
        });
        let leading = (self.line_gap * f64::from(em) / self.units).round() as i32;
        DeviceMetrics {
            em,
            ascent,
            descent,
            leading: leading.max(0),
        }
    }
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
    /// These PDF metrics always use 1440 logical DPI, independently of the printer.
    pub fn windows_metrics(
        &self,
        font: &FontSpec,
        cell_height: bool,
    ) -> Option<WindowsFontMetrics> {
        let metrics = self.device_metrics(font, cell_height, NonZeroU32::new(TWIPS_PER_INCH)?)?;
        Some(WindowsFontMetrics {
            em_twips: metrics.em,
            ascent: metrics.ascent,
            descent: metrics.descent,
            leading: metrics.leading,
        })
    }

    fn device_metrics(
        &self,
        font: &FontSpec,
        cell_height: bool,
        dpi: NonZeroU32,
    ) -> Option<DeviceMetrics> {
        let id = self.query(font)?;
        self.with_face_data(id, |bytes, index| {
            let face = ttf_parser::Face::parse(bytes, index).ok()?;
            let raw = face.raw_face();
            let os2 = raw.table(ttf_parser::Tag::from_bytes(b"OS/2"))?;
            let hhea = raw.table(ttf_parser::Tag::from_bytes(b"hhea"))?;
            let win_ascent = f64::from(u16_at(os2, 74)?);
            let win_descent = f64::from(u16_at(os2, 76)?);
            if win_ascent + win_descent <= 0.0 {
                return None;
            }
            let vdmx = raw
                .table(ttf_parser::Tag::from_bytes(b"VDMX"))
                .and_then(vdmx_records);
            let tables = MetricTables {
                units: f64::from(face.units_per_em()),
                win_ascent,
                win_descent,
                line_gap: f64::from(i16_at(hhea, 8)?),
                vdmx: vdmx.as_deref(),
            };
            Some(tables.at_dpi(font.size_pt, cell_height, dpi))
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
    printer_dpi: NonZeroU32,
}
#[cfg(feature = "cosmic")]
impl WindowsLayout {
    /// Load nominal layout metrics with the original 1440-DPI line pitch.
    pub fn new(source: crate::FontSource) -> Self {
        const DEFAULT_DPI: NonZeroU32 = NonZeroU32::new(TWIPS_PER_INCH).unwrap();
        Self::with_printer_dpi(source, DEFAULT_DPI)
    }

    /// Use the legacy Basic-Latin unkerned width and wrapping policy.
    /// Pair with the PDF backend's matching option.
    pub fn without_latin_kerning(mut self) -> Self {
        self.inner = self.inner.without_latin_kerning();
        self
    }

    /// Use a selected vertical printer resolution for line pitch. This is explicit:
    /// no host printer is queried. Resolve saved report settings or deployment
    /// defaults at the caller. Glyph sizing, first-line ascent and wrapping retain
    /// their existing behavior; only multiline pitch uses printer-pixel rounding.
    pub fn with_printer_dpi(source: crate::FontSource, printer_dpi: NonZeroU32) -> Self {
        Self {
            inner: crate::CosmicLayout::new(crate::FontProvider::from_source(source)),
            fonts: source.load(),
            printer_dpi,
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
            .device_metrics(font, false, self.printer_dpi)
            .map(|metrics| metrics.line_height_twips(self.printer_dpi))
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

    #[test]
    fn printer_pitch_corrects_opposite_rounding_errors_without_scaling_pdf_cells() {
        // Synthetic numeric metric tables capture the two opposite rounding cases:
        // English pitch shrinks from 230 twips; Arabic pitch grows from 226 twips.
        // No school report, text, or proprietary font file is needed for this test.
        let regular_rows = [(83, 74, 18), (179, 160, 38), (200, 179, 44)];
        let regular = MetricTables {
            units: 2048.0,
            win_ascent: 1854.0,
            win_descent: 434.0,
            line_gap: 67.0,
            vdmx: Some(&regular_rows),
        };
        let arabic_rows = [(83, 77, 18), (200, 184, 42)];
        let arabic = MetricTables {
            units: 2048.0,
            win_ascent: 1888.0,
            win_descent: 430.0,
            line_gap: 0.0,
            vdmx: Some(&arabic_rows),
        };
        let pdf_dpi = NonZeroU32::new(1440).unwrap();
        let printer_dpi = NonZeroU32::new(600).unwrap();
        assert_eq!(
            regular
                .at_dpi(10.0, false, pdf_dpi)
                .line_height_twips(pdf_dpi),
            230.0
        );
        assert_eq!(
            arabic
                .at_dpi(10.0, false, pdf_dpi)
                .line_height_twips(pdf_dpi),
            226.0
        );
        assert_eq!(
            regular
                .at_dpi(10.0, false, printer_dpi)
                .line_height_twips(printer_dpi),
            228.0
        );
        assert_eq!(
            arabic
                .at_dpi(10.0, false, printer_dpi)
                .line_height_twips(printer_dpi),
            228.0
        );
        // PDF positive-height selection must still choose 8.95pt, not an 83px em.
        let glyph_cell = regular.at_dpi(10.0, true, pdf_dpi);
        assert_eq!(
            (glyph_cell.em, glyph_cell.ascent, glyph_cell.descent),
            (179, 160, 38)
        );
        assert_eq!(regular.at_dpi(10.0, false, pdf_dpi).ascent, 179);
    }

    #[test]
    fn default_dpi_preserves_half_twip_rounding() {
        let tables = MetricTables {
            units: 2048.0,
            win_ascent: 1854.0,
            win_descent: 434.0,
            line_gap: 67.0,
            vdmx: None,
        };
        // 10.025f32 * 20 rounds to exactly 200.5f32. Widening the point size
        // before multiplication would instead yield 200.499992... and select 200.
        let metrics = tables.at_dpi(10.025, false, NonZeroU32::new(1440).unwrap());
        assert_eq!(metrics.em, 201);
    }
    #[test]
    fn printer_pitch_preserves_fractional_twips_and_has_linear_fallback() {
        let rows = [(83, 77, 19)];
        let tables = MetricTables {
            units: 2048.0,
            win_ascent: 1854.0,
            win_descent: 434.0,
            line_gap: 67.0,
            vdmx: Some(&rows),
        };
        let dpi = NonZeroU32::new(600).unwrap();
        assert!((tables.at_dpi(10.0, false, dpi).line_height_twips(dpi) - 237.6).abs() < 1e-9);
        // At 720 DPI the 100px em is outside this synthetic VDMX table. OS/2
        // ascent/descent and hhea leading still round in printer pixels first.
        let dpi = NonZeroU32::new(720).unwrap();
        let fallback = tables.at_dpi(10.0, false, dpi);
        assert_eq!(
            (
                fallback.em,
                fallback.ascent,
                fallback.descent,
                fallback.leading
            ),
            (100, 91, 21, 3)
        );
        assert_eq!(fallback.line_height_twips(dpi), 230.0);
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

#[cfg(all(test, feature = "cosmic"))]
#[test]
fn explicit_printer_dpi_preserves_default_layout_and_first_baseline() {
    let source = crate::FontSource::Bundled;
    let original = WindowsLayout::new(source);
    let explicit = WindowsLayout::with_printer_dpi(source, NonZeroU32::new(1440).unwrap());
    let printer = WindowsLayout::with_printer_dpi(source, NonZeroU32::new(600).unwrap());
    let font = FontSpec {
        family: "Liberation Sans".into(),
        size_pt: 10.0,
        ..Default::default()
    };
    let text = "alpha beta gamma";
    assert_eq!(
        original.line_height_twips(&font),
        explicit.line_height_twips(&font)
    );
    assert_eq!(original.ascent_twips(&font), printer.ascent_twips(&font));
    assert_eq!(
        original.width_twips(text, &font),
        printer.width_twips(text, &font)
    );
    let width = original.width_twips("alpha beta", &font) + 0.01;
    assert_eq!(
        original.wrap(text, width, &font),
        printer.wrap(text, width, &font)
    );
}
