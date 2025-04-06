use crate::{
    graphics::{Size, Surface},
    input::{Key, MouseEvent},
    system::{Handle, Theme},
    ui::{
        button::events::ButtonEvents, checkbox::events::CheckBoxEvents, colorpicker::events::ColorPickerEvents, combobox::events::ComboBoxEvents, command_bar::events::GenericCommandBarEvents, datepicker::events::DatePickerEvents, desktop::events::DesktopEvents, dropdownlist::events::GenericDropDownListEvents, keyselector::events::KeySelectorEvents, listbox::events::ListBoxEvents, listview::events::GenericListViewEvents, markdown::events::MarkdownEvents, menu::events::GenericMenuEvents, numericselector::events::GenericNumericSelectorEvents, password::events::PasswordEvents, pathfinder::events::PathFinderEvents, radiobox::events::RadioBoxEvents, selector::events::GenericSelectorEvents, textfield::events::TextFieldEvents, threestatebox::events::ThreeStateBoxEvents, togglebutton::events::ToggleButtonEvents, treeview::events::GenericTreeViewEvents, window::events::{ToolBarEvents, WindowEvents}, hnumericslider::events::GenericHNumericSliderEvents
    },
};

use super::UIElement;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum EventProcessStatus {
    Processed,
    Ignored,
}
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum ActionRequest {
    Allow,
    Deny,
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ExpandedDirection {
    OnTop,
    OnBottom,
}

pub trait OnPaint {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}
pub trait OnKeyPressed {
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
pub trait OnMouseEvent {
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
pub trait OnDefaultAction {
    fn on_default_action(&mut self) {}
}

pub trait OnResize {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {}
}

pub trait OnFocus {
    fn on_focus(&mut self) {}
    fn on_lose_focus(&mut self) {}
}

pub trait OnExpand {
    fn on_expand(&mut self, _direction: ExpandedDirection) {}
    fn on_pack(&mut self) {}
}

pub trait OnThemeChanged {
    fn on_theme_changed(&mut self, _theme: &Theme) {}
}

pub trait OnWindowRegistered {
    fn on_registered(&mut self) {}
}
pub trait OnSiblingSelected {
    #[allow(private_interfaces)]
    fn on_sibling_selected(&mut self, _handle: Handle<UIElement>) {}
}
pub trait CustomEvents {
    #[allow(private_interfaces)]
    fn on_event(&mut self, _handle: Handle<()>, _class_hash: u64, _event_id: u32) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

pub trait TimerEvents {
    fn on_start(&mut self) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_resume(&mut self, _ticks: u64) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_pause(&mut self, _ticks: u64) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }    
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

pub trait GenericBackgroundTaskEvents {
    fn on_start(&mut self, _handle: Handle<()>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_update(&mut self, _handle: Handle<()>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_finish(&mut self, _handle: Handle<()>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_query(&mut self, _handle: Handle<()>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}


pub trait Control:
    OnPaint
    + OnKeyPressed
    + OnMouseEvent
    + OnDefaultAction
    + OnResize
    + OnFocus
    + OnExpand
    + OnWindowRegistered
    + OnSiblingSelected
    + OnThemeChanged
    /* events from each control */
    + ButtonEvents
    + CheckBoxEvents
    + RadioBoxEvents
    + ToggleButtonEvents
    + PasswordEvents
    + ThreeStateBoxEvents
    + ColorPickerEvents
    + KeySelectorEvents
    + TextFieldEvents
    + GenericCommandBarEvents
    + WindowEvents
    + ToolBarEvents
    + DesktopEvents
    + GenericMenuEvents
    + CustomEvents
    + GenericSelectorEvents
    + GenericDropDownListEvents
    + ComboBoxEvents
    + GenericNumericSelectorEvents
    + DatePickerEvents
    + ListBoxEvents
    + GenericListViewEvents
    + PathFinderEvents
    + TimerEvents
    + GenericHNumericSliderEvents
    + GenericTreeViewEvents
    + MarkdownEvents
    + GenericBackgroundTaskEvents
{
}

pub trait DesktopControl {}
pub trait WindowControl {}
pub trait NotWindow {}
pub trait NotDesktop {}
pub trait NotModalWindow {}
pub trait CommandID {}
