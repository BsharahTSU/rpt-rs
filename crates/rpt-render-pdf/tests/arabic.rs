//! A synthetic bilingual PDF probe; no school records or templates are embedded.
use rpt_model::{Color, Rect, Twips};
use rpt_pages::{DrawOp, FontSpec, Page, PageSize, TextAlign, TextRun};

#[test]
fn bilingual_pdf_preserves_arabic_latin_digits_and_character_spacing() {
    let mut page = Page::new(
        1,
        PageSize {
            width: Twips(12240),
            height: Twips(15840),
        },
    );
    for (index, (text, rtl, spacing, align)) in [
        (
            "Arabic and mixed-script regression",
            false,
            0,
            TextAlign::Left,
        ),
        ("اللغة العربية", true, 0, TextAlign::Right),
        (
            "الطالب أحمد محمد - Grade A - 123",
            true,
            0,
            TextAlign::Right,
        ),
        ("Student ABC 123 - اللغة العربية", false, 0, TextAlign::Left),
        ("اللغة العربية", true, 10, TextAlign::Right),
        (
            "هذا نص عربي لاختبار المحاذاة والتباعد",
            true,
            0,
            TextAlign::Justified,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        page.push(DrawOp::Text(TextRun {
            rtl,
            bounds: Rect {
                left: Twips(720),
                top: Twips(720 + index as i32 * 900),
                width: Twips(10800),
                height: Twips(600),
            },
            text: text.into(),
            font: FontSpec {
                family: "DejaVu Sans".into(),
                size_pt: 18.0,
                ..Default::default()
            },
            color: Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
            align,
            rotation: 0.0,
            metrics: None,
            character_spacing: Twips(spacing),
            source: None,
        }));
    }
    let bytes = rpt_render_pdf::try_render_pages(&[page]).expect("bilingual PDF serializes");
    assert!(bytes.starts_with(b"%PDF-"));
    if let Ok(directory) = std::env::var("STC_RPT_ARTIFACTS") {
        std::fs::create_dir_all(&directory).unwrap();
        std::fs::write(
            std::path::Path::new(&directory).join("arabic-probe.pdf"),
            bytes,
        )
        .unwrap();
    }
}
