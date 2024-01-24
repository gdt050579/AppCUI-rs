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

use super::RuntimeManager;

struct KeyPressed {
    key: Key,
    times: u32,
}
struct MouseWheel {
    x: i32,
    y: i32,
    dir: MouseWheelDirection,
    times: u32,
}
struct PaintCommand {
    state_name: String,
}
struct MouseClick {
    x: i32,
    y: i32,
    button: MouseButton,
}
enum Command {
    KeyPressed(KeyPressed),
    Resize(Size),
    MouseMove(MouseMoveEvent),
    MouseHold(MouseButtonDownEvent),
    MouseRelease(MouseButtonUpEvent),
    MouseWheel(MouseWheel),
    MouseClick(MouseClick),
    Paint(PaintCommand),
    CheckHash(u64),
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
            Command::MouseRelease(cmd) => write!(f, "Mouse.Release({},{})\n", cmd.x, cmd.y),
            Command::MouseClick(cmd) => write!(f, "Mouse.Click({},{},{})\n", cmd.x, cmd.y, cmd.button.get_name()),
            Command::MouseWheel(cmd) => {
                if cmd.times > 1 {
                    write!(f, "Mouse.Wheel({},{},{},{})\n", cmd.x, cmd.y, cmd.dir.get_name(), cmd.times)
                } else {
                    write!(f, "Mouse.Wheel({},{},{})\n", cmd.x, cmd.y, cmd.dir.get_name())
                }
            }
            Command::Paint(cmd) => write!(f, "Paint('{}')\n", cmd.state_name),
            Command::CheckHash(hash) => write!(f, "CheckHash(0x{:x})\n", hash),
        }
    }
}
pub(super) struct EventRecorder {
    commands: Vec<Command>,
    state_id: u32,
}
impl EventRecorder {
    pub(super) fn new() -> Self {
        Self {
            commands: Vec::with_capacity(512),
            state_id: 1,
        }
    }
    pub(super) fn save(&self) {
        let mut content = String::with_capacity(self.commands.len() * 32);
        let mut step = String::with_capacity(256);
        content.push_str("Paint.Enable(false)\n");
        for cmd in &self.commands {
            step += cmd.to_string().as_str();
            match cmd {
                Command::CheckHash(_) => {
                    // we need at least one check hash
                    content += step.as_str();
                    step.clear();
                }
                _ => {}
            }
        }
        let _ = fs::write("events.txt", content);
    }
    pub(super) fn add(&mut self, sys_event: &SystemEvent, terminal: &mut Box<dyn Terminal>, surface: &Surface) {
        match sys_event {
            SystemEvent::None => {}
            SystemEvent::AppClose => {}
            SystemEvent::KeyPressed(event) => {
                if self.add_keypressed(event.key) {
                    self.save_state(terminal, surface);
                    RuntimeManager::get().request_update();
                }
            }
            SystemEvent::KeyModifierChanged(_) => {}
            SystemEvent::Resize(new_size) => self.add_resize(*new_size),
            SystemEvent::MouseButtonDown(evnt) => self.add_mouse_button_down(evnt),
            SystemEvent::MouseButtonUp(evnt) => self.add_mouse_button_up(evnt),
            SystemEvent::MouseDoubleClick(_) => {},
            SystemEvent::MouseMove(evnt) => self.add_mouse_move(evnt),
            SystemEvent::MouseWheel(evnt) => self.add_mouse_wheel(evnt),
        }
    }
    fn compute_surface_hash(surface: &Surface) -> u64 {
        // use FNV algorithm ==> https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
        let mut hash = 0xcbf29ce484222325u64;
        let mut buf = [0u8; 8];
        for ch in &surface.chars {
            buf[0] = ((ch.code as u32) & 0xFF) as u8;
            buf[1] = (((ch.code as u32) >> 8) & 0xFF) as u8;
            buf[2] = (((ch.code as u32) >> 16) & 0xFF) as u8;
            buf[3] = (((ch.code as u32) >> 24) & 0xFF) as u8;
            buf[4] = ch.foreground as u8;
            buf[5] = ch.background as u8;
            buf[6] = ((ch.flags.get_value() >> 8) & 0xFF) as u8;
            buf[7] = (ch.flags.get_value() & 0xFF) as u8;
            for b in buf {
                hash = hash ^ (b as u64);
                hash = hash.wrapping_mul(0x00000100000001B3u64);
            }
        }
        return hash;
    }
    fn add_keypressed(&mut self, key: Key) -> bool {
        if key.get_compact_code() == key!("Ctrl+Alt+Space") {
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
        let mut is_click = false;
        if let Some(last) = self.commands.last_mut() {
            match last {
                Command::MouseHold(cmd) => {
                    is_click = (cmd.x == evnt.x) && (cmd.y == evnt.y);
                }
                _ => {}
            }
        }
        if is_click {
            let button = match self.commands.pop().unwrap() {
                Command::MouseHold(cmd) => cmd.button,
                _ => MouseButton::None
            };
            self.commands.push(Command::MouseClick(MouseClick {
                x: evnt.x,
                y: evnt.y,
                button,
            }));
        } else {
            self.commands.push(Command::MouseRelease(*evnt));
        }
    }
    fn add_mouse_wheel(&mut self, evnt: &MouseWheelEvent) {
        if let Some(last) = self.commands.last_mut() {
            match last {
                Command::MouseWheel(cmd) => {
                    if (cmd.x == evnt.x) && (cmd.y == evnt.y) && (cmd.dir == evnt.direction) {
                        cmd.times += 1;
                        return;
                    }
                }
                _ => {}
            }
        }
        self.commands.push(Command::MouseWheel(MouseWheel {
            x: evnt.x,
            y: evnt.y,
            dir: evnt.direction,
            times: 1,
        }));
    }
    fn save_state(&mut self, terminal: &mut Box<dyn Terminal>, surface: &Surface) {
        let sz = surface.get_size();
        let mut screen = Surface::new(sz.width, sz.height);
        let mut state_name = format!("State_{}", self.state_id);
        loop {
            // paint
            screen.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
            screen.draw_surface(0, 0, surface);
            screen.clear(Character::with_color(Color::Gray, Color::Black));
            screen.fill_rect(
                Rect::new(0, 0, (sz.width as i32) - 1, 2),
                Character::new(' ', Color::White, Color::DarkBlue, CharFlags::None),
            );
            screen.draw_rect(
                Rect::new(0, 0, (sz.width as i32) - 1, 3),
                LineType::Single,
                CharAttribute::with_color(Color::White, Color::DarkBlue),
            );
            screen.write_string(1, 1, "State name:", CharAttribute::with_fore_color(Color::Silver), false);
            screen.fill_horizontal_line(
                12,
                1,
                (sz.width as i32) - 2,
                Character::new(' ', Color::White, Color::Black, CharFlags::None),
            );
            screen.write_string(13, 1, &state_name, CharAttribute::with_fore_color(Color::White), false);
            screen.set_cursor(13 + state_name.chars().count() as i32, 1);
            terminal.update_screen(&screen);
            // get the events
            let sys_event = terminal.get_system_event();
            match sys_event {
                SystemEvent::KeyPressed(evnt) => match evnt.key.get_compact_code() {
                    key!("Escape") => {
                        return;
                    }
                    key!("Enter") => {
                        self.state_id += 1;
                        self.commands.push(Command::Paint(PaintCommand { state_name }));
                        self.commands.push(Command::CheckHash(EventRecorder::compute_surface_hash(surface)));
                        return;
                    }
                    key!("Backspace") => {
                        // delete last character
                        state_name.pop();
                    }
                    _ => {
                        if evnt.character >= ' ' {
                            state_name.push(evnt.character);
                        }
                    }
                },
                _ => {}
            }
        }
    }
}
