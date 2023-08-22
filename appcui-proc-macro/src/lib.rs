mod arguments;
mod utils;
use arguments::*;
use proc_macro::*;
use std::str::FromStr;

extern crate proc_macro;

macro_rules! impl_default_trait {
    ($code:expr, $trait_name:expr, $skip_implementation:expr) => {
        if !$skip_implementation {
            $code.push_str("impl ");
            $code.push_str($trait_name);
            $code.push_str(" for $(STRUCT_NAME) { }\n");
        }
    };
}
macro_rules! impl_trait {
    ($code:expr, $trait_name:expr, $pass_to_parent_impl:expr, $skip_implementation:expr, $use_default_trait:expr) => {
        if !$skip_implementation {
            if $use_default_trait {
                impl_default_trait!($code, $trait_name, false);
            } else {
                $code.push_str($pass_to_parent_impl);
                $code.push_str("\n");
            }
        }
    };
}

mod templates {
    pub static IMPORTS: &str = "
    use $(ROOT)::ui::*;
    use $(ROOT)::ui::common::traits::*;
    use $(ROOT)::ui::button::events::ButtonEvents;
    use $(ROOT)::ui::checkbox::events::CheckBoxEvents;
    use $(ROOT)::ui::window::events::WindowEvents;
    use $(ROOT)::ui::command_bar::events::CommandBarEvents;
    use $(ROOT)::ui::menu::events::MenuEvents;
    use $(ROOT)::ui::menu::*;
    use $(ROOT)::graphics::*;
    use $(ROOT)::system::*;
    use $(ROOT)::input::*;
    ";

    pub static IMPORTS_INTERNAL: &str = "
    use crate::utils::*;
    use crate::ui::common::*;
    ";

    pub static DEREF_TRAIT: &str = "
    impl std::ops::Deref for $(STRUCT_NAME) {
        type Target = $(BASE);
        fn deref(&self) -> &Self::Target { return &self.base; }
    }
    impl std::ops::DerefMut for $(STRUCT_NAME) {
        fn deref_mut(&mut self) -> &mut Self::Target { return &mut self.base; }
    }
    ";

    pub static ON_PAINT_TRAIT: &str = "
    impl OnPaint for $(STRUCT_NAME) {
        fn on_paint(&self, surface: &mut Surface, theme: &Theme)  { self.base.on_paint(surface, theme); }
    }
    ";

    pub static ON_KEY_PRESSED_TRAIT: &str = "
    impl OnKeyPressed for $(STRUCT_NAME) {
        fn on_key_pressed(&mut self, key: Key, character: char)->EventProcessStatus { return self.base.on_key_pressed(key, character); }
    }
    ";

    pub static ON_MOUSE_EVENT_TRAIT: &str = "
    impl OnMouseEvent for $(STRUCT_NAME) {
        fn on_mouse_event(&mut self, event: &MouseEvent)->EventProcessStatus { return self.base.on_mouse_event(event); }
    }
    ";

    pub static ON_DEFAULT_ACTION_TRAIT: &str = "
    impl OnDefaultAction for $(STRUCT_NAME) {
        fn on_default_action(&mut self){ self.base.on_default_action(); }
    }
    ";

    pub static ON_RESIZE_TRAIT: &str = "
    impl OnResize for $(STRUCT_NAME) {
        fn on_resize(&mut self, old: Size, new: Size)  { self.base.on_resize(old, new); }
    }
    ";

    pub static ON_FOCUS_TRAIT: &str = "
    impl OnFocus for $(STRUCT_NAME) {
        fn on_focus(&mut self)  { self.base.on_focus(); }
        fn on_lose_focus(&mut self)  { self.base.on_lose_focus(); }
    }
    ";
}
fn parse_token_stream(
    args: TokenStream,
    input: TokenStream,
    base_control: &str,
    extra_trait: &str,
    allow_event_processor: bool,
    use_default_impl: bool,
) -> TokenStream {
    let mut a = Arguments::new(base_control);
    a.parse(args);
    let mut base_definition = "{\n    base: ".to_string();
    base_definition.push_str(&a.base);
    base_definition.push_str(", ");
    let mut code = input.to_string().replace("{", base_definition.as_str());
    let struct_name = utils::extract_structure_name(code.as_str());
    code.insert_str(0, "#[repr(C)]\n");
    code.insert_str(0, templates::IMPORTS);
    if a.internal_mode {
        code.insert_str(0, templates::IMPORTS_INTERNAL);
    }
    code.push_str(templates::DEREF_TRAIT);
    // check for event processors
    if (!allow_event_processor) && (!a.event_processor_list.is_empty()) {
        panic!("Current control does not support event processing. This means that you can not overwrite the following: {}. Only Window based controls are allowed to overwrite event processing traits !",&a.event_processor_list);
    }
    impl_default_trait!(code, "Control", false);
    impl_default_trait!(code, extra_trait, extra_trait.is_empty());
    // for raw controls
    if extra_trait.is_empty() {        
        impl_default_trait!(code, "NotWindow", a.window_control);
        impl_default_trait!(code, "NotDesktop", a.desktop_control);
    }

    // raw events
    impl_trait!(code, "OnPaint", templates::ON_PAINT_TRAIT, a.on_paint, use_default_impl);
    impl_trait!(code, "OnKeyPressed", templates::ON_KEY_PRESSED_TRAIT, a.on_key_pressed, use_default_impl);
    impl_trait!(code, "OnMouseEvent", templates::ON_MOUSE_EVENT_TRAIT, a.on_mouse_event, use_default_impl);
    impl_trait!(code, "OnDefaultAction", templates::ON_DEFAULT_ACTION_TRAIT, a.on_default_action, use_default_impl);
    impl_trait!(code, "OnResize", templates::ON_RESIZE_TRAIT, a.on_resize, use_default_impl);
    impl_trait!(code, "OnFocus", templates::ON_FOCUS_TRAIT, a.on_focus, use_default_impl);
    // control events
    impl_default_trait!(code, "CommandBarEvents", a.command_bar_events);
    impl_default_trait!(code, "MenuEvents", a.menu_events);
    impl_default_trait!(code, "ButtonEvents", a.button_events);
    impl_default_trait!(code, "CheckBoxEvents", a.checkbox_events);
    impl_default_trait!(code, "WindowEvents", a.window_events);

    // replace templates
    code = code
        .replace("$(STRUCT_NAME)", &struct_name)
        .replace("$(BASE)", &a.base)
        .replace("$(ROOT)", a.root);
    //println!("{}", code);
    TokenStream::from_str(&code).expect("Fail to convert string to token stream")
}
/// Use to create a custom control
/// The general format is: `#[CustomControl(overwrite = ...)]`
/// Where the overwrite parameter is a list of traits that can be overwritten that include:
/// * OnPaint 
/// * OnKeyPressed
/// * OnMouseEvents
/// * OnDefaultAction
/// * OnResize
/// * OnFocus
/// 
/// If not overwritten, a default implementation will be automatically added
/// 
/// # Example
/// ```
/// use AppCUIProcMacro::*;
/// 
/// #[CustomControl(overwrite = OnPaint+OnKeyPressed)]
/// struct MyCustomControl {
///     // custom data
/// }
/// impl MyCustomControl { /* specific methods */}
/// impl OnPaint for MyCustomControl {
///     fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
///         // add your code that draws that control here
///         // clipping is already set
///     }
/// }
/// impl OnKeyPressed for MyCustomControl {
///     fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
///         // do some actions based on the provided key
///         // this method should return `EventProcessStatus::Processed` if 
///         // the provided key was used, or `EventProcessStatus::Ignored` if 
///         // the key should be send to the parent control.
///     }
/// }
/// ```
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn CustomControl(args: TokenStream, input: TokenStream) -> TokenStream {
    parse_token_stream(args, input, "ControlBase", "", false, true)
}
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Window(args: TokenStream, input: TokenStream) -> TokenStream {
    parse_token_stream(args, input, "Window", "WindowControl", true, false)
}
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Desktop(args: TokenStream, input: TokenStream) -> TokenStream {
    parse_token_stream(args, input, "Desktop", "DesktopControl", false, true)
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
