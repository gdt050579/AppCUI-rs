use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;
static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("caption", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
    NamedParameter::new("select", "selected", ParamType::Bool),
    NamedParameter::new("selected", "selected", ParamType::Bool),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("radiobox", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("RadioBox::new");
    cb.add_string_parameter("caption", None);  
    cb.add_layout();
    cb.add_bool_parameter("selected", Some(false));
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
