use super::super::SystemEvent;
use super::super::Terminal;
use crate::graphics::*;
use crate::system::Error;

pub struct AnsiTerminal {
    size: Size,
}
impl AnsiTerminal {
    pub(crate) fn new(builder: &crate::system::Builder) -> Result<Box<dyn Terminal>, Error> {
        let mut t = AnsiTerminal { size: Size::new(80, 30) };
        if let Some(sz) = builder.size {
            t.size = sz;

            // if the terminal size is invalid, we will return an error
            return Err(Error::new(
                crate::prelude::ErrorKind::InvalidFeature,
                "AnsiTerminal is not yet implemented to support custom sizes".to_owned(),
            ));
        } else {
            // t.size = Size::new(80, 30); //default value - to be adjusted to the screen
            // //find out the size of the terminal
            // let mut s = String::new();
            // s.push_str("\x1b[18t");
            // print!("{}", s);
            // let mut buffer = [0u8; 32];
            // let mut i = 0;
            // while i < 32 {
            //     let c = std::io::stdin().bytes().next().unwrap().unwrap();
            //     if c == b'c' {
            //         break;
            //     }
            //     buffer[i] = c;
            //     i += 1;
            // }
            // let s = std::str::from_utf8(&buffer[2..i]).unwrap();
            // let mut parts = s.split(';');
            // if let Some(w) = parts.next() {
            //     if let Some(h) = parts.next() {
            //         if let Ok(w) = w.parse::<u16>() {
            //             if let Ok(h) = h.parse::<u16>() {
            //                 t.size = Size::new(w, h);
            //             }
            //         }
            //     }
            // }
        }
        Ok(Box::new(t))
    }
}
impl Terminal for AnsiTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        let mut s = String::new();
        let sz = surface.get_size();
        for y in 0..sz.height {
            for x in 0..sz.width {
                if let Some(c) = surface.get(x as i32, y as i32) {
                    match c.foreground {
                        Color::Black => s.push_str("\x1b[30m"),
                        Color::DarkBlue => s.push_str("\x1b[34m"),
                        Color::DarkGreen => s.push_str("\x1b[32m"),
                        Color::Teal => s.push_str("\x1b[36m"),
                        Color::DarkRed => s.push_str("\x1b[31m"),
                        Color::Magenta => s.push_str("\x1b[35m"),
                        Color::Olive => s.push_str("\x1b[33m"),
                        Color::Silver => s.push_str("\x1b[37m"),
                        Color::Gray => s.push_str("\x1b[90m"),
                        Color::Blue => s.push_str("\x1b[94m"),
                        Color::Green => s.push_str("\x1b[92m"),
                        Color::Aqua => s.push_str("\x1b[96m"),
                        Color::Red => s.push_str("\x1b[91m"),
                        Color::Pink => s.push_str("\x1b[95m"),
                        Color::Yellow => s.push_str("\x1b[93m"),
                        Color::White => s.push_str("\x1b[97m"),
                        Color::Transparent => {}
                    }
                    s.push(c.code);
                    s.push_str("\x1b[0m");
                }
            }
            s.push('\n');
        }
        print!("{}", s);
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn get_system_event(&mut self) -> SystemEvent {
        //sleep(std::time::Duration::from_secs(5));
        SystemEvent::None
    }
}
