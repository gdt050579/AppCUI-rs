//! A numericselector UI control for entering and adjusting numeric values.
//!
//! The NumericSelector control provides a field with increment/decrement buttons for numeric input.
//! It supports minimum and maximum value constraints, step size, and different numeric formats.

mod numericselector;
mod initialization_flags;
mod buttons;
pub mod events;
#[cfg(test)]
mod tests;

use self::buttons::Buttons;
pub use self::numericselector::NumericSelector;
pub use self::initialization_flags::Flags;
pub use self::initialization_flags::Format;
