use super::Color;
use super::Attribute;
use super::Surface;
use super::Terminal;

type HANDLE = u32;
type BOOL = u32;

const INVALID_HANDLE_VALUE: u32 = 0xFFFFFFFF;
const STD_INPUT_HANDLE: i32 = -10;
const STD_OUTPUT_HANDLE: i32 = -11;
const FALSE: u32 = 0;
const COMMON_LVB_UNDERSCORE: u16 = 0x8000;

#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Default, Copy, Clone, Debug)]
struct COORD {
    x: i16,
    y: i16,
}

#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Default, Copy, Clone, Debug)]
struct SMALL_RECT {
    left: i16,
    top: i16,
    right: i16,
    bottom: i16,
}
#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Default, Copy, Clone, Debug)]
struct CONSOLE_SCREEN_BUFFER_INFO {
    size: COORD,
    cursor_pos: COORD,
    _attributes: u16,
    _window: SMALL_RECT,
    _max_size: COORD,
}

#[repr(C)]
#[warn(non_camel_case_types)]
#[derive(Default, Copy, Clone, Debug)]
struct CHAR_INFO {
    code: u16,
    attr: u16,
}
extern "system" {
    #[warn(non_camel_case_types)]
    fn GetStdHandle(v: i32) -> HANDLE;
    #[warn(non_camel_case_types)]
    fn SetConsoleCursorPosition(handle: HANDLE, pos: COORD) -> BOOL;
    #[warn(non_camel_case_types)]
    fn WriteConsoleOutputW(
        handle: HANDLE,
        lpBuffer: *const CHAR_INFO,
        dwBufferSize: COORD,
        dwBufferCoord: COORD,
        lpWriteRegion: &SMALL_RECT,
    );
    #[warn(non_camel_case_types)]
    fn GetConsoleScreenBufferInfo(
        handle: HANDLE,
        lpConsoleScreenBufferInfo: &mut CONSOLE_SCREEN_BUFFER_INFO,
    ) -> BOOL;
}

fn get_handle(handle_id: i32) -> Option<u32> {
    unsafe {
        let h = GetStdHandle(handle_id);
        if h == INVALID_HANDLE_VALUE {
            return None;
        }
        return Some(h);
    }
}

fn get_console_screen_buffer_info(handle: HANDLE) -> Option<CONSOLE_SCREEN_BUFFER_INFO> {
    unsafe {
        let mut cbuf = CONSOLE_SCREEN_BUFFER_INFO::default();
        if GetConsoleScreenBufferInfo(handle, &mut cbuf) == FALSE {
            return None;
        }
        return Some(cbuf);
    }
}

fn set_console_cursor_pos(handle: u32, x: i32, y: i32) -> bool {
    let pos = COORD {
        x: x as i16,
        y: y as i16,
    };
    unsafe { SetConsoleCursorPosition(handle, pos) != 0 }
}

fn update_screen(handle: u32, buf: &[CHAR_INFO], width: u32, height: u32) {
    let sz = COORD {
        x: width as i16,
        y: height as i16,
    };
    let start = COORD { x: 0, y: 0 };
    let region = SMALL_RECT {
        left: 0,
        top: 0,
        right: sz.x,
        bottom: sz.y,
    };
    unsafe {
        WriteConsoleOutputW(handle, buf.as_ptr(), sz, start, &region);
    }
}

pub struct WindowsTerminal {
    stdin_handle: HANDLE,
    stdout_handle: HANDLE,
    width: u32,
    height: u32,
    chars: Vec<CHAR_INFO>,

}

impl WindowsTerminal {
    pub fn create() -> Option<Box<WindowsTerminal>> {
        let stdin = get_handle(STD_INPUT_HANDLE)?;
        let stdout = get_handle(STD_OUTPUT_HANDLE)?;
        let info = get_console_screen_buffer_info(stdout)?;
        if (info.size.x < 1) || (info.size.y < 1) {
            return None;
        }
        let mut term = Box::new(WindowsTerminal {
            stdin_handle: stdin,
            stdout_handle: stdout,
            width: info.size.x as u32,
            height: info.size.y as u32,
            chars: Vec::with_capacity(100),
        });
        term.chars.resize(
            (term.width as usize) * (term.height as usize),
            CHAR_INFO { code: 32, attr: 0 },
        );
        return Some(term);
    }
}

impl Terminal for WindowsTerminal {
    fn update_screen(&mut self, surface: &Surface) {
        // safety check --> surface size should be the same as self.width/height size
        if (surface.width != self.width) || (surface.height != self.height) {
            return;
        }
        
        // copy surface into CHAR_INFO
        let sz = self.chars.len();
        for i in 0..sz {
            let ch = &surface.chars[i];
            let screen_char = &mut (self.chars[i]);
            if ch.code != (0 as char) {
                screen_char.code = ch.code as u16;
            } else {
                screen_char.code = 32; // fallback to space
            }
            screen_char.attr = 0;
            if ch.foreground != Color::Transparent {
                screen_char.attr = (ch.foreground as u8) as u16;
            }
            if ch.background != Color::Transparent {
                screen_char.attr |= ((ch.background as u8) as u16) << 4;
            }
            if ch.attribute.contains(Attribute::Underline) {
                screen_char.attr |= COMMON_LVB_UNDERSCORE;
            }
        }
        // 
        let sz = COORD {
            x: self.width as i16,
            y: self.height as i16,
        };
        let start = COORD { x: 0, y: 0 };
        let region = SMALL_RECT {
            left: 0,
            top: 0,
            right: sz.x-1,
            bottom: sz.y-1,
        };
        unsafe {
            WriteConsoleOutputW(self.stdout_handle, self.chars.as_ptr(), sz, start, &region);
        }
    }
    fn get_width(&self) -> u32 {
        return self.width;
    }
    fn get_height(&self) -> u32 {
        return self.height;
    }
}
