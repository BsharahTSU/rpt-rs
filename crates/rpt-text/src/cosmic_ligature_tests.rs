//! Real shaping/measurement tests with an open-font synthetic dlig table.
use super::*;

fn layout(enabled: bool) -> CosmicLayout {
    let mut db = cosmic_text::fontdb::Database::new();
    db.load_font_data(rpt_test_support::fonts::discretionary_ligature_font());
    let mut layout = CosmicLayout::new(FontProvider::bundled());
    layout.font_system = RefCell::new(FontSystem::new_with_locale_and_db("en-US".into(), db));
    if enabled {
        layout.with_legacy_arabic_ligatures()
    } else {
        layout
    }
}

fn font() -> FontSpec {
    FontSpec {
        family: "DejaVu Sans".into(),
        size_pt: 12.0,
        ..Default::default()
    }
}

#[test]
fn legacy_arabic_ligatures_change_glyph_count_and_alignment_measurement() {
    let ordinary = layout(false);
    let legacy = layout(true);
    let regular = ordinary.shaped("اا", &font(), None);
    let joined = legacy.shaped("اا", &font(), None);
    let normal: Vec<_> = regular.layout_runs().flat_map(|r| r.glyphs).collect();
    let glyphs: Vec<_> = joined.layout_runs().flat_map(|r| r.glyphs).collect();
    assert_eq!(normal.len(), 2);
    assert_eq!(
        glyphs.len(),
        1,
        "exercise an actual optional Arabic ligature"
    );
    assert_eq!((glyphs[0].start, glyphs[0].end), (0, "اا".len()));
    assert!(glyphs[0].w < normal.iter().map(|g| g.w).sum::<f32>());
    assert!(
        legacy.crystal_alignment_width_twips("اا", &font())
            < ordinary.crystal_alignment_width_twips("اا", &font())
    );
    assert_eq!(legacy.wrap("اا اا", 10000.0, &font()), ["اا اا"]);
}

#[test]
fn legacy_arabic_ligatures_preserve_non_arabic_measurements_and_latin_policy() {
    let ordinary = layout(false).without_latin_kerning();
    let legacy = layout(true).without_latin_kerning();
    for text in ["ff", "AV ff", "שלום", "a\u{301}"] {
        assert_eq!(
            ordinary.width_twips(text, &font()),
            legacy.width_twips(text, &font()),
            "{text}"
        );
        assert_eq!(
            ordinary.wrap(text, 5000.0, &font()),
            legacy.wrap(text, 5000.0, &font())
        );
    }
    let buffer = legacy.shaped("ff", &font(), None);
    assert_eq!(
        buffer.layout_runs().map(|r| r.glyphs.len()).sum::<usize>(),
        2,
        "the synthetic Latin discretionary ligature must stay disabled"
    );
}

#[test]
fn legacy_arabic_ligatures_keep_mixed_direction_logical_ranges() {
    let legacy = layout(true).without_latin_kerning();
    for text in ["ff اا ff", "ש ff اا ff", "اا ff اا"] {
        let buffer = legacy.shaped(text, &font(), None);
        let glyphs: Vec<_> = buffer.layout_runs().flat_map(|r| r.glyphs).collect();
        let joined = glyphs
            .iter()
            .filter(|g| &text[g.start..g.end] == "اا")
            .count();
        assert_eq!(joined, text.matches("اا").count(), "{text}");
        assert_eq!(
            legacy.wrap(text, 10000.0, &font()),
            [text],
            "retain all logical content"
        );
        assert!(
            glyphs
                .iter()
                .filter(|g| &text[g.start..g.end] == "f")
                .count()
                >= 2
        );
    }
}
