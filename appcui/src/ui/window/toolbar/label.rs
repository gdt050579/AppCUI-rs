use crate::{
    graphics::Surface,
    system::{Handle, Theme},
};

use super::{AddToToolbar, Group, ItemBase, PaintData, ToolBarItem};

/// A label is a non-interactive toolbar item that displays text on the top or bottom part of a window.
///
/// Toolbar labels are used to show information to the user, such as status indicators, counters,
/// or other text that doesn't require user interaction. Unlike buttons or checkboxes, labels don't
/// generate events when clicked.
///
/// # Example
///
/// The following example creates a window with labels that display numbers in different formats,
/// which can be toggled using checkboxes:
///
/// ```rust, no_run
/// use appcui::prelude::*;
/// 
/// #[Window(events = ButtonEvents+CheckBoxEvents)]
/// struct NumberFormatsWindow {
///     increase_button: Handle<Button>,
///     decimal_label: Handle<toolbar::Label>,
///     hex_label: Handle<toolbar::Label>,
///     binary_label: Handle<toolbar::Label>,
///     show_decimal: Handle<CheckBox>,
///     show_hex: Handle<CheckBox>,
///     show_binary: Handle<CheckBox>,
///     number: u32,
/// }
///
/// impl NumberFormatsWindow {
///     fn new() -> Self {
///         let mut win = NumberFormatsWindow {
///             base: window!("'Number Formats',a:c,w:40,h:10"),
///             increase_button: Handle::None,
///             decimal_label: Handle::None,
///             hex_label: Handle::None,
///             binary_label: Handle::None,
///             show_decimal: Handle::None,
///             show_hex: Handle::None,
///             show_binary: Handle::None,
///             number: 42,
///         };
///         
///         // Add the increase button
///         win.increase_button = win.add(button!("'Increase',w:15,d:l"));
///         
///         // Add checkboxes to control visibility
///         win.show_decimal = win.add(checkbox!("'Show decimal',x:20,y:2,w:16,checked:true"));
///         win.show_hex = win.add(checkbox!("'Show hex',x:20,y:4,w:16,checked:true"));
///         win.show_binary = win.add(checkbox!("'Show binary',x:20,y:6,w:16,checked:true"));
///         
///         // Create toolbar groups
///         let bottom_group = win.toolbar().create_group(toolbar::GroupPosition::BottomLeft);
///         let top_group = win.toolbar().create_group(toolbar::GroupPosition::TopRight);
///         
///         // Add toolbar labels
///         win.decimal_label = win.toolbar().add(bottom_group, toolbar::Label::new("Dec:42"));
///         win.hex_label = win.toolbar().add(bottom_group, toolbar::Label::new("Hex:2A"));
///         win.binary_label = win.toolbar().add(top_group, toolbar::Label::new("Bin:101010"));
///         
///         win
///     }
///     
///     fn update_labels(&mut self) {
///         // Update all labels with the current number in different formats
///         let h = self.decimal_label; 
///         let number = self.number;
///         if let Some(label) = self.toolbar().get_mut(h) {
///             label.set_content(&format!("Dec:{}", number));
///         }
///         let h = self.hex_label;
///         if let Some(label) = self.toolbar().get_mut(h) {
///             label.set_content(&format!("Hex:{:X}", number));
///         }
///         let h = self.binary_label;
///         if let Some(label) = self.toolbar().get_mut(h) {
///             label.set_content(&format!("Bin:{:b}", number));
///         }
///     }
/// }
///
/// impl ButtonEvents for NumberFormatsWindow {
///     fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
///         self.number += 1;
///         self.update_labels();
///         EventProcessStatus::Processed
///     }
/// }
///
/// impl CheckBoxEvents for NumberFormatsWindow {
///     fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
///         if handle == self.show_decimal {
///             if let Some(label) = self.toolbar().get_mut(handle) {
///                 label.set_visible(checked);
///             }
///         } else if handle == self.show_hex {
///             if let Some(label) = self.toolbar().get_mut(handle) {
///                 label.set_visible(checked);
///             }
///         } else if handle == self.show_binary {
///             if let Some(label) = self.toolbar().get_mut(handle) {
///                 label.set_visible(checked);
///             }
///         }
///         EventProcessStatus::Processed
///     }
/// }
/// 
/// fn main() -> Result<(), appcui::system::Error> {
///     let mut app = App::new().build()?;
///     app.add_window(NumberFormatsWindow::new());
///     app.run();
///     Ok(())
/// }
/// ```
pub struct Label {
    pub(super) base: ItemBase,
    text: String,
}

add_to_toolbar_impl!(Label);

impl Label {
    /// Creates a new Label toolbar item with the specified text.
    ///
    /// The width (in characters) of the label is calculated based on the number of characters in its content.
    /// 
    /// # Parameters
    /// 
    /// * `text` - The text to be displayed on the label
    ///
    /// # Example
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// let status_label = toolbar::Label::new("Status: Ready");
    /// ```
    pub fn new(text: &str) -> Self {
        let mut obj = Label {
            base: ItemBase::new(true),
            text: String::new(),
        };
        obj.set_content(text);
        obj
    }
    
    /// Sets a new text content for the label.
    ///
    /// The width of the label is automatically updated based on the length of the new text.
    ///
    /// # Parameters
    ///
    /// * `text` - The new text to display on the label
    #[inline]
    pub fn set_content(&mut self, text: &str) {
        self.text.clear();
        self.text.push_str(text);
        self.base.set_width(text.chars().count() as u16);
        self.base.request_recompute_layout();
    }
    
    /// Returns the current text content of the label.
    #[inline(always)]
    pub fn caption(&self) -> &str {
        &self.text
    }
    
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, data: &PaintData) {
        let attr = match data.focused {
            true => theme.text.normal,
            false => theme.text.inactive,
        };
        surface.write_string(self.base.get_left(), self.base.get_y(), self.text.as_str(), attr, false);
    }

    add_toolbaritem_basic_methods!();
}

