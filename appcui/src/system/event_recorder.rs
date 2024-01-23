use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fs;

use crate::graphics::*;
use crate::input::*;
use crate::terminals::MouseButtonDownEvent;
use crate::terminals::MouseButtonUpEvent;
use crate::terminals::MouseMoveEvent;
use crate::terminals::MouseWheelEvent;
use crate::terminals::{SystemEvent, Terminal};
use AppCUIProcMacro::*;

struct KeyPressed {
    key: Key,
    times: u32,
}
enum Command {
    KeyPressed(KeyPressed),
    Resize(Size),
    MouseMove(MouseMoveEvent),
    MouseHold(MouseButtonDownEvent),
    MouseRelease(MouseButtonUpEvent),
    MouseWheel(MouseWheelEvent),
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
            Command::Resize(sz) => write!(f, "Resize({},{})\n", sz.width, sz.height),
            Command::MouseMove(cmd) => write!(f, "Mouse.Move({},{})\n", cmd.x, cmd.y),
            Command::MouseHold(cmd) => write!(f, "Mouse.Hold({},{},{})\n", cmd.x, cmd.y, cmd.button.get_name()),
            Command::MouseRelease(cmd) => write!(f, "Mouse.Release({},{},{})\n", cmd.x, cmd.y, cmd.button.get_name()),
            Command::MouseWheel(cmd) => write!(f, "Mouse.Wheel({},{},{})\n", cmd.x, cmd.y, cmd.direction.get_name()),
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
        content.push_str("Paint.Enable(false)\n");
        for cmd in &self.commands {
            step += cmd.to_string().as_str();
            content += step.as_str();
            step.clear();
        }
        let _ = fs::write("events.txt", content);
    }
    pub(super) fn add(&mut self, sys_event: &SystemEvent, terminal: &mut Box<dyn Terminal>, surface: &Surface) {
        match sys_event {
            SystemEvent::None => {}
            SystemEvent::AppClose => todo!(),
            SystemEvent::KeyPressed(event) => {
                if self.add_keypressed(event.key) {
                    self.save_state(terminal, surface);
                }
            }
            SystemEvent::KeyModifierChanged(_) => todo!(),
            SystemEvent::Resize(new_size) => self.add_resize(*new_size),
            SystemEvent::MouseButtonDown(evnt) => self.add_mouse_button_down(evnt),
            SystemEvent::MouseButtonUp(evnt) => self.add_mouse_button_up(evnt),
            SystemEvent::MouseDoubleClick(_) => todo!(),
            SystemEvent::MouseMove(evnt) => self.add_mouse_move(evnt),
            SystemEvent::MouseWheel(evnt) => self.add_mouse_wheel(evnt),
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
    fn add_mouse_move(&mut self, evnt: &MouseMoveEvent) {
        if let Some(last) = self.commands.last_mut() {
            match last {
                Command::MouseMove(cmd) => {
                    cmd.x = evnt.x;
                    cmd.y = evnt.y;
                    return;
                }
                _ => {}
            }
        }
        self.commands.push(Command::MouseMove(*evnt));
    }
    fn add_mouse_button_down(&mut self, evnt: &MouseButtonDownEvent) {
        self.commands.push(Command::MouseHold(*evnt));
    }
    fn add_mouse_button_up(&mut self, evnt: &MouseButtonUpEvent) {
        self.commands.push(Command::MouseRelease(*evnt));
    }
    fn add_mouse_wheel(&mut self, evnt: &MouseWheelEvent) {
        self.commands.push(Command::MouseWheel(*evnt));
    }
    fn save_state(&mut self, terminal: &mut Box<dyn Terminal>, surface: &Surface) {}
}
