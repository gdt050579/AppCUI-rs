use crate::{
    graphics::{Character, SpecialChar, Surface, TextAlignment, TextFormatBuilder, WrapType},
    system::{Handle, Theme},
    utils::Caption,
    utils::ExtractHotKeyMethod,
};

use super::{AddToToolbar, Group, ItemBase, PaintData, SymbolAttrState, ToolBarItem};

/// A checkbox is a toolbar item that can be positioned on the top or bottom part of a window
/// and can have two states (checked or unchecked).
///
/// Toolbar checkboxes display text and can be toggled between checked and unchecked states when clicked.
/// They can also have hotkeys defined using the '&' character in the checkbox caption. For example,
/// the caption "&Option one" will set 'Alt+O' as a hotkey for the checkbox.
///
/// To intercept checkbox state changes, implement the `ToolBarEvents` trait for the window containing
/// the checkbox and implement the `on_checkbox_clicked` method.
///
/// # Example
///
/// The following example creates a window with two checkboxes that display their state in a label:
///
/// ```rust, no_run
/// use appcui::prelude::*;
///
/// #[Window(events = ToolBarEvents)]
/// struct CheckboxWindow {
///     checkbox_one: Handle<toolbar::CheckBox>,
///     checkbox_two: Handle<toolbar::CheckBox>,
///     status_label: Handle<Label>,
/// }
///
/// impl CheckboxWindow {
///     fn new() -> Self {
///         let mut win = CheckboxWindow {
///             base: window!("'Checkbox Demo',a:c,w:40,h:6"),
///             checkbox_one: Handle::None,
///             checkbox_two: Handle::None,
///             status_label: Handle::None,
///         };
///         
///         // Create a toolbar group at the bottom right of the window
///         let group = win.toolbar().create_group(toolbar::GroupPosition::BottomLeft);
///         
///         // Add checkboxes to the toolbar group
///         let mut cb1 = toolbar::CheckBox::new("Option 1", false);
///         cb1.set_tooltip("First option");
///         win.checkbox_one = win.toolbar().add(group, cb1);
///         
///         let mut cb2 = toolbar::CheckBox::new("Option 2", false);
///         cb2.set_tooltip("Second option");
///         win.checkbox_two = win.toolbar().add(group, cb2);
///         
///         // Add a label to display the checkbox states
///         win.status_label = win.add(label!("'Select an option',a:c,w:30,h:1"));
///         
///         win
///     }
/// }
///
/// impl ToolBarEvents for CheckboxWindow {
///     fn on_checkbox_clicked(&mut self, handle: Handle<toolbar::CheckBox>, checked: bool) -> EventProcessStatus {
///         let message = if handle == self.checkbox_one {
///             format!("Option 1 is {}", if checked { "checked" } else { "unchecked" })
///         } else if handle == self.checkbox_two {
///             format!("Option 2 is {}", if checked { "checked" } else { "unchecked" })
///         } else {
///             return EventProcessStatus::Ignored;
///         };
///         
///         let h = self.status_label;
///         if let Some(label) = self.control_mut(h) {
///             label.set_caption(&message);
///         }
///         EventProcessStatus::Processed
///     }
/// }
///
/// fn main() -> Result<(), appcui::system::Error> {
///     let mut app = App::new().build()?;
///     app.add_window(CheckboxWindow::new());
///     app.run();
///     Ok(())
/// }
/// ```
pub struct CheckBox {
    pub(super) base: ItemBase,
    pub(super) caption: Caption,
    checked: bool,
}

add_to_toolbar_impl!(CheckBox);

impl CheckBox {
    /// Creates a new CheckBox toolbar item with the specified text and initial checked state.
    ///
    /// The width (in characters) of the checkbox is calculated based on the number of characters
    /// in its content plus 2 characters for the checkbox symbol.
    ///
    /// # Parameters
    ///
    /// * `text` - The caption (text) to be displayed next to the checkbox
    /// * `checked` - The initial state of the checkbox (true for checked, false for unchecked)
    ///
    /// # Example
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// let checkbox = toolbar::CheckBox::new("&Enable feature", true);
    /// ```
    pub fn new(text: &str, checked: bool) -> Self {
        let mut obj = CheckBox {
            base: ItemBase::new(crate::ui::window::Type::Normal, true),
            caption: Caption::new("", ExtractHotKeyMethod::NoHotKey),
            checked,
        };
        obj.set_content(text);
        obj
    }

    /// Sets a new caption for the checkbox.
    ///
    /// The width of the checkbox is automatically updated based on the length of the new caption.
    /// The character '&' can be used to define a hotkey for the next character.
    ///
    /// # Parameters
    ///
    /// * `text` - The new text to display next to the checkbox
    pub fn set_content(&mut self, text: &str) {
        self.caption.set_text(text, ExtractHotKeyMethod::AltPlusKey);
        self.base.set_width((self.caption.chars_count() + 2) as u16);
        self.base.request_recompute_layout();
    }

    /// Returns the current caption text of the checkbox.
    #[inline(always)]
    pub fn caption(&self) -> &str {
        self.caption.text()
    }

    /// Sets the checked state of the checkbox.
    ///
    /// # Parameters
    ///
    /// * `checked` - The new state of the checkbox (true for checked, false for unchecked)
    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
        self.base.request_recompute_layout();
    }
    pub(crate) fn reverse_check(&mut self) {
        self.set_checked(!self.checked);
    }

    /// Returns the current checked state of the checkbox.
    ///
    /// Returns `true` if the checkbox is checked or `false` otherwise.
    #[inline(always)]
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let st = SymbolAttrState::new(data);
        let text_attr = st.button_attr(theme);
        let x = self.base.left();
        let y = self.base.y();
        let mut format = TextFormatBuilder::new()
            .position(x + 2, y)
            .attribute(text_attr)
            .align(TextAlignment::Left)
            .wrap_type(WrapType::SingleLineWrap(self.caption.chars_count() as u16))
            .build();
        if self.caption.has_hotkey() {
            format.set_hotkey(st.hotkey_attr(theme), self.caption.hotkey_pos().unwrap() as u32);
        }
        surface.write_string(x, y, "  ", text_attr, false);
        surface.write_text(self.caption.text(), &format);
        if self.checked {
            surface.write_char(
                x,
                y,
                Character::with_attributes(SpecialChar::CheckMark, st.attr(theme, theme.symbol.checked)),
            );
        }
    }
    add_toolbaritem_basic_methods!();
}
