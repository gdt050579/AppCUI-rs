//! Events for windows and modal windows
//!
//! This module contains the traits and methods for handling events from windows and modal windows.
//!
//! It includes traits for handling events from windows and modal windows, such as:
//! - WindowEvents
//! - ModalWindowEvents
//! - ToolBarEvents

use super::toolbar::{Button, CheckBox, SingleChoice};
use crate::{graphics::Rect, prelude::ActionRequest, system::Handle, ui::common::traits::EventProcessStatus};

// Window events always go to the same window that triggers them --> we don't need a handle as
// we already have &mut self
pub trait WindowEvents {
    // don't need to change anything --> since layout has been change, repaint wil be force automatically
    fn on_layout_changed(&mut self, _old_layout: Rect, _new_layout: Rect) {}

    /// called whenver the window receives focus
    fn on_activate(&mut self) {}

    /// called whenever the window loses focus.
    fn on_deactivate(&mut self) {}

    /// called whenever the ENTER key is intercepted by the Window
    /// For modal windows the behavior should be to use `.exit_with(...)` method to exit.
    /// for a regular (non-modal) window this callback is never called)
    fn on_accept(&mut self) {}

    /// called whenever the ESC key is interpreted by the Window
    /// ## For a modal window
    ///
    /// If this function returns 'ActionRequest::Allow' it will translate into a call to `ModalWindow::exit()` method.
    /// If the returned value is `ActionRequest::Deny` the nothing happens and any `exit()` or `exit_with(...)` methods call will be disregarded.
    /// **OBS**: As a general rule, if should not attempt to close the modal window during this function
    ///
    /// ## For a regular (non-modal) window
    /// This method is called when the user hits the **close button** or when ESC key is iterpreted by the Window
    /// If this function returns 'ActionRequest::Allow' the window will be closed (and removed from the desktop).
    /// If the returned value is `ActionRequest::Deny` the window remains as it is.
    fn on_cancel(&mut self) -> ActionRequest {
        ActionRequest::Allow
    }
}

/// Trait for handling events from toolbar items.
///
/// This trait allows windows to receive and process events from toolbar items such as buttons,
/// checkboxes, and single choice items. When a user interacts with a toolbar item, the
/// corresponding method on this trait is called.
///
/// To use this trait, you need to:
/// 1. Include it in your Window or ModalWindow attributes with `#[Window(events = ToolBarEvents)]`
/// 2. Implement the trait methods for your window to handle toolbar item events
/// 3. Return `EventProcessStatus::Processed` if you've handled the event, or `EventProcessStatus::Ignored` otherwise
///
/// All methods have default implementations that return `EventProcessStatus::Ignored`, so you
/// only need to implement the methods for the toolbar items you're using.
pub trait ToolBarEvents {
    /// Called when a toolbar button is clicked.
    ///
    /// # Parameters
    ///
    /// * `handle` - The handle to the button that was clicked
    ///
    /// # Returns
    ///
    /// * `EventProcessStatus::Processed` if the event was handled
    /// * `EventProcessStatus::Ignored` if the event should be passed to the next handler
    fn on_button_clicked(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

    /// Called when a toolbar checkbox is clicked, changing its checked state.
    ///
    /// # Parameters
    ///
    /// * `handle` - The handle to the checkbox that was clicked
    /// * `checked` - The new checked state (true for checked, false for unchecked)
    ///
    /// # Returns
    ///
    /// * `EventProcessStatus::Processed` if the event was handled
    /// * `EventProcessStatus::Ignored` if the event should be passed to the next handler
    fn on_checkbox_clicked(&mut self, _handle: Handle<CheckBox>, _checked: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

    /// Called when a toolbar single choice item is selected.
    ///
    /// # Parameters
    ///
    /// * `handle` - The handle to the single choice item that was selected
    ///
    /// # Returns
    ///
    /// * `EventProcessStatus::Processed` if the event was handled
    /// * `EventProcessStatus::Ignored` if the event should be passed to the next handler
    fn on_choice_selected(&mut self, _handle: Handle<SingleChoice>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

/// Trait for managing modal window lifecycles and results.
///
/// This trait defines methods for showing, exiting, and returning results from modal windows.
/// Modal windows capture the entire focus during their existence and can return a result when closed.
///
/// The type parameter `T` represents the result type that the modal window will return.
///
/// When implementing a modal window, this trait is automatically implemented by the `#[ModalWindow]` macro,
/// so you typically don't need to implement it manually.
pub trait ModalWindowMethods<T> {
    /// Shows the modal window and blocks execution until the window is closed.
    ///
    /// This method displays the modal window, captures all input, and blocks the current thread
    /// until the window is closed. Once closed, it returns the result (if any) that was
    /// provided with `exit_with()`.
    ///
    /// # Returns
    ///
    /// * `Some(T)` if the window was closed with a result via `exit_with()`
    /// * `None` if the window was closed via `exit()` or `close()`
    fn show(self) -> Option<T>;

    /// Exits the modal window and returns the specified result.
    ///
    /// This method closes the modal window and makes `show()` return `Some(result)` with
    /// the provided result value.
    ///
    /// # Parameters
    ///
    /// * `result` - The value to return from the modal window
    fn exit_with(&mut self, result: T);

    /// Exits the modal window without a result.
    ///
    /// This method closes the modal window and makes `show()` return `None`.
    fn exit(&mut self);

    /// Alias for `exit()` that closes the modal window without a result.
    ///
    /// This method has the same effect as `exit()`, closing the modal window and making
    /// `show()` return `None`.
    fn close(&mut self);
}

// #[derive(Copy, Clone)]
// pub(crate) enum EventData {

// }
