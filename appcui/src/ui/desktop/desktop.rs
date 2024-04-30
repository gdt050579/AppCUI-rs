use crate::prelude::*;

#[CustomControl(overwrite=OnPaint+OnKeyPressed, internal=true, desktop=true)]
pub struct Desktop {}

impl Desktop {
    pub fn new() -> Self {
        Desktop {
            base: ControlBase::with_status_flags(
                Layout::new("x:0,y:0,w:100%,h:100%"),
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | StatusFlags::DesktopControl,
            ),
        }
    }
    fn interface_mut(&mut self) -> Option<&mut dyn Control> {
        if let Some(control) = RuntimeManager::get().get_controls_mut().get_mut(self.handle.cast()) {
            return Some(control.get_control_mut());
        }
        None
    }
    fn set_focus_for_child_window(&mut self, next_window: bool) {
        let mut idx = self.base.focused_child_index;
        let len = self.base.children.len();
        if next_window {
            idx.add(1, len, Strategy::RotateFromInvalidState);
        } else {
            idx.sub(1, len, Strategy::RotateFromInvalidState);
        }
        if (idx.in_range(len)) && (idx.index() != self.base.focused_child_index.index()) {
            let handle = self.base.children[idx.index()];
            if !handle.is_none() {
                RuntimeManager::get().request_focus_for_control(handle);
            }
        }
    }
}
impl OnPaint for Desktop {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(theme.desktop.character);
    }
}
impl OnKeyPressed for Desktop {
    fn on_key_pressed(&mut self, key: Key, _: char) -> EventProcessStatus {
        match key.get_compact_code() {
            key!("Escape") => {
                if let Some(desktop_interface) = self.interface_mut() {
                    if DesktopEvents::on_close(desktop_interface) == ActionRequest::Allow {
                        RuntimeManager::get().close();
                    }
                }
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Tab") | key!("Tab") => {
                self.set_focus_for_child_window(true);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Shift+Tab") | key!("Shift+Tab") => {
                self.set_focus_for_child_window(false);
                return EventProcessStatus::Processed;
            }
            key!("Alt+0") => {
                // Dialogs::WindowManager::Show();
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        // check controls hot keys
        if key.modifier.contains(KeyModifier::Alt) {
            let controls = RuntimeManager::get().get_controls_mut();
            for ctrl in self.children.iter() {
                if let Some(child) = controls.get_mut(*ctrl) {
                    if child.get_base_mut().hotkey() == key {
                        RuntimeManager::get().request_focus_for_control(*ctrl);
                        return EventProcessStatus::Processed;
                    }
                }
            }
        }
        EventProcessStatus::Ignored
    }
}
