use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ShowLineNumber", "ReadOnly", "ScrollBars", "HighlightCursor"]);

static POSITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("text", ParamType::String)];

static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("text", "text", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("textarea", input, POSITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("TextArea::new");
    cb.add_string_parameter("text", Some(""));
    cb.add_layout();
    cb.add_flags_parameter("flags", "textarea::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
