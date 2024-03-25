use proc_macro::{TokenStream, TokenTree};

pub(crate) trait TokenStreamToString {
    fn validate_one_string_parameter(self, name: &str)->String;
}
impl TokenStreamToString for TokenStream {
    fn validate_one_string_parameter(self, name: &str)->String {
        let mut tokens = self.into_iter().peekable();

        let mut string_param = match tokens.next() {
            Some(TokenTree::Literal(lit)) => lit.to_string(),
            _ => panic!("The parameter provided to the '{}!' macro must be a string literal.", name),
        };

        if tokens.peek().is_some() {
            panic!("Exactly one string must be provided as input.");
        }
        if (!string_param.starts_with('\"')) || (!string_param.ends_with('\"')) {
            panic!("The parameter provided to the '{}!' macro must be a string literal.", name);
        }
        if string_param.len() == 2 {
            panic!("You can not provide an empty string for '{}!' macro !", name);
        }

        string_param.remove(0);
        string_param.remove(string_param.len() - 1);

        string_param
    }
}