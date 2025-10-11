use super::control_builder::ControlBuilder;
use crate::{parameter_parser::*, utils};
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&[
    "Sizeable",
    "NoCloseButton",
    "FixedPosition",
    "ErrorWindow",
    "NotifyWindow",
    "WarningWindow",
]);
static TYPES: FlagsSignature = FlagsSignature::new(&["Classic", "Rounded", "Panel"]);
static BACKGROUNDS: FlagsSignature = FlagsSignature::new(&["Normal", "Error", "Warning", "Notification"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("title", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("title", "title", ParamType::String),
    NamedParameter::new("caption", "title", ParamType::String),
    NamedParameter::new("text", "title", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("type", "type", ParamType::String),
    NamedParameter::new("background", "background", ParamType::String),
    NamedParameter::new("back", "background", ParamType::String),
    NamedParameter::new("bg", "background", ParamType::String),
    NamedParameter::new("tag", "tag", ParamType::String),
    NamedParameter::new("hot-key", "hotkey", ParamType::String),
    NamedParameter::new("hotkey", "hotkey", ParamType::String),
    NamedParameter::new("key", "hotkey", ParamType::String),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("window", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("Window::with_type");
    cb.add_string_parameter("title", None);
    cb.add_layout();
    cb.add_flags_parameter("flags", "window::Flags", &FLAGS);
    cb.add_enum_parameter("type", "window::Type", &TYPES, Some("Classic"));
    cb.add_enum_parameter("background", "window::Background", &BACKGROUNDS, Some("Normal"));
    cb.finish_control_initialization();
    if cb.has_parameter("tag") {
        cb.add("\n\tcontrol.set_tag(");
        cb.add_string_parameter("tag", None);
        cb.add_line(");\n");  
    }
    if cb.has_parameter("hotkey") {
        let s = cb.get_value("hotkey").unwrap();
        if utils::equal_ignore_case(s, "auto") {
            cb.add_line("control.set_auto_hotkey();")
        } else {
            let key = crate::key::builder::create_string(s);
            cb.add("\n\tcontrol.set_hotkey(");
            cb.add(&key);
            cb.add_line(");\n");  
        }
    }
    cb.into()
}
