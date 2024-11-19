use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static TYPES: FlagsSignature = FlagsSignature::new(&["Normal", "Underlined"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[
    PositionalParameter::new("caption", ParamType::String),
    PositionalParameter::new("tooltip", ParamType::String)
];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("name", "caption", ParamType::String),
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
    NamedParameter::new("description", "tooltip", ParamType::String),
    NamedParameter::new("desc", "tooltip", ParamType::String),
    NamedParameter::new("tooltip", "tooltip", ParamType::String),
    NamedParameter::new("type", "type", ParamType::String),
    NamedParameter::new("select", "select", ParamType::Bool),
    NamedParameter::new("selected", "select", ParamType::Bool),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("togglebutton", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("ToggleButton::new");
    cb.add_string_parameter("caption", None);
    cb.add_string_parameter("tooltip", None);
    cb.add_layout();
    cb.add_bool_parameter("select", Some(false));
    cb.add_enum_parameter("type", "togglebutton::Type", &TYPES, Some("Normal"));
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
