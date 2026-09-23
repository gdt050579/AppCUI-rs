use super::bar::{parse_bar_attr, parse_bar_draw_mode, parse_bar_list};
use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ScrollBars", "DimBarsOnSelection"]);

static BARSCALE_MODES: &[(&'static str, &'static str)] =
    &[("FromZero", "fromzero"), ("FitData", "fitdata"), ("FromZero", "zero"), ("FitData", "fit")];

static XLABELS_MODES: &[(&'static str, &'static str)] = &[("None", "none"), ("BarLabels", "barlabels")];

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

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("type", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("type", "type", ParamType::String),
    NamedParameter::new("class", "type", ParamType::String),
    NamedParameter::new("values", "values", ParamType::List),
    NamedParameter::new("data", "values", ParamType::List),
    // scale
    NamedParameter::new("barscale", "barscale", ParamType::String),
    NamedParameter::new("bs", "barscale", ParamType::String),
    NamedParameter::new("bar-scale", "barscale", ParamType::String),
    NamedParameter::new("scale", "barscale", ParamType::String),
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
    NamedParameter::new("bs", "default-bar-spacing", ParamType::Integer),
    NamedParameter::new("s", "default-bar-spacing", ParamType::Integer),
    // default bar draw mode
    NamedParameter::new("default-bar-draw-mode", "default-bar-draw-mode", ParamType::String),
    NamedParameter::new("dbdm", "default-bar-draw-mode", ParamType::String),
    NamedParameter::new("bar-draw-mode", "default-bar-draw-mode", ParamType::String),
    NamedParameter::new("draw-mode", "default-bar-draw-mode", ParamType::String),
    NamedParameter::new("dm", "default-bar-draw-mode", ParamType::String),
    // default bar draw mode attribute
    NamedParameter::new("default-bar-draw-mode-attr", "default-bar-draw-mode-attr", ParamType::String),
    NamedParameter::new("bar-attr", "default-bar-draw-mode-attr", ParamType::String),
    NamedParameter::new("barattr", "default-bar-draw-mode-attr", ParamType::String),
    NamedParameter::new("bar-color", "default-bar-draw-mode-attr", ParamType::String),
    NamedParameter::new("barcolor", "default-bar-draw-mode-attr", ParamType::String),
    // x-asix labels
    NamedParameter::new("xlabels", "xlabels", ParamType::String),
    NamedParameter::new("x-labels", "xlabels", ParamType::String),
    NamedParameter::new("xl", "xlabels", ParamType::String),
    NamedParameter::new("xaxis", "xlabels", ParamType::String),
    NamedParameter::new("x-axis", "xlabels", ParamType::String),
    // y-axis width
    NamedParameter::new("yaxis-width", "yaxis-width", ParamType::Integer),
    NamedParameter::new("yw", "yaxis-width", ParamType::Integer),
    NamedParameter::new("y-axis-width", "yaxis-width", ParamType::Integer),
    // y-axis step
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
    let mut cb = ControlBuilder::new("vbarchart", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control_with_template("VBarChart", "new", "type");
    cb.add_layout();
    cb.add_flags_parameter("flags", "vbarchart::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_scroll_margin_setup("lsm", "tsm");
    cb.call_method_with_string_parameter_parser("set_bars_scale", "barscale", parse_barscale);
    cb.call_method_with_value_parser("set_xaxis_label_mode", "xlabels", parse_xlabels);
    cb.call_method_with_integer_parameter_and_range("set_default_bar_width", "default-bar-width", 1, 100);
    cb.call_method_with_integer_parameter_and_range("set_default_bar_spacing", "default-bar-spacing", 1, 100);
    cb.call_method_with_integer_parameter_and_range("set_yaxis_width", "yaxis-width", 0, 32);
    cb.call_method_with_integer_parameter_and_range("set_yaxis_step", "yaxis-step", 1, 255);
    cb.call_method_with_string_parameter_parser("set_default_bar_drawmode", "default-bar-draw-mode", |repr| parse_bar_draw_mode(repr, "default-bar-draw-mode", "vbarchart"));
    if cb.has_parameter("default-bar-draw-mode-attr") {
        let str_repr = String::from(cb.get_string_representation());
        let tmp = if let Some(d) = cb.get_dict("default-bar-draw-mode-attr") {
            crate::chars::builder::create_attr_from_dict(&str_repr, d)
        } else if let Some(v) = cb.get_value("default-bar-draw-mode-attr") {
            parse_bar_attr(v, "default-bar-draw-mode-attr")
        } else {
            panic!("Invalid default-bar-draw-mode-attr ! Expected a character attribute (e.g. 'red', 'red,blue' or '{{fore: red, back: blue}}')");
        };
        cb.add("control.set_default_bar_attr(");
        cb.add(&tmp);
        cb.add(");\n");
    }
    // values sunt ultimele ca sa nu se calculeze nimic pana atunci
    cb.call_method_with_list_parameter_parser("add_bars", "values", |list| parse_bar_list(list, "vbarchart"));
    cb.add_basecontrol_operations();
    cb.into()
}

fn parse_barscale(repr: &str) -> String {
    let repr = repr.trim();
    if let Some(mode) = crate::utils::find_string_in_array(BARSCALE_MODES, repr) {
        return format!("vbarchart::BarScale::{mode}");
    }
    // check to see if the repr is Fixed(min,max), allowing white spaces (between fixed)
    if let Some(params) = crate::utils::parse_function_and_parameters(repr, "Fixed") {
        assert!(params.len() == 2, "Invalid bar scale format - expecting Fixed(min,max) !");
        assert!(crate::utils::is_number(&params[0]), "Invalid bar scale format - expecting a valid number but got {} !", params[0]);
        assert!(crate::utils::is_number(&params[1]), "Invalid bar scale format - expecting a valid number but got {} !", params[1]);
        return format!("vbarchart::BarScale::Fixed({},{})", params[0], params[1]);
    }
    // check to see if the repr is FromZeroMinRange(min,max), allowing white spaces (between min and max)
    if let Some(params) = crate::utils::parse_function_and_parameters(repr, "FromZeroMinRange") {
        assert!(params.len() == 2, "Invalid bar scale format - expecting FromZeroMinRange(min,max) !");
        assert!(crate::utils::is_number(&params[0]), "Invalid bar scale format - expecting a valid number but got {} !", params[0]);
        assert!(crate::utils::is_number(&params[1]), "Invalid bar scale format - expecting a valid number but got {} !", params[1]);
        return format!("vbarchart::BarScale::FromZeroMinRange({},{})", params[0], params[1]);
    }

    panic!(
        "Invalid bar scale: {} - expected one of: {} or Fixed(min,max) or FromZeroMinRange(min,max)",
        repr,
        crate::utils::join_strings(BARSCALE_MODES)
    );
}
fn parse_xlabels(value: &mut Value) -> String {
    if let Some(list) = value.get_list() {
        return parse_xlabels_list(list);
    }
    let repr = value.get_string().trim();
    if let Some(mode) = crate::utils::find_string_in_array(XLABELS_MODES, repr) {
        return format!("vbarchart::XAxisLabelMode::{mode}");
    }
    // check to see if the repr is Index(start), allowing white spaces (between start)
    if let Some(params) = crate::utils::parse_function_and_parameters(repr, "Index") {
        assert!(params.len() == 1, "Invalid xlabels format - expecting Index(start) !");
        assert!(crate::utils::is_integer(&params[0]), "Invalid xlabels format - expecting a valid integer (i32) but got {} !", params[0]);
        return format!("vbarchart::XAxisLabelMode::Index({})", params[0]);
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
    let mut spans = String::from("vbarchart::XAxisLabelMode::Custom(&[");
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
            spans.push_str(&format!("vbarchart::BarSpan::new({start},{end},\"{label}\")"));
        } else {
            panic!("An x-axis label span must be described between brackets: {{ and }}. For example: `{{0,3,'Q1'}}` !");
        }
    }
    spans.push_str("])");
    spans
}



