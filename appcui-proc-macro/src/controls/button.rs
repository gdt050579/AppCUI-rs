use super::common;
use super::layout;
use super::utils;
use crate::parameter_parser;
use crate::parameter_parser::*;
use proc_macro::*;

static mut BUTTON_FLAGS: FlagsSignature = FlagsSignature::new(&["Flat"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("caption", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("name", "caption", ParamType::String),
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let s = utils::token_stream_to_string("button", input);
    let mut p = parameter_parser::parse(&s).unwrap();
    p.validate_positional_parameters(&s, POSILITIONAL_PARAMETERS).unwrap();
    p.validate_names_parameters(&s, NAMED_PARAMETERS).unwrap();
    p.validate_names_parameters(&s, common::CONTROL_NAMED_PARAMATERS).unwrap();
    p.check_unkwnon_params(&s).unwrap();
    // all good --> lets build the query
    let mut result = String::with_capacity(512);
    result.push_str("{\n\t");
    result.push_str("let mut but = Button::new(");
    // first add the caption
    let caption = p.get("caption").expect("First parameter (caption) has to be provided !");
    common::add_string(&mut result, caption.get_string());
    // second add the layout
    result.push_str(" , ");
    layout::add_layout(&mut result, &p);
    // lastly add the flags
    result.push_str(" , ");
    if let Some(flags) = p.get_mut("flags") {
        common::add_flags(&s, &mut result, "button::Flags", flags.get_list().unwrap(), unsafe { &mut BUTTON_FLAGS }).unwrap();
    } else {
        result.push_str("button::Flags::None");
    }
    result.push_str(");\n\t");
    // basic controls
    common::add_basecontrol_operations(&mut result, "but", &mut p);
    // finally close block
    result.push_str("but\n}");
    utils::to_token_stream(result)
}
