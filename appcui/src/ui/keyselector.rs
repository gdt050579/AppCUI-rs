//! A keyselector UI control for capturing and configuring keyboard shortcuts.
//!
//! The KeySelector control provides a specialized input field for assigning keyboard shortcuts.
//! It captures keyboard combinations and displays them in a user-friendly format.

mod keyselector;
mod initialization_flags;
pub mod events;
#[cfg(test)]
mod tests;

pub use self::keyselector::KeySelector;
pub use self::initialization_flags::Flags;