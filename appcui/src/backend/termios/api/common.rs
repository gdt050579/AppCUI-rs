//! Bindings that are identical on every supported Unix: the `<termios.h>` functions and the
//! standard file descriptors. Everything whose layout or value depends on the OS lives in the
//! platform module (`linux.rs` / `macos.rs`).
//!
//! These functions are provided by the system C library, which the Rust standard library already
//! links on every Unix target, so no extra crate is needed.

use std::ffi::c_int;

use super::platform::Termios;

/// File descriptor of the standard input
pub(crate) const STDIN_FILENO: c_int = 0;

extern "C" {
    /// Get the parameters associated with the terminal referred to by `fd`.
    /// Returns `0` on success and `-1` on error.
    pub(crate) fn tcgetattr(fd: c_int, termios: *mut Termios) -> c_int;

    /// Set the parameters associated with the terminal referred to by `fd`.
    /// `optional_actions` specifies when the change takes effect (`TCSANOW`, `TCSADRAIN`, `TCSAFLUSH`).
    /// Returns `0` on success and `-1` on error.
    pub(crate) fn tcsetattr(fd: c_int, optional_actions: c_int, termios: *const Termios) -> c_int;
}
