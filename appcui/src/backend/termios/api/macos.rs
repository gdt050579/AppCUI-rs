//! `<termios.h>` definitions for macOS (x86_64 and aarch64), from `<sys/termios.h>`.

use std::ffi::{c_int, c_uint, c_ulong};

/// `tcflag_t` (`unsigned long` on macOS)
pub(crate) type TcFlag = c_ulong;
/// `speed_t` (`unsigned long` on macOS)
pub(crate) type Speed = c_ulong;
/// `cc_t`
pub(crate) type Cc = u8;
/// `nfds_t` (`unsigned int` on macOS), the type of `poll()`'s second argument
pub(crate) type NfdsT = c_uint;
/// Type of `ioctl()`'s `request` argument (`unsigned long` on macOS)
pub(crate) type IoctlRequest = c_ulong;

/// Number of control characters (`NCCS`)
pub(crate) const NCCS: usize = 20;

/// `struct termios` as defined on macOS (72 bytes on 64-bit).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Termios {
    /// Input flags
    pub(crate) c_iflag: TcFlag,
    /// Output flags
    pub(crate) c_oflag: TcFlag,
    /// Control flags
    pub(crate) c_cflag: TcFlag,
    /// Local flags
    pub(crate) c_lflag: TcFlag,
    /// Control characters
    pub(crate) c_cc: [Cc; NCCS],
    /// Input speed
    pub(crate) c_ispeed: Speed,
    /// Output speed
    pub(crate) c_ospeed: Speed,
}

const _: () = assert!(std::mem::size_of::<Termios>() == 72);

pub(crate) mod input_flags {
    //! Flag values of the field `c_iflag` (basic terminal input control).
    use super::TcFlag;

    /// Ignore BREAK condition
    pub(crate) const IGNBRK: TcFlag = 0x0000_0001;
    /// Map BREAK to SIGINT
    pub(crate) const BRKINT: TcFlag = 0x0000_0002;
    /// Mark parity and framing errors
    pub(crate) const PARMRK: TcFlag = 0x0000_0008;
    /// Strip 8th bit off chars
    pub(crate) const ISTRIP: TcFlag = 0x0000_0020;
    /// Ignore CR
    pub(crate) const IGNCR: TcFlag = 0x0000_0080;
    /// Map CR to NL
    pub(crate) const ICRNL: TcFlag = 0x0000_0100;
    /// Enable output flow control (`Ctrl-S` / `Ctrl-Q`)
    pub(crate) const IXON: TcFlag = 0x0000_0200;
}

pub(crate) mod control_mode {
    //! Flag values of the field `c_cflag` (basic terminal hardware control).
    use super::TcFlag;

    /// 8 bits character size
    pub(crate) const CS8: TcFlag = 0x0000_0300;
}

pub(crate) mod local_flags {
    //! Flag values of the field `c_lflag`.
    use super::TcFlag;

    /// Echo input characters
    pub(crate) const ECHO: TcFlag = 0x0000_0008;
    /// Enable signals INTR, QUIT, [D]SUSP (`Ctrl-C`, `Ctrl-\`, `Ctrl-Z`)
    pub(crate) const ISIG: TcFlag = 0x0000_0080;
    /// Canonical mode: input is made available line by line
    pub(crate) const ICANON: TcFlag = 0x0000_0100;
    /// Enable DISCARD and LNEXT (`Ctrl-O`, `Ctrl-V`)
    pub(crate) const IEXTEN: TcFlag = 0x0000_0400;
}

pub(crate) mod term_cmd {
    //! `optional_actions` values passed to `tcsetattr()`.
    use std::ffi::c_int;

    /// Drain output, flush input, then change
    pub(crate) const TCSAFLUSH: c_int = 2;
}

pub(crate) mod ctrl_char_idx {
    //! Indexes into the control characters field `c_cc`.

    /// Minimum number of bytes of input needed before `read` returns
    pub(crate) const VMIN: usize = 16;
    /// Maximum amount of time to wait before `read` returns
    pub(crate) const VTIME: usize = 17;
}

/// Signal sent when the terminal window size changes
pub(crate) const SIGWINCH: c_int = 28;

pub(crate) mod ioctl_request {
    //! `ioctl()` requests for the terminal size (`<sys/ttycom.h>`).
    use super::IoctlRequest;

    /// Get the window size: `_IOR('t', 104, struct winsize)`
    pub(crate) const TIOCGWINSZ: IoctlRequest = 0x4008_7468;
    /// Set the window size: `_IOW('t', 103, struct winsize)`
    pub(crate) const TIOCSWINSZ: IoctlRequest = 0x8008_7467;
}
