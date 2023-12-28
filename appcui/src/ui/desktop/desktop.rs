use crate::prelude::*;

#[CustomControl(overwrite=OnPaint+OnKeyPressed, internal=true, desktop=true)]
pub struct Desktop {}

impl Desktop {
    pub fn new() -> Self {
        Desktop {
            base: ControlBase::with_status_flags(
                Layout::new("x:0,y:0,w:100%,h:100%"),
                StatusFlags::Visible
                    | StatusFlags::Enabled
                    | StatusFlags::AcceptInput
                    | StatusFlags::DesktopControl,
            ),
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
                RuntimeManager::get().close();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Tab") => {
                // GoToNextWindow(Members, 1);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Shift+Tab") => {
                // GoToNextWindow(Members, -1);
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
                    if child.get_base_mut().get_hotkey() == key {
                        // child.set_focus();
                        return EventProcessStatus::Processed;
                    }
                }
            }
        }
        EventProcessStatus::Ignored
    }
}

