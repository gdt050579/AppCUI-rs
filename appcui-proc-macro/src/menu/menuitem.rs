use crate::{
    parameter_parser::{self, color::Color, *},
    token_stream_to_string::TokenStreamToString,
};
use proc_macro::*;
use std::str::FromStr;

static mut CHAR_ATTR: FlagsSignature = FlagsSignature::new(&["Bold", "Italic", "Underline"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[
    PositionalParameter::new("caption", ParamType::String),
];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),

    NamedParameter::new("shortcut", "shortcut", ParamType::String),
    NamedParameter::new("shortcutkey", "shortcut", ParamType::String),

    NamedParameter::new("cmd", "cmd", ParamType::Integer),
    NamedParameter::new("command", "cmd", ParamType::Integer),
    NamedParameter::new("cmd-id", "cmd", ParamType::Integer),
    NamedParameter::new("command-id", "cmd", ParamType::Integer),

    NamedParameter::new("enable", "enable", ParamType::Bool),
    NamedParameter::new("enabled", "enable", ParamType::Bool),

    NamedParameter::new("check", "checked", ParamType::Bool),
    NamedParameter::new("checked", "checked", ParamType::Bool),

    NamedParameter::new("select", "select", ParamType::Bool),
    NamedParameter::new("selected", "select", ParamType::Bool),

    NamedParameter::new("items", "items", ParamType::Dict), // should be LIST
    NamedParameter::new("subitems", "items", ParamType::Dict), // should be LIST

    NamedParameter::new("type", "type", ParamType::String),
];


fn menuitem_from_dict(param_list: &str, dict: &mut NamedParamsMap) -> String {
    dict.validate_positional_parameters(param_list, POSILITIONAL_PARAMETERS).unwrap();
    dict.validate_named_parameters(param_list, NAMED_PARAMETERS).unwrap();
    // Step 1 --> identify the type:
    if dict.contains("type") {
    }


    let mut res = String::with_capacity(64);
    res.push_str("Character::new(");
    // if let Some(value) = dict.get("code") {
    //     let code_value = unicode_number_to_value(value.get_string());
    //     res.push_str(format!{"'\\u{{{:x}}}'",code_value}.as_str());
    // }
    res
}
pub(crate) fn create(input: TokenStream) -> TokenStream {
    let s = input.validate_one_string_parameter("menuitem");
    let mut d = parameter_parser::parse(&s).unwrap();
    let res = menuitem_from_dict(&s, &mut d);
    TokenStream::from_str(&res).expect(format!("Fail to convert 'menuitem!' macro content to token stream").as_str())
}