use super::traits::{Control, CustomEvents, EventProcessStatus};
use crate::prelude::colorpicker::events::ColorPickerEvents;
use crate::prelude::keyselector::events::KeySelectorEvents;
use crate::prelude::*;
use crate::prelude::{
    bufferview, colorpicker, combobox, datepicker, dropdownlist, keyselector, listbox, listview, numericselector, richtextfield, selector, textfield,
    threestatebox, togglebutton, GenericSelectorEvents, PathFinderEvents, RuntimeManager, ThreeStateBoxEvents,
};
use crate::system::Handle;

use crate::ui::{
    accordion, accordion::events::AccordionEvents, button, button::events::ButtonEvents, charpicker, charpicker::events::CharPickerEvents, checkbox,
    checkbox::events::CheckBoxEvents, combobox::events::ComboBoxEvents, datepicker::events::DatePickerEvents,
    dropdownlist::events::GenericDropDownListEvents, editor::events::EditorEvents, editor::events::EditorEventsType, graphview,
    graphview::events::GenericGraphViewEvents, listbox::events::ListBoxEvents, listview::events::GenericListViewEvents, markdown,
    markdown::events::MarkdownEvents, numericselector::events::GenericNumericSelectorEvents, password, password::events::PasswordEvents, radiobox,
    radiobox::events::RadioBoxEvents, richtextfield::events::RichTextFieldEvents, tab, tab::events::TabEvents, textfield::events::TextFieldEvents,
    timepicker, timepicker::events::TimePickerEvents, treeview::events::GenericTreeViewEvents, togglebutton::events::ToggleButtonEvents, threestatebox::events::ThreeStateBoxEvents,
    listbox::events::ListBoxEventTypes, listview::events::ListViewEventTypes,
    treeview::events::TreeViewEventTypes, graphview::events::GraphViewEventTypes, textfield::events::TextFieldEventsType,
    richtextfield::events::RichTextFieldEventsType,
    dropdownlist::events::GenericDropDownListEvents, graphview, graphview::events::GenericGraphViewEvents, listbox::events::ListBoxEvents,
    bufferview::events::GenericBufferViewEvents, listview::events::GenericListViewEvents, markdown, markdown::events::MarkdownEvents,
    numericselector::events::GenericNumericSelectorEvents,
    password, password::events::PasswordEvents, radiobox, radiobox::events::RadioBoxEvents, tab, tab::events::TabEvents,
    richtextfield::events::RichTextFieldEvents, textfield::events::TextFieldEvents, treeview::events::GenericTreeViewEvents,
    timepicker, timepicker::events::TimePickerEvents, hyperlink, hyperlink::events::HyperLinkEvents,
    hslider, hslider::events::GenericHSliderEvents, pathfinder, treeview,
};

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
    CharPicker(charpicker::events::EventData),
    Password(password::events::EventData),
    KeySelector(keyselector::events::EventData),
    TextField(textfield::events::EventData),
    RichTextField(richtextfield::events::EventData),
    Selector(selector::events::EventData),
    ComboBox(combobox::events::EventData),
    DropDownList(dropdownlist::events::EventData),
    NumericSelector(numericselector::events::EventData),
    DatePicker(datepicker::events::EventData),
    ListBox(listbox::events::EventData),
    ListView(listview::events::EventData),
    BufferView(bufferview::events::EventData),
    PathFinder(pathfinder::events::EventData),
    TreeView(treeview::events::EventData),
    Markdown(markdown::events::EventData),
    Accordion(accordion::events::EventData),
    Tab(tab::events::EventData),
    TimePicker(timepicker::events::EventData),
    GraphView(graphview::events::EventData),
    Editor(editor::events::EventData),
    HyperLink(hyperlink::events::EventData),
    HSliderEvents(hslider::events::EventData),
}

pub(crate) struct ControlEvent {
    pub(crate) emitter: Handle<()>,
    pub(crate) receiver: Handle<()>,
    pub(crate) data: ControlEventData,
}

impl ControlEvent {
    pub(crate) fn invoke(&self, receiver: &mut dyn Control) -> EventProcessStatus {
        match &self.data {
            ControlEventData::Button(_) => ButtonEvents::on_pressed(receiver, self.emitter.cast()),
            ControlEventData::CheckBox(data) => CheckBoxEvents::on_status_changed(receiver, self.emitter.cast(), data.checked),
            ControlEventData::RadioBox(_) => RadioBoxEvents::on_selected(receiver, self.emitter.cast()),
            ControlEventData::ToggleButton(data) => ToggleButtonEvents::on_selection_changed(receiver, self.emitter.cast(), data.status),
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
                    TextFieldEventsType::OnValidate => {
                        if let Some(tf) = RuntimeManager::get().get_control(h) {
                            TextFieldEvents::on_validate(receiver, self.emitter.cast(), tf.text())
                        } else {
                            EventProcessStatus::Ignored
                        }
                    }
                    TextFieldEventsType::OnTextChanged => TextFieldEvents::on_text_changed(receiver, self.emitter.cast()),
                }
            }
            ControlEventData::RichTextField(data) => {
                let h: Handle<richtextfield::RichTextField> = self.emitter.cast();
                match data.evtype {
                    RichTextFieldEventsType::OnValidate => {
                        if let Some(rtf) = RuntimeManager::get().get_control(h) {
                            RichTextFieldEvents::on_validate(receiver, self.emitter.cast(), rtf.text())
                        } else {
                            EventProcessStatus::Ignored
                        }
                    }
                    RichTextFieldEventsType::OnTextChanged => RichTextFieldEvents::on_text_changed(receiver, self.emitter.cast()),
                }
            }
            ControlEventData::Custom(data) => CustomEvents::on_event(receiver, self.emitter.cast(), data.class_hash, data.event_id),
            ControlEventData::Selector(data) => GenericSelectorEvents::on_selection_changed(receiver, self.emitter.cast(), data.type_id),
            ControlEventData::ComboBox(_) => ComboBoxEvents::on_selection_changed(receiver, self.emitter.cast()),
            ControlEventData::DropDownList(data) => GenericDropDownListEvents::on_selection_changed(receiver, self.emitter.cast(), data.type_id),
            ControlEventData::NumericSelector(data) => GenericNumericSelectorEvents::on_value_changed(receiver, self.emitter.cast(), data.type_id),
            ControlEventData::DatePicker(data) => DatePickerEvents::on_date_changed(receiver, self.emitter.cast(), data.date),
            ControlEventData::ListBox(data) => match data.event_type {
                ListBoxEventTypes::CurrentItemChanged => ListBoxEvents::on_current_item_changed(receiver, self.emitter.cast(), data.index),
                ListBoxEventTypes::ItemChecked => ListBoxEvents::on_item_checked(receiver, self.emitter.cast(), data.index, data.checked),
            },
            ControlEventData::ListView(data) => match data.event_type {
                ListViewEventTypes::CurrentItemChanged => GenericListViewEvents::on_current_item_changed(receiver, self.emitter.cast(), data.type_id),
                ListViewEventTypes::GroupFoldedOrUnfolded(group, collapsed) => {
                    if collapsed {
                        GenericListViewEvents::on_group_collapsed(receiver, self.emitter.cast(), data.type_id, group)
                    } else {
                        GenericListViewEvents::on_group_expanded(receiver, self.emitter.cast(), data.type_id, group)
                    }
                }
                ListViewEventTypes::SelectionChanged => GenericListViewEvents::on_selection_changed(receiver, self.emitter.cast(), data.type_id),
                ListViewEventTypes::ItemAction(index) => GenericListViewEvents::on_item_action(receiver, self.emitter.cast(), data.type_id, index),
            },
            ControlEventData::BufferView(data) => match data.event_type {
                bufferview::events::BufferViewEventTypes::CurrentPosChanged => {
                    GenericBufferViewEvents::on_current_pos_changed(receiver, self.emitter.cast(), data.type_id)
                }
                bufferview::events::BufferViewEventTypes::SelectionChanged => {
                    GenericBufferViewEvents::on_selection_changed(receiver, self.emitter.cast(), data.type_id)
                }
            },
            ControlEventData::PathFinder(_) => PathFinderEvents::on_path_updated(receiver, self.emitter.cast()),
            ControlEventData::TreeView(data) => match data.event_type {
                TreeViewEventTypes::CurrentItemChanged(item_handle) => {
                    GenericTreeViewEvents::on_current_item_changed(receiver, self.emitter.cast(), data.type_id, item_handle)
                }
                TreeViewEventTypes::ItemCollapsed(item_handle, recursive) => {
                    GenericTreeViewEvents::on_item_collapsed(receiver, self.emitter.cast(), data.type_id, item_handle, recursive)
                }
                TreeViewEventTypes::ItemExpanded(item_handle, recursive) => {
                    GenericTreeViewEvents::on_item_expanded(receiver, self.emitter.cast(), data.type_id, item_handle, recursive)
                }
                TreeViewEventTypes::ItemAction(item_handle) => {
                    GenericTreeViewEvents::on_item_action(receiver, self.emitter.cast(), data.type_id, item_handle)
                }
                TreeViewEventTypes::SelectionChanged => GenericTreeViewEvents::on_selection_changed(receiver, self.emitter.cast(), data.type_id),
            },
            ControlEventData::Markdown(data) => match &data.event_type {
                markdown::events::Data::BackEvent => MarkdownEvents::on_backspace_navigation(receiver, self.emitter.cast()),
                markdown::events::Data::LinkClickEvent(link) => MarkdownEvents::on_external_link(receiver, self.emitter.cast(), link),
            },
            ControlEventData::Accordion(data) => {
                AccordionEvents::on_panel_changed(receiver, self.emitter.cast(), data.new_panel_index, data.old_panel_index)
            }
            ControlEventData::Tab(data) => TabEvents::on_tab_changed(receiver, self.emitter.cast(), data.new_tab_index, data.old_tab_index),
            ControlEventData::CharPicker(data) => {
                CharPickerEvents::on_char_changed(receiver, self.emitter.cast(), if data.code as u32 > 0 { Some(data.code) } else { None })
            }
            ControlEventData::TimePicker(data) => TimePickerEvents::on_time_changed(receiver, self.emitter.cast(), data.time),
            ControlEventData::GraphView(data) => match data.event_type {
                GraphViewEventTypes::CurrentNodeChanged => {
                    GenericGraphViewEvents::on_current_node_changed(receiver, self.emitter.cast(), data.type_id)
                }
                GraphViewEventTypes::NodeAction(index) => GenericGraphViewEvents::on_node_action(receiver, self.emitter.cast(), data.type_id, index),
                GraphViewEventTypes::RequestNewNode(p) => GenericGraphViewEvents::on_request_new_node(receiver, self.emitter.cast(), data.type_id, p),
                GraphViewEventTypes::RequestNewEdge(from, to) => {
                    GenericGraphViewEvents::on_request_new_edge(receiver, self.emitter.cast(), data.type_id, from, to)
                }
                GraphViewEventTypes::SelectionChanged => GenericGraphViewEvents::on_selection_changed(receiver, self.emitter.cast(), data.type_id),
            },
            ControlEventData::Editor(data) => match data.evtype {
                EditorEventsType::OnCaretMoved => EditorEvents::on_caret_moved(receiver, self.emitter.cast()),
                EditorEventsType::OnCharPressed(ch) => EditorEvents::on_char_pressed(receiver, self.emitter.cast(), ch),
                EditorEventsType::OnDocumentChanged => EditorEvents::on_document_changed(receiver, self.emitter.cast()),
            },
            ControlEventData::HyperLink(_) => HyperLinkEvents::on_open(receiver, self.emitter.cast()),
            ControlEventData::HSliderEvents(data) => GenericHSliderEvents::on_value_changed(receiver, self.emitter.cast(), data.type_id),
        }
    }
}
