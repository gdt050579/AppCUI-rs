use crate::{
    controls::{
        button::ButtonClickedEvent,
        checkbox::CheckedStatusChangedEvent,
        menu::{MenuCheckBoxStateChangedEvent, MenuCommandEvent, MenuRadioBoxSelectedEvent},
        window::WindowDecoratorCheckBoxStateChangedEvent,
        window::WindowDecoratorSingleChoiceSelectedEvent,
        window::{WindowCloseEvent, WindowDecoratorButtonPressedEvent},
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

impl Event {
    pub(crate) fn get_sender(&self) -> Handle {
        match self {
            Event::CheckedStatusChanged(event) => event.handle,
            Event::WindowClose(event) => event.handle,
            Event::ButtonClicked(event) => event.handle,
            Event::WindowDecoratorButtonPressed(_) => Handle::None,
            Event::WindowDecoratorCheckBoxStateChanged(_) => Handle::None,
            Event::WindowDecoratorSingleChoiceSelected(_) => Handle::None,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum MenuEvent {
    Command(MenuCommandEvent),
    CheckBoxStateChanged(MenuCheckBoxStateChangedEvent),
    RadioBoxSelected(MenuRadioBoxSelectedEvent),
}

impl MenuEvent {
    #[inline(always)]
    pub(crate) fn get_control_receiver_handle(&self) -> Handle {
        match self {
            MenuEvent::Command(event) => event.control_receiver_handle,
            MenuEvent::CheckBoxStateChanged(event) => event.control_receiver_handle,
            MenuEvent::RadioBoxSelected(event) => event.control_receiver_handle,
        }
    }
}
