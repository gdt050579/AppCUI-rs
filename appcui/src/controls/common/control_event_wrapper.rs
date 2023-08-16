use crate::controls::button;
use crate::controls::checkbox;
pub(crate) enum ControlEvent {
    ButtonEvent(button::events::EventData),
    CheckBoxEvent(checkbox::events::EventData),
}