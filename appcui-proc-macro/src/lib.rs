mod arguments;
mod utils;
use arguments::*;
use proc_macro::*;

extern crate proc_macro;

mod templates {
    pub static DEREF_CODE: &str = "
    impl std::ops::Deref for $STRUCT_NAME$ {
        type Target = $BASE$;
        fn deref(&self) -> &Self::Target { return &self.base; }
    }
    impl std::ops::DerefMut for $STRUCT_NAME$ {
        fn deref_mut(&mut self) -> &mut Self::Target { return &mut self.base; }
    }
    ";

    pub static ON_PAINT_CODE: &str = "
    impl OnPaint for $STRUCT_NAME$ {
        fn on_paint(&self)  { self.base.on_paint(); }
    }
    ";
}
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn AppCUIControl(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut a = Arguments::new();
    a.parse(args);
    let mut base_definition = "{\n    base: ".to_string();
    base_definition.push_str(&a.base);
    base_definition.push_str(", ");
    let mut code = input.to_string().replace("{", base_definition.as_str());
    let struct_name = utils::extract_structure_name(code.as_str());
    code.push_str(templates::DEREF_CODE);
    if a.on_paint {
        code.push_str(templates::ON_PAINT_CODE);
    }

    // replace templates
    code = code
        .replace("$STRUCT_NAME$", &struct_name)
        .replace("$BASE$", &a.base);
    println!("{}", code);
    TokenStream::new()
}
