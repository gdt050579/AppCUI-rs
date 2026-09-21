use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&[
    "ScrollBars",
    "SearchBar",
    "CheckBoxes",
    "ShowGroups",
    "SmallIcons",
    "LargeIcons",
    "CustomFilter",
    "NoSelection",
    "MergeBorders",
]);

static BARSCALE_MODES: &[(&'static str, &'static str)] =
    &[("FromZero", "fromzero"), ("FitData", "fitdata"), ("FromZero", "zero"), ("FitData", "fit")];

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
    // x-asix labels
    NamedParameter::new("xlabels", "xlabels", ParamType::String),
    NamedParameter::new("x-labels", "xlabels", ParamType::String),
    NamedParameter::new("xl", "xlabels", ParamType::String),

    // extra
    NamedParameter::new("viewmode", "view", ParamType::String),
    NamedParameter::new("vm", "view", ParamType::String),
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
    if cb.has_parameter("values") {
        let mut s = String::with_capacity(256);
        let mut temp_s = String::with_capacity(64);
        if let Some(list) = cb.get_list("values") {
            for item in list.iter_mut() {
                temp_s.clear();
                temp_s.push_str(item.get_string());
                if let Some(d) = item.get_dict() {
                    let res = crate::column::builder::create_from_dict(&temp_s, d);
                    s.push_str("control.add_column(");
                    s.push_str(&res);
                    s.push_str(");\n");
                } else {
                    panic!("A column must be descipted between brackets: {{ and }}. For example: `{{Name,10,Left}}` !");
                }
            }
        } else {
            panic!("Parameter `columns` in listview must contains a list a columns: columns=[{{...}},{{...}},{{...}}] !");
        }
        cb.add_line(&s);
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

/*
bar scale
number format


class: ,values = [1,2,3,4]
sau
class: ,value = [{1,width: 10, space: 4, attr: {}}]

*/
