use super::toolbar::{Button, CheckBox, SingleChoice};
use crate::{graphics::Rect, prelude::ActionRequest, system::Handle, ui::common::traits::EventProcessStatus};

// Window events always go to the same window that triggers them --> we don't need a handle as
// we already have &mut self
pub trait WindowEvents {
    fn on_close(&mut self) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    
    // don't need to change anything --> since layout has been change, repaint wil be force automatically
    fn on_layout_changed(&mut self, _old_layout: Rect, _new_layout: Rect) {}
    
    /// called whenver the window receives focus
    fn on_activate(&mut self) {}

    /// called whenever the window loses focus. 
    fn on_deactivate(&mut self) {}
    
    /// called whenever the ENTER key is intercepted by the Window
    /// For modal windows the behavior should be to use `.exit_with(...)` method to exit.
    /// for a regular window there is no default behavior
    fn on_accept(&mut self) {}

    /// called whenever the ESC key is interpreted by the Window
    /// ## For a modal window
    /// 
    /// If this function returns 'ActionRequest::Allow' it will translate into a call to `ModalWindow::exit()` method. 
    /// If the returned value is `ActionRequest::Deny` the following nothing happens and any `exit()` or `exit_with(...)` methods call will be disregarded.
    /// **OBS**: As a general rule, if should not attempt to close the modal window during this function
    /// 
    /// ## For a regular window
    /// This method is never called when you press `Escape` on a regular (non-modal) window
    fn on_cancel(&mut self) -> ActionRequest {
        ActionRequest::Allow
    }
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
