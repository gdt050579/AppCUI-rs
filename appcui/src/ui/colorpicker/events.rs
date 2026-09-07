use super::ColorPicker;
use crate::{graphics::Color, system::Handle, ui::common::traits::EventProcessStatus};

/// Events from a [`struct@super::ColorPicker`].
///
/// Implement this on a window to react when the chosen color changes.
/// The default method returns [`EventProcessStatus::Ignored`].
pub trait ColorPickerEvents {
    /// Called when the selected `color` changes.
    fn on_color_changed(&mut self, _handle: Handle<ColorPicker>, _color: Color) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) color: Color
}
