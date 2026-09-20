//! Shared traits implemented by all UI controls.
//!
//! This module defines paint, input, resize, focus, timer, and background-task
//! hooks, plus marker traits such as [`Control`], [`WindowControl`], and
//! [`DesktopControl`]. Control-specific events live in each control's `events`
//! module and are re-exported here for the derive macros.

use crate::{
    graphics::{Size, Surface},
    input::{Key, MouseEvent},
    system::{Handle, Theme},
    ui::{
        appbar::events::AppBarEvents,
        accordion::events::AccordionEvents,
        button::events::ButtonEvents,
        checkbox::events::CheckBoxEvents,
        colorpicker::events::ColorPickerEvents,
        charpicker::events::CharPickerEvents,
        combobox::events::ComboBoxEvents,
        command_bar::events::GenericCommandBarEvents,
        datepicker::events::DatePickerEvents,
        desktop::events::DesktopEvents,
        dropdownlist::events::GenericDropDownListEvents,
        graphview::events::GenericGraphViewEvents,
        keyselector::events::KeySelectorEvents,
        listbox::events::ListBoxEvents,
        bufferview::events::GenericBufferViewEvents,
        listview::events::GenericListViewEvents,
        markdown::events::MarkdownEvents,
        menu::events::GenericMenuEvents,
        numericselector::events::GenericNumericSelectorEvents,
        password::events::PasswordEvents,
        pathfinder::events::PathFinderEvents,
        radiobox::events::RadioBoxEvents,
        selector::events::GenericSelectorEvents,
        textfield::events::TextFieldEvents,
        richtextfield::events::RichTextFieldEvents,
        threestatebox::events::ThreeStateBoxEvents,
        togglebutton::events::ToggleButtonEvents,
        treeview::events::GenericTreeViewEvents,
        window::events::{ToolBarEvents, WindowEvents},
        tab::events::TabEvents,
        timepicker::events::TimePickerEvents,
        hyperlink::events::HyperLinkEvents,
        hslider::events::GenericHSliderEvents,
        vbarchart::events::GenericVBarChartEvents,
    },
};

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
/// Whether a control consumed an input event or left it for its parent.
///
/// Return `Processed` to stop further handling, or `Ignored` to let the event
/// bubble to other controls.
pub enum EventProcessStatus {
    /// The control handled the event; do not propagate it further.
    Processed,
    /// The control did not handle the event; parents may still process it.
    Ignored,
}
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
/// A request to allow or deny an action such as closing a window.
///
/// Event handlers return `Allow` to proceed or `Deny` to cancel the action.
pub enum ActionRequest {
    /// Proceed with the action (for example, close the window).
    Allow,
    /// Cancel the action.
    Deny,
}
#[repr(u8)]
#[derive(Copy, Clone)]
/// The direction in which an expandable control (combo box, picker, and similar) opens.
///
/// `OnTop` expands above the control, `OnBottom` expands below it.
pub enum ExpandedDirection {
    /// Open the expanded panel above the control.
    OnTop,
    /// Open the expanded panel below the control.
    OnBottom,
}

/// Called when a control should draw itself.
///
/// The framework invokes this after layout. Draw into `surface` using colors and
/// glyphs from `theme`.
pub trait OnPaint {
    /// Paints the control onto `surface` using the current `theme`.
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}
/// Called when the control has focus and a key is pressed.
///
/// Return [`EventProcessStatus::Processed`] if the key was handled.
pub trait OnKeyPressed {
    /// Handles a key press. `character` is the translated character, or `'\0'` for non-character keys.
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
/// Called when a mouse event occurs over the control.
///
/// Return [`EventProcessStatus::Processed`] if the event was handled.
pub trait OnMouseEvent {
    /// Handles a mouse [`MouseEvent`] (move, click, wheel, and similar).
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
/// Called when the control's default action is invoked (for example Activate / Enter).
pub trait OnDefaultAction {
    /// Runs the control's default action.
    fn on_default_action(&mut self) {}
}

/// Called after the control's size changes.
pub trait OnResize {
    /// Notifies that the control was resized from `old_size` to `new_size`.
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {}
}

/// Called when the control gains or loses keyboard focus.
pub trait OnFocus {
    /// The control received focus.
    fn on_focus(&mut self) {}
    /// The control lost focus.
    fn on_lose_focus(&mut self) {}
}

/// Called when an expandable control (combo box, picker, and similar) opens or closes.
pub trait OnExpand {
    /// The control expanded in `direction` ([`ExpandedDirection::OnTop`] or [`OnBottom`](ExpandedDirection::OnBottom)).
    fn on_expand(&mut self, _direction: ExpandedDirection) {}
    /// The control collapsed back to its packed size.
    fn on_pack(&mut self) {}
}

/// Called when the application theme changes.
pub trait OnThemeChanged {
    /// Reloads colors or other theme-dependent state from `theme`.
    fn on_theme_changed(&mut self, _theme: &Theme) {}
}

/// Called once after the control is registered with a window.
pub trait OnWindowRegistered {
    /// The control is now attached to a window and can use window services.
    fn on_registered(&mut self) {}
}
/// Called when a sibling control (for example another radio box in the same group) is selected.
pub trait OnSiblingSelected {
    /// `handle` identifies the sibling that became selected.
    #[allow(private_interfaces)]
    fn on_sibling_selected(&mut self, _handle: Handle<()>) {}
}
/// Hook for custom, application-defined events dispatched through a type hash and event id.
pub trait CustomEvents {
    /// Handles a custom event from `handle`. `class_hash` identifies the sender type and `event_id` the event.
    #[allow(private_interfaces)]
    fn on_event(&mut self, _handle: Handle<()>, _class_hash: u64, _event_id: u32) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

/// Events from a [`crate::system::Timer`] attached to a control.
///
/// Default methods return [`EventProcessStatus::Ignored`]. `ticks` is the number of
/// timer ticks elapsed since start.
pub trait TimerEvents {
    /// The timer has started.
    fn on_start(&mut self) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// The timer resumed after a pause. `ticks` is the count at resume.
    fn on_resume(&mut self, _ticks: u64) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// The timer was paused. `ticks` is the count at pause.
    fn on_pause(&mut self, _ticks: u64) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// A periodic tick. `ticks` is the current count.
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

/// Events from a background task associated with a control.
///
/// `handle` identifies the task. Default methods return [`EventProcessStatus::Ignored`].
pub trait GenericBackgroundTaskEvents {
    /// The background task has started.
    fn on_start(&mut self, _handle: Handle<()>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// The background task reported progress.
    fn on_update(&mut self, _handle: Handle<()>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// The background task finished.
    fn on_finish(&mut self, _handle: Handle<()>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    /// The background task is querying the UI (for example for cancellation).
    fn on_query(&mut self, _handle: Handle<()>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

/// Combined capability bound implemented by every UI control.
///
/// Requires paint, input, layout, focus, and all control-specific event traits.
/// The `CustomControl`, `Window`, and `Desktop` macros implement this automatically.
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
    + RichTextFieldEvents
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
    + GenericBufferViewEvents
    + PathFinderEvents
    + TimerEvents
    + GenericTreeViewEvents
    + MarkdownEvents
    + GenericBackgroundTaskEvents
    + AccordionEvents
    + TabEvents
    + CharPickerEvents
    + GenericGraphViewEvents
    + AppBarEvents
    + TimePickerEvents
    + HyperLinkEvents
    + GenericHSliderEvents
    + GenericVBarChartEvents
{
}

/// Marker implemented by [`struct@crate::ui::Desktop`].
pub trait DesktopControl {}
/// Marker implemented by [`struct@crate::ui::Window`].
pub trait WindowControl {}
/// Marker for controls that are not windows (used to restrict generic APIs).
pub trait NotWindow {}
/// Marker for controls that are not the desktop.
pub trait NotDesktop {}
/// Marker for windows that are not modal.
pub trait NotModalWindow {}
/// Marker for types that can be used as a command identifier (typically `u32`).
pub trait CommandID {}
