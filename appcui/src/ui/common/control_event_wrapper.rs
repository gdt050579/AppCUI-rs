use textfield::TextField;

use super::traits::{Control, CustomEvents, EventProcessStatus};
use super::UIElement;
use crate::prelude::colorpicker::events::ColorPickerEvents;
use crate::prelude::keyselector::events::KeySelectorEvents;
use crate::prelude::{colorpicker, combobox, dropdownlist, keyselector, selector, textfield, threestatebox, GenericSelectorEvents, RuntimeManager, ThreeStateBoxEvents};
use crate::system::Handle;
use crate::ui::{
    button, button::events::ButtonEvents, checkbox, checkbox::events::CheckBoxEvents, password, password::events::PasswordEvents, radiobox,
    radiobox::events::RadioBoxEvents,
    textfield::events::TextFieldEvents,
    combobox::events::ComboBoxEvents,
    dropdownlist::events::GenericDropDownListEvents,
};

#[derive(Copy,Clone)]
pub(crate) struct CustomEventData {
    pub(crate) class_hash: u64,
    pub(crate) event_id: u32
}

pub(crate) enum ControlEventData {
    Custom(CustomEventData),
    Button(button::events::EventData),
    CheckBox(checkbox::events::EventData),
    RadioBox(radiobox::events::EventData),
    ThreeStateBox(threestatebox::events::EventData),
    ColorPicker(colorpicker::events::EventData),
    Password(password::events::EventData),
    KeySelector(keyselector::events::EventData),
    TextField(textfield::events::EventData),
    Selector(selector::events::EventData),
    ComboBox(combobox::events::EventData),
    DropDownList(dropdownlist::events::EventData),
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
            ControlEventData::KeySelector(data) => {
                KeySelectorEvents::on_key_changed(receiver, self.emitter.cast(), data.new_key, data.old_key )
            },
            ControlEventData::TextField(data) => {
                let h: Handle<TextField> = self.emitter.cast();
                match data.evtype {
                    textfield::events::TextFieldEventsType::OnValidate => {
                        if let Some(tf) = RuntimeManager::get().get_control(h) {
                            TextFieldEvents::on_validate(receiver, self.emitter.cast(), tf.text())
                        } else {
                            EventProcessStatus::Ignored
                        }
                    },
                    //textfield::events::TextFieldEventsType::OnTextChanged => todo!(),
                }
            },
            ControlEventData::Custom(data) => {
                CustomEvents::on_event(receiver, self.emitter.cast(),data.class_hash, data.event_id)
            },
            ControlEventData::Selector(data) => {
                GenericSelectorEvents::on_selection_changed(receiver, self.emitter.cast(), data.type_id)
            },
            ControlEventData::ComboBox(_) => {
                ComboBoxEvents::on_selection_changed(receiver, self.emitter.cast())
            },
            ControlEventData::DropDownList(data) => {
                GenericDropDownListEvents::on_selection_changed(receiver, self.emitter.cast(), data.type_id)
            }                        
        }
    }
}
