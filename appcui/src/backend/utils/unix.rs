use crate::graphics::CellSize;
use libc::{ioctl, STDOUT_FILENO, TIOCGWINSZ};

#[derive(Default)]
#[repr(C)]
pub(crate) struct Winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

/// Get the cell size in pixels using TIOCGWINSZ ioctl.
/// Returns default cell size if the terminal doesn't report pixel dimensions.
pub(crate) fn get_cell_size() -> CellSize {
    let mut w_size = Winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    // https://stackoverflow.com/questions/1022957/getting-terminal-width-in-c
    if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut w_size) } != -1
        && w_size.ws_xpixel > 0
        && w_size.ws_ypixel > 0
        && w_size.ws_col > 0
        && w_size.ws_row > 0
    {
        let cell_width = w_size.ws_xpixel / w_size.ws_col;
        let cell_height = w_size.ws_ypixel / w_size.ws_row;
        if cell_width > 0 && cell_height > 0 {
            return CellSize::new(cell_width, cell_height);
        }
    }
    CellSize::default()
}
