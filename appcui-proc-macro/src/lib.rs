
mod key_utils;
mod procmacro_builder;
mod parameter_parser;
mod controls;
use proc_macro::*;

use procmacro_builder::{AppCUITrait, TraitImplementation, TraitsConfig, BaseControlType};



extern crate proc_macro;





/// Used to create a custom control
/// The general format is: `#[CustomControl(overwrite = ..., events= ...)]`
/// Where the **overwrite** parameter is a list of traits that can be overwritten that include:
/// * OnPaint
/// * OnKeyPressed
/// * OnMouseEvents
/// * OnDefaultAction
/// * OnResize
/// * OnFocus
///and the **events** parameter is a list of events that could be received by the new control:
/// * CommandBarEvents
/// * MenuEvents
///
/// None of the **overwrite** or **events** parameters should be present. If not present, a
/// default implementation will be provided.
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
    config.set(AppCUITrait::OnWindowRegistered, TraitImplementation::DefaultNonOverwritable);
    // Raw events (implemente by default)
    config.set(AppCUITrait::OnPaint, TraitImplementation::Default);
    config.set(AppCUITrait::OnResize, TraitImplementation::Default);
    config.set(AppCUITrait::OnFocus, TraitImplementation::Default);
    config.set(AppCUITrait::OnExpand, TraitImplementation::Default);
    config.set(AppCUITrait::OnDefaultAction, TraitImplementation::Default);
    config.set(AppCUITrait::OnKeyPressed, TraitImplementation::Default);
    config.set(AppCUITrait::OnMouseEvent, TraitImplementation::Default);
    // control events
    config.set(AppCUITrait::ButtonEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::CheckBoxEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::WindowEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::MenuEvents, TraitImplementation::Default);
    config.set(AppCUITrait::CommandBarEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ToolBarEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::ColorPickerEvents, TraitImplementation::DefaultNonOverwritable);
    // desktop
    config.set(AppCUITrait::DesktopEvents, TraitImplementation::DefaultNonOverwritable);

    procmacro_builder::build(args, input, BaseControlType::CustomControl, &mut config)
}

/// Used to acustom desktop
/// The general format is: `#[Window(events = ...)]`
/// Where the **events** parameter is a list of traits that can be overwritten:
/// * WindowEvents
/// * ButtonEvents
/// * CheckBoxEvents
/// * CommandBarEvents
/// * MenuEvents
/// * ToolBarEvents
///
/// If not overwritten, a default implementation will be automatically added
///
/// # Example
/// ```rust,compile_fail
/// use appcui::prelude::*;
///
/// #[Window(events = ButtonEvens+WindowEvents)]
/// struct MyWindow {
///     // custom data
/// }
/// impl MyWindow { /* specific methods */}
/// impl ButtonEvens for MyWindow { /* ... */ }
/// impl WindowEvents for MyWindow { /* ... */ }
/// ```
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Window(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut config = TraitsConfig::new("Window");
    // Deref is mandatory
    config.set(AppCUITrait::Deref, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::Control, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::WindowControl, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::NotModalWindow, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::OnWindowRegistered, TraitImplementation::BaseFallbackNonOverwritable);
    // Raw events (implemente by default)
    config.set(AppCUITrait::OnPaint, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnResize, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnFocus, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnExpand, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::OnDefaultAction, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnKeyPressed, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnMouseEvent, TraitImplementation::BaseFallbackNonOverwritable);
    // control events
    config.set(AppCUITrait::ButtonEvents, TraitImplementation::Default);
    config.set(AppCUITrait::CheckBoxEvents, TraitImplementation::Default);
    config.set(AppCUITrait::WindowEvents, TraitImplementation::Default);
    config.set(AppCUITrait::MenuEvents, TraitImplementation::Default);
    config.set(AppCUITrait::CommandBarEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ToolBarEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ColorPickerEvents, TraitImplementation::Default);

    // desktop
    config.set(AppCUITrait::DesktopEvents, TraitImplementation::DefaultNonOverwritable);

    procmacro_builder::build(args, input, BaseControlType::Window, &mut config)
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn ModalWindow(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut config = TraitsConfig::new("ModalWindow");
    // Deref is mandatory
    config.set(AppCUITrait::Deref, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::Control, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::WindowControl, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::OnWindowRegistered, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::ModalWindowMethods, TraitImplementation::BaseFallbackNonOverwritable);
    // Raw events (implemente by default)
    config.set(AppCUITrait::OnPaint, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnResize, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnFocus, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnExpand, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::OnDefaultAction, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnKeyPressed, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnMouseEvent, TraitImplementation::BaseFallbackNonOverwritable);
    // control events
    config.set(AppCUITrait::ButtonEvents, TraitImplementation::Default);
    config.set(AppCUITrait::CheckBoxEvents, TraitImplementation::Default);
    config.set(AppCUITrait::WindowEvents, TraitImplementation::Default);
    config.set(AppCUITrait::MenuEvents, TraitImplementation::Default);
    config.set(AppCUITrait::CommandBarEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ToolBarEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ColorPickerEvents, TraitImplementation::Default);

    // desktop
    config.set(AppCUITrait::DesktopEvents, TraitImplementation::DefaultNonOverwritable);

    procmacro_builder::build(args, input, BaseControlType::ModalWindow, &mut config)
}


/// Used to create window and intercepts/process events from children controls.
/// The general format is: `#[Desktop(overwrite = ..., events= ...)]`
/// Where the **overwrite** parameter is a list of traits that can be overwritten that include:
/// * OnPaint
/// * OnResize
/// 
///and the **events** parameter is a list of events that could be received by the new control:
/// * CommandBarEvents
/// * MenuEvents
/// * DesktopEvents
///
/// If not overwritten, a default implementation will be automatically added
///
/// # Example
/// ```rust,compile_fail
/// use appcui::prelude::*;
///
/// #[Desktop(overwrite = OnPaint, events = DesktopEvents)]
/// struct MyDesktop {
///     // custom data
/// }
/// impl MyDesktop { /* specific methods */}
/// impl OnPaint for MyDesktop { /* ... */ }
/// impl DesktopEvents for MyDesktop { /* ... */ }
/// ```
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Desktop(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut config = TraitsConfig::new("Desktop");
    // Deref is mandatory
    config.set(AppCUITrait::Deref, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::Control, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::DesktopControl, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::OnWindowRegistered, TraitImplementation::DefaultNonOverwritable);
    // Raw events (implemente by default)
    config.set(AppCUITrait::OnPaint, TraitImplementation::BaseFallback);
    config.set(AppCUITrait::OnResize, TraitImplementation::Default);
    config.set(AppCUITrait::OnFocus, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::OnExpand, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::OnDefaultAction, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::OnKeyPressed, TraitImplementation::BaseFallbackNonOverwritable);
    config.set(AppCUITrait::OnMouseEvent, TraitImplementation::BaseFallbackNonOverwritable);
    // control events
    config.set(AppCUITrait::ButtonEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::CheckBoxEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::WindowEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::MenuEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::CommandBarEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ToolBarEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::ColorPickerEvents, TraitImplementation::DefaultNonOverwritable);


    // desktop
    config.set(AppCUITrait::DesktopEvents, TraitImplementation::Default);

    procmacro_builder::build(args, input, BaseControlType::Desktop, &mut config)
}

/// Use to quickly identify a key or a combination via a string
/// Usage examples:
/// * key!("F2")
/// * key!("Enter")
/// * key!("Alt+F4")
/// * key!("Ctrl+Alt+F")
/// * key!("Ctrl+Shift+Alt+Tab")
///
/// The list of all keys supported by this macro is:
/// * F-commands (`F1` to `F12`)
/// * Letters (`A` to `Z`) - with apper case
/// * Numbers (`0` to `9`)
/// * Arrows (`Up`, `Down`, `Left`, `Right`)
/// * Navigation keys (`PageUp`, `PageDown`, `Home`, `End`)
/// * Deletion and Insertions (`Delete` , `Backspace`, `Insert`)
/// * White-spaces (`Space`, `Tab`)
/// * Other (`Enter`, `Escape`)
///
/// The supported modifiers are:
/// * Shift
/// * Ctrl
/// * Alt
///
/// Modifiers can be used in combination with the simple `+` between them.
#[proc_macro]
pub fn key(input: TokenStream) -> TokenStream {
    key_utils::process_key_macro_tokens(input)
}


#[proc_macro]
pub fn button(input: TokenStream) -> TokenStream {
    crate::controls::button::create(input)
}

#[proc_macro]
pub fn checkbox(input: TokenStream) -> TokenStream {
    crate::controls::checkbox::create(input)
}

#[proc_macro]
pub fn label(input: TokenStream) -> TokenStream {
    crate::controls::label::create(input)
}

#[proc_macro]
pub fn panel(input: TokenStream) -> TokenStream {
    crate::controls::panel::create(input)
}

#[proc_macro]
pub fn window(input: TokenStream) -> TokenStream {
    crate::controls::window::create(input)
}

#[proc_macro]
pub fn toolbaritem(input: TokenStream) -> TokenStream {
    crate::controls::toolbaritem::create(input)
}
