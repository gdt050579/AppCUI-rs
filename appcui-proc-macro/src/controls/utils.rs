use proc_macro::*;
use std::str::FromStr;

pub(super) fn token_stream_to_string(name: &str, input: TokenStream) -> String {
    let mut tokens = input.into_iter().peekable();

    let mut string_param = match tokens.next() {
        Some(TokenTree::Literal(lit)) => lit.to_string(),
        _ => panic!("The parameter provided to the '{}!' macro must be a string literal.", name),
    };

    if tokens.peek().is_some() {
        panic!("Exactly one string must be provided as input.");
    }
    if (!string_param.starts_with("\"")) || (!string_param.ends_with("\"")) {
        panic!("The parameter provided to the '{}!' macro must be a string literal.", name);
    }
    if string_param.len() == 2 {
        panic!("You can not provide an empty string for '{}!' macro !",name);
    }

    string_param.remove(0);
    string_param.remove(string_param.len() - 1);

    string_param
}
pub(super) fn to_token_stream(text: String) -> TokenStream {
    TokenStream::from_str(&text).expect(format!("Fail to convert '{}!' macro content to token stream", text).as_str())
}
