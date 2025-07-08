use proc_macro::{Delimiter, TokenStream, TokenTree};

pub(crate) trait TokenStreamToString {
    fn validate_one_string_parameter(self, name: &str)->String;
}
impl TokenStreamToString for TokenStream {
    fn validate_one_string_parameter(self, name: &str)->String {
        let mut tokens = self.into_iter().peekable();

        let mut string_param = match tokens.next() {
            Some(TokenTree::Literal(lit)) => lit.to_string(),
            Some(TokenTree::Ident(ident)) => {
                panic!("The parameter provided to the '{name}!' macro must be a string literal and not an identifier ('{ident}').")
            }
            Some(TokenTree::Punct(punct)) => {
                panic!("The parameter provided to the '{name}!' macro must be a string literal and not an punctuation character ('{punct}').")
            }
            Some(TokenTree::Group(g)) => {
                if g.delimiter() == Delimiter::None {
                    let value = g.to_string();
                    if value.starts_with("\"") && value.ends_with("\"") {
                        value 
                    } else {
                        panic!("The parameter provided to the '{name}!' macro must be a string literal and not an group ('{g}' with a None delimiter).")
                    }
                } else {
                    panic!("The parameter provided to the '{}!' macro must be a string literal and not an group ('{}' with delimiter: {:?}).", name, g, g.delimiter())
                }
            }
            _ => panic!("Expecting a string literal for the '{name}!' macro !"),
        };

        if tokens.peek().is_some() {
            panic!("Exactly one string must be provided as input.");
        }
        if (!string_param.starts_with('\"')) || (!string_param.ends_with('\"')) {
            panic!("The parameter provided to the '{name}!' macro must be a string literal and not another literal: '{string_param}'");
        }
        if string_param.len() == 2 {
            panic!("You can not provide an empty string for '{name}!' macro !");
        }

        string_param.remove(0);
        string_param.remove(string_param.len() - 1);

        string_param
    }
}