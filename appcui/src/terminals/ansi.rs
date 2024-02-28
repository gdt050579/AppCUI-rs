mod terminal;

#[cfg(target_family = "unix")]
mod termios;

pub(crate) use self::terminal::AnsiTerminal;

#[cfg(target_family = "unix")]
pub(crate) use self::termios::TermiosError;
