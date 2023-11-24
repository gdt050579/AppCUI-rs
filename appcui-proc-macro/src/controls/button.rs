use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static mut BUTTON_TYPE: FlagsSignature = FlagsSignature::new(&["Normal", "Flat"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("caption", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("name", "caption", ParamType::String),
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
    NamedParameter::new("type", "type", ParamType::String),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("button", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS);
    cb.init_control("Button::new");
    cb.add_strng_parameter("caption");
    cb.add_layout();
    cb.add_enum_parameter("type", "button::Type", unsafe { &mut BUTTON_TYPE }, "Normal");
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
