use crate::{controls::button::ButtonClickedEvent, system::Handle};

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Event {
    CheckedStatusChanged,
    WindowClose(Handle),
    ButtonClicked(ButtonClickedEvent),
    TempCommand(u32),
}

impl Event {
    pub(crate) fn get_sender(&self) -> Handle {
        match self {
            Event::CheckedStatusChanged => Handle::None,
            Event::WindowClose(_) => Handle::None,
            Event::ButtonClicked(event) => event.handle,
            Event::TempCommand(_) => Handle::None,
        }
    }
}
