use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ReadOnly", "CaseSensitive"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("path", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("path", "path", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("pathfinder", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("PathFinder::new");
    cb.add_string_parameter("path", None);
    cb.add_layout();
    cb.add_flags_parameter("flags", "pathfinder::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
