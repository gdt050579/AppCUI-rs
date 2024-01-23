use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::write;
use std::fs;

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
    Resize(Size),
}
impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Command::KeyPressed(cmd) => {
                if cmd.times > 1 {
                    write!(f, "Key.Pressed({},{})\n", cmd.key, cmd.times)
                } else {
                    write!(f, "Key.Pressed({})\n", cmd.key)
                }
            }
            Command::Resize(sz) => write!(f, "Resize({},{})",sz.width,sz.height),            
        }
    }
}
pub(super) struct EventRecorder {
    commands: Vec<Command>,
}
impl EventRecorder {
    pub(super) fn new() -> Self {
        Self {
            commands: Vec::with_capacity(512),
        }
    }
    pub(super) fn save(&self) {
        let mut content = String::with_capacity(self.commands.len() * 32);
        let mut step = String::with_capacity(256);
        for cmd in &self.commands {
            step += cmd.to_string().as_str();
            content += step.as_str();
            step.clear();
        }
        let _ = fs::write("events.txt", content);
    }
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
            SystemEvent::Resize(new_size) => self.add_resize(*new_size),
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
        if let Some(last) = self.commands.last_mut() {
            match last {
                Command::KeyPressed(c) => {
                    if c.key == key {
                        c.times += 1;
                        return false;
                    }
                }
                _ => {}
            }
        }
        self.commands.push(Command::KeyPressed(KeyPressed { key, times: 1 }));
        return false;
    }
    fn add_resize(&mut self, new_size: Size) {
        if let Some(last) = self.commands.last_mut() {
            match last {
                Command::Resize(sz) => {
                    sz.width = new_size.width;
                    sz.height = new_size.height;
                    return;
                }
                _ => {}
            }
        }
        self.commands.push(Command::Resize(new_size));
    }
    fn save_state(&mut self, terminal: &mut Box<dyn Terminal>, surface: &Surface) {}
}
