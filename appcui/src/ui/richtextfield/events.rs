use crate::system::Handle;
use crate::ui::common::traits::EventProcessStatus;

pub enum RichTextFieldEventsType {
    OnValidate,
    OnTextChanged,
}

pub trait RichTextFieldEvents {
    fn on_validate(&mut self, _handle: Handle<super::RichTextField>, _text: &str) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_text_changed(&mut self, _handle: Handle<super::RichTextField>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

pub struct EventData {
    pub evtype: RichTextFieldEventsType,
}