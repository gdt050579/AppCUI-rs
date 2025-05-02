use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static TYPES: FlagsSignature = FlagsSignature::new(&["Standard", "Circle", "Diamond", "Square", "Star", "Dot"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("caption", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
    NamedParameter::new("selected", "selected", ParamType::Bool),
    NamedParameter::new("select", "selected", ParamType::Bool),
    NamedParameter::new("type", "type", ParamType::String),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("radiobox", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("RadioBox::with_type");
    cb.add_string_parameter("caption", None);
    cb.add_layout();
    cb.add_bool_parameter("selected", Some(false));
    cb.add_enum_parameter("type", "radiobox::Type", &TYPES, Some("Standard"));
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
