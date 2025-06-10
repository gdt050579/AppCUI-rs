use std::collections::VecDeque;

use super::super::Surface;
use super::super::SystemEvent;
use super::super::Terminal;
use super::command::Command;
use crate::graphics::Color;
use crate::graphics::Point;
use crate::graphics::Size;
use crate::input::KeyModifier;
use crate::system::Error;
use crate::system::{PaintMethods, RuntimeManager};

pub(crate) struct DebugTerminal {
    size: Size,
    temp_str: String,
    commands: VecDeque<Command>,
    sys_events: VecDeque<SystemEvent>,
    paint: bool,
    ignore_paint_command: bool,
    paint_title: String,
    hash_to_test: Option<u64>,
    cursor_point_to_check: Option<Point>,
    mouse_pos: Point,
    keymodifier_state: KeyModifier,
    errors_disabled: bool,
    clipboard_text: String,
}
impl DebugTerminal {
    fn build_commands(script: &str) -> VecDeque<Command> {
        let mut v: VecDeque<Command> = VecDeque::with_capacity(16);
        for line in script.lines() {
            // skip empty lines
            let trim_line = line.trim();
            if (trim_line.is_empty()) || (trim_line.starts_with(';')) || (trim_line.starts_with('#')) || (trim_line.starts_with("//")) {
                continue;
            }
            match Command::new(line.trim()) {
                Ok(cmd) => v.push_back(cmd),
                Err(err) => {
                    println!("{}", err.as_string());
                    panic!()
                }
            }
        }
        v
    }
    pub(crate) fn new(builder: &crate::system::Builder) -> Result<Self, Error> {
        let mut w = if builder.size.is_none() { 80 } else { builder.size.unwrap().width };
        let mut h = if builder.size.is_none() { 40 } else { builder.size.unwrap().height };
        w = w.clamp(10, 1000);
        h = h.clamp(10, 1000);
        let commands = DebugTerminal::build_commands(builder.debug_script.as_ref().unwrap().as_str());
        Ok(DebugTerminal {
            size: Size::new(w, h),
            temp_str: String::with_capacity((w * h) as usize),
            commands,
            sys_events: VecDeque::with_capacity(8),
            paint: false,
            ignore_paint_command: false,
            errors_disabled: false,
            paint_title: String::new(),
            hash_to_test: None,
            cursor_point_to_check: None,
            mouse_pos: Point::new(0, 0),
            keymodifier_state: KeyModifier::None,
            clipboard_text: String::new(),
        })
    }
    fn color_to_str(col: Color) -> String {
        match col {
            Color::Black => String::from("0;0;0"),
            Color::DarkRed => String::from("128;0;0"),
            Color::DarkGreen => String::from("0;128;0"),
            Color::Olive => String::from("128;128;0"),
            Color::DarkBlue => String::from("0;0;128"),
            Color::Magenta => String::from("128;0;128"),
            Color::Teal => String::from("0;128;128"),
            Color::Silver => String::from("196;196;196"),
            Color::Gray => String::from("128;128;128"),
            Color::Red => String::from("255;0;0"),
            Color::Green => String::from("0;255;0"),
            Color::Yellow => String::from("255;255;0"),
            Color::Blue => String::from("0;0;255"),
            Color::Pink => String::from("255;0;255"),
            Color::Aqua => String::from("0;255;255"),
            Color::White => String::from("255;255;255"),
            Color::RGB(r, g, b) => format!("{};{};{}", r, g, b),
            _ => String::from("255;255;255"), /* default is white */
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
            buf[4] = ch.foreground.as_color_index();
            buf[5] = ch.background.as_color_index();
            buf[6] = ((ch.flags.get_value() >> 8) & 0xFF) as u8;
            buf[7] = (ch.flags.get_value() & 0xFF) as u8;
            for b in buf {
                hash ^= b as u64;
                hash = hash.wrapping_mul(0x00000100000001B3u64);
            }
            if let Some((r, g, b)) = ch.foreground.rgb() {
                hash ^= r as u64;
                hash = hash.wrapping_mul(0x00000100000001B3u64);
                hash ^= g as u64;
                hash = hash.wrapping_mul(0x00000100000001B3u64);
                hash ^= b as u64;
                hash = hash.wrapping_mul(0x00000100000001B3u64);
            }
            if let Some((r, g, b)) = ch.background.rgb() {
                hash ^= r as u64;
                hash = hash.wrapping_mul(0x00000100000001B3u64);
                hash ^= g as u64;
                hash = hash.wrapping_mul(0x00000100000001B3u64);
                hash ^= b as u64;
                hash = hash.wrapping_mul(0x00000100000001B3u64);
            }
        }
        hash
    }
}
impl Terminal for DebugTerminal {
    fn is_single_threaded(&self) -> bool {
        true
    }

    fn update_screen(&mut self, surface: &Surface) {
        let surface_hash = DebugTerminal::compute_surface_hash(surface);
        if let Some(hash_to_test) = self.hash_to_test {
            // no need to paint --> just a check hash command
            self.paint = false;
            if hash_to_test != surface_hash {
                if self.errors_disabled {
                    println!(
                        "\x1b[91;40m[Error] Invalid hash: (expecting: 0x{:X} but found 0x{:X})\x1b[0m",
                        hash_to_test, surface_hash
                    );
                    //println!("        at: {}",&self.paint_title);
                } else {
                    panic!(
                        "Invalid hash for surface (expecting: 0x{:X} but found 0x{:X})",
                        hash_to_test, surface_hash
                    );
                }
            }
        }
        let cursor = if !surface.cursor.is_visible() {
            Point::new(-1, -1)
        } else {
            Point::new(surface.cursor.x as i32, surface.cursor.y as i32)
        };
        if let Some(point) = self.cursor_point_to_check {
            if point != cursor {
                let cursor_pos = format!("({},{})", cursor.x, cursor.y);
                let cursor_repr = if cursor.x < 0 { "Hidden" } else { cursor_pos.as_str() };
                let point_pos = format!("({},{})", point.x, point.y);
                let point_repr = if point.x < 0 { "Hidden" } else { point_pos.as_str() };
                if self.errors_disabled {
                    println!(
                        "\x1b[91;40m[Error] Invalid cursor position. Expectig the cursor to be {}, but found {}\x1b[0m",
                        point_repr, cursor_repr
                    );
                } else {
                    panic!(
                        "Invalid cursor position. Expectig the cursor to be {}, but found {}",
                        point_repr, cursor_repr
                    );
                }
            }
        }

        self.hash_to_test = None;
        self.cursor_point_to_check = None;
        // only paint if requested
        if !self.paint {
            return;
        }
        self.paint = false;

        println!();
        self.temp_str.clear();
        // firt border
        for _ in 0..=6 + self.size.width {
            self.temp_str.push('=');
        }
        println!("+{}+", self.temp_str);
        self.temp_str.clear();

        // name
        self.temp_str.push_str("| Name  : \x1b[93;40m");
        self.temp_str.push_str(&self.paint_title);
        while self.temp_str.len() < (self.size.width + 16) as usize {
            self.temp_str.push(' ');
        }
        self.temp_str.push_str("\x1b[0m|");
        println!("{}", &self.temp_str);
        self.temp_str.clear();

        // hash
        self.temp_str.push_str("| Hash  : \x1b[93;40m");
        self.temp_str.push_str(format!("0x{:X}", surface_hash).as_str());
        while self.temp_str.len() < (self.size.width + 16) as usize {
            self.temp_str.push(' ');
        }
        self.temp_str.push_str("\x1b[0m|");
        println!("{}", &self.temp_str);
        self.temp_str.clear();

        // cursor
        self.temp_str.push_str("| Cursor: \x1b[93;40m");
        if !surface.cursor.is_visible() {
            self.temp_str.push_str("Hidden");
        } else {
            self.temp_str.push_str(format!("{},{}", cursor.x, cursor.y).as_str());
        }
        while self.temp_str.len() < (self.size.width + 16) as usize {
            self.temp_str.push(' ');
        }
        self.temp_str.push_str("\x1b[0m|");
        println!("{}", &self.temp_str);
        self.temp_str.clear();

        // separator line
        self.temp_str.push('|');
        for _ in 0..=6 + self.size.width {
            self.temp_str.push('-');
        }
        self.temp_str.push('|');
        println!("{}", &self.temp_str);
        self.temp_str.clear();

        // second digit
        self.temp_str.push_str("|    | ");
        for i in 0..self.size.width {
            let digit = ((i % 100) / 10) as u8;
            if (i as i32) == self.mouse_pos.x {
                self.temp_str.push_str("\x1b[97m");
                self.temp_str.push_str("\x1b[41m");
            } else {
                self.temp_str.push_str("\x1b[0m");
            }
            if digit == 0 {
                self.temp_str.push(' ');
            } else {
                self.temp_str.push((48u8 + digit) as char);
            }
        }
        println!("{}\x1b[0m |", self.temp_str);
        self.temp_str.clear();

        // last digit
        self.temp_str.push_str("|    | ");
        for i in 0..self.size.width {
            if (i as i32) == self.mouse_pos.x {
                self.temp_str.push_str("\x1b[97m");
                self.temp_str.push_str("\x1b[41m");
            } else {
                self.temp_str.push_str("\x1b[0m");
            }
            self.temp_str.push((48u8 + ((i % 10) as u8)) as char);
        }
        println!("{}\x1b[0m |", self.temp_str);
        self.temp_str.clear();

        // separator line
        self.temp_str.push('|');
        for _ in 0..=6 + self.size.width {
            self.temp_str.push('-');
        }
        self.temp_str.push('|');
        println!("{}", &self.temp_str);
        self.temp_str.clear();

        let mut x = 0u32;
        let mut y = 0u32;
        for ch in &surface.chars {
            let mut fore = ch.foreground;
            let mut back = ch.background;
            if (x as i32 == cursor.x) && (y as i32 == cursor.y) {
                fore = ch.background;
                back = ch.foreground;
            }
            self.temp_str.push_str("\x1b[38;2;");
            self.temp_str.push_str(DebugTerminal::color_to_str(fore).as_str());
            self.temp_str.push_str("m\x1b[48;2;");
            self.temp_str.push_str(DebugTerminal::color_to_str(back).as_str());
            self.temp_str.push('m');
            if ch.code <= ' ' {
                self.temp_str.push(' ');
            } else {
                self.temp_str.push(ch.code);
            }
            self.temp_str.push_str("\x1b[0m"); // reset to default color
            x += 1;
            if x == self.size.width {
                if (y as i32) == self.mouse_pos.y {
                    println!("|\x1b[97m\x1b[41m{:>3} \x1b[0m| {} |", y, &self.temp_str);
                } else {
                    println!("|{:>3} | {} |", y, &self.temp_str);
                }
                self.temp_str.clear();
                x = 0;
                y += 1;
            }
        }
        // separator line
        self.temp_str.push('|');
        for _ in 0..=6 + self.size.width {
            self.temp_str.push('-');
        }
        self.temp_str.push('|');
        println!("{}", &self.temp_str);
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn query_system_event(&mut self) -> Option<SystemEvent> {
        // if there is any event in the que --> return that event
        if let Some(event) = self.sys_events.pop_front() {
            match event {
                SystemEvent::Resize(new_size) => {
                    self.size.width = new_size.width;
                    self.size.height = new_size.height;
                }
                SystemEvent::MouseButtonDown(evnt) => {
                    self.mouse_pos.x = evnt.x;
                    self.mouse_pos.y = evnt.y;
                }
                SystemEvent::MouseButtonUp(evnt) => {
                    self.mouse_pos.x = evnt.x;
                    self.mouse_pos.y = evnt.y;
                }
                SystemEvent::MouseDoubleClick(evnt) => {
                    self.mouse_pos.x = evnt.x;
                    self.mouse_pos.y = evnt.y;
                }
                SystemEvent::MouseMove(evnt) => {
                    self.mouse_pos.x = evnt.x;
                    self.mouse_pos.y = evnt.y;
                }
                SystemEvent::MouseWheel(evnt) => {
                    self.mouse_pos.x = evnt.x;
                    self.mouse_pos.y = evnt.y;
                }
                SystemEvent::KeyModifierChanged(evnt) => {
                    self.keymodifier_state = evnt.new_state;
                }
                _ => {}
            }
            return Some(event);
        }
        // if no events are in the event queue --> check if a command is present
        if let Some(cmd) = self.commands.pop_front() {
            cmd.generate_event(self.mouse_pos, self.keymodifier_state, &mut self.sys_events);
            // check for paint command
            if !self.ignore_paint_command {
                if let Some(title) = cmd.get_paint_command_title() {
                    self.paint_title = title;
                    RuntimeManager::get().request_repaint();
                    self.paint = true;
                    return None;
                }
            }
            match cmd {
                Command::MouseHold(_)
                | Command::MouseRelease(_)
                | Command::MouseClick(_)
                | Command::MouseDoubleClick(_)
                | Command::MouseMove(_)
                | Command::MouseDrag(_)
                | Command::MouseWheel(_)
                | Command::Paint(_)
                | Command::Resize(_)
                | Command::KeyPresed(_)
                | Command::KeyModifier(_)
                | Command::KeyTypeText(_) => {
                    return None;
                }
                Command::PaintEnable(obj) => {
                    self.ignore_paint_command = !obj.is_paint_enabled();
                    return None;
                }
                Command::ErrorDisable(obj) => {
                    self.errors_disabled = obj.is_error_disabled();
                    return None;
                }
                Command::CheckHash(obj) => {
                    self.paint = false; // I don't want to paint anything --> just store the hash
                    self.hash_to_test = Some(obj.get_hash()); // next time I paint --> I will check it
                    RuntimeManager::get().request_repaint();
                    return None;
                }
                Command::CheckCursor(obj) => {
                    self.paint = false; // I don't want to paint anything --> just store the hash
                    self.cursor_point_to_check = Some(obj.get_point()); // next time I paint --> I will check it
                    RuntimeManager::get().request_repaint();
                    return None;
                }

                Command::ClipboardSetText(obj) => {
                    self.set_clipboard_text(obj.get_text());
                    return None;
                }
                Command::ClipboardClear(_) => {
                    self.set_clipboard_text("");
                    return None;
                }
                Command::CheckClipboardText(obj) => {
                    if obj.get_text() != self.clipboard_text {
                        if self.errors_disabled {
                            println!(
                                "\x1b[91;40m[Error] Invalid clipboard text: (expecting: '{}' but found '{}')\x1b[0m",
                                obj.get_text(),
                                self.clipboard_text
                            );
                        } else {
                            panic!(
                                "Invalid clipboard text: (expecting: '{}' but found '{}')",
                                obj.get_text(),
                                self.clipboard_text
                            );
                        }
                    }
                    return None;
                }
            }
        }

        // if nothing else works, close the app (script has finished)
        Some(SystemEvent::AppClose)
    }

    fn get_clipboard_text(&self) -> Option<String> {
        if self.clipboard_text.is_empty() {
            None
        } else {
            Some(self.clipboard_text.clone())
        }
    }

    fn set_clipboard_text(&mut self, text: &str) {
        self.clipboard_text.clear();
        self.clipboard_text.push_str(text);
    }

    fn has_clipboard_text(&self) -> bool {
        !self.clipboard_text.is_empty()
    }

    fn on_resize(&mut self, new_size: Size) {
        self.size = new_size;
    }
}
