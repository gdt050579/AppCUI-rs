mod terminal;
pub (super) mod colors;

#[cfg(target_family = "unix")]
pub(crate) use self::terminal::NcursesTerminal;