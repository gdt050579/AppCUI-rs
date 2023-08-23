mod arguments;
mod templates;
mod utils;
mod appcui_traits;
mod traits_configuration;
use arguments::*;
use proc_macro::*;

use traits_configuration::TraitsConfig;
use traits_configuration::TraitImplementation;
use appcui_traits::AppCUITrait;
use std::str::FromStr;

extern crate proc_macro;

fn parse_token_stream(
    args: TokenStream,
    input: TokenStream,
    base_control: &str,
    config: &mut TraitsConfig
) -> TokenStream {
    let mut a = Arguments::new(base_control);
    a.parse(args, config);
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
    for (appcui_trait,trait_impl) in config.iter() {
        match trait_impl {
            TraitImplementation::None => {},
            TraitImplementation::Default | TraitImplementation::DefaultNonOverwritable => {
                code.push_str(appcui_trait.get_default_implementation());
            },
            TraitImplementation::BaseFallback | TraitImplementation::BaseFallbackNonOverwritable => {
                code.push_str(appcui_trait.get_basefallback_implementation());
            },
        }
        code.push_str("\n");
    }

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
/// ```rust,compile_fail
/// use appcui::prelude::*;
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
    let mut config = TraitsConfig::new("CustomControl");
    // Deref is mandatory
    config.set(AppCUITrait::Deref, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::Control, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::NotDesktop, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::NotWindow, TraitImplementation::DefaultNonOverwritable);
    // Raw events (implemente by default)
    config.set(AppCUITrait::OnPaint, TraitImplementation::Default);
    config.set(AppCUITrait::OnResize, TraitImplementation::Default);
    config.set(AppCUITrait::OnFocus, TraitImplementation::Default);
    config.set(AppCUITrait::OnDefaultAction, TraitImplementation::Default);
    config.set(AppCUITrait::OnKeyPressed, TraitImplementation::Default);
    config.set(AppCUITrait::OnMouseEvents, TraitImplementation::Default);
    // control events
    config.set(AppCUITrait::ButtonEvents, TraitImplementation::DefaultNonOverwritable);   
    config.set(AppCUITrait::CheckBoxEvents, TraitImplementation::DefaultNonOverwritable);   
    config.set(AppCUITrait::WindowEvents, TraitImplementation::DefaultNonOverwritable);   
    config.set(AppCUITrait::MenuEvents, TraitImplementation::DefaultNonOverwritable);   
    config.set(AppCUITrait::CommandBarEvents, TraitImplementation::DefaultNonOverwritable);   

    parse_token_stream(args, input, "ControlBase", &mut config)
}
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Window(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut config = TraitsConfig::new("Window");
    // Deref is mandatory
    config.set(AppCUITrait::Deref, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::Control, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::WindowControl, TraitImplementation::DefaultNonOverwritable);
    // Raw events (implemente by default)
    config.set(AppCUITrait::OnPaint, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnResize, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnFocus, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnDefaultAction, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnKeyPressed, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnMouseEvents, TraitImplementation::BaseFallbackNonOverwritable);
    // control events
    config.set(AppCUITrait::ButtonEvents, TraitImplementation::Default);   
    config.set(AppCUITrait::CheckBoxEvents, TraitImplementation::Default);   
    config.set(AppCUITrait::WindowEvents, TraitImplementation::Default);   
    config.set(AppCUITrait::MenuEvents, TraitImplementation::Default);   
    config.set(AppCUITrait::CommandBarEvents, TraitImplementation::Default);   

    parse_token_stream(args, input, "Window", &mut config)
}
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Desktop(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut config = TraitsConfig::new("Desktop");
    // Deref is mandatory
    config.set(AppCUITrait::Deref, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::Control, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::DesktopControl, TraitImplementation::DefaultNonOverwritable);
    // Raw events (implemente by default)
    config.set(AppCUITrait::OnPaint, TraitImplementation::Default);
    config.set(AppCUITrait::OnResize, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::OnFocus, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::OnDefaultAction, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::OnKeyPressed, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnMouseEvents, TraitImplementation::BaseFallbackNonOverwritable);
    // control events
    config.set(AppCUITrait::ButtonEvents, TraitImplementation::DefaultNonOverwritable);   
    config.set(AppCUITrait::CheckBoxEvents, TraitImplementation::DefaultNonOverwritable);   
    config.set(AppCUITrait::WindowEvents, TraitImplementation::DefaultNonOverwritable);   
    config.set(AppCUITrait::MenuEvents, TraitImplementation::DefaultNonOverwritable);   
    config.set(AppCUITrait::CommandBarEvents, TraitImplementation::DefaultNonOverwritable);  
    parse_token_stream(args, input, "Desktop", &mut config)
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
