use crate::{
    graphics::{Size, Surface},
    input::{Key, MouseEvent},
    prelude::textfield::events::TextFieldEvents,
    system::{Handle, Theme},
    ui::{
        button::events::ButtonEvents,
        checkbox::events::CheckBoxEvents,
        colorpicker::events::ColorPickerEvents,
        command_bar::events::GenericCommandBarEvents,
        desktop::events::DesktopEvents,
        keyselector::events::KeySelectorEvents,
        menu::events::GenericMenuEvents,
        password::events::PasswordEvents,
        radiobox::events::RadioBoxEvents,
        threestatebox::events::ThreeStateBoxEvents,
        window::events::{ToolBarEvents, WindowEvents},
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
    /* events from each control */
    + ButtonEvents
    + CheckBoxEvents
    + RadioBoxEvents
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
{
}

pub trait DesktopControl {}
pub trait WindowControl {}
pub trait NotWindow {}
pub trait NotDesktop {}
pub trait NotModalWindow {}
pub trait CommandID {}
