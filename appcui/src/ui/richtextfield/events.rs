use crate::system::Handle;
use crate::ui::common::traits::EventProcessStatus;

pub use crate::ui::textfield::events::TextFieldEventsType;

pub trait RichTextFieldEvents {
    fn on_validate(&mut self, _handle: Handle<super::RichTextField>, _text: &str) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_text_changed(&mut self, _handle: Handle<super::RichTextField>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
