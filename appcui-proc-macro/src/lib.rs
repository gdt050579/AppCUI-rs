mod arguments;
mod utils;
use proc_macro::*;
use arguments::*;

extern crate proc_macro;

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn AppCUIControl(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut a = Arguments::new();
    a.parse(args);
    let mut base_definition = "{\n    base: ".to_string();
    base_definition.push_str(&a.base);
    base_definition.push_str(", ");
    let code = input.to_string().replace("{", base_definition.as_str());

    println!("{}",code);
    TokenStream::new()
}
