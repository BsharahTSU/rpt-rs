//! Prepared-data renderer. SQL credentials stay in the adapter and never enter this process.
mod params;
use rpt_data::{
    cell_to_value_reporting, Cell, Column, DateTimeSpecials, EmptySource, Row, RowData, RowSource,
    ScopeData,
};
use rpt_formula::eval::{Date, Time, Value};
use rpt_pages::{DiagnosticKind, DrawOp, PagedDocument};
use rpt_reader::model::{FieldValueType, Report};
use rpt_render::{FontSource, PdfOptions, ReportDocument};
use serde::Deserialize;
use serde_json::{json, Value as Json};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    io::{self, Read, Write},
    num::NonZeroU32,
};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    report_path: String,
    output_path: String,
    export_format: String,
    // A captured local wall-clock value, with no UTC conversion. Older prepared inputs omit it.
    #[serde(default)]
    as_of_local: Option<String>,
    columns: Vec<InputColumn>,
    rows: Vec<Vec<Option<InputCell>>>,
    query_params: BTreeMap<String, Json>,
    report_variables: BTreeMap<String, Json>,
}
#[derive(Debug, Deserialize)]
struct InputColumn {
    name: String,
    value_type: FieldValueType,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct InputCell {
    text: Option<String>,
    bytes: Option<Vec<u8>>,
}
impl InputCell {
    fn cell(&self) -> Result<Cell, String> {
        match (&self.text, &self.bytes) {
            (Some(text), None) => Ok(Cell::Text(text.clone())),
            (None, Some(bytes)) => Ok(Cell::Bytes(bytes.clone())),
            _ => Err("A prepared cell must contain exactly one text or bytes value".into()),
        }
    }
}
#[derive(Debug)]
struct Scope<'a> {
    input: &'a Input,
    failures: RefCell<Vec<String>>,
}
impl ScopeData for Scope<'_> {
    fn rows_for(&self, report: &Report) -> Option<Box<dyn RowSource>> {
        match prepare(report, self.input) {
            Ok(data) => Some(Box::new(data)),
            Err(error) => {
                self.failures.borrow_mut().push(error);
                Some(Box::new(EmptySource))
            }
        }
    }
}
fn prepare(report: &Report, input: &Input) -> Result<RowData, String> {
    let mut columns: Vec<Column> = input
        .columns
        .iter()
        .map(|c| Column {
            name: c.name.clone(),
            value_type: c.value_type,
        })
        .collect();
    for table in &report.database.tables {
        let alias = if table.alias.is_empty() {
            &table.name
        } else {
            &table.alias
        };
        for field in &table.data_fields {
            let name = field
                .short_name
                .as_deref()
                .unwrap_or(&field.name)
                .rsplit('.')
                .next()
                .unwrap_or(&field.name);
            columns.push(Column {
                name: format!("{alias}.{name}"),
                value_type: field.value_type,
            });
        }
    }
    let mut rows = Vec::with_capacity(input.rows.len());
    for (row_index, values) in input.rows.iter().enumerate() {
        if values.len() != input.columns.len() {
            return Err("Prepared row does not match its schema".into());
        }
        let mut row = Row::default();
        for column in &columns {
            let short = column.name.rsplit('.').next().unwrap_or(&column.name);
            let index = input
                .columns
                .iter()
                .position(|c| c.name.eq_ignore_ascii_case(short));
            let cell = index
                .and_then(|i| values[i].as_ref())
                .map(InputCell::cell)
                .transpose()?;
            // DBNull stays null even for a text field; the generic saved-data converter uses
            // empty strings for absent text, which would change IsNull/selection semantics here.
            let cell = if column.value_type == FieldValueType::Boolean {
                match cell {
                    Some(Cell::Text(ref text)) if text == "1" => Some(Cell::Text("true".into())),
                    Some(Cell::Text(ref text)) if text == "0" => Some(Cell::Text("false".into())),
                    Some(Cell::Text(ref text))
                        if !text.eq_ignore_ascii_case("true")
                            && !text.eq_ignore_ascii_case("false") =>
                    {
                        return Err(format!("Invalid Boolean cell in column {}", column.name))
                    }
                    cell => cell,
                }
            } else {
                cell
            };
            let (value, coercion) = if cell.is_none() {
                (Value::Null, None)
            } else {
                cell_to_value_reporting(column.value_type, cell.as_ref())
            };
            if coercion.is_some() {
                return Err(format!(
                    "Invalid typed cell in row {row_index}, column {}",
                    column.name
                ));
            }
            row.insert(&column.name, value);
        }
        rows.push(row);
    }
    Ok(RowData::new(columns, rows))
}
fn accept_document(document: &PagedDocument) -> Result<(), String> {
    for diagnostic in &document.diagnostics {
        eprintln!(
            "{}",
            serde_json::to_string(diagnostic).map_err(|e| e.to_string())?
        );
        if diagnostic.kind != DiagnosticKind::FontSubstituted {
            return Err("Report contains an unsupported or degraded rendering diagnostic".into());
        }
    }
    for op in document.pages.iter().flat_map(|page| &page.ops) {
        if let DrawOp::Image(image) = op {
            if !document.assets.contains_key(&image.image_id) {
                return Err(format!(
                    "Missing or unsupported rendered image: {}",
                    image.image_id
                ));
            }
        }
    }
    if document.pages.is_empty() {
        return Err("Report produced no pages".into());
    }
    Ok(())
}
// Host wall-clock fields are explicit because Docker may use a different time zone.
// This is a naive calendar value; do not interpret it as UTC or apply an offset.
fn parse_local_clock(value: &str) -> Result<DateTimeSpecials, String> {
    let error =
        || "Invalid local render clock; expected yyyy-MM-ddTHH:mm:ss without an offset".to_string();
    let bytes = value.as_bytes();
    if bytes.len() != 19
        || bytes.iter().enumerate().any(|(index, byte)| match index {
            4 | 7 => *byte != b'-',
            10 => *byte != b'T',
            13 | 16 => *byte != b':',
            _ => !byte.is_ascii_digit(),
        })
    {
        return Err(error());
    }
    let pair = |index: usize| (bytes[index] - b'0') * 10 + bytes[index + 1] - b'0';
    let year = i32::from(pair(0)) * 100 + i32::from(pair(2));
    let date = Date::new(year, pair(5), pair(8));
    let time = Time::new(pair(11), pair(14), pair(17));
    if year < 1
        || !(1..=12).contains(&date.month)
        || !(1..=31).contains(&date.day)
        || Date::from_days(date.to_days()) != date
        || time.hour > 23
        || time.minute > 59
        || time.second > 59
    {
        return Err(error());
    }
    Ok(DateTimeSpecials::new(date, time))
}

fn resolve_render_clock(
    prepared: Option<&str>,
    environment: Option<&str>,
    fallback: DateTimeSpecials,
) -> Result<DateTimeSpecials, String> {
    match prepared.or(environment) {
        Some(value) => parse_local_clock(value),
        // Backward compatibility for standalone worker inputs. Normal bridge calls always pass
        // captured DateTime.Now or the host launcher's explicit local clock.
        None => Ok(fallback),
    }
}

fn render_clock(prepared: Option<&str>) -> Result<DateTimeSpecials, String> {
    if let Some(value) = prepared {
        return parse_local_clock(value);
    }
    let environment = match std::env::var("STC_RPT_LOCAL_DATETIME") {
        Ok(value) => Some(value),
        Err(std::env::VarError::NotPresent) => None,
        Err(std::env::VarError::NotUnicode(_)) => {
            return Err("Invalid local render clock encoding".into())
        }
    };
    resolve_render_clock(
        prepared,
        environment.as_deref(),
        rpt_render::default_as_of(),
    )
}

fn render_locale() -> Result<rpt_render::Locale, String> {
    let tag = std::env::var("STC_RPT_LOCALE").unwrap_or_else(|_| "en-US".into());
    let pattern = std::env::var("STC_RPT_SHORT_DATE").ok();
    locale_with_pattern(&tag, pattern.as_deref())
}
fn locale_with_pattern(tag: &str, pattern: Option<&str>) -> Result<rpt_render::Locale, String> {
    let mut locale = rpt_render::Locale::lookup(tag)
        .ok_or_else(|| format!("Unsupported render locale: {tag}"))?;
    if let Some(pattern) = pattern {
        let (order, padded, sep) = match pattern {
            "dd/MM/yyyy" => ("en-GB", true, '/'),
            "d/M/yyyy" => ("en-GB", false, '/'),
            "MM/dd/yyyy" => ("en-US", true, '/'),
            "M/d/yyyy" => ("en-US", false, '/'),
            "dd.MM.yyyy" => ("en-GB", true, '.'),
            "yyyy-MM-dd" => ("ja-JP", true, '-'),
            _ => return Err(format!("Unsupported host short-date pattern: {pattern}")),
        };
        locale.date_order = rpt_render::Locale::from_tag(order).date_order;
        locale.short_date_leading_zero = padded;
        locale.date_sep = sep;
    }
    Ok(locale)
}
// Windows printer rectangles can truncate millimetre conversions by two twips.
// Restore the declared standard sheet only within that rounding tolerance; retain
// custom/driver dimensions and the stored orientation in every other case.
fn normalize_driver_paper_rounding(report: &mut Report) {
    let p = &mut report.print_options;
    if let Some((w, h)) = p.paper_size.std_dims() {
        let actual_w = p.content_width.0 + p.margins.left.0 + p.margins.right.0;
        let actual_h = p.content_height.0 + p.margins.top.0 + p.margins.bottom.0;
        for (w, h) in [(w, h), (h, w)] {
            if (w - actual_w).abs() <= 2 && (h - actual_h).abs() <= 2 {
                p.content_width.0 += w - actual_w;
                p.content_height.0 += h - actual_h;
                break;
            }
        }
    }
}
// Crystal preserves the fractional point coordinate transform but writes an
// integer MediaBox. Keep drawing in the same bottom-left physical coordinates.
fn legacy_media_boxes(document: &mut PagedDocument) {
    for page in &mut document.pages {
        let dy = page.size.height.0.rem_euclid(20);
        page.size.height.0 -= dy;
        page.size.width.0 -= page.size.width.0.rem_euclid(20);
        page.origin.y.0 -= dy;
    }
}
// Prefer an explicit device override, then the template DEVMODE, then the STC
// 600-DPI fallback. Layout line spacing is measured
// in printer pixels, independently of the PDF exporter's 1440-DPI glyph cells.
fn parse_printer_dpi(value: Option<&str>) -> Result<NonZeroU32, String> {
    let value = value.unwrap_or("600");
    if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
        return Err("Invalid printer DPI; expected an integer from 72 to 9600".into());
    }
    let dpi = value
        .parse::<u32>()
        .ok()
        .filter(|v| (72..=9600).contains(v));
    dpi.and_then(NonZeroU32::new)
        .ok_or_else(|| "Invalid printer DPI; expected an integer from 72 to 9600".into())
}

fn resolve_printer_dpi(explicit: Option<&str>, saved: Option<u16>) -> Result<NonZeroU32, String> {
    if let Some(value) = explicit {
        return parse_printer_dpi(Some(value));
    }
    match saved {
        Some(dpi) => parse_printer_dpi(Some(&dpi.to_string())),
        None => parse_printer_dpi(None),
    }
}

fn render_printer_dpi(saved: Option<u16>) -> Result<NonZeroU32, String> {
    match std::env::var("STC_RPT_PRINTER_DPI") {
        Ok(value) => resolve_printer_dpi(Some(&value), saved),
        Err(std::env::VarError::NotPresent) => resolve_printer_dpi(None, saved),
        Err(_) => Err("Invalid printer DPI environment encoding".into()),
    }
}

fn run(input: Input) -> Result<Json, String> {
    if !matches!(input.export_format.as_str(), "pdf" | "page_ir") {
        return Err("Unsupported export format".into());
    }
    let as_of = render_clock(input.as_of_local.as_deref())?;

    let report = ReportDocument::load(&input.report_path).map_err(|e| e.to_string())?;
    let print = &report.report().print_options;
    let printer_dpi = render_printer_dpi(print.saved_printer_dpi_y.or(print.saved_printer_dpi_x))?;
    let source = prepare(report.report(), &input)?;
    let mut model = report.report().clone();
    normalize_driver_paper_rounding(&mut model);
    let parameters = params::bind(
        report.report(),
        &input.query_params,
        &input.report_variables,
    )?;
    let scope = Scope {
        input: &input,
        failures: RefCell::new(Vec::new()),
    };
    let sink = rpt_data::CollectingSink::new();
    let dataset = rpt_data::build_dataset_opts(
        &source,
        &report.report().data_definition,
        rpt_data::DatasetOptions {
            params: Some(&parameters),
            sink: Some(&sink),
            datetime: Some(as_of),
            ..Default::default()
        },
    );
    let mut document = rpt_render::render_dataset_with(
        &model,
        &dataset,
        Box::new(
            rpt_text::WindowsLayout::with_printer_dpi(FontSource::System, printer_dpi)
                .without_latin_kerning()
                .without_trailing_spaces(),
        ),
        render_locale()?,
        Some(&scope),
        Some(as_of),
    );
    document
        .diagnostics
        .extend(rpt_layout::diagnostics::from_evals(
            &sink.into_diagnostics(),
        ));
    // The legacy PDF exporter treats nominal point sizes as positive GDI cell
    // heights. Apply that conversion only after wrapping/pagination, keeping the
    // nominal baselines. This policy belongs to this compatibility adapter.
    let fonts = FontSource::System.load();
    let alignment = rpt_text::CosmicLayout::with_system_fonts().without_latin_kerning();
    for op in document.pages.iter_mut().flat_map(|p| &mut p.ops) {
        if let DrawOp::Text(text) = op {
            if let Some(metrics) = fonts.windows_metrics(&text.font, true) {
                let size = metrics.em_twips as f32 / 20.0;
                let ratio = size / text.font.size_pt;
                text.font.size_pt = size;
                if let Some(m) = &mut text.metrics {
                    m.advance.0 = if text.character_spacing.0 == 0
                        && matches!(
                            text.align,
                            rpt_pages::TextAlign::Center | rpt_pages::TextAlign::Right
                        ) {
                        // Native alignment uses integer PDF widths and rounds each shaped
                        // glyph on the 1440 logical grid. Do not change nominal wrapping or
                        // the PDF's drawn advances. Centered LTR static text retains its
                        // trailing glyphs but excludes those spaces from the alignment width.
                        let measured =
                            if matches!(text.align, rpt_pages::TextAlign::Center) && !text.rtl {
                                text.text.trim_end_matches(' ')
                            } else {
                                &text.text
                            };
                        alignment.crystal_alignment_width_twips(measured, &text.font) as i32
                    } else {
                        (m.advance.0 as f32 * ratio).round() as i32
                    };
                }
            }
        }
    }
    if !scope.failures.borrow().is_empty() {
        return Err(scope.failures.borrow().join("; "));
    }
    legacy_media_boxes(&mut document);
    if let Ok(path) = std::env::var("STC_RPT_DEBUG_IR") {
        let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
        serde_json::to_writer(file, &document).map_err(|e| e.to_string())?;
    }
    accept_document(&document)?;
    let count = document.pages.len();
    if input.export_format == "page_ir" {
        return Ok(json!({"pageCount": count, "document": document}));
    }
    let pdf = rpt_render::try_render_document(
        &document,
        &PdfOptions {
            fonts: FontSource::System,
            disable_latin_kerning: true,
            legacy_arabic_mark_offsets: true,
            ..Default::default()
        },
    )
    .map_err(|e| e.to_string())?;
    std::fs::write(&input.output_path, pdf).map_err(|e| e.to_string())?;
    Ok(json!({"pageCount": count}))
}
fn main() {
    let result = (|| {
        let mut json = String::new();
        io::stdin()
            .read_to_string(&mut json)
            .map_err(|e| e.to_string())?;
        let input = serde_json::from_str(&json).map_err(|e| e.to_string())?;
        run(input)
    })();
    match result {
        Ok(response) => {
            if let Err(error) = serde_json::to_writer(io::stdout().lock(), &response)
                .and_then(|_| io::stdout().flush().map_err(serde_json::Error::io))
            {
                eprintln!("Cannot write worker response: {error}");
                std::process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rpt_pages::{Diagnostic, Page, PageSize};
    #[test]
    fn printer_dpi_defaults_to_native_reference_and_accepts_explicit_devices() {
        assert_eq!(resolve_printer_dpi(None, None).unwrap().get(), 600);
        assert_eq!(resolve_printer_dpi(None, Some(300)).unwrap().get(), 300);
        assert_eq!(
            resolve_printer_dpi(Some("1200"), Some(600)).unwrap().get(),
            1200
        );
        assert!(resolve_printer_dpi(Some("bad"), Some(600)).is_err());
        for dpi in [72, 300, 600, 1200, 1440, 9600] {
            assert_eq!(
                parse_printer_dpi(Some(&dpi.to_string())).unwrap().get(),
                dpi
            );
        }
    }

    #[test]
    fn malformed_printer_dpi_is_rejected_without_silent_fallback() {
        for value in [
            "",
            "0",
            "71",
            "9601",
            "-600",
            "+600",
            "600.0",
            " 600",
            "600 ",
            "６００",
            "4294967296",
        ] {
            assert!(
                parse_printer_dpi(Some(value)).is_err(),
                "accepted {value:?}"
            );
        }
    }

    #[test]
    fn captured_local_clock_keeps_next_day_near_midnight_for_formula_specials() {
        // 00:15 in UTC+03 is still 21:15 on the previous UTC date.
        let utc = DateTimeSpecials::new(Date::new(2026, 10, 15), Time::new(21, 15, 7));
        let local = resolve_render_clock(Some("2026-10-16T00:15:07"), None, utc).unwrap();
        assert_ne!(local, utc);
        let registry = rpt_data::FormulaRegistry::new().with_datetime(local);
        let row = Row::default();
        let context = rpt_data::DataContext::new(&row, &registry);
        for (formula, expected) in [
            ("CurrentDate", Value::Date(Date::new(2026, 10, 16))),
            ("Today", Value::Date(Date::new(2026, 10, 16))),
            ("CurrentTime", Value::Time(Time::new(0, 15, 7))),
            (
                "CurrentDateTime",
                Value::DateTime(Date::new(2026, 10, 16), Time::new(0, 15, 7)),
            ),
        ] {
            let (ast, diagnostics) = rpt_formula::parse(formula, rpt_formula::Syntax::Crystal);
            assert!(diagnostics.is_empty());
            assert_eq!(rpt_formula::eval::eval(&ast, &context).unwrap(), expected);
        }
    }

    #[test]
    fn frozen_prepared_clock_wins_and_older_inputs_keep_their_fallback() {
        let fallback = DateTimeSpecials::new(Date::new(2026, 10, 15), Time::new(21, 15, 7));
        let frozen = parse_local_clock("2024-02-29T23:59:59").unwrap();
        assert_eq!(
            resolve_render_clock(Some("2024-02-29T23:59:59"), Some("malformed"), fallback).unwrap(),
            frozen
        );
        assert_eq!(
            resolve_render_clock(None, Some("2024-02-29T23:59:59"), fallback).unwrap(),
            frozen
        );
        assert_eq!(
            resolve_render_clock(None, None, fallback).unwrap(),
            fallback
        );
        assert!(resolve_render_clock(Some(""), Some("2024-02-29T23:59:59"), fallback).is_err());
        let old: Input = serde_json::from_value(
            json!({ "report_path": "x.rpt", "output_path": "x.pdf", "export_format": "pdf",
            "columns": [], "rows": [], "query_params": {}, "report_variables": {} }),
        )
        .unwrap();
        assert!(old.as_of_local.is_none());
    }

    #[test]
    fn malformed_local_clocks_fail_instead_of_normalizing_or_converting() {
        for value in [
            "",
            "2026-02-29T00:00:00",
            "1900-02-29T00:00:00",
            "2026-04-31T00:00:00",
            "2026-00-01T00:00:00",
            "2026-13-01T00:00:00",
            "2026-01-00T00:00:00",
            "0000-01-01T00:00:00",
            "2026-10-16T24:00:00",
            "2026-10-16T00:60:00",
            "2026-10-16T00:00:60",
            "2026-10-16 00:00:00",
            "2026-10-16T00:00:00Z",
            "2026-10-16T00:00:00+03:00",
            "2026-10-16T00:00:00.000",
            "2026-1-16T00:00:00",
            "２０２６-10-16T00:00:00",
        ] {
            assert!(parse_local_clock(value).is_err(), "accepted {value:?}");
        }
        for value in [
            "0001-01-01T00:00:00",
            "2000-02-29T00:00:00",
            "9999-12-31T23:59:59",
        ] {
            assert!(parse_local_clock(value).is_ok(), "rejected {value:?}");
        }
    }

    #[test]
    fn printer_rounding_preserves_custom_sizes_and_pdf_coordinates() {
        let mut report = Report::default();
        report.print_options.paper_size = rpt_reader::model::PaperSize::PaperA4;
        report.print_options.content_width.0 = 11904;
        report.print_options.content_height.0 = 16836;
        normalize_driver_paper_rounding(&mut report);
        assert_eq!(report.print_options.content_width.0, 11906);
        assert_eq!(report.print_options.content_height.0, 16838);
        report.print_options.content_width.0 = 12000;
        normalize_driver_paper_rounding(&mut report);
        assert_eq!(report.print_options.content_width.0, 12000);
        let mut doc = PagedDocument::default();
        let mut page = Page::new(
            1,
            PageSize {
                width: rpt_reader::model::Twips(11906),
                height: rpt_reader::model::Twips(16838),
            },
        );
        page.origin.y.0 = 245;
        doc.pages.push(page);
        legacy_media_boxes(&mut doc);
        assert_eq!(doc.pages[0].size.height.0, 16820);
        assert_eq!(doc.pages[0].size.width.0, 11900);
        assert_eq!(
            doc.pages[0].size.height.0 - doc.pages[0].origin.y.0,
            16838 - 245
        );
    }
    #[test]
    fn custom_host_date_pattern_is_independent_of_language_tag() {
        let locale = locale_with_pattern("en-US", Some("dd/MM/yyyy")).unwrap();
        assert_eq!(
            locale.date_order,
            rpt_render::Locale::from_tag("en-GB").date_order
        );
        assert_eq!(locale.date_sep, '/');
        assert!(locale.short_date_leading_zero);
        assert!(locale_with_pattern("en-US", Some("unsupported")).is_err());
        assert!(locale_with_pattern("unsupported", None).is_err());
    }
    #[test]
    fn report_errors_cannot_be_returned_as_successful_certificates() {
        let mut document = PagedDocument::default();
        document.pages.push(Page::new(1, PageSize::default()));
        document.diagnostics.push(Diagnostic::error(
            DiagnosticKind::RecordSelection,
            "dropped row",
        ));
        assert!(accept_document(&document).is_err());
    }
    #[test]
    fn missing_images_are_a_failure_even_when_layout_has_no_diagnostic() {
        let mut document = PagedDocument::default();
        let mut page = Page::new(1, PageSize::default());
        page.ops.push(DrawOp::Image(rpt_pages::ImageOp {
            bounds: Default::default(),
            image_id: "MissingLogo".into(),
            fit: rpt_pages::ImageFit::Contain,
            source: None,
        }));
        document.pages.push(page);
        assert!(accept_document(&document)
            .unwrap_err()
            .contains("MissingLogo"));
    }
    #[test]
    fn text_nulls_and_numeric_boolean_schema_adaptation_preserve_sql_semantics() {
        let mut input: Input = serde_json::from_value(json!({ "report_path": "x.rpt", "output_path": "x.pdf", "export_format": "pdf",
            "columns": [{"name":"Name", "value_type":"String"}, {"name":"Enabled", "value_type":"Boolean"}],
            "rows": [[null, {"text":"1"}]], "query_params": {}, "report_variables": {} })).unwrap();
        let rows = prepare(&Report::default(), &input).unwrap().rows();
        assert_eq!(rows[0].get("Name"), Some(&Value::Null));
        assert_eq!(rows[0].get("Enabled"), Some(&Value::Bool(true)));
        input.rows[0][1] = Some(InputCell {
            text: Some("not a boolean".into()),
            bytes: None,
        });
        assert!(prepare(&Report::default(), &input).is_err());
    }
    #[test]
    fn chinese_unicode_null_and_binary_cells_survive_the_prepared_transport() {
        let input: Input = serde_json::from_value(json!({ "report_path": "x.rpt", "output_path": "x.pdf", "export_format": "pdf",
            "columns": [{"name":"StudentName", "value_type":"String"}, {"name":"Mark", "value_type":"Number"}, {"name":"Photo", "value_type":"Blob"}],
            "rows": [[{"text":"张伟 繁體中文"}, null, {"bytes":[0,255,1]}]], "query_params": {}, "report_variables": {} })).unwrap();
        let data = prepare(&Report::default(), &input).unwrap();
        let rows = data.rows();
        assert_eq!(
            rows[0].get("StudentName"),
            Some(&rpt_formula::eval::Value::Str("张伟 繁體中文".into()))
        );
        assert_eq!(rows[0].get("Mark"), Some(&rpt_formula::eval::Value::Null));
        assert_eq!(
            rows[0].get("Photo"),
            Some(&rpt_formula::eval::Value::Bytes(vec![0, 255, 1]))
        );
    }
}
