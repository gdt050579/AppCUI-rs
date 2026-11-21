//! Bindings that are identical on every supported Unix: the `<termios.h>` functions and the
//! standard file descriptors. Everything whose layout or value depends on the OS lives in the
//! platform module (`linux.rs` / `macos.rs`).
//!
//! These functions are provided by the system C library, which the Rust standard library already
//! links on every Unix target, so no extra crate is needed.

use std::ffi::{c_int, c_short, c_ushort, c_void};

use super::platform::{IoctlRequest, NfdsT, Termios};

/// File descriptor of the standard input
pub(crate) const STDIN_FILENO: c_int = 0;

/// File descriptor of the standard output
pub(crate) const STDOUT_FILENO: c_int = 1;

/// `sighandler_t`: address of a signal handler, `SIG_DFL` (0), `SIG_IGN` (1) or `SIG_ERR`
pub(crate) type SigHandler = usize;
/// Value returned by `signal()` on error (`(sighandler_t) -1`)
pub(crate) const SIG_ERR: SigHandler = !0;

/// `struct winsize`, filled by `ioctl(TIOCGWINSZ)` (same layout on Linux and macOS)
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub(crate) struct Winsize {
    /// Rows, in characters
    pub(crate) ws_row: c_ushort,
    /// Columns, in characters
    pub(crate) ws_col: c_ushort,
    /// Horizontal size, in pixels (not used)
    pub(crate) ws_xpixel: c_ushort,
    /// Vertical size, in pixels (not used)
    pub(crate) ws_ypixel: c_ushort,
}

const _: () = assert!(std::mem::size_of::<Winsize>() == 8);

/// `poll()` event: there is data to read
pub(crate) const POLLIN: c_short = 0x0001;

/// `struct pollfd` (same layout on Linux and macOS)
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub(crate) struct PollFd {
    /// File descriptor to watch
    pub(crate) fd: c_int,
    /// Requested events
    pub(crate) events: c_short,
    /// Returned events
    pub(crate) revents: c_short,
}

const _: () = assert!(std::mem::size_of::<PollFd>() == 8);

extern "C" {
    /// Get the parameters associated with the terminal referred to by `fd`.
    /// Returns `0` on success and `-1` on error.
    pub(crate) fn tcgetattr(fd: c_int, termios: *mut Termios) -> c_int;

    /// Set the parameters associated with the terminal referred to by `fd`.
    /// `optional_actions` specifies when the change takes effect (`TCSANOW`, `TCSADRAIN`, `TCSAFLUSH`).
    /// Returns `0` on success and `-1` on error.
    pub(crate) fn tcsetattr(fd: c_int, optional_actions: c_int, termios: *const Termios) -> c_int;

    /// Read up to `count` bytes from `fd` into `buf` (`<unistd.h>`).
    /// Returns the number of bytes read, `0` at end of file, or `-1` on error.
    pub(crate) fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;

    /// Wait for one of the file descriptors in `fds` to become ready (`<poll.h>`).
    /// `timeout` is in milliseconds (`-1` waits forever).
    /// Returns the number of ready descriptors, `0` on timeout, or `-1` on error.
    pub(crate) fn poll(fds: *mut PollFd, nfds: NfdsT, timeout: c_int) -> c_int;

    /// Device-specific control call (`<sys/ioctl.h>`). Used here with `TIOCGWINSZ` / `TIOCSWINSZ`.
    /// Returns `-1` on error.
    pub(crate) fn ioctl(fd: c_int, request: IoctlRequest, ...) -> c_int;

    /// Install `handler` for the signal `signum` (`<signal.h>`).
    /// Returns the previous handler, or `SIG_ERR` on error.
    pub(crate) fn signal(signum: c_int, handler: SigHandler) -> SigHandler;
}
