use super::toolbar::{Button, CheckBox, SingleChoice};
use crate::{graphics::Rect, system::Handle, ui::common::traits::EventProcessStatus};

// Window events always go to the same window that triggers them --> we don't need a handle as
// we already have &mut self
pub trait WindowEvents {
    fn on_close(&mut self) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    // don't need to change anything --> since layout has been change, repaint wil be force automatically
    fn on_layout_changed(&mut self, _old_layout: Rect, _new_layout: Rect) {}
    fn on_activate(&mut self) {}
    fn on_deactivate(&mut self) {}
    /// called whenever the ENTER key is intercepted by the Window
    /// For modal windows the behavior should be to use `.exit_with(...)` method to exit.
    /// for a regular window there is no default behavior
    fn on_accept(&mut self) {}

    /// called whenever the ESC key is interpreted by the Window
    /// For a modal window the default behavior should be use use .exit() method to exit
    /// for a regular window there is no default behavior
    fn on_cancel(&mut self) {}
}
pub trait ToolBarEvents {
    fn on_button_clicked(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_checkbox_clicked(&mut self, _handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_choice_selected(&mut self, _handle: Handle<SingleChoice>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

pub trait ModalWindowMethods<T> {
    fn show(self) -> Option<T>;
    fn exit_with(&mut self, result: T);
    fn exit(&mut self);
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum EventData {
    OnActivate,
    OnClose,
}
