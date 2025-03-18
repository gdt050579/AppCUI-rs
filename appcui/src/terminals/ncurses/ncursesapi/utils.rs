use super::structs::*;
use crate::terminals::Error;
use crate::terminals::ErrorKind;
use std::os::unix::io::{AsRawFd, RawFd};
use std::io;

pub(crate) fn get_stdin_handle() -> Result<RawFd, Error> {
    let stdin = io::stdin();
    Ok(stdin.as_raw_fd())
}

pub(crate) fn get_stdout_handle() -> Result<RawFd, Error> {
    let stdout = io::stdout();
    Ok(stdout.as_raw_fd())
}

pub(crate) fn get_console_screen_buffer_info(fd: RawFd) -> Result<CONSOLE_SCREEN_BUFFER_INFO, Error> {
    use libc::{ioctl, winsize, TIOCGWINSZ};
    
    unsafe {
        let mut ws: winsize = std::mem::zeroed();
        if ioctl(fd, TIOCGWINSZ, &mut ws) == -1 {
            return Err(Error::new(
                ErrorKind::InitializationFailure,
                "Failed to get console screen buffer info using ioctl!".to_string(),
            ));
        }
        
        Ok(CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: COORD { X: ws.ws_col as i16, Y: ws.ws_row as i16 },
            dwCursorPosition: COORD { X: 0, Y: 0 },
            wAttributes: 0,
            srWindow: SMALL_RECT { left: 0, top: 0, right: ws.ws_col as i16, bottom: ws.ws_row as i16 },
            dwMaximumWindowSize: COORD { X: ws.ws_col as i16, Y: ws.ws_row as i16 },
        })
    }
}
