use super::Editor;
use crate::{system::Handle, ui::common::traits::EventProcessStatus};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum EditorEventsType {
    OnCursorPositionChanged,
    OnCharPressed(char),
    OnTextChanged,
}

pub trait EditorEvents {
    fn on_cursor_position_changed(&mut self, _handle: Handle<Editor>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_char_pressed(&mut self, _handle: Handle<Editor>, _ch: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_text_changed(&mut self, _handle: Handle<Editor>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
#[derive(Copy, Clone)]
pub(crate) struct EventData {
    pub(crate) evtype: EditorEventsType,
}
