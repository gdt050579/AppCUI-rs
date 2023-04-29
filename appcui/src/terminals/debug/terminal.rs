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
}
impl DebugTerminal {
    fn build_commands(script: &str) -> VecDeque<Command> {
        let mut v: VecDeque<Command> = VecDeque::with_capacity(16);
        for line in script.lines() {
            // skip empty lines
            if line.trim().len() == 0 {
                continue;
            }
            match Command::new(line.trim()) {
                Ok(cmd) => v.push_back(cmd),
                Err(err) => {
                    println!("{}",err.to_string());
                    panic!()
                },
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
            _ => "37", /* default is white */
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
}
impl Terminal for DebugTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        // only paint if requested
        if !self.paint {
            return;
        }
        self.paint = false;
        println!("\nPaint: {} -> Hash: {}",&self.paint_title,0);
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
            if let Some(title) = cmd.get_paint_command_title() {
                self.paint_title = title;
                RuntimeManager::get().request_repaint();
                self.paint = true;
            }
            return SystemEvent::None;
        }

        // if nothing else works, close the app (script has finished)
        return SystemEvent::AppClose;
    }
}
