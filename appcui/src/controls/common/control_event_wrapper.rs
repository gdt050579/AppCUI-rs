use crate::controls::button;
pub(crate) enum ControlEvent {
    ButtonEvent(button::events::EventData)
}