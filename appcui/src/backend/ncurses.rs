#[cfg(target_family = "unix")]
mod terminal;

#[cfg(target_family = "unix")]
pub (super) mod ncursesapi;

#[cfg(target_family = "unix")]
pub(crate) use self::terminal::NcursesTerminal;