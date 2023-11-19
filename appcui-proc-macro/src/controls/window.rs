use super::common;
use super::layout;
use super::utils;
use crate::parameter_parser;
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
    let s = utils::token_stream_to_string("window", input);
    let mut p = parameter_parser::parse(&s).unwrap();
    p.validate_positional_parameters(&s, POSILITIONAL_PARAMETERS).unwrap();
    p.validate_names_parameters(&s, NAMED_PARAMETERS).unwrap();
    p.validate_names_parameters(&s, common::CONTROL_NAMED_PARAMATERS).unwrap();
    p.check_unkwnon_params(&s).unwrap();
    // all good --> lets build the query
    let mut result = String::with_capacity(512);
    result.push_str("Window::new(");
    // first add the caption
    let caption = p.get("title").expect("First parameter (title) has to be provided !");
    common::add_string(&mut result, caption.get_string());
    // second add the layout
    result.push_str(" , ");
    layout::add_layout(&mut result, &p);
    // lastly add the flags
    result.push_str(" , ");
    if let Some(flags) = p.get_mut("flags") {
        common::add_flags(&s, &mut result, "window::Flags", flags.get_list().unwrap(), unsafe { &mut WINDOW_FLAGS }).unwrap();
    } else {
        result.push_str("window::Flags::None");
    }
    result.push_str(")");
    utils::to_token_stream(result)
}
