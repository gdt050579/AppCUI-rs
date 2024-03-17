use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("pass", "pass", ParamType::String),
    NamedParameter::new("password", "pass", ParamType::String),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("password", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("Password::new");
    cb.add_layout();
    cb.finish_control_initialization();
    if cb.has_parameter("pass") {
        cb.add_command("\n\tcontrol.set_password(");
        cb.add_string_parameter("pass", None);
        cb.add_line(");\n");        
    }
    cb.add_basecontrol_operations();
    cb.into()
}
