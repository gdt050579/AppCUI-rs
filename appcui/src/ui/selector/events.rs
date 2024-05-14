// use super::ColorPicker;
// use crate::{graphics::Color, system::Handle, ui::common::traits::EventProcessStatus};

// pub trait SelectorEvents {
//     fn on_selection_changed(&mut self, _handle: Handle<ColorPicker>, _color: Color) -> EventProcessStatus {
//         EventProcessStatus::Ignored
//     }
// }
// #[derive(Copy, Clone)]
// pub(crate) struct EventData {
//     pub(crate) color: Color
// }