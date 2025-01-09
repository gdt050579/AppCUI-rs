use self::{common::StatusFlags, keyselector::events::KeySelectorEvents};

use super::{events::ModalWindowMethods, Flags, Window};
use crate::prelude::*;
use std::ops::{Deref, DerefMut};

#[repr(C)]
pub struct ModalWindow<T: Sized> {
    base: Window,
    result: Option<T>,
}
impl<T> Deref for ModalWindow<T> {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<T> DerefMut for ModalWindow<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl<T> ModalWindow<T> {
    pub fn new(title: &str, layout: Layout, flags: Flags) -> Self {
        // a Modal Window does not have an implicit close button
        // as exiting has to be done from either exit(...) or exit_with(...) method.
        Self {
            base: Window::with_type_and_status_flags(title, layout, flags | Flags::NoCloseButton, window::Type::Normal, StatusFlags::ModalWindow),
            result: None,
        }
    }
    pub fn with_type(title: &str, layout: Layout, flags: Flags, window_type: window::Type) -> Self {
        // a Modal Window does not have an implicit close button
        // as exiting has to be done from either exit(...) or exit_with(...) method.
        Self {
            base: Window::with_type_and_status_flags(title, layout, flags | Flags::NoCloseButton, window_type, StatusFlags::ModalWindow),
            result: None,
        }
    }
    pub fn show<U>(object: U) -> Option<T>
    where
        U: Control + WindowControl + ModalWindowMethods<T> + 'static,
        U: DerefMut<Target = ModalWindow<T>>,
    {
        let handle = RuntimeManager::get().add_modal_window(object);
        // safety check - if we did not manage to add the window
        if handle.is_none() {
            return None;
        }
        // run the loop (the method will determine if it runs in a modal way or not)
        RuntimeManager::get().run();
        // the loop has ended , lets grab the results
        if let Some(obj) = RuntimeManager::get().get_control_mut(handle) {
            // move the result
            return obj.result.take();
        }
        None
    }
}
impl<T: 'static> Control for ModalWindow<T> {}
impl<T> OnThemeChanged for ModalWindow<T> {}
impl<T> GenericMenuEvents for ModalWindow<T> {}
impl<T> DesktopEvents for ModalWindow<T> {}
impl<T> ToolBarEvents for ModalWindow<T> {}
impl<T> WindowEvents for ModalWindow<T> {}
impl<T> GenericCommandBarEvents for ModalWindow<T> {}
impl<T> CheckBoxEvents for ModalWindow<T> {}
impl<T> RadioBoxEvents for ModalWindow<T> {}
impl<T> PasswordEvents for ModalWindow<T> {}
impl<T> KeySelectorEvents for ModalWindow<T> {}
impl<T> TextFieldEvents for ModalWindow<T> {}
impl<T> ButtonEvents for ModalWindow<T> {}
impl<T> ToggleButtonEvents for ModalWindow<T> {}
impl<T> ColorPickerEvents for ModalWindow<T> {}
impl<T> ComboBoxEvents for ModalWindow<T> {}
impl<T> DatePickerEvents for ModalWindow<T> {}
impl<T> ListBoxEvents for ModalWindow<T> {}
impl<T> CustomEvents for ModalWindow<T> {}
impl<T> GenericSelectorEvents for ModalWindow<T> {}
impl<T> GenericDropDownListEvents for ModalWindow<T> {}
impl<T> GenericNumericSelectorEvents for ModalWindow<T> {}
impl<T> GenericListViewEvents for ModalWindow<T> {}
impl<T> OnDefaultAction for ModalWindow<T> {}
impl<T> WindowControl for ModalWindow<T> {}
impl<T> OnExpand for ModalWindow<T> {}
impl<T> ThreeStateBoxEvents for ModalWindow<T> {}
impl<T> OnSiblingSelected for ModalWindow<T> {}
impl<T> PathFinderEvents for ModalWindow<T> {}
impl<T> TimerEvents for ModalWindow<T> {}

// events routed to base window
impl<T> OnFocus for ModalWindow<T> {
    fn on_focus(&mut self) {
        OnFocus::on_focus(&mut self.base);
    }

    fn on_lose_focus(&mut self) {
        OnFocus::on_lose_focus(&mut self.base);
    }
}
impl<T> OnPaint for ModalWindow<T> {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        OnPaint::on_paint(&self.base, surface, theme);
    }
}
impl<T> OnResize for ModalWindow<T> {
    fn on_resize(&mut self, old_size: Size, new_size: Size) {
        OnResize::on_resize(&mut self.base, old_size, new_size);
    }
}
impl<T: 'static> OnKeyPressed for ModalWindow<T> {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        if !self.base.is_in_resize_mode() {
            match key.value() {
                key!("Enter") => {
                    if let Some(interface) = self.interface_mut() {
                        WindowEvents::on_accept(interface);
                    }
                    EventProcessStatus::Processed
                }
                key!("Escape") => {
                    if let Some(interface) = self.interface_mut() {
                        let result = WindowEvents::on_cancel(interface);
                        if result == ActionRequest::Allow {
                            // force the exit with None
                            self.exit();
                        } else {
                            // clean the result
                            self.result = None;
                            RuntimeManager::get().cancel_exit_from_execution_loop();
                        }
                    }
                    EventProcessStatus::Processed
                }
                _ => {
                    OnKeyPressed::on_key_pressed(&mut self.base, key, character)
                }
            }
        } else {
            OnKeyPressed::on_key_pressed(&mut self.base, key, character)
        }
    }
}
impl<T> OnMouseEvent for ModalWindow<T> {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        OnMouseEvent::on_mouse_event(&mut self.base, event)
    }
}
impl<T> OnWindowRegistered for ModalWindow<T> {
    fn on_registered(&mut self) {
        self.base.on_registered();
    }
}
impl<T: 'static> ModalWindowMethods<T> for ModalWindow<T> {
    fn show(self) -> Option<T> {
        // do nothing
        None
    }

    fn exit_with(&mut self, result: T) {
        self.result = Some(result);
        RuntimeManager::get().exit_execution_loop();
    }

    fn exit(&mut self) {
        self.result = None;
        RuntimeManager::get().exit_execution_loop();
    }

    fn close(&mut self) {
        self.exit();
    }
}
