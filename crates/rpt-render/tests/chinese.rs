//! Chinese glyph coverage, unspaced wrapping and an actual PDF text round-trip.
//! Run in Dockerfile.stc-dev, which supplies Noto CJK and Poppler.
#![cfg(target_os = "linux")]
use rpt_pages::{DiagnosticKind, DrawOp, FontSpec, Severity};
use rpt_reader::model::{
    Area, AreaSectionKind, Color, Font, FontColor, Rect, Report, ReportObject, ReportObjectKind,
    Section, TextObject, Twips,
};
use rpt_render::{FontSource, PdfOptions, RenderOptions};
use std::process::Command;

const FAMILY: &str = "Noto Sans CJK SC";
const PARAGRAPH: &str = "中文报告测试学生成绩数学英语总分优秀没有空格也应正确自动换行。中文报告测试学生成绩数学英语总分优秀。";
const TRADITIONAL: &str = "繁體中文學生成績報告：數學與英語總分優秀。";
const MIXED: &str = "中文报告：Student ABC 123，数学 98.5 分。";

fn chinese_font() -> FontSpec {
    FontSpec {
        family: FAMILY.into(),
        size_pt: 14.0,
        ..Default::default()
    }
}

fn report() -> Report {
    let font = FontColor {
        font: Font {
            name: FAMILY.into(),
            size_pt: 14.0,
            ..Default::default()
        },
        color: Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        },
        ..Default::default()
    };
    let mut sections = Vec::new();
    for (name, text, width) in [
        ("ChineseParagraph", PARAGRAPH, 2400),
        ("MixedChinese", MIXED, 10800),
        ("TraditionalChinese", TRADITIONAL, 10800),
        ("ArialChinese", "Student 123 中文成績", 10800),
    ] {
        let mut object = ReportObject {
            name: name.into(),
            bounds: Rect {
                left: Twips(100),
                top: Twips(0),
                width: Twips(width),
                height: Twips(500),
            },
            kind: ReportObjectKind::Text(TextObject {
                text: text.into(),
                display: text.into(),
                font_color: {
                    let mut selected = font.clone();
                    if name == "TraditionalChinese" {
                        selected.font.name = "Noto Sans CJK TC".into();
                    }
                    if name == "ArialChinese" {
                        selected.font.name = "Arial".into();
                    }
                    selected
                },
                ..Default::default()
            }),
            ..Default::default()
        };
        object.format.can_grow = true;
        sections.push(Section {
            name: name.into(),
            kind: AreaSectionKind::ReportHeader,
            height: Twips(800),
            objects: vec![object],
            ..Default::default()
        });
    }
    let mut report = Report::default();
    report.print_options.content_width = Twips(11000);
    report.print_options.content_height = Twips(15000);
    report.report_definition.areas = vec![Area {
        name: "Header".into(),
        kind: AreaSectionKind::ReportHeader,
        sections,
        ..Default::default()
    }];
    report
}

fn pages() -> rpt_pages::PagedDocument {
    let report = report();
    rpt_render::render_with(
        &report,
        RenderOptions {
            fonts: FontSource::System,
            ..Default::default()
        },
    )
}

#[test]
fn chinese_font_covers_every_requested_han_character() {
    let db = rpt_text::FontDb::with_system_fonts();
    let face = db
        .query(&chinese_font())
        .expect("install fonts-noto-cjk or use Dockerfile.stc-dev");
    for character in PARAGRAPH
        .chars()
        .chain(MIXED.chars())
        .chain(TRADITIONAL.chars())
    {
        assert!(
            db.face_covers(face, character),
            "Chinese font lacks U+{:04X} ({character})",
            character as u32
        );
    }
}

#[test]
fn unspaced_chinese_wraps_without_losing_or_reordering_characters() {
    let pages = pages();
    assert!(!pages.pages.is_empty());
    let runs: Vec<_> = pages
        .pages
        .iter()
        .flat_map(|page| &page.ops)
        .filter_map(|op| match op {
            DrawOp::Text(run)
                if run.source.as_ref().and_then(|s| s.object_name.as_deref())
                    == Some("ChineseParagraph") =>
            {
                Some(run)
            }
            _ => None,
        })
        .collect();
    assert!(runs.len() >= 3, "CJK text must wrap without spaces");
    assert_eq!(
        runs.iter().map(|run| run.text.as_str()).collect::<String>(),
        PARAGRAPH
    );
    for run in runs {
        assert!(!run.rtl);
        assert!(
            run.metrics.as_ref().unwrap().advance.0 <= run.bounds.width.0 + 2,
            "Chinese line exceeds its measured box: {:?}",
            run.text
        );
    }
}

#[test]
fn chinese_and_mixed_text_survive_pdf_font_embedding_and_extraction() {
    assert_chinese_pdf_round_trip(pages(), "default");
}

#[test]
fn printer_line_metrics_preserve_chinese_pdf_text_and_fallbacks() {
    let report = report();
    let dataset = rpt_data::build_dataset(&rpt_data::EmptySource, &report.data_definition);
    let mut document = rpt_render::render_dataset_with(
        &report,
        &dataset,
        Box::new(
            rpt_text::WindowsLayout::with_printer_dpi(
                FontSource::System,
                std::num::NonZeroU32::new(600).unwrap(),
            )
            .without_latin_kerning()
            .with_legacy_arabic_ligatures()
            .without_trailing_spaces(),
        ),
        Default::default(),
        None,
        None,
    );
    // Exercise final alignment quantization through an actual PDF as well as the
    // printer metrics. Shaping/wrapping has already selected these CJK/mixed lines.
    let alignment = rpt_text::CosmicLayout::with_system_fonts()
        .without_latin_kerning()
        .with_legacy_arabic_ligatures();
    for op in document.pages.iter_mut().flat_map(|page| &mut page.ops) {
        if let DrawOp::Text(run) = op {
            run.align = rpt_pages::TextAlign::Center;
            if let Some(metrics) = &mut run.metrics {
                metrics.advance =
                    Twips(alignment.crystal_alignment_width_twips(&run.text, &run.font) as i32);
            }
        }
    }
    assert_chinese_pdf_round_trip(document, "printer-600");
}

fn assert_chinese_pdf_round_trip(pages: rpt_pages::PagedDocument, profile: &str) {
    assert_eq!(
        pages.diagnostics.len(),
        1,
        "only the intentional Arial fallback is expected: {:?}",
        pages.diagnostics
    );
    assert_eq!(pages.diagnostics[0].kind, DiagnosticKind::FontSubstituted);
    assert_eq!(pages.diagnostics[0].severity, Severity::Warning);
    let bytes = rpt_render::try_render_document(
        &pages,
        &PdfOptions {
            fonts: FontSource::System,
            disable_latin_kerning: profile == "printer-600",
            legacy_arabic_mark_offsets: profile == "printer-600",
            legacy_arabic_ligatures: profile == "printer-600",
            legacy_bmp_quantization: profile == "printer-600",
            ..Default::default()
        },
    )
    .expect("CJK PDF must serialize");
    assert!(
        String::from_utf8_lossy(&bytes).contains("NotoSansCJK"),
        "actual Chinese font must be embedded"
    );
    let path =
        std::env::temp_dir().join(format!("stc-chinese-{profile}-{}.pdf", std::process::id()));
    std::fs::write(&path, &bytes).unwrap();
    let result = Command::new("pdftotext")
        .args(["-enc", "UTF-8"])
        .arg(&path)
        .arg("-")
        .output();
    let _ = std::fs::remove_file(&path);
    let output = result.expect("Poppler pdftotext is required; run Dockerfile.stc-dev");
    assert!(
        output.status.success(),
        "pdftotext rejected the Chinese PDF"
    );
    let extracted = String::from_utf8(output.stdout).expect("UTF-8 PDF text");
    let compact = |text: &str| {
        text.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    };
    assert!(
        compact(&extracted).contains(&compact(PARAGRAPH)),
        "Chinese paragraph round-trip failed: {extracted}"
    );
    assert!(
        compact(&extracted).contains(&compact(MIXED)),
        "mixed Chinese/Latin/digits round-trip failed: {extracted}"
    );
    assert!(
        compact(&extracted).contains(&compact("Student 123 中文成績")),
        "Arial fallback round-trip failed: {extracted}"
    );
    assert!(
        compact(&extracted).contains(&compact(TRADITIONAL)),
        "traditional Chinese round-trip failed: {extracted}"
    );
    if let Ok(directory) = std::env::var("STC_RPT_ARTIFACTS") {
        std::fs::create_dir_all(&directory).unwrap();
        std::fs::write(
            std::path::Path::new(&directory).join(format!("chinese-{profile}-probe.pdf")),
            bytes,
        )
        .unwrap();
    }
}

#[test]
fn chinese_in_an_arial_report_uses_a_covering_fallback_face() {
    let db = rpt_text::FontDb::with_system_fonts();
    let spec = FontSpec {
        family: "Arial".into(),
        size_pt: 14.0,
        ..Default::default()
    };
    let text = "Student 123 中文成績";
    let segments = db.segment_by_coverage(&spec, text);
    assert!(!segments.is_empty());
    assert_eq!(
        segments
            .iter()
            .map(|seg| &text[seg.range.clone()])
            .collect::<String>(),
        text
    );
    for segment in segments {
        for character in text[segment.range.clone()].chars() {
            assert!(
                db.face_covers(segment.face, character),
                "fallback must paint U+{:04X} ({character})",
                character as u32
            );
        }
    }
}

#[test]
fn chinese_leading_spaces_reduce_first_line_capacity_without_losing_text() {
    use rpt_pages::TextLayout;
    let layout = rpt_text::WindowsLayout::with_printer_dpi(
        FontSource::System,
        std::num::NonZeroU32::new(600).unwrap(),
    )
    .without_latin_kerning();
    for text in ["中文报告", "繁體中文"] {
        let font = chinese_font();
        let width = layout.width_twips(text, &font) + 0.01;
        assert_eq!(layout.wrap(text, width, &font), [text]);
        let padded = format!("  {text}");
        let lines = layout.wrap(&padded, width, &font);
        assert_eq!(
            lines.len(),
            2,
            "leading spaces must consume first-line capacity"
        );
        assert!(lines[0].starts_with("  "));
        assert_eq!(
            lines.concat(),
            padded,
            "all spaces and Han characters survive"
        );
    }
}

#[test]
fn chinese_terminal_space_lrm_preserves_text_without_a_phantom_line() {
    use rpt_pages::TextLayout;
    let layout = rpt_text::WindowsLayout::with_printer_dpi(
        FontSource::System,
        std::num::NonZeroU32::new(600).unwrap(),
    )
    .without_latin_kerning();
    for (family, text, first_half) in [
        ("Noto Sans CJK SC", "中文报告", "中文"),
        ("Noto Sans CJK TC", "繁體中文", "繁體"),
    ] {
        let font = FontSpec {
            family: family.into(),
            ..chinese_font()
        };
        for body in [text.to_string(), format!("  {text}")] {
            for width in [
                layout.width_twips(&body, &font) + 0.01,
                layout.width_twips(first_half, &font) + 0.01,
            ] {
                let plain = layout.wrap(&body, width, &font);
                for tail in [" \u{200e}", "\u{200e} ", " \u{200e} "] {
                    let padded = format!("{body}{tail}");
                    let mut expected = plain.clone();
                    expected.last_mut().unwrap().push_str(tail);
                    let actual = layout.wrap(&padded, width, &font);
                    assert_eq!(actual, expected, "terminal controls must not add a line");
                    assert_eq!(
                        actual.concat(),
                        padded,
                        "logical CJK text and controls survive"
                    );
                }
            }
        }
    }
}
