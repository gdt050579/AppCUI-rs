use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ShowMarkers"]);
static POSITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("content", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("content", "content", ParamType::String),
    NamedParameter::new("text", "content", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("markdown_composer", input, POSITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("MarkdownComposer::from", );
    
    cb.add_string_parameter("content", Some(""));
    cb.add_layout();
    cb.add_flags_parameter("flags", "markdown_composer::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
