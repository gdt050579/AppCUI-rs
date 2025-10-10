use std::ptr::NonNull;

use crate::{
    graphics::{Surface, TextAlignment, TextFormatBuilder, WrapType},
    system::{Handle, Theme},
    utils::Caption,
    utils::ExtractHotKeyMethod,
};

use super::{AddToToolbar, Group, ItemBase, PaintData, SymbolAttrState, ToolBar, ToolBarItem};

/// A single choice is a toolbar item that can be positioned on the top or bottom part of a window
/// and can have two states (selected or unselected).
///
/// Within a toolbar group, only one single choice item can be selected at a time, making these items
/// suitable for mutually exclusive options. Single choice items are similar to radio buttons in
/// traditional UIs.
///
/// They can also have hotkeys defined using the '&' character in the caption. For example,
/// the caption "First &choice" will set 'Alt+C' as a hotkey for the single choice item.
///
/// To intercept single choice selections, implement the `ToolBarEvents` trait for the window containing
/// the single choice items and implement the `on_choice_selected` method.
///
/// # Example
///
/// The following example creates a window with two single choice items that display their selection state in a label:
///
/// ```rust, no_run
/// use appcui::prelude::*;
///
/// #[Window(events = ToolBarEvents)]
/// struct SingleChoiceWindow {
///     option_one: Handle<toolbar::SingleChoice>,
///     option_two: Handle<toolbar::SingleChoice>,
///     status_label: Handle<Label>,
/// }
///
/// impl SingleChoiceWindow {
///     fn new() -> Self {
///         let mut win = SingleChoiceWindow {
///             base: window!("'Single Choice Demo',a:c,w:40,h:6"),
///             option_one: Handle::None,
///             option_two: Handle::None,
///             status_label: Handle::None,
///         };
///         
///         // Create a toolbar group at the bottom left of the window
///         let group = win.toolbar().create_group(toolbar::GroupPosition::BottomLeft);
///         
///         // Add single choice items to the toolbar group
///         let mut opt1 = toolbar::SingleChoice::new("Option 1");
///         opt1.set_tooltip("First option");
///         win.option_one = win.toolbar().add(group, opt1);
///         
///         let mut opt2 = toolbar::SingleChoice::new("Option 2");
///         opt2.set_tooltip("Second option");
///         win.option_two = win.toolbar().add(group, opt2);
///         
///         // Select the first option by default
///         let h = win.option_one;
///         if let Some(choice) = win.toolbar().get_mut(h) {
///             choice.select();
///         }
///         
///         // Add a label to display the selection state
///         win.status_label = win.add(label!("'Option 1 is selected',a:c,w:30,h:1"));
///         
///         win
///     }
/// }
///
/// impl ToolBarEvents for SingleChoiceWindow {
///     fn on_choice_selected(&mut self, handle: Handle<toolbar::SingleChoice>) -> EventProcessStatus {
///         let message = if handle == self.option_one {
///             "Option 1 is selected"
///         } else if handle == self.option_two {
///             "Option 2 is selected"
///         } else {
///             return EventProcessStatus::Ignored;
///         };
///         
///         let h = self.status_label;
///         if let Some(label) = self.control_mut(h) {
///             label.set_caption(message);
///         }
///         EventProcessStatus::Processed
///     }
/// }
///
/// fn main() -> Result<(), appcui::system::Error> {
///     let mut app = App::new().build()?;
///     app.add_window(SingleChoiceWindow::new());
///     app.run();
///     Ok(())
/// }
/// ```
pub struct SingleChoice {
    pub(super) base: ItemBase,
    pub(super) caption: Caption,
    selected: bool,
    pub(super) tooldbar: Option<NonNull<ToolBar>>,
}

add_to_toolbar_impl!(SingleChoice);

impl SingleChoice {
    /// Creates a new SingleChoice toolbar item with the specified text.
    ///
    /// The width (in characters) of the single choice item is calculated based on the number of characters
    /// in its content.
    ///
    /// # Parameters
    ///
    /// * `text` - The caption (text) to be displayed on the single choice item
    ///
    /// # Example
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// let choice = toolbar::SingleChoice::new("&Light theme");
    /// ```
    pub fn new(text: &str) -> Self {
        let mut obj = SingleChoice {
            base: ItemBase::new(crate::ui::window::Type::Classic, true),
            caption: Caption::new("", ExtractHotKeyMethod::NoHotKey),
            selected: false,
            tooldbar: None,
        };
        obj.set_caption(text);
        obj
    }

    /// Sets a new caption for the single choice item.
    ///
    /// The width of the single choice is automatically updated based on the length of the new caption.
    /// The character '&' can be used to define a hotkey for the next character.
    ///
    /// # Parameters
    ///
    /// * `text` - The new text to display on the single choice item
    pub fn set_caption(&mut self, text: &str) {
        self.caption.set_text(text, ExtractHotKeyMethod::AltPlusKey);
        self.base.set_width(self.caption.chars_count() as u16);
        self.base.request_recompute_layout();
    }

    /// Returns the current caption text of the single choice item.
    #[inline(always)]
    pub fn caption(&self) -> &str {
        self.caption.text()
    }

    /// Returns the selected state of the single choice item.
    ///
    /// Returns `true` if the single choice item is selected or `false` otherwise.
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Selects this single choice item and deselects all other single choice items in the same group.
    ///
    /// # Panics
    ///
    /// This method will panic if the single choice item has not been added to a toolbar.
    /// Make sure to add the item to a toolbar before calling this method.
    #[inline(always)]
    pub fn select(&mut self) {
        if let Some(toolbar_ptr) = self.tooldbar.as_mut() {
            let toolbar = unsafe { toolbar_ptr.as_mut() };
            toolbar.update_singlechoice_group_id(self.base.handle());
        } else {
            panic!("Attempt to use SingleChoice select without having the object added to a toolbar !");
        }
    }

    pub(crate) fn update_select_status(&mut self, value: bool) {
        self.selected = value;
    }

    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let mut st = SymbolAttrState::new(data);
        if (self.selected) && (data.focused) {
            st = SymbolAttrState::Pressed;
        }
        let mut format = TextFormatBuilder::new()
            .position(self.base.left(), self.base.y())
            .attribute(st.button_attr(theme))
            .align(TextAlignment::Left)
            .wrap_type(WrapType::SingleLineWrap(self.caption.chars_count() as u16))
            .build();
        if self.caption.has_hotkey() {
            format.set_hotkey(st.hotkey_attr(theme), self.caption.hotkey_pos().unwrap() as u32);
        }
        surface.write_text(self.caption.text(), &format);
    }
    add_toolbaritem_basic_methods!();
}
