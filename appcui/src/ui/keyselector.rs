//! A keyselector UI control for capturing and configuring keyboard shortcuts.
//!
//! The KeySelector control provides a specialized input field for assigning keyboard shortcuts.
//! It captures keyboard combinations and displays them in a user-friendly format.

pub mod events;
mod initialization_flags;
mod keyselector;
#[cfg(test)]
mod tests;

pub use self::initialization_flags::Flags;
pub use self::keyselector::KeySelector;
