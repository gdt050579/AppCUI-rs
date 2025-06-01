//! A threestatebox UI control that can toggle between three distinct states.
//!
//! The ThreeStateBox control extends the standard checkbox with a third indeterminate state.
//! It cycles between unchecked, checked, and indeterminate states when toggled.

mod threestatebox;
mod initialization_flags;
pub mod events;
#[cfg(test)]
mod tests;

pub use self::threestatebox::ThreeStateBox;
pub use self::initialization_flags::State;
pub use self::initialization_flags::Type;