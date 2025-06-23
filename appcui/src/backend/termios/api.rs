//! Interface to the <termios.h> API
pub(crate) mod io;
pub(crate) mod sizing;

// Define C system binding calls
extern "C" {
    // "termios.h" function bindings
    /// Set new attributes for the current terminal
    pub fn tcsetattr(file_des: u32, optional_actions: u32, termios: *const Termios) -> isize;

    /// Get the termios structure
    pub fn tcgetattr(file_des: u32, termios: *mut Termios) -> isize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TcFlag(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Speed(u64);

// Note: Structure is present in both 32-bit and 64-bit versions in the headers, however 32-bit
// does not really seem like a real use-case scenario
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Termios {
    // Input flags
    c_iflag: TcFlag,
    // Output flags
    c_oflag: TcFlag,
    // Control flags
    c_cflag: TcFlag,
    // Local flags
    c_lflag: TcFlag,
    // Control chars
    c_cc: [u8; CONTROL_CHARS_LEN],
    // Input speed
    c_ispeed: Speed,
    // Output speed
    c_ospeed: Speed,
}

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
        let result = unsafe { tcgetattr(io::STDIN_FILENO, &mut orig_termios) };

        // If an error results, we return it back
        if result == -1 {
            return Err(TermiosError::TcGetAttr);
        }

        // We create a new termios structure. The reason for that is we want to keep the previous
        // one cached, such that we can return the terminal to the user in the same state we got it
        let mut raw_termios = orig_termios.clone();

        // Disabling IXON also disables: `Ctrl-S` and `Ctrl-Q` used for software control flow.
        // https://en.wikipedia.org/wiki/Software_flow_control
        raw_termios.c_iflag.0 &= !(input_flags::IGNBRK
            | input_flags::BRKINT
            | input_flags::PARMRK
            | input_flags::ISTRIP
            | input_flags::IGNCR
            | input_flags::ICRNL
            | input_flags::IXON);
        raw_termios.c_cflag.0 |= control_mode::CS8;
        // Disable output processing
        // raw_termios.c_oflag.0 |= !(output_flags::OPOST);
        // Disable echoing input submitted to stdin
        // Disabling ISIG also disables: `Ctrl-C` (SIGINT) and `Ctrl-Z` (SIGSUSP)
        // Disabling IEXTEN also disables: `Ctrl-V` (paste) and `Ctrl-O` function
        raw_termios.c_lflag.0 &= !(local_flags::ECHO | local_flags::ICANON | local_flags::ISIG | local_flags::IEXTEN);

        // Set control conditions like characters and time to wait for
        raw_termios.c_cc[ctrl_char_idx::VMIN] = 1;
        raw_termios.c_cc[ctrl_char_idx::VTIME] = 0;

        let result = unsafe { tcsetattr(io::STDIN_FILENO, term_cmd::TC_SET_ATTR_FLUSH, &raw_termios) };

        // If an error results, we return it back
        if result == -1 {
            return Err(TermiosError::TcSetAttr);
        }

        Ok(orig_termios)
    }
    pub(crate) fn restore(&mut self) {
        let _ = unsafe { tcsetattr(io::STDIN_FILENO, term_cmd::TC_SET_ATTR_FLUSH, self) };
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

const CONTROL_CHARS_LEN: usize = 20;

mod input_flags {
    //! Flag values of the fields `c_iflag` which describe the basic terminal input control.

    /// Ignore BREAK condition
    pub const IGNBRK: u64 = 0x0000_0001;
    /// Map BREAK to SIGINTR
    pub const BRKINT: u64 = 0x0000_0002;
    /// Ignore/discard parity errors
    pub const _IGNPAR: u64 = 0x0000_0004;
    /// Mark parity and framing errors
    pub const PARMRK: u64 = 0x0000_0008;
    /// Enable checking of parity-errors
    pub const _INPCK: u64 = 0x0000_0010;
    /// Strip 8th bit off chars
    pub const ISTRIP: u64 = 0x0000_0020;
    /// Map NL into CR
    pub const _INLCR: u64 = 0x0000_0040;
    /// Ignore CR
    pub const IGNCR: u64 = 0x0000_0080;
    /// Map CR to NL (ala CRMOD)
    pub const ICRNL: u64 = 0x0000_0100;
    /// Enable output flow control
    pub const IXON: u64 = 0x0000_0200;
    /// Enable input flow control
    pub const _IXOFF: u64 = 0x0000_0400;
    /// Any char will restart after stop
    pub const _IXANY: u64 = 0x0000_0800;
    /// Ring bell on input queue full
    pub const _IMAXBEL: u64 = 0x0000_2000;
    /// Maintain state for UTF-8 VERASE
    pub const _IUTF8: u64 = 0x0000_4000;
}

mod output_flags {
    //! Flags in this module describe basic terminal output control.

    /// If set, the remaining flag masks are interpreted according to their description; otherwise
    /// characters are transmitted without change.
    pub const _OPOST: u64 = 0x0000_0001;
}

mod control_mode {
    //! Control mode values are also flags which describe the basic terminal harware control. Not
    //! all values specified are supported by all hardware.

    /// Character size mask
    pub const _CSIZE: u64 = 0x0000_0300;
    /// 8 bits character size
    pub const CS8: u64 = 0x0000_0300;
    /// Enable receiver. If ther receiver is not enabled, no character is received.
    pub const _CREAD: u64 = 0x0000_0800;
    /// Parity enable
    pub const _PARENB: u64 = 0x0000_1000;
    /// Ignore modem status lines
    pub const _CLOCAL: u64 = 0x0000_8000;
}

mod local_flags {
    //! Dumping ground for ther state

    /// Enable echoing. Here we are referrring to echoing characters from the `stdin` stream back
    /// to the `stdout` automatically
    pub const ECHO: u64 = 0x0000_0008;
    /// Echo NL even if ECHO is off
    pub const _ECHONL: u64 = 0x0000_0010;
    /// Enable signals INTR, QUIT, [D]SUSP
    pub const ISIG: u64 = 0x0000_0080;
    /// Canonicalize input lines. Basically meaning that the terminal only registers the input
    /// after the newline key (e.g. Enter) has been pressed. This allows the user to preprocess its
    /// input line by line and even delete characters when needed.
    pub const ICANON: u64 = 0x0000_0100;
    /// Enable DISCARD and LNEXT
    pub const IEXTEN: u64 = 0x0000_0400;
}

mod term_cmd {
    //! Terminal cmommands passed to `tcsetattr()` for setting the termios structure.

    // Make change immediate
    pub const _TC_SET_ATTR_NOW: u32 = 0;
    // Drain output, then change
    pub const _TC_SET_ATTR_DRAIN: u32 = 1;
    // Drain output, flush input
    pub const TC_SET_ATTR_FLUSH: u32 = 2;
}

mod ctrl_char_idx {
    //! Indexed into the control characters `c_cc` field from `Termios`

    /// Sets the minimum number of bytes of input needed before the `read` returns
    pub const VMIN: usize = 16;
    /// Sets the maximum amount of time to wait before the `read` returns
    pub const VTIME: usize = 17;
}
