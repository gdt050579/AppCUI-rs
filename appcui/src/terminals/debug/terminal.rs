use std::collections::VecDeque;

use super::super::Surface;
use super::super::SystemEvent;
use super::super::Terminal;
use super::command::Command;
use crate::graphics::Color;
use crate::system::Error;
use crate::system::InitializationData;
use crate::system::RuntimeManager;

pub(crate) struct DebugTerminal {
    width: u32,
    height: u32,
    temp_str: String,
    commands: VecDeque<Command>,
    sys_events: VecDeque<SystemEvent>,
    paint: bool,
    paint_title: String,
    hash_to_test: Option<u64>,
}
impl DebugTerminal {
    fn build_commands(script: &str) -> VecDeque<Command> {
        let mut v: VecDeque<Command> = VecDeque::with_capacity(16);
        for line in script.lines() {
            // skip empty lines
            let trim_line = line.trim();
            if (trim_line.len() == 0)
                || (trim_line.starts_with(";"))
                || (trim_line.starts_with("#"))
                || (trim_line.starts_with("//"))
            {
                continue;
            }
            match Command::new(line.trim()) {
                Ok(cmd) => v.push_back(cmd),
                Err(err) => {
                    println!("{}", err.to_string());
                    panic!()
                }
            }
        }
        v
    }
    pub(crate) fn create(data: &InitializationData) -> Result<Box<dyn Terminal>, Error> {
        let mut w = if data.size.is_none() {
            80
        } else {
            data.size.unwrap().width as u32
        };
        let mut h = if data.size.is_none() {
            40
        } else {
            data.size.unwrap().height as u32
        };
        w = w.clamp(10, 1000);
        h = h.clamp(10, 1000);
        let commands = DebugTerminal::build_commands(data.debug_script.as_str());
        Ok(Box::new(DebugTerminal {
            width: w,
            height: h,
            temp_str: String::with_capacity((w * h) as usize),
            commands,
            sys_events: VecDeque::with_capacity(8),
            paint: false,
            paint_title: String::new(),
            hash_to_test: None,
        }))
    }
    fn forecolor_to_str(col: Color) -> &'static str {
        match col {
            Color::Black => "30",
            Color::DarkRed => "31",
            Color::DarkGreen => "32",
            Color::Olive => "33",
            Color::DarkBlue => "34",
            Color::Magenta => "35",
            Color::Teal => "36",
            Color::Silver => "37",
            Color::Gray => "90",
            Color::Red => "91",
            Color::Green => "92",
            Color::Yellow => "93",
            Color::Blue => "94",
            Color::Pink => "95",
            Color::Aqua => "96",
            Color::White => "97",
            _ => "37", /* default is silver */
        }
    }
    fn backcolor_to_str(col: Color) -> &'static str {
        match col {
            Color::Black => "40",
            Color::DarkRed => "41",
            Color::DarkGreen => "42",
            Color::Olive => "43",
            Color::DarkBlue => "44",
            Color::Magenta => "45",
            Color::Teal => "46",
            Color::Silver => "47",
            Color::Gray => "100",
            Color::Red => "101",
            Color::Green => "102",
            Color::Yellow => "103",
            Color::Blue => "104",
            Color::Pink => "105",
            Color::Aqua => "106",
            Color::White => "107",
            _ => "40", /* default is black */
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
}
impl Terminal for DebugTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        let surface_hash = DebugTerminal::compute_surface_hash(surface);
        if let Some(hash_to_test) = self.hash_to_test {
            if hash_to_test != surface_hash {
                panic!(
                    "Invalid hash for surface (expecting: 0x{:X} but found 0x{:X})",
                    hash_to_test, surface_hash
                );
            }
            // no need to paint --> just a check hash command
            self.paint = false;
        }
        self.hash_to_test = None;
        // only paint if requested
        if !self.paint {
            return;
        }
        self.paint = false;
        println!(
            "\nPaint: {} -> Hash: 0x{:X}",
            &self.paint_title, surface_hash
        );
        self.temp_str.clear();
        let mut x = 0u32;
        for ch in &surface.chars {
            self.temp_str.push_str("\x1b[");
            self.temp_str
                .push_str(DebugTerminal::forecolor_to_str(ch.foreground));
            self.temp_str.push(';');
            self.temp_str
                .push_str(DebugTerminal::backcolor_to_str(ch.background));
            self.temp_str.push_str("m");
            if ch.code < ' ' {
                self.temp_str.push(' ');
            } else {
                self.temp_str.push(ch.code);
            }
            self.temp_str.push_str("\x1b[0m"); // reset to default color
            x += 1;
            if x == self.width {
                println!("{}", &self.temp_str);
                self.temp_str.clear();
                x = 0;
            }
        }
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }
    fn on_resize(&mut self, new_size: crate::graphics::Size) {
        self.width = new_size.width;
        self.height = new_size.height;
    }
    fn get_system_event(&mut self) -> SystemEvent {
        // if there is any event in the que --> return that event
        if let Some(event) = self.sys_events.pop_front() {
            match event {
                SystemEvent::Resize(new_size) => {
                    self.width = new_size.width;
                    self.height = new_size.height;
                }
                _ => {}
            }
            return event;
        }
        // if no events are in the event queue --> check if a command is present
        if let Some(cmd) = self.commands.pop_front() {
            cmd.generate_event(&mut self.sys_events);
            // check for paint command
            if let Some(title) = cmd.get_paint_command_title() {
                self.paint_title = title;
                RuntimeManager::get().request_repaint();
                self.paint = true;
                return SystemEvent::None;
            }
            // check for CheckHash command
            if let Some(hash) = cmd.get_screen_hash() {
                self.paint = false; // I don't want to paint anything --> just store the hash
                self.hash_to_test = Some(hash); // next time I paint --> I will check it
                RuntimeManager::get().request_repaint();
                return SystemEvent::None;
            }
            return SystemEvent::None;
        }

        // if nothing else works, close the app (script has finished)
        return SystemEvent::AppClose;
    }
}
