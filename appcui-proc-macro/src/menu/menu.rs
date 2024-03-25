use crate::{
    parameter_parser::{self, *},
    token_stream_to_string::TokenStreamToString,
};
use proc_macro::*;
use std::str::FromStr;

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[
    PositionalParameter::new("caption", ParamType::String),
];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
    NamedParameter::new("items", "items", ParamType::List),    
    NamedParameter::new("subitems", "items", ParamType::List),
    NamedParameter::new("class", "class", ParamType::String),
];

pub(crate) fn create(input: TokenStream, class: Option<&str>) -> TokenStream {
    let s = input.validate_one_string_parameter("menu");
    let mut d = parameter_parser::parse(&s).unwrap();
    d.validate_positional_parameters(&s, POSILITIONAL_PARAMETERS).unwrap();
    d.validate_named_parameters(&s, NAMED_PARAMETERS).unwrap();

    let res = super::common::build_menu(&s, &mut d, class);
    TokenStream::from_str(&res).expect("Fail to convert 'menu!' macro content to token stream")
}