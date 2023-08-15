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
        fn on_key_pressed(&mut self, key: Key, character: char)->EventProcessStatus { return self.base.on_key_pressed(key, character); }
    }
    ";

    pub static ON_MOUSE_EVENT_TRAIT: &str = "
    impl OnMouseEvent for $STRUCT_NAME$ {
        fn on_mouse_event(&mut self, event: &MouseEvent)->EventProcessStatus { return self.base.on_mouse_event(event); }
    }
    ";

    pub static ON_DEFAULT_ACTION_TRAIT: &str = "
    impl OnDefaultAction for $STRUCT_NAME$ {
        fn on_default_action(&mut self){ self.base.on_default_action(); }
    }
    ";

    pub static ON_RESIZE_TRAIT: &str = "
    impl OnResize for $STRUCT_NAME$ {
        fn on_resize(&mut self, old: Size, new: Size)  { self.base.on_resize(old, new); }
    }
    ";

    pub static ON_FOCUS_TRAIT: &str = "
    impl OnFocus for $STRUCT_NAME$ {
        fn on_focus(&mut self)  { self.base.on_focus(); }
        fn on_lose_focus(&mut self)  { self.base.on_lose_focus(); }
    }
    ";

    pub static ON_EVENT_TRAIT: &str = "
    impl OnEvent for $STRUCT_NAME$ {
        fn on_event(&mut self, event: Event) -> EventProcessStatus  { 
            return OnEvent::on_event(&mut self.base,event);
        }
    }
    ";

    pub static COMMANDBAR_EVENTS_TRAIT: &str = "
    impl CommandBarEvents for $STRUCT_NAME$ {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            CommandBarEvents::on_update_commandbar(&self.base, commandbar);
        }
        fn on_event(&mut self, command_id: u32) {
            CommandBarEvents::on_event(&mut self.base, command_id);
        }
    }
    ";

    pub static MENU_EVENTS_TRAIT: &str = "
    impl MenuEvents for $STRUCT_NAME$ {
        fn on_menu_open(&self, menu: &mut Menu) {
            MenuEvents::on_menu_open(&self.base, menu);
        }
        fn on_event(&mut self, event: MenuEvent) {
            MenuEvents::on_event(&mut self.base,event);
        }
        fn on_update_menubar(&self, menubar: &mut MenuBar) {
            MenuEvents::on_update_menubar(&self.base, menubar);
        }
    }
    ";    
}
fn parse_token_stream(args: TokenStream, input: TokenStream, base_control: &str, extra_code: &str) -> TokenStream {
    let mut a = Arguments::new(base_control);
    a.parse(args);
    let mut base_definition = "{\n    base: ".to_string();
    base_definition.push_str(&a.base);
    base_definition.push_str(", ");
    let mut code = input.to_string().replace("{", base_definition.as_str());
    let struct_name = utils::extract_structure_name(code.as_str());
    code.insert_str(0, "#[repr(C)]\n");
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
    if !a.on_resize {
        code.push_str(templates::ON_RESIZE_TRAIT);
    }
    if !a.on_focus {
        code.push_str(templates::ON_FOCUS_TRAIT);
    }
    if !a.on_event {
        code.push_str(templates::ON_EVENT_TRAIT);
    }
    if !a.command_bar_events {
        code.push_str(templates::COMMANDBAR_EVENTS_TRAIT);
    }
    if !a.menu_events {
        code.push_str(templates::MENU_EVENTS_TRAIT);
    }
    // add the extra code
    code.push_str("\n");
    code.push_str(extra_code);
    // replace templates
    code = code
        .replace("$STRUCT_NAME$", &struct_name)
        .replace("$BASE$", &a.base);
    //println!("{}", code);
    TokenStream::from_str(&code).expect("Fail to convert string to token stream")
}
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn AppCUIControl(args: TokenStream, input: TokenStream) -> TokenStream {
    parse_token_stream(args, input, "ControlBase", "")
}
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn AppCUIWindow(args: TokenStream, input: TokenStream) -> TokenStream {
    parse_token_stream(args, input, "Window", "impl WindowControl for $STRUCT_NAME$ {}")
}
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn AppCUIDesktop(args: TokenStream, input: TokenStream) -> TokenStream {
    parse_token_stream(args, input, "Desktop", "impl DesktopControl for $STRUCT_NAME$ {}")
}

#[proc_macro]
pub fn key(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().peekable();

    let string_param = match tokens.next() {
        Some(TokenTree::Literal(lit)) => lit.to_string(),
        _ => panic!("The parameter provided to the key macro must be a string literal."),
    };

    if tokens.peek().is_some() {
        panic!("Exactly one string must be provided as input.");
    }
    if (!string_param.starts_with("\"")) || (!string_param.ends_with("\"")) {
        panic!("The parameter provided to the key macro must be a string literal.");
    }

    let value = parse_string_key_representation(&string_param[1..&string_param.len() - 1]);
    let mut string_repr = value.to_string();
    string_repr.push_str("u16");
    TokenStream::from_str(&string_repr).expect("Fail to convert key to token stream")
}

fn parse_string_key_representation(string: &str) -> u16 {
    let mut key_value = 0u16;
    let mut modifiers = 0u16;

    for key in string.split('+') {
        let modifier = match key {
            "Ctrl" => 0x200,
            "Alt" => 0x100,
            "Shift" => 0x400,
            _ => 0,
        };

        let key_code = if modifier != 0 {
            0
        } else {
            parse_key_name(key)
        };
        if (modifier == 0) && (key_code == 0) {
            panic!("Unknown key or modifier: {}", key);
        }
        if ((modifiers & modifier) != 0) && (modifier != 0) {
            panic!("Duplicate modifier: {}", key);
        }
        if (key_value != 0) && (key_code != 0) {
            panic!("A key can only be added once: {}", key);
        }
        modifiers |= modifier;
        key_value = key_code;
    }
    if (modifiers == 0) && (key_value == 0) {
        panic!("Invalid key combination: {}", string);
    }
    modifiers | key_value
}

fn parse_key_name(key: &str) -> u16 {
    match key {
        "F1" => 1,
        "F2" => 2,
        "F3" => 3,
        "F4" => 4,
        "F5" => 5,
        "F6" => 6,
        "F7" => 7,
        "F8" => 8,
        "F9" => 9,
        "F10" => 10,
        "F11" => 11,
        "F12" => 12,
        "Enter" => 13,
        "Escape" => 14,
        "Insert" => 15,
        "Delete" => 16,
        "Backspace" => 17,
        "Tab" => 18,
        "Left" => 19,
        "Up" => 20,
        "Down" => 21,
        "Right" => 22,
        "PageUp" => 23,
        "PageDown" => 24,
        "Home" => 25,
        "End" => 26,
        "Space" => 27,
        "A" => 28,
        "B" => 29,
        "C" => 30,
        "D" => 31,
        "E" => 32,
        "F" => 33,
        "G" => 34,
        "H" => 35,
        "I" => 36,
        "J" => 37,
        "K" => 38,
        "L" => 39,
        "M" => 40,
        "N" => 41,
        "O" => 42,
        "P" => 43,
        "Q" => 44,
        "R" => 45,
        "S" => 46,
        "T" => 47,
        "U" => 48,
        "V" => 49,
        "W" => 50,
        "X" => 51,
        "Y" => 52,
        "Z" => 53,
        "0" => 54,
        "1" => 55,
        "2" => 56,
        "3" => 57,
        "4" => 58,
        "5" => 59,
        "6" => 60,
        "7" => 61,
        "8" => 62,
        "9" => 63,
        _ => 0,
    }
}
