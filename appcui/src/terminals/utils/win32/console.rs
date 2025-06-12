use super::{api, constants, structs};
use crate::prelude::Size;
use crate::terminals::Error;
use crate::terminals::ErrorKind;

#[derive(Copy, Clone)]
pub(crate) struct Console {
    stdin: structs::HANDLE,
    stdout: structs::HANDLE,
    stdin_original_mode_flags: u32,
    stdout_original_mode_flags: u32,
    size: Size,
    visible_region: structs::SMALL_RECT,
}

impl Console {
    pub(crate) fn new(builder: &crate::system::Builder, vt: bool) -> Result<Self, Error> {
        unsafe {
            let h_stdin = api::GetStdHandle(constants::STD_INPUT_HANDLE);
            if h_stdin == constants::INVALID_HANDLE_VALUE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    "Unable to get a valid stdin handle from GetStdHandle WinApi function !".to_string(),
                ));
            }
            let h_stdout = api::GetStdHandle(constants::STD_OUTPUT_HANDLE);
            if h_stdout == constants::INVALID_HANDLE_VALUE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    "Unable to get a valid stdout handle from GetStdHandle WinApi function !".to_string(),
                ));
            }
            if let Some(new_size) = builder.size {
                Self::resize(new_size, h_stdout)?;
            }
            if let Some(title) = &builder.title {
                Self::set_title(title)?;
            }

            let mut stdin_original_mode_flags = 0u32;
            if api::GetConsoleMode(h_stdin, &mut stdin_original_mode_flags) == constants::FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    "GetConsoleMode failed to aquire original mode for current console stdin !".to_string(),
                ));
            }
            let mut stdout_original_mode_flags = 0u32;
            if api::GetConsoleMode(h_stdout, &mut stdout_original_mode_flags) == constants::FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    "GetConsoleMode failed to aquire original mode for current console stdout !".to_string(),
                ));
            }


            if api::SetConsoleMode(
                h_stdin,
                constants::ENABLE_WINDOW_INPUT | constants::ENABLE_MOUSE_INPUT | constants::ENABLE_EXTENDED_FLAGS,
            ) == constants::FALSE
            {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!("Fail to set current console flags to 'ENABLE_WINDOW_INPUT | ENABLE_MOUSE_INPUT | ENABLE_EXTENDED_FLAGS' via SetConsoleMode API.\nWindow code error: {} ",api::GetLastError()),
                ));
            }
            if vt {
                if api::SetConsoleMode(h_stdout, stdout_original_mode_flags | constants::ENABLE_VIRTUAL_TERMINAL_PROCESSING) == constants::FALSE {
                    return Err(Error::new(
                        ErrorKind::InitializationFailure,
                        format!("Fail to set current console flags to 'ENABLE_VIRTUAL_TERMINAL_PROCESSING' via SetConsoleMode API.\nWindow code error: {} ",api::GetLastError()),
                    ));
                }
            }


            let info = Self::screen_buffer_info(h_stdout)?;
            if (info.size.x < 1) || (info.size.y < 1) {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "Invalid console size returned by GetConsoleScreenBufferInfo: width={},height={}\nWindow code error: {}",
                        info.size.x,
                        info.size.y,
                        api::GetLastError()
                    ),
                ));
            }
            // analyze the visible (window) part
            if (info.window.left > info.window.right) || (info.window.left < 0) {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "Invalid console visible size returned by GetConsoleScreenBufferInfo: left={},top={},right={},bottom={}\nLeft value should be smaller tham the Right value\nWindow code error: {}",
                        info.window.left,
                        info.window.top,
                        info.window.right,
                        info.window.bottom,
                        api::GetLastError()
                    )
                ));
            }
            if (info.window.top > info.window.bottom) || (info.window.top < 0) {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "Invalid console visible size returned by GetConsoleScreenBufferInfo: left={},top={},right={},bottom={}\nTop value should be smaller tham the Bottom value\nWindow code error: {}",
                        info.window.left,
                        info.window.top,
                        info.window.right,
                        info.window.bottom,
                        api::GetLastError()
                    )
                ));
            }
            let w = (info.window.right as u32) + 1 - (info.window.left as u32);
            let h = (info.window.bottom as u32) + 1 - (info.window.top as u32);

            Ok(Self {
                stdin: h_stdin,
                stdout: h_stdout,
                size: Size::new(w, h),
                stdin_original_mode_flags: stdin_original_mode_flags,
                stdout_original_mode_flags: stdout_original_mode_flags,
                visible_region: info.window,
            })
        }
    }
    #[inline(always)]
    pub(crate) fn stdin(&self) -> structs::HANDLE {
        self.stdin
    }
    #[inline(always)]
    pub(crate) fn stdout(&self) -> structs::HANDLE {
        self.stdout
    }
    #[inline(always)]
    pub(crate) fn size(&self) -> Size {
        self.size
    }
    #[inline(always)]
    pub(crate) fn visible_region(&self) -> structs::SMALL_RECT {
        self.visible_region
    }
    pub(crate) fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    pub(crate) fn set_visible_region(&mut self, region: structs::SMALL_RECT) {
        self.visible_region = region;
    }

    fn screen_buffer_info(stdout: structs::HANDLE) -> Result<structs::CONSOLE_SCREEN_BUFFER_INFO, Error> {
        unsafe {
            let mut cbuf = structs::CONSOLE_SCREEN_BUFFER_INFO::default();
            if api::GetConsoleScreenBufferInfo(stdout, &mut cbuf) == constants::FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "GetConsoleScreenBufferInfo failed to get information on current console !\nWindow code error: {}",
                        api::GetLastError()
                    ),
                ));
            }
            Ok(cbuf)
        }
    }
    pub(crate) fn query_screen_buffer_info(&self) -> Result<structs::CONSOLE_SCREEN_BUFFER_INFO, Error> {
        Self::screen_buffer_info(self.stdout)
    }
    fn string_to_wide(text: &str) -> Result<Vec<u16>, Error> {
        let mut result: Vec<u16> = Vec::with_capacity(text.len() + 1);
        for c in text.chars() {
            let unicode_id = c as u32;
            if unicode_id >= 0xFFFF {
                return Err(Error::new(
                    ErrorKind::InvalidParameter,
                    format!("Fail convert the string '{}' to windows WTF-16", text),
                ));
            }
            if unicode_id == 0 {
                return Err(Error::new(
                    ErrorKind::InvalidParameter,
                    format!("Found NULL (\\0 character) in title '{}'. This can not be accurately translated into windows WTF-16 that is NULL terminated !", text),
                ));
            }
            result.push(unicode_id as u16);
        }
        result.push(0);
        Ok(result)
    }
    pub(crate) fn set_title(title: &str) -> Result<(), Error> {
        let title_wtf16 = Self::string_to_wide(title)?;

        unsafe {
            if api::SetConsoleTitleW(title_wtf16.as_ptr()) == constants::FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "SetConsoleTitleW failed while attemting change the title of the console to '{}'. Error Code = {} !",
                        title,
                        api::GetLastError()
                    ),
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn resize(size: Size, stdout: structs::HANDLE) -> Result<(), Error> {
        // sanity check
        if (size.width > 30000) || (size.width < 5) {
            return Err(Error::new(
                ErrorKind::InvalidParameter,
                format!(
                    "The width paramater for console resize shoule be between 5 and 30000. Current value is invalid: 'width={}'",
                    size.width
                ),
            ));
        }
        if (size.height > 30000) || (size.height < 5) {
            return Err(Error::new(
                ErrorKind::InvalidParameter,
                format!(
                    "The height paramater for console resize shoule be between 5 and 30000. Current value is invalid: 'height={}'",
                    size.height
                ),
            ));
        }
        let window_size = structs::SMALL_RECT {
            left: 0,
            top: 0,
            right: size.width as i16 - 1,
            bottom: size.height as i16 - 1,
        };
        unsafe {
            if api::SetConsoleWindowInfo(stdout, constants::TRUE, &window_size) == constants::FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "SetConsoleWindowsInfo failed while attemting to resize console to {}x{}. Error Code = {} !",
                        size.width,
                        size.height,
                        api::GetLastError()
                    ),
                ));
            }
        }
        let buffer_size = structs::COORD {
            x: size.width as i16,
            y: size.height as i16,
        };
        unsafe {
            if api::SetConsoleScreenBufferSize(stdout, buffer_size) == constants::FALSE {
                return Err(Error::new(
                    ErrorKind::InitializationFailure,
                    format!(
                        "SetConsoleScreenBufferSize failed while attemting to resize console buttef to {}x{}. Error Code = {} !",
                        size.width,
                        size.height,
                        api::GetLastError()
                    ),
                ));
            }
        }
        Ok(())
    }
}
