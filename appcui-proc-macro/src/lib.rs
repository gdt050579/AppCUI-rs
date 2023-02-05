mod arguments;
mod utils;
use arguments::*;
use proc_macro::*;
use std::str::FromStr;

extern crate proc_macro;

mod templates {
    pub static DEREF_TRAIT: &str = "
    impl std::ops::Deref for $STRUCT_NAME$ {
        type Target = $BASE$;
        fn deref(&self) -> &Self::Target { return &self.base; }
    }
    impl std::ops::DerefMut for $STRUCT_NAME$ {
        fn deref_mut(&mut self) -> &mut Self::Target { return &mut self.base; }
    }
    ";

    pub static CONTROL_TRAIT: &str = "
    impl Control for $STRUCT_NAME$ {
    }
    ";

    pub static ON_PAINT_TRAIT: &str = "
    impl OnPaint for $STRUCT_NAME$ {
        fn on_paint(&self, surface: &mut Surface, theme: &Theme)  { self.base.on_paint(surface, theme); }
    }
    ";

    pub static ON_KEY_PRESSED_TRAIT: &str = "
    impl OnKeyPressed for $STRUCT_NAME$ {
        fn on_key_pressed(&mut self, key: Key, character: char)->KeyPressedResult { return self.base.on_key_pressed(key, character); }
    }
    ";   
    
    pub static ON_MOUSE_EVENT_TRAIT: &str = "
    impl OnMouseEvent for $STRUCT_NAME$ {
        fn on_mouse_event(&mut self, event: &MouseEvent){ self.base.on_mouse_event(event); }
    }
    "; 

    pub static ON_DEFAULT_ACTION_TRAIT: &str = "
    impl OnDefaultAction for $STRUCT_NAME$ {
        fn on_default_action(&mut self){ self.base.on_default_action(); }
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
    code.push_str(templates::DEREF_TRAIT);
    code.push_str(templates::CONTROL_TRAIT);

    // defaults for various events
    if !a.on_paint {
        code.push_str(templates::ON_PAINT_TRAIT);
    }
    if !a.on_key_pressed {
        code.push_str(templates::ON_KEY_PRESSED_TRAIT);
    }
    if !a.on_mouse_event {
        code.push_str(templates::ON_MOUSE_EVENT_TRAIT);
    }
    if !a.on_default_action {
        code.push_str(templates::ON_DEFAULT_ACTION_TRAIT);
    }
    // replace templates
    code = code
        .replace("$STRUCT_NAME$", &struct_name)
        .replace("$BASE$", &a.base);
    println!("{}", code);
    TokenStream::from_str(&code).expect("Fail to convert string to token stream")
}
