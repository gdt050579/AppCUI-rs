use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static mut WINDOW_FLAGS: FlagsSignature = FlagsSignature::new(&[
    "Sizeable",
    "NoCloseButton",
    "FixedPosition",
    "ErrorWindow",
    "NotifyWindow",
    "WarningWindow",
]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("title", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("title", "title", ParamType::String),
    NamedParameter::new("caption", "title", ParamType::String),
    NamedParameter::new("text", "title", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("window", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS);
    cb.init_control("Window::new");
    cb.add_strng_parameter("title");  
    cb.add_layout();
    cb.add_flags_parameter("flags", "window::Flags", unsafe { &mut WINDOW_FLAGS });
    cb.finish_control_initialization();
    cb.into()
}

