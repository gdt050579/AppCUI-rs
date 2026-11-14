//! Interface to the <termios.h> API

#[cfg(test)]
mod tests;

pub(crate) mod common;
pub(crate) mod io;
pub(crate) mod sizing;

#[cfg(target_os = "linux")]
pub(crate) mod linux;
#[cfg(target_os = "linux")]
pub(crate) use linux as platform;

#[cfg(target_os = "macos")]
pub(crate) mod macos;
#[cfg(target_os = "macos")]
pub(crate) use macos as platform;

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
compile_error!("The termios backend only has <termios.h> definitions for Linux and macOS.");

use common::{tcgetattr, tcsetattr, STDIN_FILENO};
use platform::{control_mode, ctrl_char_idx, input_flags, local_flags, term_cmd};

pub(crate) use platform::Termios;

impl Termios {
    // Normally the terminal is in canonical mode, which means:
    // 1. It interprets the keys we press
    // 2. It does not take the input until `Enter` key is hit
    // Enabling raw mode disables these 2 features and also ignore special key presses like:
    // `Ctrl-V`, `Ctrl-C', `Ctrl-D`, etc
    pub fn enable_raw_mode() -> Result<Self, TermiosError> {
        // Define a deafult `Termios` structure
        let mut orig_termios: Termios = Termios::default();

        // Call and fill it from the OS data
        let result = unsafe { tcgetattr(STDIN_FILENO, &mut orig_termios) };

        // If an error results, we return it back
        if result == -1 {
            return Err(TermiosError::TcGetAttr);
        }

        // We create a new termios structure. The reason for that is we want to keep the previous
        // one cached, such that we can return the terminal to the user in the same state we got it
        let mut raw_termios = orig_termios.clone();

        // Disabling IXON also disables: `Ctrl-S` and `Ctrl-Q` used for software control flow.
        // https://en.wikipedia.org/wiki/Software_flow_control
        raw_termios.c_iflag &= !(input_flags::IGNBRK
            | input_flags::BRKINT
            | input_flags::PARMRK
            | input_flags::ISTRIP
            | input_flags::IGNCR
            | input_flags::ICRNL
            | input_flags::IXON);
        raw_termios.c_cflag |= control_mode::CS8;
        // Disable echoing input submitted to stdin
        // Disabling ISIG also disables: `Ctrl-C` (SIGINT) and `Ctrl-Z` (SIGSUSP)
        // Disabling IEXTEN also disables: `Ctrl-V` (paste) and `Ctrl-O` function
        raw_termios.c_lflag &= !(local_flags::ECHO | local_flags::ICANON | local_flags::ISIG | local_flags::IEXTEN);

        // Set control conditions like characters and time to wait for
        raw_termios.c_cc[ctrl_char_idx::VMIN] = 1;
        raw_termios.c_cc[ctrl_char_idx::VTIME] = 0;

        let result = unsafe { tcsetattr(STDIN_FILENO, term_cmd::TCSAFLUSH, &raw_termios) };

        // If an error results, we return it back
        if result == -1 {
            return Err(TermiosError::TcSetAttr);
        }

        Ok(orig_termios)
    }
    pub(crate) fn restore(&mut self) {
        let _ = unsafe { tcsetattr(STDIN_FILENO, term_cmd::TCSAFLUSH, self) };
    }
}

#[derive(Debug)]
pub enum TermiosError {
    TcGetAttr,
    TcSetAttr,
    ReadStdInFailed,
    UnknownLetter(io::UnknownLetter),
    UnknownKey,
}

impl From<io::UnknownLetter> for TermiosError {
    fn from(err: io::UnknownLetter) -> Self {
        Self::UnknownLetter(err)
    }
}
