use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static TYPES: FlagsSignature = FlagsSignature::new(&["Border", "Window", "Page", "TopBar", "Raised", "Sunken"]);
static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("caption", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("title", "caption", ParamType::String),
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
    NamedParameter::new("type", "type", ParamType::String),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("panel", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    if cb.has_parameter("type") {
        cb.init_control("Panel::with_type");
    } else {
        cb.init_control("Panel::new");
    }    
    cb.add_string_parameter("caption", Some(""));
    cb.add_layout();
    if cb.has_parameter("type") {
        cb.add_enum_parameter("type", "panel::Type", &TYPES, Some("Border"));
    }
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
