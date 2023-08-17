use super::traits::Control;
use crate::system::Handle;
use crate::ui::{button, button::events::ButtonEvents, checkbox, checkbox::events::CheckBoxEvents};

pub(crate) enum ControlEventData {
    ButtonEvent(button::events::EventData),
    CheckBoxEvent(checkbox::events::EventData),
}

pub(crate) struct ControlEvent {
    pub(crate) emitter: Handle,
    pub(crate) receiver: Handle,
    pub(crate) data: ControlEventData,
}

impl ControlEvent {
    pub(crate) fn invoke(&self, receiver: &mut dyn Control) {
        let result = match &self.data {
            ControlEventData::ButtonEvent(_) => {
                ButtonEvents::on_pressed(receiver, self.emitter);
            }
            ControlEventData::CheckBoxEvent(data) => {
                CheckBoxEvents::on_status_changed(receiver, self.emitter, data.checked);
            }
        };
    }
}
