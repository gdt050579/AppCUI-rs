use super::common;
use super::utils;
use crate::parameter_parser::ParamSignature;
use crate::parameter_parser::ParamType;
use crate::parameter_parser;
use proc_macro::*;

static SIGNATURE: &[ParamSignature] = &[
    // mandatory (positional)
    ParamSignature::mandatory("name", "caption", ParamType::String),
    // optionals
    ParamSignature::optional("caption", "caption", ParamType::String),
    ParamSignature::optional("text", "caption", ParamType::String),
    ParamSignature::optional("flags", "flags", ParamType::Flags),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let s = utils::token_stream_to_string("button", input);
    let mut p = parameter_parser::parse(&s).unwrap();
    p.validate_signature(&s, SIGNATURE).unwrap();
    p.validate_signature(&s, common::SIGNATURE).unwrap();
    p.check_unkwnon_params(&s).unwrap();
    // all good --> lets build the query
    let mut result = String::with_capacity(512);
    result.push_str("{\n\t");
    result.push_str("let but = Button::new(");
    // first add the caption
    // second add the layout
    result.push_str(" , Layout::new(");
    // lastly add the flags
    result.push_str(");\n\t");
    // finally close block
    result.push_str("but\n}");
    utils::to_token_stream(result)
}
