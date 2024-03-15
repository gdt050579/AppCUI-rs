use super::traits::{Control, EventProcessStatus};
use super::UIElement;
use crate::prelude::{colorpicker, threestatebox, ThreeStateBoxEvents};
use crate::prelude::colorpicker::events::ColorPickerEvents;
use crate::system::Handle;
use crate::ui::{
    button, button::events::ButtonEvents, 
    checkbox, checkbox::events::CheckBoxEvents, 
    radiobox, radiobox::events::RadioBoxEvents, 
};

pub(crate) enum ControlEventData {
    ButtonEvent(button::events::EventData),
    CheckBoxEvent(checkbox::events::EventData),
    RadioBoxEvent(radiobox::events::EventData),
    ThreeStateBoxEvent(threestatebox::events::EventData),
    ColorPickerEvent(colorpicker::events::EventData),
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
                return ButtonEvents::on_pressed(receiver, self.emitter.cast());
            }
            ControlEventData::CheckBoxEvent(data) => {
                return CheckBoxEvents::on_status_changed(
                    receiver,
                    self.emitter.cast(),
                    data.checked,
                );
            }
            ControlEventData::RadioBoxEvent(data) => {
                return RadioBoxEvents::on_selected(
                    receiver,
                    self.emitter.cast(),
                );
            }
            ControlEventData::ColorPickerEvent(data) => {
                return ColorPickerEvents::on_color_changed(
                    receiver,
                    self.emitter.cast(),
                    data.color,
                );
            }
            ControlEventData::ThreeStateBoxEvent(data) => {
                return ThreeStateBoxEvents::on_status_changed(
                    receiver,
                    self.emitter.cast(),
                    data.state,
                );
            }
        }
    }
}
