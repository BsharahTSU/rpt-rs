//! Declared report parameter binding with current/default values and legacy aliases.
use rpt_data::{normalize_param_name, Parameters};
use rpt_formula::eval::{Date, Time, Value};
use rpt_reader::model::{
    ParameterField, ParameterValue, ParameterValueKind as Vk, RangeBoundType, Report,
};
use serde_json::Value as Json;
use std::collections::BTreeMap;

pub(crate) fn aliases(name: &str) -> Vec<String> {
    let full = name.to_lowercase();
    let short = full
        .strip_prefix("pm-")
        .unwrap_or(&full)
        .rsplit('.')
        .next()
        .unwrap_or(&full)
        .trim_start_matches('@')
        .to_string();
    let base = short
        .trim_end_matches(|c: char| c.is_ascii_digit())
        .to_string();
    vec![full.trim_start_matches('@').to_string(), short, base]
}

pub(crate) fn bind(
    report: &Report,
    query: &BTreeMap<String, Json>,
    display: &BTreeMap<String, Json>,
) -> Result<Parameters, String> {
    let mut supplied = BTreeMap::new();
    for (key, value) in query.iter().chain(display) {
        supplied.insert(key.trim_start_matches('@').to_lowercase(), value);
    }
    let mut parameters = Parameters::new();
    for (field, definition) in report.data_definition.parameter_fields() {
        let supplied_value = aliases(&field.name)
            .iter()
            .find_map(|key| supplied.get(key).copied());
        let value = if let Some(raw) = supplied_value {
            if raw.is_null() {
                Value::Null
            } else {
                if raw.is_array() || raw.is_object() {
                    return Err(format!("Report parameter {} must be scalar", field.name));
                }
                let text = raw
                    .as_str()
                    .map(str::to_string)
                    .unwrap_or_else(|| raw.to_string());
                let value = coerce(definition.value_kind, &text)
                    .map_err(|_| format!("Report parameter {} has an invalid type", field.name))?;
                if definition.allow_multiple_values {
                    Value::Array(vec![value])
                } else {
                    value
                }
            }
        } else if let Some(value) = stored_default(definition) {
            value
        } else if definition.optional_prompt || field.name.to_lowercase().starts_with("pm-") {
            Value::Null
        } else {
            return Err(format!("Missing required report parameter: {}", field.name));
        };
        parameters.insert(normalize_param_name(&field.name), value);
    }
    Ok(parameters)
}

fn stored_default(pf: &ParameterField) -> Option<Value> {
    let kind = pf.value_kind;
    // The current value (last-used) wins over the default pick list, matching the SDK's effective
    // -value resolution; fall back to the declared defaults.
    let source: &[ParameterValue] = if pf.has_current_value && !pf.current_values.is_empty() {
        &pf.current_values
    } else {
        &pf.default_values
    };
    if source.is_empty() {
        return None;
    }
    if pf.allow_multiple_values {
        let vals: Vec<Value> = source
            .iter()
            .filter_map(|pv| default_one(kind, pv))
            .collect();
        (!vals.is_empty()).then_some(Value::Array(vals))
    } else {
        source.first().and_then(|pv| default_one(kind, pv))
    }
}

/// Coerce one stored [`ParameterValue`] to a [`Value`] of the declared kind: a discrete value maps
/// through [`coerce`]; a range maps to a [`Value::Range`] with each end's inclusivity from its bound
/// type. A *bounded* end that fails to coerce makes the whole range unusable.
fn default_one(kind: Vk, pv: &ParameterValue) -> Option<Value> {
    match &pv.range {
        None => coerce(kind, &pv.value).ok(),
        Some(r) => Some(Value::Range {
            lo: Box::new(range_end(kind, &pv.value, r.lower_bound)?),
            hi: Box::new(range_end(kind, &r.end_value, r.upper_bound)?),
            lo_incl: matches!(r.lower_bound, RangeBoundType::BoundInclusive),
            hi_incl: matches!(r.upper_bound, RangeBoundType::BoundInclusive),
        }),
    }
}

/// One end of a stored range parameter value. `NoBound` is an **open** end: it stores no value, so
/// it yields [`Value::Null`], which the evaluator reads as unbounded. Coercing the empty string it
/// stores would fail for every non-string kind and discard the whole range — binding the parameter
/// to `Null`, against which the selection formula keeps no record at all.
fn range_end(kind: Vk, s: &str, bound: RangeBoundType) -> Option<Value> {
    match bound {
        RangeBoundType::NoBound => Some(Value::Null),
        _ => coerce(kind, s).ok(),
    }
}

fn coerce(kind: Vk, s: &str) -> Result<Value, String> {
    let parse_num = |s: &str| {
        s.trim()
            .parse::<f64>()
            .map_err(|_| format!("{s:?} is not a number"))
    };
    match kind {
        Vk::NumberParameter => Ok(Value::Number(parse_num(s)?)),
        Vk::CurrencyParameter => Ok(Value::Currency(parse_num(s)?)),
        Vk::BooleanParameter => match s.trim().to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" | "y" => Ok(Value::Bool(true)),
            "false" | "0" | "no" | "n" => Ok(Value::Bool(false)),
            _ => Err(format!("{s:?} is not a boolean (true/false/1/0/yes/no)")),
        },
        Vk::DateParameter => parse_date(s).map(Value::Date),
        Vk::TimeParameter => parse_time(s).map(Value::Time),
        Vk::DateTimeParameter => {
            let (d, t) = s
                .split_once(['T', ' '])
                .ok_or_else(|| format!("{s:?} is not a datetime (YYYY-MM-DDTHH:MM:SS)"))?;
            Ok(Value::DateTime(parse_date(d)?, parse_time(t)?))
        }
        // StringParameter and any Other kind: pass through verbatim.
        _ => Ok(Value::Str(s.to_string())),
    }
}

/// Parse an ISO `YYYY-MM-DD` date.
fn parse_date(s: &str) -> Result<Date, String> {
    let p: Vec<&str> = s.trim().split('-').collect();
    match p.as_slice() {
        [y, m, d] => Ok(Date::new(
            y.parse().map_err(|_| bad_date(s))?,
            m.parse().map_err(|_| bad_date(s))?,
            d.parse().map_err(|_| bad_date(s))?,
        )),
        _ => Err(bad_date(s)),
    }
}

/// Parse an ISO `HH:MM[:SS]` time.
fn parse_time(s: &str) -> Result<Time, String> {
    let p: Vec<&str> = s.trim().split(':').collect();
    let get = |i: usize| p.get(i).unwrap_or(&"0").parse().map_err(|_| bad_time(s));
    match p.len() {
        2 | 3 => Ok(Time::new(get(0)?, get(1)?, get(2)?)),
        _ => Err(bad_time(s)),
    }
}

fn bad_date(s: &str) -> String {
    format!("{s:?} is not a date (expected YYYY-MM-DD)")
}

fn bad_time(s: &str) -> String {
    format!("{s:?} is not a time (expected HH:MM:SS)")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_aliases_keep_exact_then_qualified_then_numberless_names() {
        assert_eq!(
            aliases("Pm-proc.@StudentID1"),
            vec!["pm-proc.@studentid1", "studentid1", "studentid"]
        );
    }
}
