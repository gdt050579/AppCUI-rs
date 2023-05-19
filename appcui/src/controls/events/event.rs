use crate::{
    controls::{
        button::ButtonClickedEvent, checkbox::CheckedStatusChangedEvent,
        window::WindowDecoratorButtonPressedEvent,
        window::WindowDecoratorCheckBoxStateChangedEvent,
        window::WindowDecoratorSingleChoiceSelectedEvent,
    },
    system::Handle,
};

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Event {
    CheckedStatusChanged(CheckedStatusChangedEvent),
    WindowClose(Handle),
    ButtonClicked(ButtonClickedEvent),
    WindowDecoratorButtonPressed(WindowDecoratorButtonPressedEvent),
    WindowDecoratorCheckBoxStateChanged(WindowDecoratorCheckBoxStateChangedEvent),
    WindowDecoratorSingleChoiceSelected(WindowDecoratorSingleChoiceSelectedEvent),
    TempCommand(u32),
}

impl Event {
    pub(crate) fn get_sender(&self) -> Handle {
        match self {
            Event::CheckedStatusChanged(event) => event.handle,
            Event::WindowClose(_) => Handle::None,
            Event::ButtonClicked(event) => event.handle,
            Event::WindowDecoratorButtonPressed(_) => Handle::None,
            Event::WindowDecoratorCheckBoxStateChanged(_) => Handle::None,
            Event::WindowDecoratorSingleChoiceSelected(_) => Handle::None,
            Event::TempCommand(_) => Handle::None,
        }
    }
}
