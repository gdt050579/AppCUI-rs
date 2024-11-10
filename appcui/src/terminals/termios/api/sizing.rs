// reference: https://man7.org/linux/man-pages/man2/TIOCGWINSZ.2const.html

use std::sync::atomic::{AtomicBool, Ordering};

use libc::{ioctl, sighandler_t, signal, SIGWINCH, SIG_ERR, STDOUT_FILENO, TIOCGWINSZ};

use crate::prelude::Size;

pub static RESIZE_EVENT: AtomicBool = AtomicBool::new(false);  // event for resize event

#[repr(C)]
struct Winsize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,  // not used
    ws_ypixel: u16,  // not used
}

impl Winsize {
    pub fn empty() -> Self {
        Winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 }
    }
}

extern "C" fn handle_resize(_: libc::c_int) {
    RESIZE_EVENT.store(true, Ordering::SeqCst);
}

pub(crate) fn listen_for_resizes () -> Result<(), std::io::Error> {
    unsafe {
        if SIG_ERR == signal(SIGWINCH, handle_resize as sighandler_t) {
            return Err(std::io::Error::last_os_error());
        };
    };

    Ok(())
}

pub(crate) fn get_terminal_size () -> Result<Size, std::io::Error> {
    let mut w_size = Winsize::empty();
    if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut w_size) } == -1 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(Size::new(w_size.ws_col.into(), w_size.ws_row.into()))
}
