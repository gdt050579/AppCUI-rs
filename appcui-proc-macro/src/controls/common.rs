use crate::parameter_parser::NamedParameter;
use crate::parameter_parser::NamedParamsMap;
use crate::parameter_parser::ParamType;
use crate::parameter_parser::Value;

pub(super) static CONTROL_NAMED_PARAMATERS: &[NamedParameter] = &[
    // generic characteristics
    NamedParameter::new("visible", "visible", ParamType::Bool),
    NamedParameter::new("enabled", "enabled", ParamType::Bool),
    NamedParameter::new("enable", "enabled", ParamType::Bool),
    // layout
    NamedParameter::new("x", "x", ParamType::Layout),
    NamedParameter::new("y", "y", ParamType::Layout),
    NamedParameter::new("left", "left", ParamType::Layout),
    NamedParameter::new("l", "left", ParamType::Layout),
    NamedParameter::new("right", "right", ParamType::Layout),
    NamedParameter::new("r", "right", ParamType::Layout),
    NamedParameter::new("top", "top", ParamType::Layout),
    NamedParameter::new("t", "top", ParamType::Layout),
    NamedParameter::new("bottom", "bottom", ParamType::Layout),
    NamedParameter::new("b", "bottom", ParamType::Layout),
    NamedParameter::new("width", "width", ParamType::Layout),
    NamedParameter::new("w", "width", ParamType::Layout),
    NamedParameter::new("height", "height", ParamType::Layout),
    NamedParameter::new("h", "height", ParamType::Layout),
    NamedParameter::new("align", "align", ParamType::Alignament),
    NamedParameter::new("a", "align", ParamType::Alignament),
    NamedParameter::new("alignament", "align", ParamType::Alignament),
    NamedParameter::new("dock", "dock", ParamType::Alignament),
    NamedParameter::new("d", "dock", ParamType::Alignament),
];

pub(super) fn add_string(s: &mut String, text: &str) {
    s.push('"');
    s.push_str(text);
    s.push('"');
}

fn get_bool_value(params: &mut NamedParamsMap, name: &str, value_if_not_found: bool) -> bool {
    if let Some(value) = params.get_mut(name) {
        if let Some(bvalue) = value.get_bool() {
            return bvalue;
        }
    }
    return value_if_not_found;
}
pub(super) fn add_basecontrol_operations(s: &mut String, name: &str, params: &mut NamedParamsMap) {
    if get_bool_value(params, "enabled", true) == false {
        s.push_str(name);
        s.push_str(".set_enabled(false);\n\t");
    }
    if get_bool_value(params, "visible", true) == false {
        s.push_str(name);
        s.push_str(".set_visible(false);\n\t");
    }
}

pub(super) fn add_flags(s: &mut String, flag_name: &str, values: &Vec<Value>) {
    if values.len() == 0 {
        s.push_str(flag_name);
        s.push_str("::None");
    }
}
