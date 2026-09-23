use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ScrollBars"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("size", ParamType::Size)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("size", "size", ParamType::Size),
    NamedParameter::new("sz", "size", ParamType::Size),
    NamedParameter::new("surface", "size", ParamType::Size),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("background", "back", ParamType::Dict),
    NamedParameter::new("back", "back", ParamType::Dict),
    NamedParameter::new("left-scroll-margin", "lsm", ParamType::Integer),
    NamedParameter::new("lsm", "lsm", ParamType::Integer),
    NamedParameter::new("top-scroll-margin", "tsm", ParamType::Integer),
    NamedParameter::new("tsm", "tsm", ParamType::Integer),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("canvas", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("Canvas::new");
    cb.add_size_parameter("size", None);
    cb.add_layout();
    cb.add_flags_parameter("flags", "canvas::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.call_method_with_dict_parameter_parser("set_background", "back", |repr, dict| crate::chars::builder::create_from_dict(&repr, dict));
    cb.add_scroll_margin_setup("lsm","tsm");
    cb.into()
}
