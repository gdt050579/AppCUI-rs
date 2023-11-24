use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;


static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("caption", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("name", "caption", ParamType::String),
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
];


pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("label", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS);
    cb.init_control("Label::new");
    cb.add_strng_parameter("caption");  
    cb.add_layout();
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
