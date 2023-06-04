use crate::{
    controls::{
        button::ButtonClickedEvent,
        checkbox::CheckedStatusChangedEvent,
        window::WindowDecoratorCheckBoxStateChangedEvent,
        window::WindowDecoratorSingleChoiceSelectedEvent,
        window::{WindowCloseEvent, WindowDecoratorButtonPressedEvent}, menu::{MenuCommandEvent, MenuCheckBoxStateChangedEvent, MenuRadioBoxSelectedEvent},
    },
    system::Handle,
};

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Event {
    CheckedStatusChanged(CheckedStatusChangedEvent),
    WindowClose(WindowCloseEvent),
    ButtonClicked(ButtonClickedEvent),
    WindowDecoratorButtonPressed(WindowDecoratorButtonPressedEvent),
    WindowDecoratorCheckBoxStateChanged(WindowDecoratorCheckBoxStateChangedEvent),
    WindowDecoratorSingleChoiceSelected(WindowDecoratorSingleChoiceSelectedEvent),
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum MenuEvent {
    Command(MenuCommandEvent),
    CheckBoxStateChanged(MenuCheckBoxStateChangedEvent),
    RadioBoxSelected(MenuRadioBoxSelectedEvent),
}

impl Event {
    pub(crate) fn get_sender(&self) -> Handle {
        match self {
            Event::CheckedStatusChanged(event) => event.handle,
            Event::WindowClose(event) => event.handle,
            Event::ButtonClicked(event) => event.handle,
            Event::WindowDecoratorButtonPressed(_) => Handle::None,
            Event::WindowDecoratorCheckBoxStateChanged(_) => Handle::None,
            Event::WindowDecoratorSingleChoiceSelected(_) => Handle::None,
            Event::MenuCommand(_) => Handle::None,
            Event::MenuCheckBoxStateChanged(_) => Handle::None,
            Event::MenuRadioBoxSelected(_) => Handle::None,
        }
    }
}
