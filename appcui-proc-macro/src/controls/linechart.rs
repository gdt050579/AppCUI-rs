use super::control_builder::ControlBuilder;
use crate::bar::{parse_bar_attr, parse_bar_list, parse_line_type_name, parse_point_kind};
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ScrollBars", "ShowZeroLineOnYAxis", "DimPointsOnSelection", "GroupHover"]);

static SCALE_MODES: &[(&str, &str)] = &[
    ("FromZero", "fromzero"),
    ("FromZero", "zero"),
    ("FitData", "fitdata"),
    ("FitData", "fit"),
    ("Fixed", "fixed"),
    ("Fixed", "fix"),
    ("FromZeroMinRange", "fromzero-min-range"),
    ("FromZeroMinRange", "zero-min-range"),
    ("FromZeroMinRange", "fromzerominrange"),
];

static XLABELS_MODES: &[(&str, &str)] = &[("None", "none"), ("Labels", "labels")];

static LINE_MODES: &[(&str, &str)] = &[
    ("None", "none"),
    ("Braille", "braille"),
    ("Line", "line"),
    ("Orthogonal", "orthogonal"),
    ("Orthogonal", "ortho"),
];

static ORTHOGONAL_DIRECTIONS: &[(&str, &str)] = &[
    ("HorizontalFirst", "horizontalfirst"),
    ("HorizontalFirst", "horizontal-first"),
    ("VerticalFirst", "verticalfirst"),
    ("VerticalFirst", "vertical-first"),
    ("HorizontalUntilMiddle", "horizontaluntilmiddle"),
    ("HorizontalUntilMiddle", "horizontal-until-middle"),
    ("VerticalUntilMiddle", "verticaluntilmiddle"),
    ("VerticalUntilMiddle", "vertical-until-middle"),
    ("Auto", "auto"),
];

static MARKERS: &[(&str, &str)] = &[
    ("None", "none"),
    ("Small", "small"),
    ("Small", "point"),
    ("Framed", "framed"),
    ("Framed", "framedpoint"),
    ("Framed", "framed-point"),
];

static XLABEL_SPAN_POSITIONAL: &[PositionalParameter] = &[
    PositionalParameter::new("start", ParamType::Integer),
    PositionalParameter::new("end", ParamType::Integer),
    PositionalParameter::new("label", ParamType::String),
];
static XLABEL_SPAN_NAMED: &[NamedParameter] = &[
    NamedParameter::new("start", "start", ParamType::Integer),
    NamedParameter::new("end", "end", ParamType::Integer),
    NamedParameter::new("count", "end", ParamType::Integer),
    NamedParameter::new("label", "label", ParamType::String),
    NamedParameter::new("text", "label", ParamType::String),
    NamedParameter::new("caption", "label", ParamType::String),
];

static SERIES_POSITIONAL: &[PositionalParameter] = &[PositionalParameter::new("label", ParamType::String)];
static SERIES_NAMED: &[NamedParameter] = &[
    NamedParameter::new("label", "label", ParamType::String),
    NamedParameter::new("name", "label", ParamType::String),
    NamedParameter::new("caption", "label", ParamType::String),
    NamedParameter::new("text", "label", ParamType::String),
    NamedParameter::new("values", "values", ParamType::List),
    NamedParameter::new("data", "values", ParamType::List),
    NamedParameter::new("points", "values", ParamType::List),
    NamedParameter::new("line", "line", ParamType::String),
    NamedParameter::new("line-mode", "line", ParamType::String),
    NamedParameter::new("linemode", "line", ParamType::String),
    NamedParameter::new("lm", "line", ParamType::String),
    NamedParameter::new("attr", "attr", ParamType::String),
    NamedParameter::new("attribute", "attr", ParamType::String),
    NamedParameter::new("charattr", "attr", ParamType::String),
    NamedParameter::new("color", "attr", ParamType::String),
];

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("type", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("type", "type", ParamType::String),
    NamedParameter::new("class", "type", ParamType::String),
    // data
    NamedParameter::new("series", "series", ParamType::List),
    NamedParameter::new("labels", "labels", ParamType::List),
    NamedParameter::new("categories", "labels", ParamType::List),
    NamedParameter::new("xaxis-labels", "labels", ParamType::List),
    // scale
    NamedParameter::new("scale", "scale", ParamType::String),
    NamedParameter::new("barscale", "scale", ParamType::String),
    // defaults
    NamedParameter::new("line", "line", ParamType::String),
    NamedParameter::new("default-line", "line", ParamType::String),
    NamedParameter::new("line-mode", "line", ParamType::String),
    NamedParameter::new("lm", "line", ParamType::String),
    NamedParameter::new("marker", "marker", ParamType::String),
    NamedParameter::new("default-marker", "marker", ParamType::String),
    NamedParameter::new("dmk", "marker", ParamType::String),
    NamedParameter::new("spacing", "spacing", ParamType::Integer),
    NamedParameter::new("default-spacing", "spacing", ParamType::Integer),
    NamedParameter::new("space", "spacing", ParamType::Integer),
    NamedParameter::new("s", "spacing", ParamType::Integer),
    // numeric format
    NamedParameter::new("numeric-format", "numeric-format", ParamType::String),
    NamedParameter::new("nf", "numeric-format", ParamType::String),
    // x-axis
    NamedParameter::new("xlabels", "xlabels", ParamType::String),
    NamedParameter::new("x-labels", "xlabels", ParamType::String),
    NamedParameter::new("xl", "xlabels", ParamType::String),
    NamedParameter::new("xaxis", "xlabels", ParamType::String),
    NamedParameter::new("x-axis", "xlabels", ParamType::String),
    // y-axis
    NamedParameter::new("yaxis", "yaxis", ParamType::Bool),
    NamedParameter::new("y-axis", "yaxis", ParamType::Bool),
    NamedParameter::new("yaxis-visible", "yaxis", ParamType::Bool),
    NamedParameter::new("grid", "grid", ParamType::Bool),
    NamedParameter::new("ygrid", "grid", ParamType::Bool),
    NamedParameter::new("yaxis-grid", "grid", ParamType::Bool),
    NamedParameter::new("yaxis-width", "yaxis-width", ParamType::Integer),
    NamedParameter::new("yw", "yaxis-width", ParamType::Integer),
    NamedParameter::new("y-axis-width", "yaxis-width", ParamType::Integer),
    NamedParameter::new("yaxis-step", "yaxis-step", ParamType::Integer),
    NamedParameter::new("ystep", "yaxis-step", ParamType::Integer),
    NamedParameter::new("y-axis-step", "yaxis-step", ParamType::Integer),
    NamedParameter::new("step", "yaxis-step", ParamType::Integer),
    // extra
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("left-scroll-margin", "lsm", ParamType::Integer),
    NamedParameter::new("lsm", "lsm", ParamType::Integer),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("linechart", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control_with_template("LineChart", "new", "type");
    cb.add_layout();
    cb.add_flags_parameter("flags", "linechart::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_scroll_margin_setup("lsm", "tsm");
    cb.call_method_with_string_parameter_parser("set_scale", "scale", parse_scale);
    cb.call_method_with_string_parameter_parser("set_default_line", "line", |repr| parse_line_mode(repr, "line"));
    cb.call_method_with_string_parameter_parser("set_default_marker", "marker", parse_marker);
    cb.call_method_with_integer_parameter_and_range("set_default_spacing", "spacing", 0, 100);
    cb.call_method_with_integer_parameter_and_range("set_yaxis_width", "yaxis-width", 0, 32);
    cb.call_method_with_integer_parameter_and_range("set_yaxis_step", "yaxis-step", 1, 255);
    if let Some(visible) = cb.get_bool("yaxis") {
        cb.add_line(&format!("control.set_yaxis_visible({visible});"));
    }
    if let Some(show_grid) = cb.get_bool("grid") {
        cb.add_line(&format!("control.set_yaxis_show_grid({show_grid});"));
    }
    if cb.has_parameter("numeric-format") {
        let str_repr = String::from(cb.get_string_representation());
        let tmp = if let Some(d) = cb.get_dict("numeric-format") {
            crate::numericformat::builder::create_from_dict(&str_repr, d)
        } else if let Some(v) = cb.get_value("numeric-format") {
            let mut parsed = crate::parameter_parser::parse(v).unwrap_or_else(|e| {
                panic!("Invalid numeric-format: {v} !{e:?}");
            });
            crate::numericformat::builder::create_from_dict(v, &mut parsed)
        } else {
            panic!("Invalid numeric-format ! Expected a number format (e.g. 'dec', 'hex, prefix: 0x' or '{{dec, group: 3}}')");
        };
        cb.add("control.set_number_format(");
        cb.add(&tmp);
        cb.add(");\n");
    }
    // names of the X positions; when no xlabels mode is given, they are also shown under the axis
    if let Some(list) = cb.get_list("labels") {
        let labels = parse_labels(list);
        cb.add_line(&format!("control.set_xaxis_labels({labels});"));
        if !cb.has_parameter("xlabels") {
            cb.add_line("control.set_xaxis_label_mode(linechart::XAxisLabelMode::Labels);");
        }
    }
    cb.call_method_with_value_parser("set_xaxis_label_mode", "xlabels", parse_xlabels);
    // the series are added last, so that everything above is set up before the data is laid out
    if let Some(list) = cb.get_list("series") {
        let series = parse_series_list(list);
        for s in series {
            cb.add_line(&format!("control.add_series({s});"));
        }
    }
    cb.add_basecontrol_operations();
    cb.into()
}

fn parse_scale(repr: &str) -> String {
    let repr = repr.trim();
    let expected = || format!("Invalid scale: {} - expected one of: {}", repr, crate::utils::join_strings(SCALE_MODES));
    let Some(fncall) = crate::fncall::FnCall::new(repr) else {
        panic!("{}", expected());
    };
    let Some(name) = fncall.match_name(SCALE_MODES) else {
        panic!("{}", expected());
    };
    match name {
        "Fixed" | "FromZeroMinRange" => {
            assert!(fncall.params_count() == 2, "Invalid scale format - expecting 2 parameters (min,max) !");
            assert!(fncall.is_param_number(0), "Invalid scale format - expecting a valid number for min but got {} !", fncall.param(0).unwrap());
            assert!(fncall.is_param_number(1), "Invalid scale format - expecting a valid number for max but got {} !", fncall.param(1).unwrap());
            let min = fncall.param(0).unwrap();
            let max = fncall.param(1).unwrap();
            format!("linechart::BarScale::{name} {{ min: {min}, max: {max} }}")
        }
        _ => {
            assert!(fncall.params_count() == 0, "Invalid scale format - {name} takes no parameters !");
            format!("linechart::BarScale::{name}")
        }
    }
}

fn parse_line_mode(repr: &str, key: &str) -> String {
    let repr = repr.trim();
    let expected = || {
        format!(
            "Invalid {key}: {repr} - expected None, Braille, Line(line type) or Orthogonal(line type, direction)"
        )
    };
    let Some(fncall) = crate::fncall::FnCall::new(repr) else {
        panic!("{}", expected());
    };
    let Some(name) = fncall.match_name(LINE_MODES) else {
        panic!("{}", expected());
    };
    match name {
        "None" | "Braille" => {
            assert!(fncall.params_count() == 0, "Invalid {key} - {name} takes no parameters !");
            format!("linechart::LineMode::{name}")
        }
        "Line" => {
            let line = match fncall.params_count() {
                0 => String::from("LineType::Single"),
                1 => parse_line_type_name(fncall.param(0).unwrap(), key),
                _ => panic!("Invalid {key} - expecting Line or Line(line type), for example Line(Double) !"),
            };
            format!("linechart::LineMode::Line({line})")
        }
        _ => {
            let line = match fncall.params_count() {
                0 => String::from("LineType::Single"),
                1 | 2 => parse_line_type_name(fncall.param(0).unwrap(), key),
                _ => panic!("Invalid {key} - expecting Orthogonal, Orthogonal(line type) or Orthogonal(line type, direction) !"),
            };
            let direction = match fncall.param(1) {
                None => "Auto",
                Some(d) => crate::utils::find_string_in_array(ORTHOGONAL_DIRECTIONS, d.trim()).unwrap_or_else(|| {
                    panic!(
                        "Invalid {key} - unknown orthogonal direction '{d}' ! Expected one of: {}",
                        crate::utils::join_strings(ORTHOGONAL_DIRECTIONS)
                    )
                }),
            };
            format!("linechart::LineMode::Orthogonal({line}, OrthogonalDirection::{direction})")
        }
    }
}

fn parse_marker(repr: &str) -> String {
    let repr = repr.trim();
    let expected = || format!("Invalid marker: {repr} - expected None, Small(kind) / Point(kind) or Framed(kind) / FramedPoint(kind)");
    let Some(fncall) = crate::fncall::FnCall::new(repr) else {
        panic!("{}", expected());
    };
    let Some(name) = fncall.match_name(MARKERS) else {
        panic!("{}", expected());
    };
    assert!(fncall.params_count() <= 1, "Invalid marker - {name} takes at most one parameter (the marker kind) !");
    match name {
        "None" => {
            assert!(fncall.params_count() == 0, "Invalid marker - None takes no parameters !");
            String::from("linechart::Marker::None")
        }
        "Small" => format!("linechart::Marker::Small({})", parse_point_kind(fncall.param(0), "marker", "linechart", false)),
        _ => format!("linechart::Marker::Framed({})", parse_point_kind(fncall.param(0), "marker", "linechart", true)),
    }
}

fn parse_labels(list: &mut [Value]) -> String {
    let labels: Vec<String> = list
        .iter_mut()
        .map(|item| {
            assert!(!item.is_list() && !item.is_dict(), "Invalid labels - every label must be a text, for example labels: [Jan, Feb, 'Mar 2026'] !");
            format!("{:?}", item.get_string())
        })
        .collect();
    format!("[{}]", labels.join(","))
}

fn parse_xlabels(value: &mut Value) -> String {
    if let Some(list) = value.get_list() {
        return parse_xlabels_list(list);
    }
    let repr = value.get_string().trim();
    if let Some(mode) = crate::utils::find_string_in_array(XLABELS_MODES, repr) {
        return format!("linechart::XAxisLabelMode::{mode}");
    }
    if let Some(fncall) = crate::fncall::FnCall::new(repr) {
        if fncall.name().eq_ignore_ascii_case("index") {
            assert!(fncall.params_count() == 1, "Invalid xlabels format - expecting 1 parameter (start) !");
            assert!(fncall.is_param_integer(0), "Invalid xlabels format - expecting a valid integer (i32) for start but got {} !", fncall.param(0).unwrap());
            return format!("linechart::XAxisLabelMode::Index({})", fncall.param(0).unwrap());
        }
    }
    panic!(
        "Invalid xlabels: {} - expected one of: {}, Index(start) or a list of spans",
        repr,
        crate::utils::join_strings(XLABELS_MODES)
    );
}

fn parse_xlabel_u32(dict: &NamedParamsMap, key: &str) -> u32 {
    let Some(v) = dict.get(key) else {
        panic!("Invalid xlabels format - missing '{key}' ! Expected {{start,end,label}}");
    };
    let s = v.get_string();
    match s.parse::<u32>() {
        Ok(n) => n,
        Err(_) => panic!("Invalid xlabels format - expecting a valid positive integer (u32) for '{key}' but got {s} !"),
    }
}

fn parse_xlabels_list(list: &mut [Value]) -> String {
    let mut spans = Vec::with_capacity(list.len());
    let mut temp_s = String::with_capacity(16);
    for item in list.iter_mut() {
        temp_s.clear();
        temp_s.push_str(item.get_string());
        let Some(d) = item.get_dict() else {
            panic!("An x-axis label span must be described between brackets: {{ and }}. For example: `{{0,3,'Q1'}}` !");
        };
        d.validate_positional_parameters(&temp_s, XLABEL_SPAN_POSITIONAL).unwrap();
        d.validate_named_parameters(&temp_s, XLABEL_SPAN_NAMED).unwrap();
        let start = parse_xlabel_u32(d, "start");
        let end = parse_xlabel_u32(d, "end");
        let Some(label) = d.get("label") else {
            panic!("Invalid xlabels format - missing 'label' ! Expected {{start,end,label}}");
        };
        spans.push(format!("linechart::BarSpan::new({start},{end},{:?})", label.get_string()));
    }
    format!("linechart::XAxisLabelMode::Custom(&[{}])", spans.join(","))
}

fn parse_series_list(list: &mut [Value]) -> Vec<String> {
    let mut result = Vec::with_capacity(list.len());
    let mut temp_s = String::with_capacity(32);
    for item in list.iter_mut() {
        temp_s.clear();
        temp_s.push_str(item.get_string());
        let Some(d) = item.get_dict() else {
            panic!("A series must be described between brackets: {{ and }}. For example: `{{CPU, values: [10, 20, 15], line: Braille, attr: red}}` !");
        };
        d.validate_positional_parameters(&temp_s, SERIES_POSITIONAL).unwrap();
        d.validate_named_parameters(&temp_s, SERIES_NAMED).unwrap();
        let label = d.get("label").map(|v| v.get_string().to_string()).unwrap_or_default();
        let mut s = format!("linechart::SeriesBuilder::new({label:?})");
        if let Some(line) = d.get("line") {
            s.push_str(&format!(".line({})", parse_line_mode(line.get_string(), "line")));
        }
        let attr = match d.get_mut("attr") {
            Some(v) if v.is_dict() => {
                let inner = v.get_dict().expect("attr was checked to be a dictionary");
                Some(crate::chars::builder::create_attr_from_dict(&temp_s, inner))
            }
            Some(v) => Some(parse_bar_attr(v.get_string(), "attr")),
            None => None,
        };
        if let Some(a) = attr {
            s.push_str(&format!(".attr({a})"));
        }
        if let Some(values) = d.get_mut("values") {
            let Some(values) = values.get_list() else {
                panic!("Invalid series values - expecting a list, for example values: [10, 20, {{15, label: peak}}] !");
            };
            s.push_str(&format!(".values({})", parse_bar_list(values, "linechart")));
        }
        s.push_str(".build()");
        result.push(s);
    }
    result
}
