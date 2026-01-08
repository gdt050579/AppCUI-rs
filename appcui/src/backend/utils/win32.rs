pub(crate) mod api;
mod clipboard;
mod console;
pub(crate) mod constants;
mod original_screen;
pub(crate) mod structs;

#[cfg(test)]
mod tests;

pub(crate) use clipboard::Clipboard;
pub(crate) use console::Console;
use original_screen::OriginalScreen;

use crate::graphics::CellSize;

// https://stackoverflow.com/questions/6812224/getting-terminal-size-in-c-for-windows
pub(crate) fn get_cell_size() -> CellSize {
    unsafe {
        let h_stdout = api::GetStdHandle(constants::STD_OUTPUT_HANDLE);
        if h_stdout == constants::INVALID_HANDLE_VALUE {
            return CellSize::default();
        }

        let mut font_info = structs::CONSOLE_FONT_INFOEX::default();
        if api::GetCurrentConsoleFontEx(h_stdout, constants::FALSE, &mut font_info) == constants::FALSE {
            return CellSize::default();
        }

        let cell_width = font_info.dw_font_size.x as u16;
        let cell_height = font_info.dw_font_size.y as u16;

        if cell_width > 0 && cell_height > 0 {
            CellSize::new(cell_width, cell_height)
        } else {
            CellSize::default()
        }
    }
}
