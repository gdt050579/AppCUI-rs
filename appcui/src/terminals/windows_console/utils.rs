use super::winapi;
use super::structs::*;
use super::constants::*;
use crate::terminals::Error;
use crate::terminals::ErrorKind;

pub(super) fn get_stdin_handle() -> Result<HANDLE, Error> {
    unsafe {
        let h = winapi::GetStdHandle(STD_INPUT_HANDLE);
        if h == INVALID_HANDLE_VALUE {
            return Err(Error::new(
                ErrorKind::InitializationFailure,
                "Unable to get a valid stdin handle from GetStdHandle WinApi function !".to_string(),
            ));
        }
        Ok(h)
    }
}
pub(super) fn get_stdout_handle() -> Result<HANDLE, Error> {
    unsafe {
        let h = winapi::GetStdHandle(STD_OUTPUT_HANDLE);
        if h == INVALID_HANDLE_VALUE {
            return Err(Error::new(
                ErrorKind::InitializationFailure,
                "Unable to get a valid stdout handle from GetStdHandle WinApi function !".to_string(),
            ));
        }
        Ok(h)
    }
}

pub(super) fn get_console_screen_buffer_info(handle: HANDLE) -> Result<CONSOLE_SCREEN_BUFFER_INFO, Error> {
    unsafe {
        let mut cbuf = CONSOLE_SCREEN_BUFFER_INFO::default();
        if winapi::GetConsoleScreenBufferInfo(handle, &mut cbuf) == FALSE {
            return Err(Error::new(
                ErrorKind::InitializationFailure,
                format!(
                    "GetConsoleScreenBufferInfo failed to get information on current console !\nWindow code error: {}",
                    winapi::GetLastError()
                ),
            ));
        }
        Ok(cbuf)
    }
}
