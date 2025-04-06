use textfield::TextField;

use super::traits::{Control, CustomEvents, EventProcessStatus};
use super::UIElement;
use crate::prelude::colorpicker::events::ColorPickerEvents;
use crate::prelude::keyselector::events::KeySelectorEvents;
use crate::prelude::{
    colorpicker, combobox, datepicker, dropdownlist, keyselector, listbox, listview, numericselector, selector, textfield, threestatebox,
    togglebutton, GenericSelectorEvents, PathFinderEvents, RuntimeManager, ThreeStateBoxEvents,
};
use crate::system::Handle;

use crate::ui::{
    button, button::events::ButtonEvents, checkbox, checkbox::events::CheckBoxEvents, combobox::events::ComboBoxEvents,
    datepicker::events::DatePickerEvents, dropdownlist::events::GenericDropDownListEvents, listbox::events::ListBoxEvents,
    listview::events::GenericListViewEvents, numericselector::events::GenericNumericSelectorEvents, password, password::events::PasswordEvents,
    radiobox, radiobox::events::RadioBoxEvents, textfield::events::TextFieldEvents, treeview::events::GenericTreeViewEvents, hnumericslider::events::GenericHNumericSliderEvents,
    markdown, markdown::events::MarkdownEvents
};
use crate::ui::{hnumericslider, pathfinder, treeview};

#[derive(Copy, Clone)]
pub(crate) struct CustomEventData {
    pub(crate) class_hash: u64,
    pub(crate) event_id: u32,
}

pub(crate) enum ControlEventData {
    Custom(CustomEventData),
    Button(button::events::EventData),
    CheckBox(checkbox::events::EventData),
    RadioBox(radiobox::events::EventData),
    ToggleButton(togglebutton::events::EventData),
    ThreeStateBox(threestatebox::events::EventData),
    ColorPicker(colorpicker::events::EventData),
    Password(password::events::EventData),
    KeySelector(keyselector::events::EventData),
    TextField(textfield::events::EventData),
    Selector(selector::events::EventData),
    ComboBox(combobox::events::EventData),
    DropDownList(dropdownlist::events::EventData),
    NumericSelector(numericselector::events::EventData),
    DatePicker(datepicker::events::EventData),
    ListBox(listbox::events::EventData),
    ListView(listview::events::EventData),
    PathFinder(pathfinder::events::EventData),
    TreeView(treeview::events::EventData),
    HNumericSlider(hnumericslider::events::EventData),
    Markdown(markdown::events::EventData)
}

pub(crate) struct ControlEvent {
    pub(crate) emitter: Handle<UIElement>,
    pub(crate) receiver: Handle<UIElement>,
    pub(crate) data: ControlEventData,
}

impl ControlEvent {
    pub(crate) fn invoke(&self, receiver: &mut dyn Control) -> EventProcessStatus {
        match &self.data {
            ControlEventData::Button(_) => ButtonEvents::on_pressed(receiver, self.emitter.cast()),
            ControlEventData::CheckBox(data) => CheckBoxEvents::on_status_changed(receiver, self.emitter.cast(), data.checked),
            ControlEventData::RadioBox(_) => RadioBoxEvents::on_selected(receiver, self.emitter.cast()),
            ControlEventData::ToggleButton(data) => {
                togglebutton::events::ToggleButtonEvents::on_selection_changed(receiver, self.emitter.cast(), data.status)
            }
            ControlEventData::ColorPicker(data) => ColorPickerEvents::on_color_changed(receiver, self.emitter.cast(), data.color),
            ControlEventData::ThreeStateBox(data) => ThreeStateBoxEvents::on_status_changed(receiver, self.emitter.cast(), data.state),
            ControlEventData::Password(data) => {
                if data.accept {
                    PasswordEvents::on_accept(receiver, self.emitter.cast())
                } else {
                    PasswordEvents::on_cancel(receiver, self.emitter.cast())
                }
            }
            ControlEventData::KeySelector(data) => KeySelectorEvents::on_key_changed(receiver, self.emitter.cast(), data.new_key, data.old_key),
            ControlEventData::TextField(data) => {
                let h: Handle<TextField> = self.emitter.cast();
                match data.evtype {
                    textfield::events::TextFieldEventsType::OnValidate => {
                        if let Some(tf) = RuntimeManager::get().get_control(h) {
                            TextFieldEvents::on_validate(receiver, self.emitter.cast(), tf.text())
                        } else {
                            EventProcessStatus::Ignored
                        }
                    } //textfield::events::TextFieldEventsType::OnTextChanged => todo!(),
                }
            }
            ControlEventData::Custom(data) => CustomEvents::on_event(receiver, self.emitter.cast(), data.class_hash, data.event_id),
            ControlEventData::Selector(data) => GenericSelectorEvents::on_selection_changed(receiver, self.emitter.cast(), data.type_id),
            ControlEventData::ComboBox(_) => ComboBoxEvents::on_selection_changed(receiver, self.emitter.cast()),
            ControlEventData::DropDownList(data) => GenericDropDownListEvents::on_selection_changed(receiver, self.emitter.cast(), data.type_id),
            ControlEventData::NumericSelector(data) => GenericNumericSelectorEvents::on_value_changed(receiver, self.emitter.cast(), data.type_id),
            ControlEventData::DatePicker(data) => DatePickerEvents::on_date_change(receiver, self.emitter.cast(), data.date),
            ControlEventData::ListBox(data) => match data.event_type {
                listbox::events::ListBoxEventTypes::CurrentItemChanged => {
                    ListBoxEvents::on_current_item_changed(receiver, self.emitter.cast(), data.index)
                }
                listbox::events::ListBoxEventTypes::ItemChecked => {
                    ListBoxEvents::on_item_checked(receiver, self.emitter.cast(), data.index, data.checked)
                }
            },
            ControlEventData::ListView(data) => match data.event_type {
                listview::events::ListViewEventTypes::CurrentItemChanged => {
                    GenericListViewEvents::on_current_item_changed(receiver, self.emitter.cast(), data.type_id)
                }
                listview::events::ListViewEventTypes::GroupFoldedOrUnfolded(group, collapsed) => {
                    if collapsed {
                        GenericListViewEvents::on_group_collapsed(receiver, self.emitter.cast(), data.type_id, group)
                    } else {
                        GenericListViewEvents::on_group_expanded(receiver, self.emitter.cast(), data.type_id, group)
                    }
                }
                listview::events::ListViewEventTypes::SelectionChanged => {
                    GenericListViewEvents::on_selection_changed(receiver, self.emitter.cast(), data.type_id)
                }
                listview::events::ListViewEventTypes::ItemAction(index) => {
                    GenericListViewEvents::on_item_action(receiver, self.emitter.cast(), data.type_id, index)
                }
            },
            ControlEventData::PathFinder(_) => PathFinderEvents::on_path_updated(receiver, self.emitter.cast()),
            ControlEventData::TreeView(data) => match data.event_type {
                treeview::events::TreeViewEventTypes::CurrentItemChanged(item_handle) => {
                    GenericTreeViewEvents::on_current_item_changed(receiver, self.emitter.cast(), data.type_id, item_handle)
                }
                treeview::events::TreeViewEventTypes::ItemCollapsed(item_handle, recursive) => {
                    GenericTreeViewEvents::on_item_collapsed(receiver, self.emitter.cast(), data.type_id, item_handle, recursive)
                },
                treeview::events::TreeViewEventTypes::ItemExpanded(item_handle, recursive) => {
                    GenericTreeViewEvents::on_item_expanded(receiver, self.emitter.cast(), data.type_id, item_handle, recursive)
                },
                treeview::events::TreeViewEventTypes::ItemAction(item_handle) => {
                    GenericTreeViewEvents::on_item_action(receiver, self.emitter.cast(), data.type_id, item_handle)
                }
                treeview::events::TreeViewEventTypes::SelectionChanged => {
                    GenericTreeViewEvents::on_selection_changed(receiver, self.emitter.cast(), data.type_id)
                }
            },
            ControlEventData::HNumericSlider(data) => {
                GenericHNumericSliderEvents::on_value_changed(receiver, self.emitter.cast(), data.type_id)
            },
            
            ControlEventData::Markdown(data) => match &data.event_type {
                markdown::events::Data::BackEvent => {
                    MarkdownEvents::on_backspace_navigation(receiver, self.emitter.cast())
                },
                markdown::events::Data::LinkClickEvent(link) => {
                    MarkdownEvents::on_external_link(receiver, self.emitter.cast(), link)
                },
            }
        }
    }
}
