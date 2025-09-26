use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["Seconds", "AMPM"]);
static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("time", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("time", "time", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("timepicker", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("TimePicker::new");
    cb.add_string_parameter("time", Some(""));
    cb.add_layout();
    cb.add_flags_parameter("flags", "timepicker::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
