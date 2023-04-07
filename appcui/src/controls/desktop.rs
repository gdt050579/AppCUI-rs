use super::events::*;
use super::ControlBase;
use super::Layout;
use super::StatusFlags;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;
use AppCUIProcMacro::*;

#[AppCUIControl(overwrite=OnPaint+OnKeyPressed)]
pub struct Desktop {}

impl Desktop {
    pub(crate) fn new() -> Self {
        Desktop {
            base: ControlBase::new(
                Layout::new("x:0,y:0,w:100%,h:100%"),
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput,
            ),
        }
    }
}
impl OnPaint for Desktop {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(theme.desktop.character);
        surface.write_string(
            0,
            0,
            "Desktop",
            CharAttribute::with_color(Color::White, Color::Red),
            false,
        );
    }
}
impl OnKeyPressed for Desktop {
    fn on_key_pressed(&mut self, key: Key, _: char) -> EventProcessStatus {
        match key.get_compact_code() {
            key!("Escape") => {
                RuntimeManager::get().close();
                return EventProcessStatus::Processed;
            },
            key!("Ctrl+Tab")=> {
                // GoToNextWindow(Members, 1);
                return  EventProcessStatus::Processed;
            },
            key!("Ctrl+Shift+Tab") => {
                // GoToNextWindow(Members, -1);
                return EventProcessStatus::Processed;
            },
            key!("Alt+0") => {
                // Dialogs::WindowManager::Show();
                return EventProcessStatus::Processed;
            },
            _ => {}
        }
        // check controls hot keys
        if key.modifier.contains(KeyModifier::Alt) {
            let rm = RuntimeManager::get();
            for ctrl in self.children.iter() {
                if let Some(child) = rm.get_control(*ctrl) {
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