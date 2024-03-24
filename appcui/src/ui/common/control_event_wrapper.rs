use super::traits::{Control, EventProcessStatus};
use super::UIElement;
use crate::prelude::colorpicker::events::ColorPickerEvents;
use crate::prelude::{colorpicker, threestatebox, ThreeStateBoxEvents};
use crate::system::Handle;
use crate::ui::{
    button, button::events::ButtonEvents, checkbox, checkbox::events::CheckBoxEvents, password, password::events::PasswordEvents, radiobox,
    radiobox::events::RadioBoxEvents,
};

pub(crate) enum ControlEventData {
    ButtonEvent(button::events::EventData),
    CheckBoxEvent(checkbox::events::EventData),
    RadioBoxEvent(radiobox::events::EventData),
    ThreeStateBoxEvent(threestatebox::events::EventData),
    ColorPickerEvent(colorpicker::events::EventData),
    PasswordEvent(password::events::EventData),
}

pub(crate) struct ControlEvent {
    pub(crate) emitter: Handle<UIElement>,
    pub(crate) receiver: Handle<UIElement>,
    pub(crate) data: ControlEventData,
}

impl ControlEvent {
    pub(crate) fn invoke(&self, receiver: &mut dyn Control) -> EventProcessStatus {
        match &self.data {
            ControlEventData::ButtonEvent(_) => {
                ButtonEvents::on_pressed(receiver, self.emitter.cast())
            }
            ControlEventData::CheckBoxEvent(data) => {
                CheckBoxEvents::on_status_changed(receiver, self.emitter.cast(), data.checked)
            }
            ControlEventData::RadioBoxEvent(_) => {
                RadioBoxEvents::on_selected(receiver, self.emitter.cast())
            }
            ControlEventData::ColorPickerEvent(data) => {
                ColorPickerEvents::on_color_changed(receiver, self.emitter.cast(), data.color)
            }
            ControlEventData::ThreeStateBoxEvent(data) => {
                ThreeStateBoxEvents::on_status_changed(receiver, self.emitter.cast(), data.state)
            }
            ControlEventData::PasswordEvent(data) => {
                if data.accept {
                    PasswordEvents::on_accept(receiver, self.emitter.cast())
                } else {
                    PasswordEvents::on_cancel(receiver, self.emitter.cast())
                }
            }
        }
    }
}
