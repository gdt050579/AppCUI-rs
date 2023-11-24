use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static mut BUTTON_FLAGS: FlagsSignature = FlagsSignature::new(&["Flat"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("caption", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("name", "caption", ParamType::String),
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("button", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS);
    cb.init_control("Button::new");
    cb.add_strng_parameter("caption");  
    cb.add_layout();
    cb.add_flags("flags", "button::Flags", unsafe { &mut BUTTON_FLAGS });
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}

