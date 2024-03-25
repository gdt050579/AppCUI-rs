use crate::{
    parameter_parser,
    token_stream_to_string::TokenStreamToString,
};
use proc_macro::*;
use std::str::FromStr;

pub(crate) fn create(input: TokenStream, class: Option<&str>) -> TokenStream {
    let s = input.validate_one_string_parameter("menuitem");
    let mut d = parameter_parser::parse(&s).unwrap();
    let res = super::common::menuitem_from_dict(&s, &mut d, class);
    TokenStream::from_str(&res).expect("Fail to convert 'menuitem!' macro content to token stream")
}