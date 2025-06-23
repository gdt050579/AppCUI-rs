#[cfg(target_family = "unix")]
mod implementation;

#[cfg(target_family = "unix")]
pub (super) mod ncursesapi;

#[cfg(target_family = "unix")]
pub(crate) use self::implementation::NcursesTerminal;