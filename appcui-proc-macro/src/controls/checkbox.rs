use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static TYPES: FlagsSignature = FlagsSignature::new(&["Standard", "Ascii", "CheckBox", "CheckMark"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("caption", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
    NamedParameter::new("checked", "checked", ParamType::Bool),
    NamedParameter::new("check", "checked", ParamType::Bool),
    NamedParameter::new("type", "type", ParamType::String),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("checkbox", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("CheckBox::with_type");
    cb.add_string_parameter("caption", None);  
    cb.add_layout();
    cb.add_bool_parameter("checked", Some(false));
    cb.add_enum_parameter("type", "checkbox::Type", &TYPES, Some("Standard"));
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
