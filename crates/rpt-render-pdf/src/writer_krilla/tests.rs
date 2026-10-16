//! The advance contract between the producer's measurement and this writer's drawing.
//!
//! The layout engine measures a run's width — and picks its wrap point — as the natural advance plus
//! `character_spacing` once per Unicode scalar. This writer re-shapes the same string to obtain
//! glyphs, so it must arrive at the same total, or the text is wrapped as if spaced and drawn as if
//! not. Asserted at the shaper, where the rule lives, and once through the serialized content stream,
//! where its effect is observable.

use super::*;
use rpt_pages::TextMetrics;
use rpt_test_support::pdf::operator_listing;

/// 12 pt, the size the typography fixtures use.
const SIZE: f32 = 12.0;
/// 0.5 pt = 10 twips per scalar — the spacing the `paragraph_typography` fixture stores.
const SPACING_PT: f32 = 0.5;

/// The single bundled face that covers `text` for `family`, loaded for shaping.
fn face_of(fonts: &mut FontCache, family: &str, text: &str) -> Rc<LoadedFace> {
    let spec = FontSpec {
        family: family.to_string(),
        size_pt: SIZE,
        ..FontSpec::default()
    };
    let segments = fonts.db.segment_by_coverage(&spec, text);
    assert_eq!(segments.len(), 1, "{text:?} must shape in a single face");
    fonts.face(segments[0].face).expect("a bundled face loads")
}

/// Spacing is charged once per scalar of the run, the trailing one included — the producer's advance
/// includes it, so a run drawn without it ends short of the box it was measured into.
#[test]
fn character_spacing_is_charged_once_per_scalar_including_the_trailing_one() {
    let text = "Order Amount";
    let mut fonts = FontCache::new(FontSource::Bundled);
    let face = face_of(&mut fonts, "Liberation Sans", text);
    let shaper = Shaper::new(&face, false).expect("the bundled face parses for shaping");

    let natural = shaper.shape(text, SIZE, 0.0).1;
    let spaced = shaper.shape(text, SIZE, SPACING_PT).1;
    let expected = natural + SPACING_PT * text.chars().count() as f32;
    assert!(
        (spaced - expected).abs() < 0.001,
        "drawn {spaced} vs measured {expected}"
    );
    // Zero spacing must leave the advances exactly as the face shaped them.
    assert_eq!(shaper.shape(text, SIZE, 0.0).1, natural);
}

/// A ligature shapes several scalars into one glyph, so the per-glyph reading of the rule loses the
/// spacing of every extra scalar it covers. This is the divergence the field exists to prevent, and
/// the per-glyph total is asserted *not* to reconcile so the loop cannot quietly adopt it.
#[test]
fn a_ligature_owes_spacing_for_every_scalar_it_covers() {
    // DejaVu Sans is the bundled symbol face and the only one with a ligature table.
    let text = "office";
    let mut fonts = FontCache::new(FontSource::Bundled);
    let face = face_of(&mut fonts, "DejaVu Sans", text);
    let shaper = Shaper::new(&face, false).expect("the bundled face parses for shaping");

    let (glyphs, natural) = shaper.shape(text, SIZE, 0.0);
    let scalars = text.chars().count();
    assert!(
        glyphs.len() < scalars,
        "the fixture needs a real ligature: {} glyphs for {scalars} scalars",
        glyphs.len()
    );

    let spaced = shaper.shape(text, SIZE, SPACING_PT).1;
    let per_scalar = natural + SPACING_PT * scalars as f32;
    let per_glyph = natural + SPACING_PT * glyphs.len() as f32;
    assert!(
        (spaced - per_scalar).abs() < 0.001,
        "drawn {spaced} vs measured {per_scalar}"
    );
    assert!(
        (spaced - per_glyph).abs() > 0.1,
        "the per-glyph rule must not reconcile — that is the bug this charges per scalar to avoid"
    );
}

/// The spacing reaches the page: the writer emits it as the `TJ` adjustment between glyphs, in
/// thousandths of the text-space unit, which is what a viewer applies. A backend that dropped it
/// would still serialize a clean PDF, so the operator listing is where the omission is visible.
#[test]
fn the_content_stream_carries_the_spacing_between_glyphs() {
    let mut page = Page::new(
        1,
        PageSize {
            width: Twips(12240),
            height: Twips(15840),
        },
    );
    // No kern pair in the string, so the only `TJ` adjustment a viewer sees is the spacing.
    let text = "nun";
    page.push(DrawOp::Text(TextRun {
        rtl: false,
        bounds: Rect {
            left: Twips(720),
            top: Twips(720),
            width: Twips(4000),
            height: Twips(320),
        },
        text: text.to_string(),
        font: FontSpec {
            family: "Liberation Sans".to_string(),
            size_pt: SIZE,
            ..FontSpec::default()
        },
        color: Color {
            a: 255,
            r: 0,
            g: 0,
            b: 0,
        },
        align: TextAlign::Left,
        rotation: 0.0,
        metrics: Some(TextMetrics {
            advance: Twips(1000),
            ascent: Twips(240),
            line_height: Twips(276),
        }),
        character_spacing: Twips(10),
        source: None,
    }));
    let listing = operator_listing(&crate::render_pages(&[page]));
    // -41.667 = -(10 twips / 20 twips-per-pt) / 12 pt × 1000, once after each of the first two
    // scalars; the trailing one moves the pen past the end of the run and needs no adjustment.
    assert!(
        listing.contains("-41.667"),
        "no per-scalar TJ adjustment in:\n{listing}"
    );
}

#[test]
fn arabic_clusters_keep_valid_logical_ranges_and_per_scalar_spacing() {
    let text = "اللغة العربية";
    let mut fonts = FontCache::new(FontSource::Bundled);
    let face = face_of(&mut fonts, "DejaVu Sans", text);
    let shaper = Shaper::new(&face, false).expect("Arabic-capable bundled font");
    let natural = shaper.shape(text, SIZE, 0.0).1;
    let spaced = shaper.shape(text, SIZE, SPACING_PT).1;
    assert!((spaced - natural - SPACING_PT * text.chars().count() as f32).abs() < 0.001);
}

#[test]
fn mixed_arabic_and_latin_runs_follow_the_stored_paragraph_direction() {
    let fonts = FontCache::new(FontSource::Bundled);
    let run = TextRun {
        rtl: true,
        bounds: Rect {
            left: Twips(0),
            top: Twips(0),
            width: Twips(8000),
            height: Twips(400),
        },
        text: "العربية ABC 123".into(),
        font: FontSpec {
            family: "DejaVu Sans".into(),
            size_pt: SIZE,
            ..Default::default()
        },
        color: Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        },
        align: TextAlign::Right,
        rotation: 0.0,
        metrics: None,
        character_spacing: Twips(10),
        source: None,
    };
    let segments = fonts.db.segment_by_coverage(&run.font, &run.text);
    let visual = visual_face_runs(&run, &segments);
    assert!(run.text[visual.first().unwrap().0.range.clone()].contains("ABC"));
    assert!(!visual.first().unwrap().1);
    assert!(visual.last().unwrap().1);
    let mut page = Page::new(
        1,
        PageSize {
            width: Twips(12240),
            height: Twips(15840),
        },
    );
    page.push(DrawOp::Text(run));
    let pdf = crate::try_render_pages(&[page]).expect("mixed RTL PDF serializes");
    assert!(pdf.starts_with(b"%PDF-"));
}

#[test]
fn dotted_and_dashed_rules_survive_pdf_serialization() {
    let mut page = Page::new(
        1,
        PageSize {
            width: Twips(4000),
            height: Twips(4000),
        },
    );
    for (i, style) in [
        rpt_pages::LineStyle::Dotted,
        rpt_pages::LineStyle::Dashed,
        rpt_pages::LineStyle::Single,
    ]
    .into_iter()
    .enumerate()
    {
        page.push(DrawOp::Line(LineOp {
            from: rpt_pages::Point::new(100, 100 + i as i32 * 200),
            to: rpt_pages::Point::new(3000, 100 + i as i32 * 200),
            stroke: rpt_pages::Stroke {
                color: Color {
                    a: 255,
                    r: 0,
                    g: 0,
                    b: 0,
                },
                width: Twips(10),
                style,
            },
            source: None,
        }));
    }
    let listing = operator_listing(&crate::render_pages(&[page]));
    assert!(listing.contains("[ 1 3 ] 0 d"), "{listing}");
    assert!(listing.contains("[ 3 3 ] 0 d"), "{listing}");
}

#[test]
fn zero_width_is_a_pdf_device_hairline() {
    let mut page = Page::new(
        1,
        PageSize {
            width: Twips(4000),
            height: Twips(4000),
        },
    );
    page.push(DrawOp::Line(LineOp {
        from: rpt_pages::Point::new(100, 100),
        to: rpt_pages::Point::new(3000, 100),
        stroke: rpt_pages::Stroke {
            color: Color {
                a: 255,
                r: 0,
                g: 0,
                b: 0,
            },
            width: Twips(0),
            style: rpt_pages::LineStyle::Dotted,
        },
        source: None,
    }));
    let listing = operator_listing(&crate::render_pages(&[page]));
    assert!(listing.contains("0 w"), "{listing}");
    assert!(listing.contains("[ 1 3 ] 0 d"), "{listing}");
    assert!(
        listing.contains("150 5 l"),
        "the hairline must not disappear: {listing}"
    );
}

#[test]
fn legacy_latin_kerning_is_opt_in_and_uses_nominal_pair_advances() {
    let mut fonts = FontCache::new(FontSource::Bundled);
    let face = face_of(&mut fonts, "Liberation Sans", "AV Wr A");
    let ordinary = Shaper::new(&face, false).unwrap();
    let compatibility = Shaper::new(&face, true).unwrap();
    let mut adjusted_pairs = 0;
    for pair in ["AV", "Wr", " A"] {
        let nominal: f32 = pair
            .chars()
            .map(|ch| ordinary.shape(&ch.to_string(), SIZE, 0.0).1)
            .sum();
        let normal = ordinary.shape(pair, SIZE, 0.0).1;
        let unkerned = compatibility.shape(pair, SIZE, 0.0).1;
        assert!(
            (unkerned - nominal).abs() < 0.001,
            "{pair:?}: {unkerned} vs {nominal}"
        );
        if (normal - nominal).abs() > 0.01 {
            adjusted_pairs += 1;
        }
        let spaced = compatibility.shape(pair, SIZE, SPACING_PT).1;
        assert!((spaced - unkerned - SPACING_PT * pair.chars().count() as f32).abs() < 0.001);
    }
    assert!(
        adjusted_pairs >= 2,
        "fixture must exercise real kerning pairs"
    );
}

#[test]
fn latin_feature_masks_keep_exclusive_utf8_boundaries() {
    let text = "中文 AV العربية Wr中文";
    assert!(latin_kerning_features(text, false).is_empty());
    let features = latin_kerning_features(text, true);
    assert_eq!(features.len(), 2);
    let slices: Vec<_> = features
        .iter()
        .map(|feature| {
            assert_eq!(feature.tag, harfrust::Tag::new(b"kern"));
            assert_eq!(feature.value, 0);
            &text[feature.start as usize..feature.end as usize]
        })
        .collect();
    assert_eq!(slices, [" AV ", " Wr"]);
    let last = features.last().unwrap();
    assert_eq!(&text[last.end as usize..], "中文");
}

fn assert_same_shaped_glyphs(left: &[KrillaGlyph], right: &[KrillaGlyph]) {
    assert_eq!(left.len(), right.len());
    for (left, right) in left.iter().zip(right) {
        assert_eq!(left.glyph_id, right.glyph_id);
        assert_eq!(left.text_range, right.text_range);
        assert_eq!(left.x_advance, right.x_advance);
        assert_eq!(left.x_offset, right.x_offset);
        assert_eq!(left.y_offset, right.y_offset);
        assert_eq!(left.y_advance, right.y_advance);
    }
}

#[test]
fn latin_kerning_option_preserves_arabic_joining_and_latin_ligatures() {
    let mut fonts = FontCache::new(FontSource::Bundled);
    for text in ["لا", "السَّلَامُ", "ffi", "A\u{301}"] {
        let face = face_of(&mut fonts, "DejaVu Sans", text);
        let ordinary = Shaper::new(&face, false).unwrap();
        let compatibility = Shaper::new(&face, true).unwrap();
        let (normal_glyphs, normal_width) = ordinary.shape(text, SIZE, SPACING_PT);
        let (compat_glyphs, compat_width) = compatibility.shape(text, SIZE, SPACING_PT);
        assert_same_shaped_glyphs(&normal_glyphs, &compat_glyphs);
        assert_eq!(normal_width, compat_width);
        assert!(
            normal_glyphs.len() < text.chars().count(),
            "fixture must exercise joining/ligatures: {text}"
        );
    }
}

#[test]
fn pdf_options_select_unkerned_latin_without_changing_arabic_content_stream() {
    let render_text = |text: &str, family: &str, disabled| {
        let mut page = Page::new(
            1,
            PageSize {
                width: Twips(12240),
                height: Twips(15840),
            },
        );
        page.push(DrawOp::Text(TextRun {
            rtl: text.starts_with('ل'),
            bounds: Rect {
                left: Twips(720),
                top: Twips(720),
                width: Twips(6000),
                height: Twips(400),
            },
            text: text.into(),
            font: FontSpec {
                family: family.into(),
                size_pt: SIZE,
                ..Default::default()
            },
            color: Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
            align: TextAlign::Left,
            rotation: 0.0,
            metrics: None,
            character_spacing: Twips(0),
            source: None,
        }));
        crate::try_render_pages_with_options(
            &[page],
            &BTreeMap::new(),
            &PdfOptions {
                disable_latin_kerning: disabled,
                ..Default::default()
            },
        )
        .unwrap()
    };
    assert!(!PdfOptions::default().disable_latin_kerning);
    let normal = operator_listing(&render_text("AV Wr A", "Liberation Sans", false));
    let compatibility = operator_listing(&render_text("AV Wr A", "Liberation Sans", true));
    assert_ne!(
        normal, compatibility,
        "the opt-in must reach serialized glyph advances"
    );
    assert_eq!(
        render_text("لا", "DejaVu Sans", false),
        render_text("لا", "DejaVu Sans", true),
        "an Arabic-only PDF must retain identical shaping and bytes"
    );
}
