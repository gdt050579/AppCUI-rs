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
/// * `type` - the type of the checkbox. The following values are supported:
///   - **Standard** - a standard checkbox (`[✓]` or `[ ]`)
///   - **Ascii** - an ascii checkbox (`[X]` or `[ ]`)
///   - **CheckBox** - a checkbox with a check symbol (`☑` or `☐`)
///   - **CheckMark** - a checkbox with a check mark (`✔` or `x`)
///   - **FilledBox** - a checkbox with a filled box (`▣` or `▢`)
///   - **YesNo** - a checkbox with a yes or no symbol (`[Y]` or `[N]`)
///   - **PlusMinus** - a checkbox with a plus or minus symbol (`➕` or `➖`)
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

/// Creates a new canvas control for custom drawing operations.
/// The format is `canvas!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `size` or `sz` or `surface` (required, first positional parameter) - Size of the canvas. Can be specified in two formats:
///   - `width,height` - Using comma as separator (e.g. `40,20`)
///   - `width x height` - Using 'x' as separator (e.g. `40x20`)
///     Values must be positive integers between 1 and 32000
/// * `flags` - Control flags (optional). Can be:
///   - **ScrollBars** - Shows scroll bars when content exceeds the canvas size
/// * `background` or `back` - Background character and attributes (optional). Format: `{char,color,background_color}`
/// * `left-scroll-margin` or `lsm` - Left scroll margin in characters (optional)
/// * `top-scroll-margin` or `tsm` - Top scroll margin in characters (optional)
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
/// // Basic canvas with size using comma
/// let canvas = canvas!("'40,20', x=1, y=1");
/// 
/// // Basic canvas with size using 'x' format
/// let canvas = canvas!("40x20, x=1, y=1");
/// 
/// // Canvas with scrollbars and background
/// let canvas = canvas!(
///     "size: 50x25,
///     flags: ScrollBars,
///     back: {' ', White, Blue},
///     lsm: 2,
///     tsm: 1,
///     x=2, y=2"
/// );
/// 
/// // Canvas with custom background
/// let canvas = canvas!(
///     "'30,15',
///     back: {'#', Yellow, DarkBlue},
///     x=3, y=3"
/// );
/// ```
#[proc_macro]
pub fn canvas(input: TokenStream) -> TokenStream {
    crate::controls::canvas::create(input)
}

/// Creates a new image viewer control for displaying images.
/// The format is `imageviewer!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `image` - String representation of the image to display (optional). The string should be formatted as follows:
///   - Each line represents a row of pixels
///   - Each character represents a pixel with its color
///   - Colors are represented by characters (e.g., 'R' for red, 'G' for green, 'B' for blue, 'Y' for yellow)
///   - Use ' ' (space) for transparent pixels
///   - Use '\n' for new lines
/// * `flags` - Control flags (optional). Can be:
///   - **ScrollBars** - Shows scroll bars when image exceeds the viewer size
/// * `render` or `rendermethod` or `rm` - Image rendering method (optional, defaults to SmallBlocks). Can be:
///   - **SmallBlocks** - Uses small blocks for rendering
///   - **LargeBlocks64Colors** - Uses large blocks with 64 colors
///   - **GrayScale** - Renders in grayscale
///   - **AsciiArt** - Renders as ASCII art
/// * `scale` - Image scale percentage (optional, defaults to 100%). Supported values:
///   - 100% (NoScale)
///   - 50% (Scale50)
///   - 33% (Scale33)
///   - 25% (Scale25)
///   - 20% (Scale20)
///   - 10% (Scale10)
///   - 5% (Scale5)
/// * `background` or `back` - Background character and attributes (optional). Format: `{char,color,background_color}`
/// * `left-scroll-margin` or `lsm` - Left scroll margin in characters (optional)
/// * `top-scroll-margin` or `tsm` - Top scroll margin in characters (optional)
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
/// // Basic image viewer with a smiley face
/// let viewer = imageviewer!(
///     "image: '|  YYYYY  |\n| Y     Y |\n|Y  Y Y  Y|\n|Y       Y|\n|Y  YYY  Y|\n| Y     Y |\n|  YYYYY  |',
///     x=1, y=1, width=40, height=20"
/// );
/// 
/// // Image viewer with custom rendering and background
/// let viewer = imageviewer!(
///     "image: '|  YYYYY  |\n| Y     Y |\n|Y  Y Y  Y|\n|Y       Y|\n|Y  YYY  Y|\n| Y     Y |\n|  YYYYY  |',
///     render: GrayScale,
///     back: {' ', White, Blue},
///     lsm: 2,
///     tsm: 1,
///     x=2, y=2, width=50, height=25"
/// );
/// ```
#[proc_macro]
pub fn imageviewer(input: TokenStream) -> TokenStream {
    crate::controls::imageviewer::create(input)
}

/// Creates a new tab control for organizing content into multiple pages.
/// The format is `tab!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `tabs` - List of tab captions. Format: `[Tab1, Tab2, ...]`
/// * `type` - Tab type (optional). Can be:
///   - **OnTop** (default) - Tabs are displayed at the top
///   - **OnBottom** - Tabs are displayed at the bottom
///   - **OnLeft** - Tabs are displayed on the left side
///   - **HiddenTabs** - Tabs are hidden
/// * `flags` - Control flags (optional). Can be:
///   - **TransparentBackground** - Uses transparent background
///   - **TabsBar** - Shows a bar for tabs
/// * `tabwidth` or `tab-width` or `tw` - Width of each tab (optional)
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
/// // Basic tab with top tabs
/// let tab = tab!("tabs=['Tab 1', 'Tab 2'], x=1, y=1, width=40, height=20");
/// 
/// // Tab with bottom tabs and transparent background
/// let tab = tab!(
///     "tabs: ['First', 'Second', 'Third'],
///     type: OnBottom,
///     flags: TransparentBackground,
///     x=2, y=2, width=50, height=25"
/// );
/// 
/// // Tab with left tabs and custom width
/// let tab = tab!(
///     "tabs=['Settings', 'Help'],
///     type: OnLeft,
///     tabwidth: 15,
///     x=3, y=3, width=60, height=30"
/// );
/// ```
/// 
/// The caption of each tab may contain the special character `&` that indicates that the next character is a hot-key.
/// For example, constructing a tab with the caption `&Start` will set up the text of the tab to `Start` and will set up character `S` as the hot key to activate that tab.
#[proc_macro]
pub fn tab(input: TokenStream) -> TokenStream {
    crate::controls::tab::create(input)
}

/// Creates a new accordion control for displaying collapsible content sections.
/// The format is `accordion!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `panels` (required) - List of panel captions. Format: `[Panel1, Panel2, ...]`
/// * `flags` - Control flags (optional). Can be:
///   - **TransparentBackground** - Uses transparent background
/// * Position and size:
///   - `x`, `y` - Position coordinates
///   - `width`/`w`, `height`/`h` - Control dimensions
/// * Layout:
///   - `align`/`a` - Alignment value (optional). Can be:
///     - **topleft**/**lefttop**/**tl**/**lt** - Aligns from top-left corner
///     - **top**/**t** - Aligns from top center
///     - **topright**/**righttop**/**tr**/**rt** - Aligns from top-right corner
///     - **right**/**r** - Aligns from right center
///     - **bottomright**/**rightbottom**/**br**/**rb** - Aligns from bottom-right corner
///     - **bottom**/**b** - Aligns from bottom center
///     - **bottomleft**/**leftbottom**/**lb**/**bl** - Aligns from bottom-left corner
///     - **left**/**l** - Aligns from left center
///     - **center**/**c** - Aligns from center
///   - `dock`/`d` - Docking value (optional). Can be:
///     - **topleft**/**lefttop**/**tl**/**lt** - Docks to top-left corner
///     - **top**/**t** - Docks to top
///     - **topright**/**righttop**/**tr**/**rt** - Docks to top-right corner
///     - **right**/**r** - Docks to right
///     - **bottomright**/**rightbottom**/**br**/**rb** - Docks to bottom-right corner
///     - **bottom**/**b** - Docks to bottom
///     - **bottomleft**/**leftbottom**/**lb**/**bl** - Docks to bottom-left corner
///     - **left**/**l** - Docks to left
///     - **center**/**c** - Docks to center
/// * Margins: `left`/`l`, `right`/`r`, `top`/`t`, `bottom`/`b`
/// * State: `enabled`, `visible`
/// 
/// # Examples
/// ```rust,compile_fail
/// use appcui::prelude::*;
/// 
/// // Basic accordion with panels
/// let acc = accordion!("panels=['Section 1', 'Section 2'], x=1, y=1, width=40, height=20");
/// 
/// // Accordion with transparent background
/// let acc = accordion!(
///     "panels: ['First', 'Second', 'Third'],
///     flags: TransparentBackground,
///     x=2, y=2, width=50, height=25"
/// );
/// 
/// // Accordion with custom layout
/// let acc = accordion!(
///     "panels=['Settings', 'Help'],
///     dock: left,
///     width: 20,
///     height: 30"
/// );
/// ```
/// 
/// The caption of each panel may contain the special character `&` that indicates that the next character is a hot-key.
/// For example, constructing a panel with the caption `&Start` will set up the text of the panel to `Start` and will set up character `S` as the hot key to activate that panel.
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


/// Creates a new selector control for choosing enum values.
/// The format is `selector!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `enum` or `class` (required, first positional parameter) - The enum type to use for selection
/// * `value` - Initial selected enum variant (optional)
/// * `flags` - Control flags (optional). Can be:
///   - **AllowNoneVariant** - Allows selecting no value (None)
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
/// // Basic selector
/// let sel = selector!("MyEnum, x=1, y=1, width=20");
/// 
/// // Selector with initial value
/// let sel = selector!(
///     "enum: MyEnum,
///     value: Variant1,
///     x=2, y=2, width=25"
/// );
/// 
/// // Selector that allows no selection
/// let sel = selector!(
///     "MyEnum,
///     flags: AllowNoneVariant,
///     x=3, y=3, width=30"
/// );
/// ```
#[proc_macro]
pub fn selector(input: TokenStream) -> TokenStream {
    crate::controls::selector::create(input)
}

/// Creates a new combobox control for selecting from a list of items.
/// The format is `combobox!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `items` - List of strings to populate the combobox (required). Format: `['item1', 'item2', ...]`
/// * `index` or `selected_index` - Index of the initially selected item (optional, 0-based)
/// * `flags` - Control flags (optional). Can be:
///   - **ShowDescription** - Shows a description for each item
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
/// // Basic combobox with items
/// let cb = combobox!("items=['Option 1', 'Option 2', 'Option 3'], x=1, y=1, width=20");
/// 
/// // Combobox with initial selection
/// let cb = combobox!(
///     "items: ['Red', 'Green', 'Blue'],
///     index: 1,
///     x=2, y=2, width=25"
/// );
/// 
/// // Combobox with descriptions
/// let cb = combobox!(
///     "items: ['Item 1', 'Item 2'],
///     flags: ShowDescription,
///     x=3, y=3, width=30"
/// );
/// ```
#[proc_macro]
pub fn combobox(input: TokenStream) -> TokenStream {
    crate::controls::combobox::create(input)
}

/// Creates a new dropdown list control for selecting from a list of items.
/// The format is `dropdownlist!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `class` or `type` (required, first positional parameter) - The type of items to display in the dropdown
/// * `flags` - Control flags (optional). Can be:
///   - **AllowNoneSelection** - Allows selecting no value (None)
///   - **ShowDescription** - Shows a description for each item
/// * `symbolsize` - Size of the symbol displayed for each item (optional). Can be:
///   - **0** - No symbol
///   - **1** - Small symbol
///   - **2** - Medium symbol
///   - **3** - Large symbol
/// * `none` - Text to display when no item is selected (optional)
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
/// // Basic dropdown list
/// let dd = dropdownlist!("MyEnum, x=1, y=1, width=20");
/// 
/// // Dropdown with flags and symbol size
/// let dd = dropdownlist!(
///     "type: MyEnum,
///     flags: AllowNoneSelection+ShowDescription,
///     symbolsize: 2,
///     none: 'Select an option',
///     x=2, y=2, width=25"
/// );
/// 
/// // Dropdown with custom none text
/// let dd = dropdownlist!(
///     "MyEnum,
///     none: 'No selection',
///     x=3, y=3, width=30"
/// );
/// ```
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

/// Creates a new numeric selector control for selecting numeric values.
/// The format is `numericselector!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `type` or `class` (required, first positional parameter) - The numeric type to use. Supported types:
///   - Integer types: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `isize`, `usize`
///   - Floating point types: `f32`, `f64`
/// * `value` (optional, second positional parameter) - Initial value
/// * `min` (optional, third positional parameter) - Minimum allowed value
/// * `max` (optional, fourth positional parameter) - Maximum allowed value
/// * `step` (optional, fifth positional parameter) - Step size for increment/decrement
/// * `flags` - Control flags (optional). Can be:
///   - **HideButtons** - Hides the increment/decrement buttons
///   - **ReadOnly** - Makes the control read-only
/// * `format` or `numericformat` or `nf` - Number format (optional). Can be:
///   - **Decimal** (default) - Standard decimal format
///   - **Percentage** - Displays value as percentage
///   - **DigitGrouping** - Uses digit grouping (e.g. 1,000)
///   - **Hex** - Displays value in hexadecimal
///   - **Size** - Displays value as a size (e.g. KB, MB)
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
/// // Basic integer selector
/// let ns = numericselector!("i32, value: 42, x=1, y=1, width=10");
/// 
/// // Float selector with custom range and step
/// let ns = numericselector!(
///     "f64, 
///     value: 3.14, 
///     min: 0.0, 
///     max: 10.0, 
///     step: 0.1, 
///     format: Percentage,
///     x=2, y=2, width=15"
/// );
/// 
/// // Read-only selector with digit grouping
/// let ns = numericselector!("u64, flags: ReadOnly, format: DigitGrouping, x=3, y=3, width=20");
/// ```
#[proc_macro]
pub fn numericselector(input: TokenStream) -> TokenStream {
    crate::controls::numericselector::create(input)
}

/// Creates a new menu item for use in menus.
/// The format is `menuitem!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `caption` or `text` (required, first positional parameter) - The text displayed in the menu
/// * `shortcut` or `shortcutkey` or `key` (optional, second positional parameter) - Keyboard shortcut for the menu item
/// * `cmd` or `command` or `cmd-id` or `command-id` (required, third positional parameter) - Command identifier
/// * `type` - Menu item type (optional, auto-detected based on other parameters). Can be:
///   - **Command** (default) - Regular menu command
///   - **CheckBox** - Checkable menu item
///   - **SingleChoice** - Radio button style menu item
///   - **SubMenu** - Menu item that opens a submenu
///   - **Line** or **Separator** - Menu separator line
/// * `enable` or `enabled` - Whether the menu item is enabled (optional, defaults to true)
/// * `check` or `checked` - Whether a checkbox menu item is checked (optional, defaults to false)
/// * `select` or `selected` - Whether a single choice menu item is selected (optional, defaults to false)
/// * `items` or `subitems` - List of submenu items (required for SubMenu type)
/// * `class` - Class name for command resolution (optional)
/// 
/// # Examples
/// ```rust,compile_fail
/// use appcui::prelude::*;
/// 
/// // Basic command menu item
/// let item = menuitem!("'Open File', shortcut: 'Ctrl+O', cmd: 'OpenFile'");
/// 
/// // Checkbox menu item
/// let item = menuitem!(
///     "caption: 'Show Toolbar',
///     shortcut: 'Ctrl+T',
///     cmd: 'ToggleToolbar',
///     checked: true"
/// );
/// 
/// // Single choice menu item
/// let item = menuitem!(
///     "'View Mode',
///     shortcut: 'Ctrl+M',
///     cmd: 'ChangeViewMode',
///     selected: true"
/// );
/// 
/// // Submenu with items
/// let item = menuitem!(
///     "'Recent Files',
///     items: [
///         {'Open File 1', cmd: 'OpenFile1'},
///         {'Open File 2', cmd: 'OpenFile2'}
///     ]"
/// );
/// 
/// // Separator line
/// let item = menuitem!("'---'");
/// ```
#[proc_macro]
pub fn menuitem(input: TokenStream) -> TokenStream {
    crate::menu::menuitem::create(input, None)
}

/// Creates a new menu that can contain menu items.
/// The format is `menu!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `caption` or `text` (required, first positional parameter) - The text displayed as the menu title
/// * `items` or `subitems` - List of menu items to include in the menu (optional)
/// * `class` - Class name for command resolution (optional)
/// 
/// Menu items can be created using the `menuitem!` macro and can be of various types:
/// * Regular commands
/// * Checkboxes
/// * Single choice items
/// * Submenus
/// * Separator lines
/// 
/// # Examples
/// ```rust,compile_fail
/// use appcui::prelude::*;
/// 
/// // Basic menu with items
/// let menu = menu!(
///     "'File',
///     items: [
///         {'Open File', shortcut: 'Ctrl+O', cmd: 'OpenFile'},
///         {'Save', shortcut: 'Ctrl+S', cmd: 'SaveFile'},
///         {'---'},
///         {'Exit', shortcut: 'Alt+F4', cmd: 'Exit'}
///     ]"
/// );
/// 
/// // Menu with submenus
/// let menu = menu!(
///     "'View',
///     items: [
///         {'Show Toolbar', shortcut: 'Ctrl+T', cmd: 'ToggleToolbar', checked: true},
///         {'---'},
///         {'Zoom',
///             items: [
///                 {'Zoom In', shortcut: 'Ctrl++', cmd: 'ZoomIn'},
///                 {'Zoom Out', shortcut: 'Ctrl+-', cmd: 'ZoomOut'},
///                 {'Reset Zoom', shortcut: 'Ctrl+0', cmd: 'ResetZoom'}
///             ]
///         }
///     ]"
/// );
/// 
/// // Menu with class specification
/// let menu = menu!(
///     "'Edit',
///     class: 'MyWindow',
///     items: [
///         {'Cut', shortcut: 'Ctrl+X', cmd: 'Cut'},
///         {'Copy', shortcut: 'Ctrl+C', cmd: 'Copy'},
///         {'Paste', shortcut: 'Ctrl+V', cmd: 'Paste'}
///     ]"
/// );
/// ```
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

/// Creates a new toggle button control that can be toggled on/off.
/// The format is `togglebutton!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `caption` or `name` or `text` (required, first positional parameter) - The text displayed on the button
/// * `tooltip` or `description` or `desc` (optional, second positional parameter) - Tooltip text shown on hover
/// * `type` - Button type (optional). Can be:
///   - **Normal** (default) - Standard toggle button
///   - **Underlined** - Button with underlined text
/// * `select` or `selected` or `state` - Initial selected state (optional, defaults to false)
/// * `group` or `single_selection` - Whether the button is part of a single-selection group (optional, defaults to false)
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
/// // Basic toggle button
/// let btn = togglebutton!("'Enable Feature', x=1, y=1, width=20");
/// 
/// // Toggle button with tooltip and initial state
/// let btn = togglebutton!(
///     "caption: 'Auto-save',
///     tooltip: 'Enable automatic saving of changes',
///     selected: true,
///     x=2, y=2, width=25"
/// );
/// 
/// // Underlined toggle button in a single-selection group
/// let btn = togglebutton!(
///     "'Option A',
///     type: Underlined,
///     group: true,
///     x=3, y=3, width=15"
/// );
/// ```
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

/// Creates a new tree view control for displaying hierarchical data.
/// The format is `treeview!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `type` or `class` (required, first positional parameter) - The type of items to display in the tree
/// * `columns` - Column definitions for the tree view (optional). Format: `[{Name,Width,Align},...]`
/// * `flags` - Control flags (optional). Can be:
///   - **ScrollBars** - Shows scroll bars
///   - **SearchBar** - Enables search functionality
///   - **ShowGroups** - Enables item grouping
///   - **SmallIcons** - Uses small icons
///   - **LargeIcons** - Uses large icons
///   - **CustomFilter** - Enables custom filtering
///   - **NoSelection** - Disables item selection
///   - **HideHeader** - Hides the column header
/// * `left-scroll-margin` or `lsm` - Left scroll margin in characters (optional)
/// * `top-scroll-margin` or `tsm` - Top scroll margin in characters (optional)
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
/// // Basic tree view
/// let tv = treeview!("MyItemType, x=1, y=1, width=40, height=20");
/// 
/// // Tree view with columns and search
/// let tv = treeview!(
///     "type: MyItemType,
///     columns: [{Name,20,Left}, {Size,10,Right}, {Date,15,Left}],
///     flags: ScrollBars+SearchBar,
///     x=2, y=2, width=50, height=25"
/// );
/// 
/// // Tree view with icons and custom margins
/// let tv = treeview!(
///     "MyItemType,
///     flags: SmallIcons+ShowGroups,
///     lsm: 2,
///     tsm: 1,
///     x=3, y=3, width=60, height=30"
/// );
/// ```
#[proc_macro]
pub fn treeview(input: TokenStream) -> TokenStream {
    crate::controls::treeview::create(input)
}

/// Creates a new markdown viewer control for displaying formatted text content.
/// The format is `markdown!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `content` or `text` (required, first positional parameter) - The markdown content to display
/// * `flags` - Control flags (optional). Can be:
///   - **ScrollBars** - Shows scroll bars when content exceeds the control size
/// * `left-scroll-margin` or `lsm` - Left scroll margin in characters (optional)
/// * `top-scroll-margin` or `tsm` - Top scroll margin in characters (optional)
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
/// // Basic markdown viewer
/// let md = markdown!("'# Hello World\nThis is a **markdown** example', x=1, y=1, width=40, height=10");
/// 
/// // Markdown with scrollbars and margins
/// let md = markdown!(
///     "content: '# Documentation\n\n## Features\n* Feature 1\n* Feature 2',
///     flags: ScrollBars,
///     lsm: 2,
///     tsm: 1,
///     x=2, y=2, width=50, height=15"
/// );
/// 
/// // Docked markdown viewer
/// let md = markdown!("'# Help\n\nPress F1 for more information', dock: right, width=30");
/// ```
#[proc_macro]
pub fn markdown(input: TokenStream) -> TokenStream {
    crate::controls::markdown::create(input)
}

/// Creates a new progress bar control for displaying progress of an operation.
/// The format is `progressbar!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `count` or `c` or `total` - Total number of steps/items to process (optional)
/// * `value` or `progress` or `v` - Current progress value (optional)
/// * `text` or `caption` - Text to display on the progress bar (optional)
/// * `paused` or `pause` - Whether the progress bar is paused (optional)
/// * `flags` - Control flags (optional). Can be:
///   - **HidePercentage** - Hides the percentage display
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
/// // Basic progress bar
/// let pb = progressbar!("x=1, y=1, width=40");
/// 
/// // Progress bar with total count and current value
/// let pb = progressbar!(
///     "count: 100,
///     value: 42,
///     text: 'Processing...',
///     x=2, y=2, width=50"
/// );
/// 
/// // Paused progress bar without percentage
/// let pb = progressbar!(
///     "count: 50,
///     value: 25,
///     paused: true,
///     flags: HidePercentage,
///     x=3, y=3, width=30"
/// );
/// ```
#[proc_macro]
pub fn progressbar(input: TokenStream) -> TokenStream {
    crate::controls::progressbar::create(input)
}

/// Creates a new text area control for multi-line text input and display.
/// The format is `textarea!("attributes")` where the attributes are pairs of key-value, separated by comma.
/// 
/// # Parameters
/// * `text` (optional, first positional parameter) - Initial text content to display
/// * `flags` - Control flags (optional). Can be:
///   - **ShowLineNumber** - Displays line numbers on the left side
///   - **ReadOnly** - Makes the text area read-only
///   - **ScrollBars** - Shows scroll bars when content exceeds the control size
///   - **HighlightCursor** - Highlights the current cursor position
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
/// // Basic text area
/// let ta = textarea!("x=1, y=1, width=40, height=10");
/// 
/// // Text area with initial content and line numbers
/// let ta = textarea!(
///     "text: 'Hello\nWorld!',
///     flags: ShowLineNumber+ScrollBars,
///     x=2, y=2, width=50, height=15"
/// );
/// 
/// // Read-only text area with highlighted cursor
/// let ta = textarea!(
///     "'This is read-only text',
///     flags: ReadOnly+HighlightCursor,
///     dock: right,
///     width=30"
/// );
/// ```
#[proc_macro]
pub fn textarea(input: TokenStream) -> TokenStream {
    crate::controls::textarea::create(input)
}