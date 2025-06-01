//! A radiobox UI control for selecting a single option from a mutually exclusive set.
//!
//! The RadioBox control provides a circular button that can be selected or deselected.
//! It's typically used in groups where only one option can be active at a time.

mod radiobox;
mod initialization_flags;

pub mod events;
#[cfg(test)]
mod tests;
pub use self::radiobox::RadioBox;
pub use self::initialization_flags::Type;
