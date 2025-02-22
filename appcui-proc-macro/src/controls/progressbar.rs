use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["HidePercentage"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("c", "count", ParamType::Integer),
    NamedParameter::new("count", "count", ParamType::Integer),
    NamedParameter::new("total", "count", ParamType::Integer),
    NamedParameter::new("value", "value", ParamType::Integer),
    NamedParameter::new("progress", "value", ParamType::Integer),
    NamedParameter::new("v", "value", ParamType::Integer),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("text", "text", ParamType::String),
    NamedParameter::new("caption", "text", ParamType::String),
    NamedParameter::new("paused", "pause", ParamType::Bool),
    NamedParameter::new("pause", "pause", ParamType::Bool),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("progressbar", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("ProgressBar::new");
    cb.add("0");
    cb.add_layout();
    cb.add_flags_parameter("flags", "progressbar::Flags", &FLAGS);
    cb.finish_control_initialization();

    if cb.has_parameter("count") {
        let value = cb.get_i32("count").unwrap_or(-1);
        if value >= 0 {
            cb.add_line(format!("control.reset({});", value).as_str());
        } else {
            panic!("Invalid 'count' parameter (should be a positive number) for progress bar !");
        }
    }
    if cb.has_parameter("value") {
        let value = cb.get_i32("value").unwrap_or(-1);
        if value >= 0 {
            cb.add_line(format!("control.update_progress({});", value).as_str());
        } else {
            panic!("Invalid 'value' parameter (should be a positive number) for progress bar !");
        }
    }
    if cb.has_parameter("text") {
        let value = cb.get_value("text").unwrap_or("").to_string();
        cb.add_line(format!("control.update_text(\"{}\");", value).as_str());
    }
    if cb.has_parameter("pause") {
        let should_paused = cb.get_bool("pause").unwrap_or(false);
        if should_paused {
            cb.add_line("control.pause();");
        }
    }
    cb.add_basecontrol_operations();
    cb.into()
}
