use super::TextAlignament;
use crate::{
    parameter_parser::{self, color::Color, *},
    token_stream_to_string::TokenStreamToString,
};
use proc_macro::*;
use std::fmt::Write;
use std::str::FromStr;

static mut TEXT_ALIGM: FlagsSignature = FlagsSignature::new(&["Left", "Right", "Center"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[
    PositionalParameter::new("caption", ParamType::String),
    PositionalParameter::new("width", ParamType::String),
    PositionalParameter::new("align", ParamType::String),
    PositionalParameter::new("type", ParamType::String),
];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    // caption
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("name", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
    // width
    NamedParameter::new("width", "width", ParamType::String),
    NamedParameter::new("w", "width", ParamType::String),
    // align
    NamedParameter::new("align", "align", ParamType::String),
    NamedParameter::new("a", "align", ParamType::String),
    NamedParameter::new("alignament", "align", ParamType::String),
    // type
    NamedParameter::new("type", "type", ParamType::String),
    NamedParameter::new("datatype", "type", ParamType::String),
    NamedParameter::new("dt", "type", ParamType::String),
];

pub(crate) fn create_from_dict(param_list: &str, dict: &mut NamedParamsMap) -> String {
    dict.validate_positional_parameters(param_list, POSILITIONAL_PARAMETERS).unwrap();
    dict.validate_named_parameters(param_list, NAMED_PARAMETERS).unwrap();
    let mut res = String::with_capacity(64);
    // add caption
    res.push_str("Column::new(\"");
    if let Some(value) = dict.get("caption") {
        res.push_str(value.get_string());
        res.push_str("\", ");
    } else {
        panic!("The caption of the column is missing ! Have you forget to add `caption='...'` ?");
    }
    // width
    if let Some(value) = dict.get("width") {
        // compute the value
        if let Ok(width) = value.get_string().parse::<u8>() {
            write!(res, "{},", width).unwrap();
        } else {
            panic!("Invalid value for the width of the column: '{}'",value.get_string());
        }
    } else {
        // default is the size of the caption
        let width = dict.get("caption").unwrap().get_string().chars().count().min(100);
        write!(res, "{},", width + 2).unwrap();
    }
    // add alignament
    res.push_str("TextAlignament::");
    if let Some(value) = dict.get("align") {
        if let Some(align) = TextAlignament::from_hash(crate::utils::compute_hash(value.get_string())) {
            res.push_str(align.get_name());
            res.push(',');
        } else {
            panic!("Invalid alignament value for column ! Available options are: Left or L, Right or R, Center or C");
        }
    } else {
        // default to Left
        res.push_str("Left, ");
    }
    // finalize
    res.push(')');
    res
}

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let s = input.validate_one_string_parameter("column");
    let mut d = parameter_parser::parse(&s).unwrap();
    let res = create_from_dict(&s, &mut d);
    TokenStream::from_str(&res).expect("Fail to convert 'column!' macro content to token stream")
}
