use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&[
    "ScrollBars",
    "DimBarsOnSelection",
]);

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

static VALUE_BAR_POSITIONAL: &[PositionalParameter] = &[PositionalParameter::new("value", ParamType::String)];
static VALUE_BAR_NAMED: &[NamedParameter] = &[
    NamedParameter::new("value", "value", ParamType::String),
    NamedParameter::new("val", "value", ParamType::String),
    NamedParameter::new("v", "value", ParamType::String),
    NamedParameter::new("width", "width", ParamType::Integer),
    NamedParameter::new("w", "width", ParamType::Integer),
    NamedParameter::new("thickness", "width", ParamType::Integer),
    NamedParameter::new("space", "space", ParamType::Integer),
    NamedParameter::new("spacing", "space", ParamType::Integer),
    NamedParameter::new("s", "space", ParamType::Integer),
    NamedParameter::new("attr", "attr", ParamType::String),
    NamedParameter::new("attribute", "attr", ParamType::String),
    NamedParameter::new("charattr", "attr", ParamType::String),
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
    if let Some(repr) = cb.get_value("barscale") {
        let tmp = parse_barscale(repr);
        cb.add("control.set_bars_scale(vbarchart::BarScale::");
        cb.add(&tmp);
        cb.add(");\n");
    }
    if let Some(v) = cb.get_list("xlabels") {
        let tmp = parse_xlabels_list(v);
        cb.add("control.set_xaxis_label_mode(vbarchart::XAxisLabelMode::Custom(");
        cb.add(&tmp);
        cb.add("));\n");        
    } else {
        if let Some(repr) = cb.get_value("xlabels") {
            let tmp = parse_xlabels(repr);
            cb.add("control.set_xaxis_label_mode(vbarchart::XAxisLabelMode::");
            cb.add(&tmp);
            cb.add(");\n");
        }
    }
    if let Some(v) = cb.get_list("values") {
        let tmp = parse_values_list(v);
        cb.add("let values = ");
        cb.add(&tmp);
        cb.add(";\n");
        cb.add("control.add_bars(values);\n");        
    } 
    if let Some(v) = cb.get_value("default-bar-width") {
        let tmp = parse_bar_u8(v, "default-bar-width");
        cb.add("control.set_default_bar_width(");
        cb.add(format!("{tmp}").as_str());
        cb.add(");\n");
    }
    if let Some(v) = cb.get_value("default-bar-spacing") {
        let tmp = parse_bar_u8(v, "default-bar-spacing");
        cb.add("control.set_default_bar_spacing(");
        cb.add(format!("{tmp}").as_str());
        cb.add(");\n");
    }
    if let Some(v) = cb.get_value("yaxis-width") {
        let tmp = parse_bar_u8(v, "yaxis-width");
        cb.add("control.set_yaxis_width(");
        cb.add(format!("{tmp}").as_str());
        cb.add(");\n");
    }
    if let Some(v) = cb.get_value("yaxis-step") {
        let tmp = parse_bar_u8(v, "yaxis-step");
        cb.add("control.set_yaxis_step(");
        cb.add(format!("{tmp}").as_str());
        cb.add(");\n");
    }
    cb.add_basecontrol_operations();
    cb.into()
}

fn parse_barscale(repr: &str) -> String {
    let repr = repr.trim();
    if let Some(mode) = crate::utils::find_string_in_array(BARSCALE_MODES, repr) {
        return String::from(mode);
    }
    // check to see if the repr is Fixed(min,max), allowing white spaces (between fixed)
    if let Some(params) = crate::utils::parse_function_and_parameters(repr, "Fixed") {
        if params.len() != 2 {
            panic!("Invalid bar scale format - expecting Fixed(min,max) !");
        }
        // check if first paramter is a valid f64
        // if error throw a panic
        let _ = params[0]
            .parse::<f64>()
            .expect(&format!("Invalid bar scale format - expecting a valid number but got {} !", params[0]));
        let _ = params[1]
            .parse::<f64>()
            .expect(&format!("Invalid bar scale format - expecting a valid number but got {} !", params[1]));
        return format!("Fixed({},{})", params[0], params[1]);
    }
    // check to see if the repr is FromZeroMinRange(min,max), allowing white spaces (between min and max)
    if let Some(params) = crate::utils::parse_function_and_parameters(repr, "FromZeroMinRange") {
        if params.len() != 2 {
            panic!("Invalid bar scale format - expecting FromZeroMinRange(min,max) !");
        }
        // check if first paramter is a valid f64
        // if error throw a panic
        let _ = params[0]
            .parse::<f64>()
            .expect(&format!("Invalid bar scale format - expecting a valid number but got {} !", params[0]));
        let _ = params[1]
            .parse::<f64>()
            .expect(&format!("Invalid bar scale format - expecting a valid number but got {} !", params[1]));
        return format!("FromZeroMinRange({},{})", params[0], params[1]);
    }

    panic!(
        "Invalid bar scale: {} - expected one of: {} or Fixed(min,max) or FromZeroMinRange(min,max)",
        repr,
        crate::utils::join_strings(BARSCALE_MODES)
    );
}

fn parse_xlabels(repr: &str) -> String {
    let repr = repr.trim();
    if let Some(mode) = crate::utils::find_string_in_array(XLABELS_MODES, repr) {
        return String::from(mode);
    }
    // check to see if the repr is Index(start), allowing white spaces (between start)
    if let Some(params) = crate::utils::parse_function_and_parameters(repr, "Index") {
        if params.len() != 1 {
            panic!("Invalid xlabels format - expecting Index(start) !");
        }
        let _ = params[0].parse::<i32>().expect(&format!(
            "Invalid xlabels format - expecting a valid integer (i32) but got {} !",
            params[0]
        ));
        return format!("Index({})", params[0]);
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
    let mut spans = String::from("&[");
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
    spans.push(']');
    spans
}

fn validate_bar_number(repr: &str) {
    if repr.parse::<f64>().is_err() {
        panic!("Invalid values format - expecting a valid number but got {repr} !");
    }
}

fn parse_bar_u8(repr: &str, key: &str) -> u8 {
    repr.parse::<u8>().unwrap_or_else(|_| {
        panic!("Invalid values format - expecting a number between 0 and 255 for '{key}' but got {repr} !");
    })
}

fn parse_bar_from_dict(dict: &mut NamedParamsMap, param_list: &str) -> String {
    dict.validate_positional_parameters(param_list, VALUE_BAR_POSITIONAL).unwrap();
    dict.validate_named_parameters(param_list, VALUE_BAR_NAMED).unwrap();
    let value = match dict.get("value") {
        Some(v) => {
            let s = v.get_string();
            validate_bar_number(s);
            s.to_string()
        }
        None => panic!("Invalid values format - missing 'value' ! Expected {{value,width: 10, space: 4, attr: {{...}}}}"),
    };
    let width = dict.get("width").map(|v| parse_bar_u8(v.get_string(), "width"));
    let space = dict.get("space").map(|v| parse_bar_u8(v.get_string(), "space"));
    let label = dict.get("label").map(|v| v.get_string().to_string());
    let attr = if dict.contains("attr") {
        if let Some(attr_val) = dict.get_mut("attr") {
            if let Some(attr_dict) = attr_val.get_dict() {
                Some(crate::chars::builder::create_attr_from_dict(param_list, attr_dict))
            } else {
                Some(format!("charattr!(\"{}\")", attr_val.get_string()))
            }
        } else {
            None
        }
    } else {
        None
    };
    let mut res = format!("vbarchart::BarBuilder::new({value})");
    if let Some(w) = width {
        res.push_str(&format!(".thickness({w})"));
    }
    if let Some(s) = space {
        res.push_str(&format!(".spacing({s})"));
    }
    if let Some(l) = label {
        res.push_str(&format!(".label(\"{l}\")"));
    }
    if let Some(a) = attr {
        res.push_str(".attr(");
        res.push_str(&a);
        res.push(')');
    }
    res.push_str(".build()");
    res
}

fn parse_values_list(list: &mut Vec<Value>) -> String {
    // format should be either [value,value,value,...] where each value is a valid number
    // or [{value,width: 10, space: 4, attr: {}},{value,width: 10, space: 4, attr: {}},...]
    // where attr is a charattr!
    let mut items = Vec::with_capacity(list.len());
    let mut has_dict = false;
    let mut temp_s = String::with_capacity(16);
    for item in list.iter_mut() {
        temp_s.clear();
        temp_s.push_str(item.get_string());
        if let Some(d) = item.get_dict() {
            has_dict = true;
            items.push(parse_bar_from_dict(d, &temp_s));
        } else if item.is_list() {
            panic!("Invalid values format - a value must be a number or a dictionary {{value,width: 10, space: 4, attr: {{...}}}} !");
        } else {
            let s = item.get_string();
            validate_bar_number(s);
            items.push(s.to_string());
        }
    }
    if has_dict {
        for item in items.iter_mut() {
            if !item.starts_with("vbarchart::BarBuilder") {
                *item = format!("vbarchart::BarBuilder::new({item}).build()");
            }
        }
        format!("[{}]", items.join(","))
    } else {
        format!("&[{}]", items.join(","))
    }
}
/*
bar scale
number format


class: ,values = [1,2,3,4]
sau
class: ,value = [{1,width: 10, space: 4, attr: {}}]

*/
