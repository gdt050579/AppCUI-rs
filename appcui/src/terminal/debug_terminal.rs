use super::Surface;
use super::SystemEvent;
use super::Terminal;
use crate::graphics::Color;
use crate::system::Error;

pub(crate) struct DebugTerminal {
    width: u32,
    height: u32,
    temp_str: String,
}
impl DebugTerminal {
    pub(crate) fn create() -> Result<Box<dyn Terminal>,Error> {
        Ok(Box::new(DebugTerminal {
            width: 80,
            height: 25,
            temp_str: String::with_capacity(80 * 6),
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

    fn get_system_event(&mut self) -> SystemEvent {
        SystemEvent::None
    }
}
