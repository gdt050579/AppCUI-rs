use super::control_builder::ControlBuilder;
use crate::bar::{parse_bar_attr, parse_bar_draw_mode, validate_bar_number};
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ScrollBars", "DimBarsOnSelection", "ShowZeroLineOnYAxis"]);

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
    ("Percentage", "percentage"),
    ("Percentage", "percent"),
];

static XLABELS_MODES: &[(&str, &str)] = &[
    ("None", "none"),
    ("StackLabels", "stacklabels"),
    ("StackLabels", "stack-labels"),
    ("StackLabels", "labels"),
];

static DRAW_MODES: &[(&str, &str)] = &[
    ("Fill", "fill"),
    ("Rectangle", "rectangle"),
    ("Rectangle", "rect"),
    ("FilledRectangle", "filledrectangle"),
    ("FilledRectangle", "filledrect"),
    ("FilledRectangle", "filled-rectangle"),
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

static BAR_POSITIONAL: &[PositionalParameter] = &[PositionalParameter::new("value", ParamType::String)];
static BAR_NAMED: &[NamedParameter] = &[
    NamedParameter::new("value", "value", ParamType::String),
    NamedParameter::new("val", "value", ParamType::String),
    NamedParameter::new("v", "value", ParamType::String),
    NamedParameter::new("label", "label", ParamType::String),
    NamedParameter::new("text", "label", ParamType::String),
    NamedParameter::new("caption", "label", ParamType::String),
    NamedParameter::new("attr", "attr", ParamType::String),
    NamedParameter::new("attribute", "attr", ParamType::String),
    NamedParameter::new("charattr", "attr", ParamType::String),
    NamedParameter::new("draw-mode", "draw-mode", ParamType::String),
    NamedParameter::new("drawmode", "draw-mode", ParamType::String),
    NamedParameter::new("dm", "draw-mode", ParamType::String),
    NamedParameter::new("mode", "draw-mode", ParamType::String),
];

static STACK_POSITIONAL: &[PositionalParameter] = &[PositionalParameter::new("label", ParamType::String)];
static STACK_NAMED: &[NamedParameter] = &[
    NamedParameter::new("label", "label", ParamType::String),
    NamedParameter::new("text", "label", ParamType::String),
    NamedParameter::new("caption", "label", ParamType::String),
    NamedParameter::new("bars", "bars", ParamType::List),
    NamedParameter::new("values", "bars", ParamType::List),
    NamedParameter::new("data", "bars", ParamType::List),
    NamedParameter::new("width", "width", ParamType::Integer),
    NamedParameter::new("w", "width", ParamType::Integer),
    NamedParameter::new("thickness", "width", ParamType::Integer),
    NamedParameter::new("space", "space", ParamType::Integer),
    NamedParameter::new("spacing", "space", ParamType::Integer),
    NamedParameter::new("s", "space", ParamType::Integer),
];

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("type", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("type", "type", ParamType::String),
    NamedParameter::new("class", "type", ParamType::String),
    NamedParameter::new("stacks", "stacks", ParamType::List),
    NamedParameter::new("values", "stacks", ParamType::List),
    NamedParameter::new("data", "stacks", ParamType::List),
    // scale
    NamedParameter::new("stacksscale", "scale", ParamType::String),
    NamedParameter::new("stacks-scale", "scale", ParamType::String),
    NamedParameter::new("ss", "scale", ParamType::String),
    NamedParameter::new("scale", "scale", ParamType::String),
    // default bar width
    NamedParameter::new("default-bar-width", "default-bar-width", ParamType::Integer),
    NamedParameter::new("dbw", "default-bar-width", ParamType::Integer),
    NamedParameter::new("barwidth", "default-bar-width", ParamType::Integer),
    NamedParameter::new("bar-width", "default-bar-width", ParamType::Integer),
    NamedParameter::new("bw", "default-bar-width", ParamType::Integer),
    // default bar spacing
    NamedParameter::new("default-bar-spacing", "default-bar-spacing", ParamType::Integer),
    NamedParameter::new("dbs", "default-bar-spacing", ParamType::Integer),
    NamedParameter::new("bar-spacing", "default-bar-spacing", ParamType::Integer),
    NamedParameter::new("spacing", "default-bar-spacing", ParamType::Integer),
    NamedParameter::new("space", "default-bar-spacing", ParamType::Integer),
    NamedParameter::new("s", "default-bar-spacing", ParamType::Integer),
    // default bar draw mode
    NamedParameter::new("default-bar-draw-mode", "default-bar-draw-mode", ParamType::String),
    NamedParameter::new("dbdm", "default-bar-draw-mode", ParamType::String),
    NamedParameter::new("bar-draw-mode", "default-bar-draw-mode", ParamType::String),
    NamedParameter::new("draw-mode", "default-bar-draw-mode", ParamType::String),
    NamedParameter::new("dm", "default-bar-draw-mode", ParamType::String),
    // default bar attribute
    NamedParameter::new("default-bar-attr", "default-bar-attr", ParamType::String),
    NamedParameter::new("bar-attr", "default-bar-attr", ParamType::String),
    NamedParameter::new("barattr", "default-bar-attr", ParamType::String),
    NamedParameter::new("bar-color", "default-bar-attr", ParamType::String),
    NamedParameter::new("barcolor", "default-bar-attr", ParamType::String),
    // formats
    NamedParameter::new("numeric-format", "numeric-format", ParamType::String),
    NamedParameter::new("nf", "numeric-format", ParamType::String),
    NamedParameter::new("percentage-format", "percentage-format", ParamType::String),
    NamedParameter::new("pf", "percentage-format", ParamType::String),
    // x-axis labels
    NamedParameter::new("xlabels", "xlabels", ParamType::String),
    NamedParameter::new("x-labels", "xlabels", ParamType::String),
    NamedParameter::new("xl", "xlabels", ParamType::String),
    NamedParameter::new("xaxis", "xlabels", ParamType::String),
    NamedParameter::new("x-axis", "xlabels", ParamType::String),
    // y-axis
    NamedParameter::new("yaxis-width", "yaxis-width", ParamType::Integer),
    NamedParameter::new("yw", "yaxis-width", ParamType::Integer),
    NamedParameter::new("y-axis-width", "yaxis-width", ParamType::Integer),
    NamedParameter::new("yaxis-step", "yaxis-step", ParamType::Integer),
    NamedParameter::new("ystep", "yaxis-step", ParamType::Integer),
    NamedParameter::new("y-axis-step", "yaxis-step", ParamType::Integer),
    NamedParameter::new("step", "yaxis-step", ParamType::Integer),
    NamedParameter::new("yaxis-visible", "yaxis-visible", ParamType::Bool),
    NamedParameter::new("y-axis-visible", "yaxis-visible", ParamType::Bool),
    NamedParameter::new("yaxis", "yaxis-visible", ParamType::Bool),
    NamedParameter::new("yaxis-grid", "yaxis-grid", ParamType::Bool),
    NamedParameter::new("y-axis-grid", "yaxis-grid", ParamType::Bool),
    NamedParameter::new("grid", "yaxis-grid", ParamType::Bool),
    // extra
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("left-scroll-margin", "lsm", ParamType::Integer),
    NamedParameter::new("lsm", "lsm", ParamType::Integer),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("vstackedbarchart", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control_with_template("VStackedBarChart", "new", "type");
    cb.add_layout();
    cb.add_flags_parameter("flags", "vstackedbarchart::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_scroll_margin_setup("lsm", "tsm");
    cb.call_method_with_string_parameter_parser("set_stacks_scale", "scale", parse_scale);
    cb.call_method_with_value_parser("set_xaxis_label_mode", "xlabels", parse_xlabels);
    cb.call_method_with_integer_parameter_and_range("set_default_bar_width", "default-bar-width", 1, 100);
    cb.call_method_with_integer_parameter_and_range("set_default_bar_spacing", "default-bar-spacing", 0, 100);
    cb.call_method_with_integer_parameter_and_range("set_yaxis_width", "yaxis-width", 1, 32);
    cb.call_method_with_integer_parameter_and_range("set_yaxis_step", "yaxis-step", 1, 255);
    cb.call_method_with_string_parameter_parser("set_default_bar_drawmode", "default-bar-draw-mode", |repr| {
        parse_stack_draw_mode(repr, "default-bar-draw-mode")
    });
    if let Some(visible) = cb.get_bool("yaxis-visible") {
        cb.add_line(&format!("control.set_yaxis_visible({visible});"));
    }
    if let Some(grid) = cb.get_bool("yaxis-grid") {
        cb.add_line(&format!("control.set_yaxis_show_grid({grid});"));
    }
    if cb.has_parameter("default-bar-attr") {
        let str_repr = String::from(cb.get_string_representation());
        let tmp = if let Some(d) = cb.get_dict("default-bar-attr") {
            crate::chars::builder::create_attr_from_dict(&str_repr, d)
        } else if let Some(v) = cb.get_value("default-bar-attr") {
            parse_bar_attr(v, "default-bar-attr")
        } else {
            panic!("Invalid default-bar-attr ! Expected a character attribute (e.g. 'red', 'red,blue' or '{{fore: red, back: blue}}')");
        };
        cb.add("control.set_default_bar_attr(");
        cb.add(&tmp);
        cb.add(");\n");
    }
    add_format(&mut cb, "numeric-format", "set_number_format");
    add_format(&mut cb, "percentage-format", "set_percentage_format");
    // stacks are added last, so nothing is recomputed before all the settings are applied
    if cb.has_parameter("stacks") {
        let param_list = cb.get_string_representation().to_string();
        let Some(list) = cb.get_list("stacks") else {
            panic!("Parameter 'stacks' in vstackedbarchart must be a list of stacks: stacks: [[1, 2], {{label: Q1, bars: [3, 4]}}] !");
        };
        let stacks = parse_stacks(list, &param_list);
        for stack in stacks {
            cb.add_line(&format!("control.add_stack({stack});"));
        }
    }
    cb.add_basecontrol_operations();
    cb.into()
}

fn add_format(cb: &mut ControlBuilder, key: &str, method: &str) {
    if cb.has_parameter(key) {
        let str_repr = String::from(cb.get_string_representation());
        let tmp = if let Some(d) = cb.get_dict(key) {
            crate::numericformat::builder::create_from_dict(&str_repr, d)
        } else if let Some(v) = cb.get_value(key) {
            let mut parsed = crate::parameter_parser::parse(v).unwrap_or_else(|e| {
                panic!("Invalid {key}: {v} !{e:?}");
            });
            crate::numericformat::builder::create_from_dict(v, &mut parsed)
        } else {
            panic!("Invalid {key} ! Expected a number format (e.g. 'dec', 'hex, prefix: 0x' or '{{dec, group: 3}}')");
        };
        cb.add(&format!("control.{method}("));
        cb.add(&tmp);
        cb.add(");\n");
    }
}

fn parse_scale(repr: &str) -> String {
    let repr = repr.trim();
    if let Some(fncall) = crate::fncall::FnCall::new(repr) {
        if let Some(name) = fncall.match_name(SCALE_MODES) {
            match name {
                "Fixed" | "FromZeroMinRange" => {
                    assert!(fncall.params_count() == 2, "Invalid stacks scale format - expecting 2 parameters (min,max) !");
                    assert!(fncall.is_param_number(0), "Invalid stacks scale format - expecting a valid number for min but got {} !", fncall.param(0).unwrap());
                    assert!(fncall.is_param_number(1), "Invalid stacks scale format - expecting a valid number for max but got {} !", fncall.param(1).unwrap());
                    let min = fncall.param(0).unwrap();
                    let max = fncall.param(1).unwrap();
                    return format!("vstackedbarchart::StackBarScale::{name} {{ min: {min}, max: {max} }}");
                }
                _ => {
                    assert!(fncall.params_count() == 0, "Invalid stacks scale format - {name} does not have any parameters !");
                    return format!("vstackedbarchart::StackBarScale::{name}");
                }
            }
        }
    }
    panic!("Invalid stacks scale: {} - expected one of: {}", repr, crate::utils::join_strings(SCALE_MODES));
}

fn parse_stack_draw_mode(repr: &str, key: &str) -> String {
    let repr = repr.trim();
    let Some(fncall) = crate::fncall::FnCall::new(repr) else {
        panic!("Invalid {key} - expecting a draw mode ({}) but got {repr} !", crate::utils::join_strings(DRAW_MODES));
    };
    if fncall.match_name(DRAW_MODES).is_none() {
        panic!("Invalid {key} - expecting a draw mode ({}) but got {repr} !", crate::utils::join_strings(DRAW_MODES));
    }
    // the generic bar parser produces `vstackedbarchart::BarDrawMode::<variant>(...)`; only Fill, Rectangle and FilledRectangle are allowed
    parse_bar_draw_mode(repr, key, "vstackedbarchart").replace("vstackedbarchart::BarDrawMode::", "vstackedbarchart::StackBarDrawMode::")
}

fn parse_xlabels(value: &mut Value) -> String {
    if let Some(list) = value.get_list() {
        return parse_xlabels_list(list);
    }
    let repr = value.get_string().trim();
    if let Some(mode) = crate::utils::find_string_in_array(XLABELS_MODES, repr) {
        return format!("vstackedbarchart::XAxisLabelMode::{mode}");
    }
    if let Some(fncall) = crate::fncall::FnCall::new(repr) {
        if fncall.name().eq_ignore_ascii_case("index") {
            assert!(fncall.params_count() == 1, "Invalid xlabels format - expecting 1 parameter (start) !");
            assert!(fncall.is_param_integer(0), "Invalid xlabels format - expecting a valid integer (i32) for start but got {} !", fncall.param(0).unwrap());
            return format!("vstackedbarchart::XAxisLabelMode::Index({})", fncall.param(0).unwrap());
        }
    }
    panic!(
        "Invalid xlabels: {} - expected one of: {} or Index(start)",
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

fn parse_xlabels_list(list: &mut Vec<Value>) -> String {
    // format should be [{start,end,label},{start,end,label},...]
    let mut spans = String::from("vstackedbarchart::XAxisLabelMode::Custom(&[");
    let mut first = true;
    let mut temp_s = String::with_capacity(16);
    for item in list.iter_mut() {
        temp_s.clear();
        temp_s.push_str(item.get_string());
        if let Some(d) = item.get_dict() {
            d.validate_positional_parameters(&temp_s, XLABEL_SPAN_POSITIONAL).unwrap();
            d.validate_named_parameters(&temp_s, XLABEL_SPAN_NAMED).unwrap();
            let start = parse_xlabel_u32(d, "start");
            let end = parse_xlabel_u32(d, "end");
            let label = match d.get("label") {
                Some(v) => v.get_string().to_string(),
                None => panic!("Invalid xlabels format - missing 'label' ! Expected {{start,end,label}}"),
            };
            if !first {
                spans.push(',');
            }
            first = false;
            spans.push_str(&format!("vstackedbarchart::BarSpan::new({start},{end},\"{label}\")"));
        } else {
            panic!("An x-axis label span must be described between brackets: {{ and }}. For example: `{{0,3,'Q1'}}` !");
        }
    }
    spans.push_str("])");
    spans
}

fn parse_u8(repr: &str, key: &str) -> u8 {
    repr.parse::<u8>()
        .unwrap_or_else(|_| panic!("Invalid stacks format - expecting a number between 0 and 255 for '{key}' but got {repr} !"))
}

fn parse_bar_from_dict(dict: &mut NamedParamsMap, param_list: &str) -> String {
    dict.validate_positional_parameters(param_list, BAR_POSITIONAL).unwrap();
    dict.validate_named_parameters(param_list, BAR_NAMED).unwrap();
    let value = match dict.get("value") {
        Some(v) => {
            let s = v.get_string();
            validate_bar_number(s);
            s.to_string()
        }
        None => panic!("Invalid bar format - missing 'value' ! Expected {{value, label: Name, attr: {{...}}, draw-mode: Fill}}"),
    };
    let label = dict.get("label").map(|v| v.get_string().to_string());
    let draw_mode = dict.get("draw-mode").map(|v| parse_stack_draw_mode(v.get_string(), "draw-mode"));
    let attr = match dict.get_mut("attr") {
        Some(v) if v.is_dict() => {
            let inner = v.get_dict().expect("attr was checked to be a dictionary");
            Some(crate::chars::builder::create_attr_from_dict(param_list, inner))
        }
        Some(v) => Some(parse_bar_attr(v.get_string(), "attr")),
        None => None,
    };
    let mut res = format!("vstackedbarchart::StackBarBuilder::new({value})");
    if let Some(l) = label {
        res.push_str(&format!(".label(\"{l}\")"));
    }
    if let Some(a) = attr {
        res.push_str(".attr(");
        res.push_str(&a);
        res.push(')');
    }
    if let Some(dm) = draw_mode {
        res.push_str(".draw_mode(");
        res.push_str(&dm);
        res.push(')');
    }
    res.push_str(".build()");
    res
}

fn parse_bars(list: &mut [Value]) -> String {
    let mut items = Vec::with_capacity(list.len());
    let mut temp_s = String::with_capacity(16);
    for item in list.iter_mut() {
        temp_s.clear();
        temp_s.push_str(item.get_string());
        if let Some(d) = item.get_dict() {
            items.push(parse_bar_from_dict(d, &temp_s));
        } else if item.is_list() {
            panic!("Invalid bars format - a bar must be a number or a dictionary {{value, label, attr, draw-mode}} !");
        } else {
            let s = item.get_string();
            validate_bar_number(s);
            items.push(format!("vstackedbarchart::StackBarBuilder::new({s}).build()"));
        }
    }
    format!("[{}]", items.join(","))
}

fn parse_stacks(list: &mut [Value], _param_list: &str) -> Vec<String> {
    let mut stacks = Vec::with_capacity(list.len());
    let mut temp_s = String::with_capacity(16);
    for item in list.iter_mut() {
        temp_s.clear();
        temp_s.push_str(item.get_string());
        if let Some(bars) = item.get_list() {
            stacks.push(format!("vstackedbarchart::BarsStackBuilder::new().bars({}).build()", parse_bars(bars)));
        } else if let Some(d) = item.get_dict() {
            d.validate_positional_parameters(&temp_s, STACK_POSITIONAL).unwrap();
            d.validate_named_parameters(&temp_s, STACK_NAMED).unwrap();
            let label = d.get("label").map(|v| v.get_string().to_string());
            let width = d.get("width").map(|v| parse_u8(v.get_string(), "width"));
            let space = d.get("space").map(|v| parse_u8(v.get_string(), "space"));
            let mut res = String::from("vstackedbarchart::BarsStackBuilder::new()");
            if let Some(l) = label {
                res.push_str(&format!(".label(\"{l}\")"));
            }
            if let Some(w) = width {
                res.push_str(&format!(".thickness({w})"));
            }
            if let Some(s) = space {
                res.push_str(&format!(".spacing({s})"));
            }
            if let Some(v) = d.get_mut("bars") {
                let Some(bars) = v.get_list() else {
                    panic!("Invalid stacks format - 'bars' must be a list of bars: bars: [1, 2, {{3, label: x}}] !");
                };
                res.push_str(&format!(".bars({})", parse_bars(bars)));
            }
            res.push_str(".build()");
            stacks.push(res);
        } else {
            panic!("Invalid stacks format - a stack must be a list of bars ([1, 2, 3]) or a dictionary {{label: Q1, bars: [1, 2, 3]}} !");
        }
    }
    stacks
}
