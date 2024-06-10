use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("date", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("date", "date", ParamType::String),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("datepicker", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("DatePicker::new");
    cb.add_string_parameter("date", Some(""));
    cb.add_layout();
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
