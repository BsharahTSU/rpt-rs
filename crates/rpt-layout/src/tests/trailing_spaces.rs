//! Complete-line compatibility preparation must agree with measurement and emission.
//! Synthetic widths make the verified centered-field anchor change independently calculable.
use super::*;
use rpt_model::{Alignment, FieldFormat, ReadingOrder};
use rpt_pages::{FontSpec, TextAlign, TextLayout};

#[derive(Debug)]
struct PreparedLayout(bool);
impl TextLayout for PreparedLayout {
    fn prepare_line<'a>(&self, text: &'a str) -> &'a str {
        if self.0 {
            text.trim_end_matches(' ')
        } else {
            text
        }
    }
    fn width_twips(&self, text: &str, _font: &FontSpec) -> f64 {
        text.chars()
            .map(|ch| if ch == ' ' { 41.0 } else { 100.0 })
            .sum()
    }
    fn line_height_twips(&self, _font: &FontSpec) -> f64 {
        240.0
    }
    fn ascent_twips(&self, _font: &FontSpec) -> f64 {
        160.0
    }
    fn wrap(&self, text: &str, _width: f64, _font: &FontSpec) -> Vec<String> {
        // A controlled wrapper which retains line-final whitespace, so post-wrap
        // preparation is exercised independently of a real shaper's own trimming.
        if text.starts_with("wrap ") {
            assert_eq!(
                text, "wrap tail",
                "prepare the complete field paragraph before wrapping"
            );
            vec!["wrap  ".into(), "tail  ".into()]
        } else {
            vec![text.into()]
        }
    }
}

fn render(objects: Vec<ReportObject>, value: &str, enabled: bool) -> rpt_pages::PagedDocument {
    let mut report = Report::default();
    report.print_options.content_width = Twips(12000);
    report.print_options.content_height = Twips(16000);
    report.report_definition.areas = vec![area(
        AreaSectionKind::ReportHeader,
        vec![section(
            AreaSectionKind::ReportHeader,
            "Header",
            1000,
            objects,
        )],
    )];
    let saved = saved_data(&[("t.value", FieldValueType::String)], &[&[value]]);
    let ds = build_dataset(&SavedDataSource::new(&saved), &report.data_definition);
    let formulas = rpt_data::compile_formulas(&report.data_definition);
    crate::layout_with(&report, &ds, &formulas, Box::new(PreparedLayout(enabled)))
}

fn field(align: Alignment, rtl: bool) -> ReportObject {
    let mut obj = db_field_object("Value", "{t.value}", 0);
    obj.format.horizontal_alignment = align;
    if let ReportObjectKind::Field(f) = &mut obj.kind {
        let mut fmt = FieldFormat::default();
        fmt.string.reading_order = if rtl {
            ReadingOrder::RightToLeft
        } else {
            ReadingOrder::LeftToRight
        };
        f.format = Some(fmt);
    }
    obj
}

fn anchor(run: &rpt_pages::TextRun) -> f64 {
    let slack = f64::from(run.bounds.width.0 - run.metrics.as_ref().unwrap().advance.0);
    f64::from(run.bounds.left.0)
        + match run.align {
            TextAlign::Center => slack / 2.0,
            TextAlign::Right => slack,
            _ => 0.0,
        }
}

#[test]
fn centered_fields_prepare_text_for_alignment_advance_and_bidi_in_both_directions() {
    for value in ["  alpha  beta ", "  مرحبا بالعالم ", "  中文 評量 "] {
        for rtl in [false, true] {
            for align in [
                Alignment::LeftAlign,
                Alignment::HorizontalCenterAlign,
                Alignment::RightAlign,
            ] {
                let original = render(vec![field(align, rtl)], value, false);
                let prepared = render(vec![field(align, rtl)], value, true);
                let old = text_runs(&original)[0];
                let new = text_runs(&prepared)[0];
                let eligible = matches!(align, Alignment::HorizontalCenterAlign);
                assert_eq!(
                    old.text, value,
                    "ordinary constructed-model behavior stays unchanged"
                );
                assert_eq!(
                    new.text,
                    if eligible {
                        value.trim_end_matches(' ')
                    } else {
                        value
                    }
                );
                assert_eq!(new.rtl, rtl);
                assert_eq!(new.align, old.align);
                assert_eq!(
                    old.metrics.as_ref().unwrap().advance.0
                        - new.metrics.as_ref().unwrap().advance.0,
                    if eligible { 41 } else { 0 }
                );
                assert_eq!(anchor(new) - anchor(old), if eligible { 20.5 } else { 0.0 });
                if !eligible {
                    assert_eq!(
                        new, old,
                        "other alignments keep all emitted text and metrics"
                    );
                }
            }
        }
    }
}

#[test]
fn field_preparation_keeps_blank_line_height_and_prepares_wrapped_results() {
    let doc = render(
        vec![field(Alignment::HorizontalCenterAlign, false)],
        "alpha  \n   \nbeta ",
        true,
    );
    let runs = text_runs(&doc);
    assert_eq!(
        runs.iter().map(|r| r.text.as_str()).collect::<Vec<_>>(),
        ["alpha", "", "beta"]
    );
    assert_eq!(runs[1].metrics.as_ref().unwrap().advance, Twips(0));
    assert_eq!(runs[1].bounds.top.0 - runs[0].bounds.top.0, 240);
    assert_eq!(runs[2].bounds.top.0 - runs[1].bounds.top.0, 240);

    let mut obj = field(Alignment::HorizontalCenterAlign, false);
    obj.format.can_grow = true;
    let doc = render(vec![obj], "wrap tail  ", true);
    let runs = text_runs(&doc);
    assert_eq!(
        runs.iter().map(|r| r.text.as_str()).collect::<Vec<_>>(),
        ["wrap", "tail"]
    );
    assert!(runs
        .iter()
        .all(|r| r.metrics.as_ref().unwrap().advance == Twips(400)));
}

#[test]
fn preparation_does_not_trim_nbsp_tabs_or_spaces_before_an_interior_tab() {
    for value in ["alpha\u{a0}", "alpha \t", "alpha\t"] {
        let old = render(
            vec![field(Alignment::HorizontalCenterAlign, false)],
            value,
            false,
        );
        let new = render(
            vec![field(Alignment::HorizontalCenterAlign, false)],
            value,
            true,
        );
        assert_eq!(text_runs(&new), text_runs(&old), "{value:?}");
    }
    let doc = render(
        vec![field(Alignment::HorizontalCenterAlign, false)],
        "AAA  \tB ",
        true,
    );
    let runs = text_runs(&doc);
    assert_eq!(
        runs.iter().map(|r| r.text.as_str()).collect::<Vec<_>>(),
        ["AAA  ", "B"]
    );
    // AAA+two spaces is382 twips and must advance to720, not360, at the tab.
    assert_eq!(runs[1].bounds.left.0 - runs[0].bounds.left.0, 720);
    assert_eq!(runs[0].metrics.as_ref().unwrap().advance, Twips(382));
}

#[test]
fn unverified_static_text_and_field_headings_preserve_their_trailing_spaces() {
    let doc = render(
        vec![
            field(Alignment::HorizontalCenterAlign, false),
            text_object("Static", "static  ", 300),
            field_heading_object("Heading", "heading  ", 600),
        ],
        "field  ",
        true,
    );
    let runs = text_runs(&doc);
    assert_eq!(
        runs.iter().map(|r| r.text.as_str()).collect::<Vec<_>>(),
        ["field", "static  ", "heading  "]
    );
}

#[test]
fn spaced_adapter_prepares_before_counting_retained_scalars() {
    let inner = PreparedLayout(true);
    let spaced = crate::text::SpacedLayout::new(&inner, Twips(7));
    let font = FontSpec::default();
    assert_eq!(spaced.prepare_line("  A B  "), "  A B");
    assert_eq!(
        spaced.width_twips(spaced.prepare_line("  A B  "), &font),
        323.0 + 5.0 * 7.0
    );
    // Fragment measurement must not silently trim spaces before a tab/next segment.
    assert_eq!(spaced.width_twips("A ", &font), 141.0 + 2.0 * 7.0);
}

#[test]
fn page_total_field_rewrite_reapplies_only_centered_field_preparation() {
    for (align, rtl) in [
        (Alignment::HorizontalCenterAlign, false),
        (Alignment::LeftAlign, false),
        (Alignment::RightAlign, false),
        (Alignment::HorizontalCenterAlign, true),
    ] {
        let mut obj = field(align, rtl);
        if let ReportObjectKind::Field(f) = &mut obj.kind {
            f.ref_kind = FieldRefKind::Special;
            f.data_source = "TotalPageCount".into();
            f.value_type = FieldValueType::Number;
            let fmt = f.format.as_mut().unwrap();
            fmt.common.use_system_defaults = false;
            fmt.numeric.currency_symbol = rpt_model::CurrencySymbolFormat::FloatingSymbol;
            fmt.numeric.currency_symbol_text = "  ".into();
            fmt.numeric.currency_position =
                rpt_model::CurrencyPosition::TrailingCurrencyOutsideNegative;
            fmt.numeric.decimal_places = 0;
        }
        let old = render(vec![obj.clone()], "", false);
        let new = render(vec![obj], "", true);
        let eligible = matches!(align, Alignment::HorizontalCenterAlign);
        // Number fields reserve a leading sign cell. Preparation must keep that cell
        // while removing only the two trailing symbol spaces reintroduced by the fixup.
        assert_eq!(
            text_runs(&old)[0].text,
            " 1  ",
            "the final rewrite must really introduce a suffix"
        );
        assert_eq!(
            text_runs(&new)[0].text,
            if eligible { " 1" } else { " 1  " }
        );
        assert_eq!(
            text_runs(&new)[0].metrics.as_ref().unwrap().advance,
            Twips(if eligible { 141 } else { 223 })
        );
        if !eligible {
            assert_eq!(text_runs(&new), text_runs(&old));
        }
    }
}

#[test]
fn currency_rewrite_reapplies_only_centered_field_preparation() {
    use rpt_format_value::{CurrencyFormat, CurrencyPosition, NumberFormat};
    for (align, rtl) in [
        (Alignment::HorizontalCenterAlign, false),
        (Alignment::LeftAlign, false),
        (Alignment::RightAlign, false),
        (Alignment::HorizontalCenterAlign, true),
    ] {
        let seed = render(vec![field(align, rtl)], "1$", true);
        let mut first = text_runs(&seed)[0].clone();
        first.character_spacing = Twips(7);
        let mut second = first.clone();
        second.text = "2$".into();
        let mut page = rpt_pages::Page::new(1, seed.pages[0].size);
        page.push(DrawOp::Text(first));
        page.push(DrawOp::Text(second));
        let spec = CurrencyFormat {
            number: NumberFormat::integer(),
            symbol: "$".into(),
            position: CurrencyPosition::TrailingNoSpace,
        };
        let fixups: Vec<_> = [1.0, 2.0]
            .into_iter()
            .enumerate()
            .map(|(op_index, value)| crate::CurrencyFixup {
                page_index: 0,
                op_index,
                object: "Amount".into(),
                mark: crate::resolve::CurrencyMark {
                    value,
                    spec: spec.clone(),
                },
            })
            .collect();
        let mut pages = vec![page];
        crate::currency::apply(&mut pages, &fixups, &PreparedLayout(true));
        let DrawOp::Text(first) = &pages[0].ops[0] else {
            panic!("first field");
        };
        let DrawOp::Text(second) = &pages[0].ops[1] else {
            panic!("second field");
        };
        let eligible = matches!(align, Alignment::HorizontalCenterAlign);
        assert_eq!(first.text, "1$");
        assert_eq!(second.text, if eligible { "2" } else { "2  " });
        assert_eq!(
            second.metrics.as_ref().unwrap().advance,
            Twips(if eligible { 107 } else { 203 })
        );
    }
}

#[test]
fn centered_fields_keep_unknown_direction_and_static_rtl_text_unchanged() {
    let mut unknown = field(Alignment::HorizontalCenterAlign, false);
    if let ReportObjectKind::Field(f) = &mut unknown.kind {
        f.format.as_mut().unwrap().string.reading_order = ReadingOrder::Other(99);
    }
    let mut static_rtl = text_object("StaticRtl", "مرحبا  ", 300);
    static_rtl.format.horizontal_alignment = Alignment::HorizontalCenterAlign;
    if let ReportObjectKind::Text(t) = &mut static_rtl.kind {
        t.reading_order = ReadingOrder::RightToLeft;
    }
    let objects = vec![unknown, static_rtl];
    let ordinary = render(objects.clone(), "  中文  ", false);
    let prepared = render(objects, "  中文  ", true);
    assert_eq!(text_runs(&prepared), text_runs(&ordinary));
}
