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
    Button(button::events::EventData),
    CheckBox(checkbox::events::EventData),
    RadioBox(radiobox::events::EventData),
    ThreeStateBox(threestatebox::events::EventData),
    ColorPicker(colorpicker::events::EventData),
    Password(password::events::EventData),
}

pub(crate) struct ControlEvent {
    pub(crate) emitter: Handle<UIElement>,
    pub(crate) receiver: Handle<UIElement>,
    pub(crate) data: ControlEventData,
}

impl ControlEvent {
    pub(crate) fn invoke(&self, receiver: &mut dyn Control) -> EventProcessStatus {
        match &self.data {
            ControlEventData::Button(_) => {
                ButtonEvents::on_pressed(receiver, self.emitter.cast())
            }
            ControlEventData::CheckBox(data) => {
                CheckBoxEvents::on_status_changed(receiver, self.emitter.cast(), data.checked)
            }
            ControlEventData::RadioBox(_) => {
                RadioBoxEvents::on_selected(receiver, self.emitter.cast())
            }
            ControlEventData::ColorPicker(data) => {
                ColorPickerEvents::on_color_changed(receiver, self.emitter.cast(), data.color)
            }
            ControlEventData::ThreeStateBox(data) => {
                ThreeStateBoxEvents::on_status_changed(receiver, self.emitter.cast(), data.state)
            }
            ControlEventData::Password(data) => {
                if data.accept {
                    PasswordEvents::on_accept(receiver, self.emitter.cast())
                } else {
                    PasswordEvents::on_cancel(receiver, self.emitter.cast())
                }
            }
        }
    }
}
