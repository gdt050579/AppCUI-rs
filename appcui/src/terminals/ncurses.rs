mod terminal;
pub (super) mod colors;
pub (super) mod ncursesapi;

#[cfg(target_family = "unix")]
pub(crate) use self::terminal::NcursesTerminal;