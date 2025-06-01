//! A password UI control for secure text input with masked characters.
//!
//! The Password control provides a specialized text input field that hides the entered text.
//! It offers the same editing capabilities as TextField but displays mask characters instead of actual input.

mod password;
pub mod events;
#[cfg(test)]
mod tests;

pub use self::password::Password;
