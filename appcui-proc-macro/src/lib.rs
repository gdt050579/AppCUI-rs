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
    TokenStream::new()
}
