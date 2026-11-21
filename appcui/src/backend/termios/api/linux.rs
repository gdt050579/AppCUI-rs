//! `<termios.h>` definitions for Linux.
//!
//! Values come from the kernel's `asm-generic/termbits.h` (used by x86, x86_64, arm, aarch64,
//! riscv and loongarch). They are the same for glibc and musl. PowerPC, MIPS, SPARC and a few
//! other architectures use a different layout and different values, so they are rejected below.

#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "riscv32",
    target_arch = "riscv64",
    target_arch = "loongarch64"
)))]
compile_error!("The termios backend does not have <termios.h> definitions for this Linux architecture.");

use std::ffi::{c_int, c_uint, c_ulong};

/// `tcflag_t` (`unsigned int` on Linux)
pub(crate) type TcFlag = c_uint;
/// `speed_t` (`unsigned int` on Linux)
pub(crate) type Speed = c_uint;
/// `cc_t`
pub(crate) type Cc = u8;
/// `nfds_t` (`unsigned long` on Linux), the type of `poll()`'s second argument
pub(crate) type NfdsT = c_ulong;
/// Type of `ioctl()`'s `request` argument: `unsigned long` in glibc, `int` in musl
#[cfg(not(target_env = "musl"))]
pub(crate) type IoctlRequest = c_ulong;
/// Type of `ioctl()`'s `request` argument: `unsigned long` in glibc, `int` in musl
#[cfg(target_env = "musl")]
pub(crate) type IoctlRequest = c_int;

/// Number of control characters (`NCCS`)
pub(crate) const NCCS: usize = 32;

/// `struct termios` as defined by glibc / musl on Linux (60 bytes).
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
    /// Line discipline (Linux only)
    pub(crate) c_line: Cc,
    /// Control characters
    pub(crate) c_cc: [Cc; NCCS],
    /// Input speed
    pub(crate) c_ispeed: Speed,
    /// Output speed
    pub(crate) c_ospeed: Speed,
}

const _: () = assert!(std::mem::size_of::<Termios>() == 60);

pub(crate) mod input_flags {
    //! Flag values of the field `c_iflag` (basic terminal input control).
    use super::TcFlag;

    /// Ignore BREAK condition
    pub(crate) const IGNBRK: TcFlag = 0o000001;
    /// Map BREAK to SIGINT
    pub(crate) const BRKINT: TcFlag = 0o000002;
    /// Mark parity and framing errors
    pub(crate) const PARMRK: TcFlag = 0o000010;
    /// Strip 8th bit off chars
    pub(crate) const ISTRIP: TcFlag = 0o000040;
    /// Ignore CR
    pub(crate) const IGNCR: TcFlag = 0o000200;
    /// Map CR to NL
    pub(crate) const ICRNL: TcFlag = 0o000400;
    /// Enable output flow control (`Ctrl-S` / `Ctrl-Q`)
    pub(crate) const IXON: TcFlag = 0o002000;
}

pub(crate) mod control_mode {
    //! Flag values of the field `c_cflag` (basic terminal hardware control).
    use super::TcFlag;

    /// 8 bits character size
    pub(crate) const CS8: TcFlag = 0o000060;
}

pub(crate) mod local_flags {
    //! Flag values of the field `c_lflag`.
    use super::TcFlag;

    /// Enable signals INTR, QUIT, SUSP (`Ctrl-C`, `Ctrl-\`, `Ctrl-Z`)
    pub(crate) const ISIG: TcFlag = 0o000001;
    /// Canonical mode: input is made available line by line
    pub(crate) const ICANON: TcFlag = 0o000002;
    /// Echo input characters
    pub(crate) const ECHO: TcFlag = 0o000010;
    /// Enable implementation-defined input processing (`Ctrl-V`, `Ctrl-O`)
    pub(crate) const IEXTEN: TcFlag = 0o100000;
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
    pub(crate) const VMIN: usize = 6;
    /// Maximum amount of time to wait before `read` returns
    pub(crate) const VTIME: usize = 5;
}

/// Signal sent when the terminal window size changes
pub(crate) const SIGWINCH: c_int = 28;

pub(crate) mod ioctl_request {
    //! `ioctl()` requests for the terminal size (`asm-generic/ioctls.h`).
    use super::IoctlRequest;

    /// Get the window size
    pub(crate) const TIOCGWINSZ: IoctlRequest = 0x5413;
    /// Set the window size
    pub(crate) const TIOCSWINSZ: IoctlRequest = 0x5414;
}
