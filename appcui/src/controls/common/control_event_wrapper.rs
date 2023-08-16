use crate::controls::button;
use crate::controls::checkbox;
use crate::system::Handle;

pub(crate) enum ControlEventData {
    ButtonEvent(button::events::EventData),
    CheckBoxEvent(checkbox::events::EventData),
}

pub(crate) struct ControlEvent {
    pub(crate) emitter: Handle,
    pub(crate) receiver: Handle,
    pub(crate) data: ControlEventData
}

