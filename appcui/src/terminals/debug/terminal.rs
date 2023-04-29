use super::super::Surface;
use super::super::SystemEvent;
use super::super::Terminal;
use super::command::Command;
use crate::graphics::Color;
use crate::system::Error;
use crate::system::InitializationData;

pub(crate) struct DebugTerminal {
    width: u32,
    height: u32,
    temp_str: String,
    commands: Vec<Command>,
}
impl DebugTerminal {
    fn build_commands(script: &str)->Result<Vec<Command>,Error> {
        let mut v: Vec<Command> = Vec::with_capacity(16);
        for line in script.lines() {
            // skip empty lines
            if line.trim().len()==0 {
                continue;
            }
            match Command::new(line.trim()) {
                Ok(cmd) => v.push(cmd),
                Err(_) => {
                    return Err(Error::ScriptParsingError)
                }
            }
        }
        Ok(v)
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
        let commands = DebugTerminal::build_commands(data.debug_script.as_str())?;
        Ok(Box::new(DebugTerminal {
            width: w,
            height: h,
            temp_str: String::with_capacity((w * h) as usize),
            commands,
        }))
    }
}
impl Terminal for DebugTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        self.temp_str.clear();
        let mut x = 0u32;
        for ch in &surface.chars {
            self.temp_str.push_str("\x1b[");
            match ch.foreground {
                Color::Black => self.temp_str.push_str("30"),
                Color::DarkRed => self.temp_str.push_str("31"),
                Color::DarkGreen => self.temp_str.push_str("32"),
                Color::Olive => self.temp_str.push_str("33"),
                Color::DarkBlue => self.temp_str.push_str("34"),
                Color::Magenta => self.temp_str.push_str("35"),
                Color::Teal => self.temp_str.push_str("36"),
                Color::Silver => self.temp_str.push_str("37"),
                Color::Gray => self.temp_str.push_str("90"),
                Color::Red => self.temp_str.push_str("91"),
                Color::Green => self.temp_str.push_str("92"),
                Color::Yellow => self.temp_str.push_str("93"),
                Color::Blue => self.temp_str.push_str("94"),
                Color::Pink => self.temp_str.push_str("95"),
                Color::Aqua => self.temp_str.push_str("96"),
                Color::White => self.temp_str.push_str("97"),
                _ => self.temp_str.push_str("37"), /* default is white */
            }
            self.temp_str.push(';');
            match ch.background {
                Color::Black => self.temp_str.push_str("40"),
                Color::DarkRed => self.temp_str.push_str("41"),
                Color::DarkGreen => self.temp_str.push_str("42"),
                Color::Olive => self.temp_str.push_str("43"),
                Color::DarkBlue => self.temp_str.push_str("44"),
                Color::Magenta => self.temp_str.push_str("45"),
                Color::Teal => self.temp_str.push_str("46"),
                Color::Silver => self.temp_str.push_str("47"),
                Color::Gray => self.temp_str.push_str("100"),
                Color::Red => self.temp_str.push_str("101"),
                Color::Green => self.temp_str.push_str("102"),
                Color::Yellow => self.temp_str.push_str("103"),
                Color::Blue => self.temp_str.push_str("104"),
                Color::Pink => self.temp_str.push_str("105"),
                Color::Aqua => self.temp_str.push_str("106"),
                Color::White => self.temp_str.push_str("107"),
                _ => self.temp_str.push_str("40"), /* default is white */
            }
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
        SystemEvent::None
    }
}
