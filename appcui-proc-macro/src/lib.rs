mod token_stream_to_string;
mod chars;
mod column;
mod key;
mod menu;
mod procmacro_builder;
mod parameter_parser;
mod derives;
mod controls;
mod utils;
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
/// 
/// and the **events** parameter is a list of events that could be received by the new control:
/// * CommandBarEvents
/// * MenuEvents
///
/// If none of the **overwrite** or **events** parameters is present, a default implementation
/// will be provided.
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
    config.set(AppCUITrait::OnSiblingSelected, TraitImplementation::Default);
    config.set(AppCUITrait::OnThemeChanged, TraitImplementation::Default);

    // control events
    config.set(AppCUITrait::ButtonEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::CheckBoxEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::RadioBoxEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::ToggleButtonEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::WindowEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::MenuEvents, TraitImplementation::Default);
    config.set(AppCUITrait::CommandBarEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ToolBarEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::ColorPickerEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::ThreeStateBoxEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::PasswordEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::KeySelectorEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::TextFieldEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::GenericSelectorEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::ComboBoxEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::GenericDropDownListEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::GenericNumericSelectorEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::DatePickerEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::ListBoxEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::GenericListViewEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::PathFinderEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::GenericTreeViewEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::MarkdownEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::GenericBackgroundTaskEvents, TraitImplementation::DefaultNonOverwritable);

    // custom events
    config.set(AppCUITrait::CustomEvents, TraitImplementation::DefaultNonOverwritable);

    // timer events
    config.set(AppCUITrait::TimerEvents, TraitImplementation::Default);


    // desktop
    config.set(AppCUITrait::DesktopEvents, TraitImplementation::DefaultNonOverwritable);

    procmacro_builder::build(args, input, BaseControlType::CustomControl, &mut config)
}

/// Used to create a custom window that can process events from its controls
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
/// impl ButtonEvents for MyWindow { /* ... */ }
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
    config.set(AppCUITrait::OnSiblingSelected, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::OnThemeChanged, TraitImplementation::Default);


    // control events
    config.set(AppCUITrait::ButtonEvents, TraitImplementation::Default);
    config.set(AppCUITrait::CheckBoxEvents, TraitImplementation::Default);
    config.set(AppCUITrait::RadioBoxEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ToggleButtonEvents, TraitImplementation::Default);
    config.set(AppCUITrait::WindowEvents, TraitImplementation::Default);
    config.set(AppCUITrait::MenuEvents, TraitImplementation::Default);
    config.set(AppCUITrait::CommandBarEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ToolBarEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ColorPickerEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ThreeStateBoxEvents, TraitImplementation::Default);
    config.set(AppCUITrait::PasswordEvents, TraitImplementation::Default);
    config.set(AppCUITrait::KeySelectorEvents, TraitImplementation::Default);
    config.set(AppCUITrait::TextFieldEvents, TraitImplementation::Default);
    config.set(AppCUITrait::GenericSelectorEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ComboBoxEvents, TraitImplementation::Default);
    config.set(AppCUITrait::GenericDropDownListEvents, TraitImplementation::Default);
    config.set(AppCUITrait::GenericNumericSelectorEvents, TraitImplementation::Default);
    config.set(AppCUITrait::DatePickerEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ListBoxEvents, TraitImplementation::Default);
    config.set(AppCUITrait::GenericListViewEvents, TraitImplementation::Default);
    config.set(AppCUITrait::PathFinderEvents, TraitImplementation::Default);
    config.set(AppCUITrait::GenericTreeViewEvents, TraitImplementation::Default);
    config.set(AppCUITrait::MarkdownEvents, TraitImplementation::Default);
    config.set(AppCUITrait::GenericBackgroundTaskEvents, TraitImplementation::Default);

    // custom events
    config.set(AppCUITrait::CustomEvents, TraitImplementation::Default);

    // timer events
    config.set(AppCUITrait::TimerEvents, TraitImplementation::Default);

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
    config.set(AppCUITrait::OnSiblingSelected, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::OnThemeChanged, TraitImplementation::Default);

    // control events
    config.set(AppCUITrait::ButtonEvents, TraitImplementation::Default);
    config.set(AppCUITrait::CheckBoxEvents, TraitImplementation::Default);
    config.set(AppCUITrait::RadioBoxEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ToggleButtonEvents, TraitImplementation::Default);
    config.set(AppCUITrait::WindowEvents, TraitImplementation::Default);
    config.set(AppCUITrait::MenuEvents, TraitImplementation::Default);
    config.set(AppCUITrait::CommandBarEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ToolBarEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ColorPickerEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ThreeStateBoxEvents, TraitImplementation::Default);
    config.set(AppCUITrait::PasswordEvents, TraitImplementation::Default);
    config.set(AppCUITrait::KeySelectorEvents, TraitImplementation::Default);
    config.set(AppCUITrait::TextFieldEvents, TraitImplementation::Default);
    config.set(AppCUITrait::GenericSelectorEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ComboBoxEvents, TraitImplementation::Default);
    config.set(AppCUITrait::GenericDropDownListEvents, TraitImplementation::Default);
    config.set(AppCUITrait::GenericNumericSelectorEvents, TraitImplementation::Default);
    config.set(AppCUITrait::DatePickerEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ListBoxEvents, TraitImplementation::Default);
    config.set(AppCUITrait::GenericListViewEvents, TraitImplementation::Default);
    config.set(AppCUITrait::PathFinderEvents, TraitImplementation::Default);
    config.set(AppCUITrait::GenericTreeViewEvents, TraitImplementation::Default);
    config.set(AppCUITrait::MarkdownEvents, TraitImplementation::Default);
    config.set(AppCUITrait::GenericBackgroundTaskEvents, TraitImplementation::Default);



    // custom events
    config.set(AppCUITrait::CustomEvents, TraitImplementation::DefaultNonOverwritable);

    // timer events
    config.set(AppCUITrait::TimerEvents, TraitImplementation::Default);

    // desktop
    config.set(AppCUITrait::DesktopEvents, TraitImplementation::Default);

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
    config.set(AppCUITrait::OnSiblingSelected, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::OnThemeChanged, TraitImplementation::Default);

    // control events
    config.set(AppCUITrait::ButtonEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::CheckBoxEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::RadioBoxEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::ToggleButtonEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::WindowEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::MenuEvents, TraitImplementation::Default);
    config.set(AppCUITrait::CommandBarEvents, TraitImplementation::Default);
    config.set(AppCUITrait::ToolBarEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::ColorPickerEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::ThreeStateBoxEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::PasswordEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::KeySelectorEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::TextFieldEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::GenericSelectorEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::ComboBoxEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::GenericDropDownListEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::GenericNumericSelectorEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::DatePickerEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::ListBoxEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::GenericListViewEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::PathFinderEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::GenericTreeViewEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::MarkdownEvents, TraitImplementation::DefaultNonOverwritable);
    config.set(AppCUITrait::GenericBackgroundTaskEvents, TraitImplementation::DefaultNonOverwritable);

    // custom events
    config.set(AppCUITrait::CustomEvents, TraitImplementation::DefaultNonOverwritable);

    // timer events
    config.set(AppCUITrait::TimerEvents, TraitImplementation::Default);

    // desktop
    config.set(AppCUITrait::DesktopEvents, TraitImplementation::Default);

    procmacro_builder::build(args, input, BaseControlType::Desktop, &mut config)
}


#[proc_macro_derive(ListItem, attributes(Column))]
pub fn listitem_derive(input: TokenStream) -> TokenStream {
    crate::derives::listitem::derive(input)
}

#[proc_macro_derive(EnumSelector, attributes(VariantInfo))]
pub fn enumselector_derive(input: TokenStream) -> TokenStream {
    crate::derives::enumselector::derive(input)
}

#[proc_macro_derive(DropDownListType, attributes(VariantInfo))]
pub fn dropdownlisttype_derive(input: TokenStream) -> TokenStream {
    crate::derives::dropdownlisttype::derive(input)
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
    crate::key::create(input)
}

#[proc_macro]
pub fn char(input: TokenStream) -> TokenStream {
    crate::chars::create(input)
}

#[proc_macro]
pub fn charattr(input: TokenStream) -> TokenStream {
    crate::chars::create_attr(input)
}

#[proc_macro]
pub fn headercolumn(input: TokenStream) -> TokenStream {
    crate::column::create(input)
}


/// Creates a new button control. The format is `button!("attributes")` where the attributes are pairs of key-value , separated by comma, in the format `key=value` or `key:value`. 
/// If the `value` is a string, use single quotes to delimit the value. 
/// The following attributes are supported:
/// * `name` or `caption` or `text` - the text displayed on the button
/// * `type` - the type of the button. The following values are supported:
///   - **Normal** - a normal button
///   - **Flat** - a flat button
/// * position attributes: `x` and  `y`,
/// * size attributes: `width` or `w` (alias)
/// * margin attributes: `left` or `l`(alias), `right` or `r`(alias), `top` or `t`(alias), `bottom` or `b`(alias)
/// * Alignament attributes: 
///   - `align` or `a`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
///   - `dock` or `d`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
/// * State attributes: `enabled`, `visible`
/// 
/// # Example
/// 
/// ```button!("caption='Click me!', type=Flat, x=10, y=10, width=20")```
/// 
/// Alternatively, the first parameter (if the key is not specified) is consider the caption:
/// 
/// ```button!("'Click me!', x:0, y=10, w:20")```
#[proc_macro]
pub fn button(input: TokenStream) -> TokenStream {
    crate::controls::button::create(input)
}

/// Creates a new checkbox control. The format is `checkbox!("attributes")` where the attributes are pairs of key-value , separated by comma, in the format `key=value` or `key:value`.
/// If the `value` is a string, use single quotes to delimit the value.
/// The following attributes are supported:
/// * `caption` or `text` - the text displayed near the checkbox
/// * `checked` or `check` - if the checkbox is checked or not
/// * position attributes: `x` and  `y`,
/// * size attributes: `width` or `w` (alias), `height` or `h` (alias), 
/// * margin attributes: `left` or `l`(alias), `right` or `r`(alias), `top` or `t`(alias), `bottom` or `b`(alias)   
/// * Alignament attributes:
///   - `align` or `a`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
///   - `dock` or `d`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
/// * State attributes: `enabled`, `visible`
/// 
/// # Example
/// 
/// ```checkbox!("caption='Check me!', x=10, y=10, width=20, height=2")```
/// 
/// Alternatively, the first parameter (if the key is not specified) is consider the caption:
/// 
/// ```checkbox!("'Check me!', x:0, y=10, w:20")```
#[proc_macro]
pub fn checkbox(input: TokenStream) -> TokenStream {
    crate::controls::checkbox::create(input)
}

/// Creates a new radiobox control. The format is `radiobox!("attributes")` where the attributes are pairs of key-value , separated by comma, in the format `key=value` or `key:value`.
/// If the `value` is a string, use single quotes to delimit the value.
/// The following attributes are supported:
/// * `caption` or `text` - the text displayed near the radiobox
/// * `selected` or `selec` - if the radiobox is selected or not
/// * position attributes: `x` and  `y`,
/// * size attributes: `width` or `w` (alias), `height` or `h` (alias), 
/// * margin attributes: `left` or `l`(alias), `right` or `r`(alias), `top` or `t`(alias), `bottom` or `b`(alias)   
/// * Alignament attributes:
///   - `align` or `a`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
///   - `dock` or `d`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
/// * State attributes: `enabled`, `visible`
/// 
/// # Example
/// 
/// ```radiobox!("caption='Select me!', x=10, y=10, width=20, height=2")```
/// 
/// Alternatively, the first parameter (if the key is not specified) is consider the caption:
/// 
/// ```radiobox!("'Select me!', x:0, y=10, w:20")```
#[proc_macro]
pub fn radiobox(input: TokenStream) -> TokenStream {
    crate::controls::radiobox::create(input)
}

/// Creates a new label control. The format is `label!("attributes")` where the attributes are pairs of key-value , separated by comma, in the format `key=value` or `key:value`.
/// If the `value` is a string, use single quotes to delimit the value.
/// The following attributes are supported:
/// * `caption` or `text` - the text displayed on the label
/// * position attributes: `x` and  `y`,
/// * size attributes: `width` or `w` (alias), `height` or `h` (alias),
/// * margin attributes: `left` or `l`(alias), `right` or `r`(alias), `top` or `t`(alias), `bottom` or `b`(alias)
/// * Alignament attributes:
///   - `align` or `a`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
///   - `dock` or `d`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
/// * State attributes: `enabled`, `visible`
/// 
/// # Example
/// 
/// ```label!("caption='Hello!', x=10, y=10, width=20, height=2")```
/// 
/// Alternatively, the first parameter (if the key is not specified) is consider the caption:
/// 
/// ```label!("'Hello!', x:0, y=10, w:20")```
#[proc_macro]
pub fn label(input: TokenStream) -> TokenStream {
    crate::controls::label::create(input)
}

/// Creates a new panel control. The format is `panel!("attributes")` where the attributes are pairs of key-value , separated by comma, in the format `key=value` or `key:value`.
/// If the `value` is a string, use single quotes to delimit the value.
/// The following attributes are supported:
/// * `caption` or `tile` or `text` - the text displayed on the panel
/// * `type` - the type of the panel. The following values are supported:
///  - **Border** - a normal panel with a border
///  - **Window** - a panel that looks like a window
///  - **Page** - a panel that looks like a page
///  - **TopBar** - a panel that looks like a top bar
/// * position attributes: `x` and  `y`,
/// * size attributes: `width` or `w` (alias), `height` or `h` (alias),
/// * margin attributes: `left` or `l`(alias), `right` or `r`(alias), `top` or `t`(alias), `bottom` or `b`(alias)
/// * Alignament attributes:
///   - `align` or `a`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
///   - `dock` or `d`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
/// * State attributes: `enabled`, `visible`
/// 
/// # Example
/// 
/// ```panel!("caption='Hello!', x=10, y=10, width=20, height=10")```
/// 
/// Alternatively, the first parameter (if the key is not specified) is consider the caption:
/// 
/// ```panel!("'Hello!', x:0, y=10, w:20, h:10, type=Window")```
#[proc_macro]
pub fn panel(input: TokenStream) -> TokenStream {
    crate::controls::panel::create(input)
}

/// Creates a new password control. The format is `password!("attributes")` where the attributes are pairs of key-value , separated by comma, in the format `key=value` or `key:value`.
/// If the `value` is a string, use single quotes to delimit the value.
/// The following attributes are supported:
/// * `password` or `pass`- the password displayed in the control
/// * position attributes: `x` and  `y`,
/// * size attributes: `width` or `w` (alias), `height` or `h` (alias),
/// * margin attributes: `left` or `l`(alias), `right` or `r`(alias), `top` or `t`(alias), `bottom` or `b`(alias)
/// * Alignament attributes:
///   - `align` or `a`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
///   - `dock` or `d`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
/// * State attributes: `enabled`, `visible`
/// 
/// # Example
///
/// ```password!("password='1234', x=10, y=10, width=20")```
#[proc_macro]
pub fn password(input: TokenStream) -> TokenStream {
    crate::controls::password::create(input)
}

#[proc_macro]
pub fn window(input: TokenStream) -> TokenStream {
    crate::controls::window::create(input)
}

#[proc_macro]
pub fn toolbaritem(input: TokenStream) -> TokenStream {
    crate::controls::toolbaritem::create(input)
}

/// Creates a new colorpicker control. The format is `colorpicker!("attributes")` where the attributes are pairs of key-value , separated by comma, in the format `key=value` or `key:value`.
/// If the `value` is a string, use single quotes to delimit the value.
/// The following attributes are supported:
/// * `color` - the color selected by the colorpicker. Could be one of the following: **Black**, **DakrBlue**, **DarkGreen**, **Teal**, **DarkRed**, **Magenta**, **Olive**, **Gray**, **Silver**, **Blue**, **Green**, **Aqua**, **Red**, **Pink**, **Yellow**, **White** or **Transparent**
/// * position attributes: `x` and  `y`,
/// * size attributes: `width` or `w` (alias),
/// * margin attributes: `left` or `l`(alias), `right` or `r`(alias), `top` or `t`(alias), `bottom` or `b`(alias)
/// * Alignament attributes:
///   - `align` or `a`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
///   - `dock` or `d`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
/// * State attributes: `enabled`, `visible`
/// 
/// # Example
///
/// ```colorpicker!("color=Red, x=10, y=10, width=20")```
/// 
/// Alternatively, the first parameter (if the key is not specified) is consider the color:
/// 
/// ```colorpicker!("Red, x:0, y=10, w:20")```
#[proc_macro]
pub fn colorpicker(input: TokenStream) -> TokenStream {
    crate::controls::colorpicker::create(input)
}

/// Creates a new three-state box control. The format is `threestatebox!("attributes")` where the attributes are pairs of key-value , separated by comma, in the format `key=value` or `key:value`.
/// If the `value` is a string, use single quotes to delimit the value.
/// The following attributes are supported:
/// * `caption` or `text` - the text displayed near the threestatebox
/// * `state` - the state of the threestatebox. The following values are supported:
///   - **Checked** - the threestatebox is checked
///   - **Unchecked** - the threestatebox is unchecked
///   - **Unknown** - the threestatebox is in indeterminate state
/// * position attributes: `x` and  `y`,
/// * size attributes: `width` or `w` (alias), `height` or `h` (alias),
/// * margin attributes: `left` or `l`(alias), `right` or `r`(alias), `top` or `t`(alias), `bottom` or `b`(alias)
/// * Alignament attributes:
///   - `align` or `a`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
///   - `dock` or `d`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
/// * State attributes: `enabled`, `visible`
/// 
/// # Example
/// 
/// ```threestatebox!("caption='Check me!', x=10, y=10, width=20, height=2")```
/// 
/// Alternatively, the first parameter (if the key is not specified) is consider the caption:
/// 
/// ```threestatebox!("'Check me!', x:0, y=10, w:20")```
#[proc_macro]
pub fn threestatebox(input: TokenStream) -> TokenStream {
    crate::controls::threestatebox::create(input)
}

#[proc_macro]
pub fn canvas(input: TokenStream) -> TokenStream {
    crate::controls::canvas::create(input)
}

#[proc_macro]
pub fn imageviewer(input: TokenStream) -> TokenStream {
    crate::controls::imageviewer::create(input)
}

#[proc_macro]
pub fn tab(input: TokenStream) -> TokenStream {
    crate::controls::tab::create(input)
}

#[proc_macro]
pub fn accordion(input: TokenStream) -> TokenStream {
    crate::controls::accordion::create(input)
}

/// Creates a new keyselector control. The format is `keyselector!("attributes")` where the attributes are pairs of key-value , separated by comma, in the format `key=value` or `key:value`.
/// If the `value` is a string, use single quotes to delimit the value.
/// The following attributes are supported:
/// * `key` - the key selected by the keyselector. The key should be a valid key (see the `key!` macro)
/// * `flags` - the flags of the keyselector. The following values are supported:
///   - **AcceptEnter** - the keyselector will process the Enter key
///   - **AcceptEscape** - the keyselector will process the Escape key
///   - **AcceptTab** - the keyselector will process the Tab key
///   - **ReadOnly** - the keyselector is read-only
/// * position attributes: `x` and  `y`,
/// * size attributes: `width` or `w` (alias), 
/// * margin attributes: `left` or `l`(alias), `right` or `r`(alias), `top` or `t`(alias), `bottom` or `b`(alias)
/// * Alignament attributes:
///   - `align` or `a`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
///   - `dock` or `d`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
/// * State attributes: `enabled`, `visible`
/// 
/// # Example
/// 
/// ```keyselector!("key='F2', x=10, y=10, width=20")```
/// 
/// Alternatively, the first parameter (if the key is not specified) is consider the key:
/// 
/// ```keyselector!("'Ctrl+Alt+F2', x:0, y=10, w:20")```
#[proc_macro]
pub fn keyselector(input: TokenStream) -> TokenStream {
    crate::controls::keyselector::create(input)
}

/// Creates a new textfield control. The format is `textfield!("attributes")` where the attributes are pairs of key-value , separated by comma, in the format `key=value` or `key:value`.
/// If the `value` is a string, use single quotes to delimit the value.
/// The following attributes are supported:
/// * `text` - the text displayed in the textfield
/// * `flags` - the flags of the textfield. The following values are supported:
///   - **ProcessEnter** - the textfield will process the Enter key
///   - **ReadOnly** - the textfield is read-only
///   - **DisableAutoSelectOnFocus** - the text will not be selected when the textfield receives the focus
/// * position attributes: `x` and  `y`,
/// * size attributes: `width` or `w` (alias), `height` or `h` (alias),
/// * margin attributes: `left` or `l`(alias), `right` or `r`(alias), `top` or `t`(alias), `bottom` or `b`(alias)
/// * Alignament attributes:
///   - `align` or `a`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
///   - `dock` or `d`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
/// * State attributes: `enabled`, `visible`
/// 
/// # Example
/// 
/// ```textfield!("text='Hello!', x=10, y=10, width=20, height=2")```
/// 
/// Alternatively, the first parameter (if the key is not specified) is consider the text:
/// 
/// ```textfield!("'Hello!', x:0, y=10, w:20")```
#[proc_macro]
pub fn textfield(input: TokenStream) -> TokenStream {
    crate::controls::textfield::create(input)
}


#[proc_macro]
pub fn selector(input: TokenStream) -> TokenStream {
    crate::controls::selector::create(input)
}

#[proc_macro]
pub fn combobox(input: TokenStream) -> TokenStream {
    crate::controls::combobox::create(input)
}

#[proc_macro]
pub fn dropdownlist(input: TokenStream) -> TokenStream {
    crate::controls::dropdownlist::create(input)
}

/// Creates a new listbox control. The format is `listbox!("attributes")` where the attributes are pairs of key-value, separated by comma, in the format `key=value` or `key:value`.
/// If the `value` is a string, use single quotes to delimit the value.
/// The following attributes are supported:
/// * `flags` - The flags of the listbox. The following values are supported:
///   - **ScrollBars** - Adds scrollbars to the listbox
///   - **SearchBar** - Adds a search bar for filtering items
///   - **CheckBoxes** - Adds checkboxes for multiple selection
///   - **AutoScroll** - Automatically scrolls to newly added items
///   - **HighlightSelectedItemWhenInactive** - Highlights selected item even when inactive
/// * `items` - A list of strings to populate the listbox with. Format: `['item1', 'item2', ...]`
/// * `index` or `selected_index` - The index of the initially selected item (0-based)
/// * `lsm` or `left-scroll-margin` - Left scroll margin in characters
/// * `tsm` or `top-scroll-margin` - Top scroll margin in characters
/// * `em` or `empty-message` - Message to display when the listbox is empty
/// * Position and size:
///   - `x`, `y` - Position coordinates
///   - `width`/`w`, `height`/`h` - Control dimensions
/// * Layout:
///   - `align`/`a` - Alignment: Left, Right, Top, Bottom, Center, TopLeft, TopRight, BottomLeft, BottomRight
///   - `dock`/`d` - Docking: Left, Right, Top, Bottom, Center, TopLeft, TopRight, BottomLeft, BottomRight
/// * Margins: `left`/`l`, `right`/`r`, `top`/`t`, `bottom`/`b`
/// * State: `enabled`, `visible`
/// 
/// # Examples
/// ```rust,compile_fail
/// use appcui::prelude::*;
/// 
/// // Basic listbox with items
/// let lb = listbox!("items=['Red', 'Green', 'Blue'], x=1, y=1, width=20, height=10");
/// 
/// // Listbox with scrollbars and search
/// let lb = listbox!("flags: ScrollBars+SearchBar, x=0, y=0, width=30, height=15");
/// 
/// // Listbox with checkboxes and initial selection
/// let lb = listbox!("flags: CheckBoxes, items=['Option 1', 'Option 2'], index: 1, x=2, y=2, width=25, height=8");
/// ```
#[proc_macro]
pub fn listbox(input: TokenStream) -> TokenStream {
    crate::controls::listbox::create(input)
}

#[proc_macro]
pub fn numericselector(input: TokenStream) -> TokenStream {
    crate::controls::numericselector::create(input)
}

#[proc_macro]
pub fn menuitem(input: TokenStream) -> TokenStream {
    crate::menu::menuitem::create(input, None)
}

#[proc_macro]
pub fn menu(input: TokenStream) -> TokenStream {
    crate::menu::menu::create(input, None)
}

/// Creates a new horizontal line control.
/// The format is `hline!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `text` or `caption` (optional, first positional parameter) - Text to display if HasTitle flag is set
/// * `flags` - Line initialization flags (optional). Can be:
///   - **DoubleLine** - Uses double line characters instead of single
///   - **HasTitle** - Shows the text/caption in the middle of the line
/// * Position and size:
///   - `x`, `y` - Position coordinates
///   - `width`/`w` - Width of the line (required)
/// * Layout:
///   - `align`/`a` - Alignment: Left, Right, Top, Bottom, Center, etc.
///   - `dock`/`d` - Docking: Left, Right, Top, Bottom, Center, etc.
/// * Margins: `left`/`l`, `right`/`r`, `top`/`t`, `bottom`/`b`
/// * State: `enabled`, `visible`
/// 
/// # Examples
/// ```rust,compile_fail
/// use appcui::prelude::*;
/// 
/// // Simple line
/// let line = hline!("x=0, y=0, width=40");
/// 
/// // Double line with title
/// let line = hline!("'Section Title', flags: [DoubleLine,HasTitle], width=40");
/// ```
#[proc_macro]
pub fn hline(input: TokenStream) -> TokenStream {
    crate::controls::hline::create(input)
}

/// Creates a new vertical line control.
/// The format is `vline!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `flags` - Line initialization flags (optional). Can be:
///   - **DoubleLine** - Uses double line characters instead of single
/// * Position and size:
///   - `x`, `y` - Position coordinates
///   - `height`/`h` - Height of the line (required)
/// * Layout:
///   - `align`/`a` - Alignment: Left, Right, Top, Bottom, Center, etc.
///   - `dock`/`d` - Docking: Left, Right, Top, Bottom, Center, etc.
/// * Margins: `left`/`l`, `right`/`r`, `top`/`t`, `bottom`/`b`
/// * State: `enabled`, `visible`
/// 
/// # Examples
/// ```rust,compile_fail
/// use appcui::prelude::*;
/// 
/// // Simple line
/// let line = vline!("x=0, y=0, height=20");
/// 
/// // Double line
/// let line = vline!("flags: DoubleLine, height=20, dock: left");
/// ```
#[proc_macro]
pub fn vline(input: TokenStream) -> TokenStream {
    crate::controls::vline::create(input)
}

/// Creates a new vertical splitter control for resizing two vertical panes.
/// The format is `vsplitter!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `pos` (optional, first positional parameter) - Initial position of the splitter
/// * `resize`, `resize-behavior`, `on-resize`, `rb` - Resize behavior (optional). Can be:
///   - **PreserveAspectRatio** (default) - Maintains relative sizes when parent resizes
///   - **PreserveLeftPanelSize** - Keeps left panel size fixed
///   - **PreserveRightPanelSize** - Keeps right panel size fixed
/// * `min-left-width`, `mintopwidth`, `mlw` - Minimum width for the left panel
/// * `min-right-width`, `minbottomwidth`, `mrw` - Minimum width for the right panel
/// * Position and size:
///   - `x`, `y` - Position coordinates
///   - `height`/`h` - Height of the splitter (required)
/// * Layout:
///   - `align`/`a` - Alignment: Left, Right, Top, Bottom, Center, etc.
///   - `dock`/`d` - Docking: Left, Right, Top, Bottom, Center, etc.
/// * Margins: `left`/`l`, `right`/`r`, `top`/`t`, `bottom`/`b`
/// * State: `enabled`, `visible`
/// 
/// # Examples
/// ```rust,compile_fail
/// use appcui::prelude::*;
/// 
/// // Simple splitter
/// let split = vsplitter!("width=40, height=20");
/// 
/// // Advanced configuration
/// let split = vsplitter!(
///     "x=0, y=0, height=20, width=40,
///     resize: PreserveLeftPanelSize, 
///     minleftwidth: 30, 
///     minrightwidth: 40"
/// );
/// ```
#[proc_macro]
pub fn vsplitter(input: TokenStream) -> TokenStream {
    crate::controls::vsplitter::create(input)
}

/// Creates a new horizontal splitter control for resizing two horizontal panes.
/// The format is `hsplitter!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `pos` (optional, first positional parameter) - Initial position of the splitter
/// * `resize`, `resize-behavior`, `on-resize`, `rb` - Resize behavior (optional). Can be:
///   - **PreserveAspectRatio** (default) - Maintains relative sizes when parent resizes
///   - **PreserveTopPanelSize** - Keeps top panel size fixed
///   - **PreserveBottomPanelSize** - Keeps bottom panel size fixed
/// * `min-top-height`, `mintopheight`, `mth` - Minimum height for the top panel
/// * `min-bottom-height`, `minbottomheight`, `mbh` - Minimum height for the bottom panel
/// * Position and size:
///   - `x`, `y` - Position coordinates
///   - `width`/`w` - Width of the splitter (required)
/// * Layout:
///   - `align`/`a` - Alignment: Left, Right, Top, Bottom, Center, etc.
///   - `dock`/`d` - Docking: Left, Right, Top, Bottom, Center, etc.
/// * Margins: `left`/`l`, `right`/`r`, `top`/`t`, `bottom`/`b`
/// * State: `enabled`, `visible`
/// 
/// # Examples
/// ```rust,compile_fail
/// use appcui::prelude::*;
/// 
/// // Simple splitter
/// let split = hsplitter!("x:0, y:0, width=40, height=20");
/// 
/// // Advanced configuration
/// let split = hsplitter!(
///     "x=0, y=0, width=40, height=20,
///     resize: PreserveTopPanelSize, 
///     mintopheight: 10, 
///     minbottomheight: 15"
/// );
/// ```
#[proc_macro]
pub fn hsplitter(input: TokenStream) -> TokenStream {
    crate::controls::hsplitter::create(input)
}

/// Creates a new DatePicker control for selecting dates.
/// The format is `datepicker!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `date` (optional, first positional parameter) - Initial date in YYYY-MM-DD format or any format supported by NaiveDate
/// * Position and size:
///   - `x`, `y` - Position coordinates
///   - `width`/`w`, `height`/`h` - Control dimensions
/// * Layout:
///   - `align`/`a` - Alignment: Left, Right, Top, Bottom, Center, etc.
///   - `dock`/`d` - Docking: Left, Right, Top, Bottom, Center, etc.
/// * Margins: `left`/`l`, `right`/`r`, `top`/`t`, `bottom`/`b`
/// * State: `enabled`, `visible`
/// 
/// # Examples
/// ```rust,compile_fail
/// use appcui::prelude::*;
/// 
/// // With explicit date
/// let dp = datepicker!("2024-06-13, x=1, y=1, width=19, height=1");
/// 
/// // With named parameter and layout
/// let dp = datepicker!("date: 2024-06-13, dock: center, width: 19, margin: 1");
/// ```
#[proc_macro]
pub fn datepicker(input: TokenStream) -> TokenStream {
    crate::controls::datepicker::create(input)
}

/// Creates a new ListView control for displaying a list of items of type T.
/// The format is `listview!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `type` or `class` (required, first positional parameter) - The type T of items to display
/// * `flags` - ListView initialization flags (optional). Can be:
///   - **ScrollBars** - Shows scroll bars
///   - **SearchBar** - Enables search functionality
///   - **CheckBoxes** - Adds checkboxes to items
///   - **ShowGroups** - Enables item grouping
///   - **SmallIcons** - Uses small icons
///   - **LargeIcons** - Uses large icons
///   - **CustomFilter** - Enables custom filtering
///   - **NoSelection** - Disables item selection
/// * `view` or `viewmode` or `vm` - View mode (optional). Can be:
///   - **Details** - Shows items in details view with columns
///   - **Columns(N)** - Shows items in N columns (N from 1 to 10)
/// * `columns` - Column definitions for details view (optional). Format: [{Name,Width,Align},...] 
/// * `lsm` or `left-scroll-margin` - Left scroll margin in characters (optional)
/// * `tsm` or `top-scroll-margin` - Top scroll margin in characters (optional)
/// * Layout parameters: x, y, width/w, height/h, align/a, dock/d, etc.
/// 
/// # Examples
/// ```rust,compile_fail
/// use appcui::prelude::*;
/// 
/// // Basic usage
/// let lv = listview!("type: MyType, flags: ScrollBars, x=0, y=0, width=40, height=20");
/// 
/// // With columns in details view
/// let lv = listview!(
///     "MyType, 
///     view: Details, 
///     columns: [{Name,10,left}, {Age,5,right}], 
///     x=1, y=1, width=50, height=25"
/// );
/// 
/// // Multi-column view
/// let lv = listview!("class: MyType, view: Columns(3), x=2, y=2, width=60, height=30");
/// ```
/// 
/// The type T must implement the `ListItem` trait. For columns, use the `#[Column]` attribute 
/// on struct fields to define how they should be displayed.
#[proc_macro]
pub fn listview(input: TokenStream) -> TokenStream {
    crate::controls::listview::create(input)
}

#[proc_macro]
pub fn togglebutton(input: TokenStream) -> TokenStream {
    crate::controls::togglebutton::create(input)
}

/// Creates a new pathfinder control. The format is `pathfinder!("attributes")` where the attributes are pairs of key-value , separated by comma, in the format `key=value` or `key:value`.
/// If the `value` is a string, use single quotes to delimit the value.
/// The following attributes are supported:
/// * `path` - the path displayed in the pathfinder
/// * `flags` - the flags of the pathfinder. The following values are supported:
///   - **ReadOnly** - the pathfinder is read-only
///   - **CaseSensitive** - the pathfinder is case-sensitive
/// * position attributes: `x` and  `y`,
/// * size attributes: `width` or `w` (alias),
/// * margin attributes: `left` or `l`(alias), `right` or `r`(alias), `top` or `t`(alias), `bottom` or `b`(alias)   
/// * Alignament attributes:
///   - `align` or `a`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
///   - `dock` or `d`(alias) - one of **Left**, **Right**, **Top**, **Bottom**, **Center**, **TopLeft**, **TopRight**, **BottomLeft**, **BottomRight**
/// * State attributes: `enabled`, `visible`
/// 
/// # Example
/// 
/// ```pathfinder!("path='C:\\', x=10, y=10, width=20")```
/// 
/// Alternatively, the first parameter (if the key is not specified) is consider the path:
/// 
/// ```pathfinder!("'C:\\Windows\\', x:0, y=10, w:20")```
#[proc_macro]
pub fn pathfinder(input: TokenStream) -> TokenStream {
    crate::controls::pathfinder::create(input)
}

#[proc_macro]
pub fn treeview(input: TokenStream) -> TokenStream {
    crate::controls::treeview::create(input)
}

#[proc_macro]
pub fn markdown(input: TokenStream) -> TokenStream {
    crate::controls::markdown::create(input)
}

#[proc_macro]
pub fn progressbar(input: TokenStream) -> TokenStream {
    crate::controls::progressbar::create(input)
}