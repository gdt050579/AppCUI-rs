//! A button UI control that handles click events and provides various visual styles.
//!
//! The Button control provides a clickable interface element that can trigger actions.
//! It supports different styles including normal and flat buttons with customizable captions.

mod button;
pub mod events;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub use self::button::Button;
pub use self::initialization_flags::Type;
