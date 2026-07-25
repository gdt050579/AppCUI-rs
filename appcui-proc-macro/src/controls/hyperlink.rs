use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("name", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("name", "name", ParamType::String),
    NamedParameter::new("text", "name", ParamType::String),
    NamedParameter::new("url", "url", ParamType::String),
    NamedParameter::new("link", "url", ParamType::String),
    NamedParameter::new("tooltip", "tooltip", ParamType::String),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("hyperlink", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("HyperLink::with_tooltip");
    cb.add_string_parameter("name", Some(""));
    cb.add_string_parameter("url", None);
    cb.add_string_parameter("tooltip", Some(""));
    cb.add_layout();
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
