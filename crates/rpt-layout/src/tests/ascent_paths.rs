//! Native controls select printer ascent for static text, explicit RTL fields, and RTF fields.
//! Distinct synthetic metrics expose dispatch mistakes without depending on proprietary fonts.
use super::*;
use rpt_model::{Alignment, FieldFormat, ReadingOrder, TextFormat};
use rpt_pages::{FontSpec, TextLayout};

#[derive(Debug)]
struct PathMetrics;
impl TextLayout for PathMetrics {
    fn width_twips(&self, text: &str, _font: &FontSpec) -> f64 {
        text.chars().count() as f64 * 100.0
    }
    fn line_height_twips(&self, _font: &FontSpec) -> f64 {
        240.0
    }
    fn ascent_twips(&self, _font: &FontSpec) -> f64 {
        193.0
    }
    fn field_ascent_twips(&self, _font: &FontSpec) -> f64 {
        177.0
    }
    fn wrap(&self, text: &str, _width: f64, _font: &FontSpec) -> Vec<String> {
        if text == "empty-wrap" {
            vec![]
        } else {
            vec!["first".into(), "second".into()]
        }
    }
}

fn field(name: &str, text_format: Option<TextFormat>, order: ReadingOrder) -> ReportObject {
    let mut obj = db_field_object(name, "{t.value}", 0);
    if let ReportObjectKind::Field(f) = &mut obj.kind {
        if let Some(text_format) = text_format {
            let mut format = FieldFormat::default();
            format.string.text_format = text_format;
            format.string.reading_order = order;
            f.format = Some(format);
        }
    }
    obj
}

fn render(
    objects: Vec<ReportObject>,
    kind: AreaSectionKind,
    value: &str,
    metrics: Box<dyn TextLayout>,
) -> rpt_pages::PagedDocument {
    let mut report = Report::default();
    report.print_options.content_width = Twips(12000);
    report.print_options.content_height = Twips(16000);
    report.report_definition.areas =
        vec![area(kind, vec![section(kind, "Metrics", 1000, objects)])];
    let saved = saved_data(&[("t.value", FieldValueType::String)], &[&[value]]);
    let ds = build_dataset(&SavedDataSource::new(&saved), &report.data_definition);
    let formulas = rpt_data::compile_formulas(&report.data_definition);
    crate::layout_with(&report, &ds, &formulas, metrics)
}

#[test]
fn verified_field_paths_select_distinct_ascents_and_other_formats_keep_prior_metrics() {
    use ReadingOrder::{LeftToRight as Ltr, RightToLeft as Rtl};
    let cases = [
        ("MissingFormat", None, Ltr, 177),
        ("PlainLtr", Some(TextFormat::StandardText), Ltr, 177),
        ("PlainRtl", Some(TextFormat::StandardText), Rtl, 193),
        ("RichLtr", Some(TextFormat::RTFText), Ltr, 193),
        ("RichRtl", Some(TextFormat::RTFText), Rtl, 193),
        ("HtmlUnverified", Some(TextFormat::HTMLText), Ltr, 177),
        ("UnknownUnverified", Some(TextFormat::Other(99)), Ltr, 177),
        (
            "UnknownDirection",
            Some(TextFormat::StandardText),
            ReadingOrder::Other(99),
            177,
        ),
        ("HtmlExplicitRtl", Some(TextFormat::HTMLText), Rtl, 193),
    ];
    let mut objects: Vec<_> = cases
        .iter()
        .map(|(name, fmt, order, _)| field(name, *fmt, *order))
        .collect();
    objects.push(text_object("Static", "label", 0));
    objects.push(field_heading_object("Heading", "heading", 0));
    let doc = render(
        objects,
        AreaSectionKind::ReportHeader,
        "sample",
        Box::new(PathMetrics),
    );
    let runs = text_runs(&doc);
    assert_eq!(runs.len(), cases.len() + 2);
    for (name, _, _, expected) in cases {
        let run = runs
            .iter()
            .find(|run| run.source.as_ref().unwrap().object_name.as_deref() == Some(name))
            .unwrap();
        assert_eq!(
            run.metrics.as_ref().unwrap().ascent,
            Twips(expected),
            "{name}"
        );
        assert_eq!(run.metrics.as_ref().unwrap().line_height, Twips(240));
        assert_eq!(
            run.bounds.top,
            Twips(0),
            "the route changes ascent, not authored position"
        );
    }
    for run in &runs[cases.len()..] {
        assert_eq!(run.metrics.as_ref().unwrap().ascent, Twips(193));
    }
}

#[test]
fn grow_alignment_and_insufficient_height_do_not_select_a_printer_field_path() {
    for grow in [false, true] {
        for align in [
            Alignment::LeftAlign,
            Alignment::HorizontalCenterAlign,
            Alignment::RightAlign,
        ] {
            for height in [200, 240, 300] {
                let mut obj = field(
                    "Plain",
                    Some(TextFormat::StandardText),
                    ReadingOrder::LeftToRight,
                );
                obj.format.can_grow = grow;
                obj.format.horizontal_alignment = align;
                obj.bounds.height = Twips(height);
                // The wrapper returns two lines for grow objects, so the policy is also
                // pinned when line height exceeds the authored box. It keeps prior metrics.
                let doc = render(
                    vec![obj],
                    AreaSectionKind::ReportHeader,
                    "sample",
                    Box::new(PathMetrics),
                );
                let runs = text_runs(&doc);
                assert_eq!(runs.len(), if grow { 2 } else { 1 });
                assert!(runs
                    .iter()
                    .all(|run| run.metrics.as_ref().unwrap().ascent == Twips(177)));
            }
        }
    }
}

#[test]
fn fixed_footer_preserves_format_dispatch_even_when_growing_is_disabled() {
    for (format, order, expected) in [
        (TextFormat::StandardText, ReadingOrder::LeftToRight, 177),
        (TextFormat::StandardText, ReadingOrder::RightToLeft, 193),
        (TextFormat::RTFText, ReadingOrder::LeftToRight, 193),
    ] {
        let mut obj = field("Footer", Some(format), order);
        obj.format.can_grow = true;
        obj.bounds.height = Twips(239); // Below one line's pitch, matching the unresolved boundary.
        let doc = render(
            vec![obj],
            AreaSectionKind::PageFooter,
            "sample",
            Box::new(PathMetrics),
        );
        let runs = text_runs(&doc);
        assert_eq!(
            runs.len(),
            1,
            "fixed furniture does not use the two-line wrapper"
        );
        assert_eq!(runs[0].metrics.as_ref().unwrap().ascent, Twips(expected));
    }
}

#[test]
fn empty_wrapper_fallback_uses_the_same_field_metric_path() {
    for (format, expected) in [(TextFormat::StandardText, 177), (TextFormat::RTFText, 193)] {
        let mut obj = field("Empty", Some(format), ReadingOrder::LeftToRight);
        obj.format.can_grow = true;
        let doc = render(
            vec![obj],
            AreaSectionKind::ReportHeader,
            "empty-wrap",
            Box::new(PathMetrics),
        );
        let runs = text_runs(&doc);
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].text, "");
        assert_eq!(runs[0].metrics.as_ref().unwrap().ascent, Twips(expected));
        assert_eq!(runs[0].metrics.as_ref().unwrap().line_height, Twips(240));
    }
}

#[test]
fn layouts_without_a_field_override_keep_existing_behavior() {
    #[derive(Debug)]
    struct ExistingMetrics;
    impl TextLayout for ExistingMetrics {
        fn width_twips(&self, text: &str, font: &FontSpec) -> f64 {
            PathMetrics.width_twips(text, font)
        }
        fn ascent_twips(&self, _font: &FontSpec) -> f64 {
            193.0
        }
    }
    let doc = render(
        vec![
            field("Plain", None, ReadingOrder::LeftToRight),
            field("Rich", Some(TextFormat::RTFText), ReadingOrder::LeftToRight),
            field(
                "Rtl",
                Some(TextFormat::StandardText),
                ReadingOrder::RightToLeft,
            ),
            text_object("Static", "label", 0),
        ],
        AreaSectionKind::ReportHeader,
        "sample",
        Box::new(ExistingMetrics),
    );
    assert_eq!(text_runs(&doc).len(), 4);
    assert!(text_runs(&doc)
        .iter()
        .all(|run| run.metrics.as_ref().unwrap().ascent == Twips(193)));
}

#[test]
fn spacing_adapter_forwards_both_ascent_contracts() {
    let inner = PathMetrics;
    let spaced = crate::text::SpacedLayout::new(&inner, Twips(7));
    let font = FontSpec::default();
    assert_eq!(spaced.ascent_twips(&font), 193.0);
    assert_eq!(spaced.field_ascent_twips(&font), 177.0);
}
