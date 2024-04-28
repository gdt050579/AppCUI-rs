use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&[
    "Sizeable",
    "NoCloseButton",
    "FixedPosition",
    "ErrorWindow",
    "NotifyWindow",
    "WarningWindow",
]);
static TYPES: FlagsSignature = FlagsSignature::new(&["Normal", "Error", "Warning", "Notification"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("title", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("title", "title", ParamType::String),
    NamedParameter::new("caption", "title", ParamType::String),
    NamedParameter::new("text", "title", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("type", "type", ParamType::String),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("window", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("Window::with_type");
    cb.add_string_parameter("title", None);
    cb.add_layout();
    cb.add_flags_parameter("flags", "window::Flags", &FLAGS);
    cb.add_enum_parameter("type", "window::Type", &TYPES, Some("Normal"));
    cb.finish_control_initialization();
    cb.into()
}
