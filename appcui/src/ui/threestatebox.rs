//! A threestatebox UI control that can toggle between three distinct states.
//!
//! The ThreeStateBox control extends the standard checkbox with a third indeterminate state.
//! It cycles between unchecked, checked, and indeterminate states when toggled.

pub mod events;
mod initialization_flags;
#[cfg(test)]
mod tests;
mod threestatebox;

pub use self::initialization_flags::State;
pub use self::initialization_flags::Type;
pub use self::threestatebox::ThreeStateBox;
