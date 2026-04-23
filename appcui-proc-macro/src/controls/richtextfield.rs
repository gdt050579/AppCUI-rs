use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ProcessEnter", "Readonly", "DisableAutoSelectOnFocus"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("text", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("text", "text", ParamType::String),
    NamedParameter::new("caption", "text", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("parser", "parser", ParamType::Function),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("richtextfield", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    if cb.has_parameter("parser") {
        cb.init_control("RichTextField::with_parser");
    } else {
        cb.init_control("RichTextField::new");
    }
    cb.add_string_parameter("text", Some(""));
    cb.add_layout();
    cb.add_flags_parameter("flags", "richtextfield::Flags", &FLAGS);
    if cb.has_parameter("parser") {
        cb.add_function("parser", None, true);
    }
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
