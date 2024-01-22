use crate::graphics::*;
use crate::input::*;
use crate::terminals::{SystemEvent, Terminal};
use AppCUIProcMacro::*;

struct KeyPressed {
    key: Key,
    times: u32,
}
enum Command {
    KeyPressed(KeyPressed),
}
pub(super) struct EventRecorder {
    events: Vec<Command>,
}
impl EventRecorder {
    pub(super) fn new() -> Self {
        Self {
            events: Vec::with_capacity(512),
        }
    }
    pub(super) fn save() {}
    pub(super) fn add(&mut self, sys_event: &SystemEvent, terminal: &mut Box<dyn Terminal>, surface: &Surface) {
        match sys_event {
            SystemEvent::None => todo!(),
            SystemEvent::AppClose => todo!(),
            SystemEvent::KeyPressed(event) => {
                if self.add_keypressed(event.key) {
                    self.save_state(terminal, surface);
                }
            }
            SystemEvent::KeyModifierChanged(_) => todo!(),
            SystemEvent::Resize(_) => todo!(),
            SystemEvent::MouseButtonDown(_) => todo!(),
            SystemEvent::MouseButtonUp(_) => todo!(),
            SystemEvent::MouseDoubleClick(_) => todo!(),
            SystemEvent::MouseMove(_) => todo!(),
            SystemEvent::MouseWheel(_) => todo!(),
        }
    }
    fn add_keypressed(&mut self, key: Key) -> bool {
        if key.get_compact_code() == key!("Ctrl+Alt+Shift+Space") {
            // save state
            return true;
        }
        let count = self.events.len();
        if count > 0 {
            let cmd = &mut self.events[count - 1];
            match cmd {
                Command::KeyPressed(c) => {
                    if c.key == key {
                        c.times += 1;
                        return false;
                    }
                }
                _ => {}
            }
        }
        self.events.push(Command::KeyPressed(KeyPressed { key, times: 1 }));
        return false;
    }
    fn save_state(&mut self, terminal: &mut Box<dyn Terminal>, surface: &Surface) {

    }
}
