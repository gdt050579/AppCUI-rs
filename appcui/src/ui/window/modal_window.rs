use super::{Flags, Window, events::ModalWindowMethods};
use crate::prelude::*;
use std::ops::{Deref, DerefMut};


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
        Self {
            base: Window::new(title, layout, flags),
            result: None,
        }
    }
}
impl<T> Control for ModalWindow<T> { }
impl<T> MenuEvents for ModalWindow<T> {} 
impl<T> DesktopEvents for ModalWindow<T> {} 
impl<T> ToolBarEvents for ModalWindow<T> {} 
impl<T> WindowEvents for ModalWindow<T> {} 
impl<T> CommandBarEvents for ModalWindow<T> {} 
impl<T> CheckBoxEvents for ModalWindow<T> {} 
impl<T> ButtonEvents for ModalWindow<T> {} 
impl<T> OnWindowRegistered for ModalWindow<T> {} 
impl<T> OnDefaultAction for ModalWindow<T> {}
impl<T> WindowControl for ModalWindow<T> {}

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
impl<T> OnKeyPressed for ModalWindow<T> {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        return OnKeyPressed::on_key_pressed(&mut self.base, key, character); 
    }
}
impl<T> OnMouseEvent for ModalWindow<T> {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        return OnMouseEvent::on_mouse_event(&mut self.base, event);
    }
}






impl<T> ModalWindowMethods<T> for ModalWindow<T> {
    fn show(self) -> Option<T> {
        let handle = RuntimeManager::get().add_modal_window(self);
        // safety check - if we did not manage to add the window
        if handle.is_none() {
            return None;
        }        
        // run the loop (the method will determine if it runs in a modal way or not)
        RuntimeManager::get().run();
        // the loop has ended , lets grab the results
        if let Some(obj) = RuntimeManager::get().get_control_mut(handle) {
            // mve the result
            return obj.result.take();
        }
        None
    }

    fn exit_with(&mut self, result: T) {
        self.result = Some(result);
    }

    fn exit(&mut self) {
        self.result = None;
    }
}
