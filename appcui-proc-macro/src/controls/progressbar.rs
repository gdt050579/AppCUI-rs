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
    cb.call_method_with_integer_parameter_and_range("reset", "count", 0, i32::MAX);
    cb.call_method_with_integer_parameter_and_range("update_progress", "value", 0, i32::MAX);
    cb.call_method_with_string_parameter("update_text", "text");
    if cb.has_parameter("pause") {
        let should_paused = cb.get_bool("pause").unwrap_or(false);
        if should_paused {
            cb.add_line("control.pause();");
        }
    }
    cb.add_basecontrol_operations();
    cb.into()
}
