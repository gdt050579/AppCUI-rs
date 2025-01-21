// reference: https://man7.org/linux/man-pages/man2/TIOCGWINSZ.2const.html

use std::sync::{Arc, Condvar, Mutex, OnceLock};

use libc::{ioctl, sighandler_t, signal, SIGWINCH, SIG_ERR, STDOUT_FILENO, TIOCGWINSZ, TIOCSWINSZ};

use crate::prelude::Size;

pub struct ResizeNotification {
    pub mutex: Mutex<Size>,
    pub cond_var: Condvar
}

pub static RESIZE_ARC: OnceLock<Arc<ResizeNotification>> = OnceLock::new();

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

pub fn get_resize_notification() -> &'static Arc<ResizeNotification> {
    RESIZE_ARC.get_or_init(|| Arc::new(ResizeNotification {mutex: Mutex::new(Size::default()), cond_var: Condvar::new()}))
}

extern "C" fn handle_resize(_: libc::c_int) {
    let resize_not = get_resize_notification();
    let mut guard = resize_not.mutex.lock().unwrap();
    *guard = Size::default();
    if let Ok(size) = get_terminal_size() {
        *guard = size;
        resize_not.cond_var.notify_one();
    }
}

pub(crate) fn listen_for_resizes () -> Result<(), std::io::Error> {
    let _ = get_resize_notification();
    
    unsafe {
        if SIG_ERR == signal(SIGWINCH, handle_resize as sighandler_t) {
            return Err(std::io::Error::last_os_error());
        };
    };

    Ok(())
}

pub(crate) fn get_terminal_size() -> Result<Size, std::io::Error> {
    let mut w_size = Winsize::empty();
    if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut w_size) } == -1 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(Size::new(w_size.ws_col.into(), w_size.ws_row.into()))
}

pub(crate) fn set_terminal_size(size: &Size) -> Result<(), std::io::Error> {
    let w_size = Winsize {
        ws_col: size.width as u16,
        ws_row: size.height as u16,
        ws_xpixel: 0,
        ws_ypixel: 0
    };

    if unsafe {
        ioctl(STDOUT_FILENO, TIOCSWINSZ, &w_size)
    } == -1 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(())
}
