use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ScrollBars"]);
static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("content", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("content", "content", ParamType::String),
    NamedParameter::new("cont", "content", ParamType::String),
    NamedParameter::new("c", "content", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("left-scroll-margin", "lsm", ParamType::Integer),
    NamedParameter::new("lsm", "lsm", ParamType::Integer),
    NamedParameter::new("top-scroll-margin", "tsm", ParamType::Integer),
    NamedParameter::new("tsm", "tsm", ParamType::Integer),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("markdown", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("Markdown::new", );
    cb.add_string_parameter("content", None);
    cb.add_layout();
    cb.add_flags_parameter("flags", "markdown::Flags", &FLAGS);
    cb.finish_control_initialization();

    cb.add_scroll_margin_setup("lsm", "tsm");
    cb.into()
}
