use crate::{
    graphics::{Surface, TextAlignament, TextFormatBuilder, WrapType},
    system::{Handle, Theme},
    utils::{Caption, ExtractHotKeyMethod},
};

use super::{AddToToolbar, Group, ItemBase, PaintData, SymbolAttrState, ToolBarItem};

/// A button is a clickable toolbar item that can be positioned on the top or bottom part of a window.
///
/// Toolbar buttons display text and can trigger actions when clicked. They can also have hotkeys
/// defined using the '&' character in the button caption. For example, the caption "St&art" will
/// set 'Alt+A' as a hotkey for the button.
///
/// To intercept button clicks, implement the `ToolBarEvents` trait for the window containing the button
/// and implement the `on_button_clicked` method.
///
/// # Example
///
/// The following example creates a window with a counter that can be incremented or decremented
/// using toolbar buttons:
///
/// ```rust, no_run
/// use appcui::prelude::*;
/// 
/// #[Window(events = ToolBarEvents)]
/// struct CounterWindow {
///     increase_button: Handle<toolbar::Button>,
///     decrease_button: Handle<toolbar::Button>,
///     counter_label: Handle<Label>,
///     counter: i32,
/// }
///
/// impl CounterWindow {
///     fn new() -> Self {
///         let mut win = CounterWindow {
///             base: window!("'Counter',d:c,w:40,h:6"),
///             increase_button: Handle::None,
///             decrease_button: Handle::None,
///             counter_label: Handle::None,
///             counter: 0,
///         };
///         
///         // Create a toolbar group at the bottom right of the window
///         let group = win.toolbar().create_group(toolbar::GroupPosition::BottomRight);
///         
///         // Add buttons to the toolbar group
///         let mut btn_minus = toolbar::Button::new("-"); 
///         btn_minus.set_tooltip("Decrease counter");
///         win.decrease_button = win.toolbar().add(group, btn_minus);
///         let mut btn_plus = toolbar::Button::new("+");
///         btn_plus.set_tooltip("Increase counter");
///         win.increase_button = win.toolbar().add(group, btn_plus);
///                
///         // Add a label to display the counter value
///         win.counter_label = win.add(label!("0,d:c,w:10,h:1"));
///         
///         win
///     }
///     
///     // Update the counter label
///     fn update_counter(&mut self) {
///         let h = self.counter_label;
///         let text = format!("{}", self.counter);
///         if let Some(label) = self.control_mut(h) {
///             label.set_caption(&text);
///         }
///     }
/// }
///
/// impl ToolBarEvents for CounterWindow {
///     fn on_button_clicked(&mut self, handle: Handle<toolbar::Button>) -> EventProcessStatus {
///         if handle == self.increase_button {
///             self.counter += 1;
///             self.update_counter();
///             EventProcessStatus::Processed
///         } else if handle == self.decrease_button {
///             self.counter -= 1;
///             self.update_counter();
///             EventProcessStatus::Processed
///         } else {
///             EventProcessStatus::Ignored
///         }
///     }
/// }
///
/// fn main() -> Result<(), appcui::system::Error> {
///     let mut app = App::new().build()?;
///     app.add_window(CounterWindow::new());
///     app.run();
///     Ok(())
/// }
/// ```
pub struct Button {
    pub(super) base: ItemBase,
    pub(super) caption: Caption,
}
add_to_toolbar_impl!(Button);

impl Button {
    /// Creates a new Button toolbar item with the specified text.
    ///
    /// The width (in characters) of the button is calculated based on the number of characters in its content.
    /// 
    /// # Parameters
    /// 
    /// * `text` - The caption (text) to be displayed on the button
    ///
    /// # Example
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// let button = toolbar::Button::new("&Save");
    /// ```
    pub fn new(text: &str) -> Self {
        let mut obj = Button {
            base: ItemBase::new(true),
            caption: Caption::new("", ExtractHotKeyMethod::NoHotKey),
        };
        obj.set_caption(text);
        obj
    }
    
    /// Sets a new caption for the button.
    ///
    /// The width of the button is automatically updated based on the length of the new caption.
    /// The character '&' can be used to define a hotkey for the next character.
    ///
    /// # Parameters
    ///
    /// * `text` - The new text to display on the button
    pub fn set_caption(&mut self, text: &str) {
        self.caption.set_text(text, ExtractHotKeyMethod::AltPlusKey);
        self.base.set_width(self.caption.chars_count() as u16);
        self.base.request_recompute_layout();
    }
    
    /// Returns the current caption text of the button.
    #[inline(always)]
    pub fn get_content(&self) -> &str {
        self.caption.text()
    }
    
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let st = SymbolAttrState::new(data);
        let mut format = TextFormatBuilder::new()
            .position(self.base.get_left(), self.base.get_y())
            .attribute(st.get_button_attr(theme))
            .align(TextAlignament::Left)
            .wrap_type(WrapType::SingleLineWrap(self.caption.chars_count() as u16))
            .build();
        if self.caption.has_hotkey() {
            format.set_hotkey(st.get_hotkey_attr(theme), self.caption.hotkey_pos().unwrap() as u32);
        }
        surface.write_text(self.caption.text(), &format);
    }
    add_toolbaritem_basic_methods!();
}
