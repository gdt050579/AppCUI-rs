//! A numericselector UI control for entering and adjusting numeric values.
//!
//! The NumericSelector control provides a field with increment/decrement buttons for numeric input.
//! It supports minimum and maximum value constraints, step size, and different numeric formats.

mod buttons;
pub mod events;
mod initialization_flags;
pub mod number;
mod numericselector;
#[cfg(test)]
mod tests;

use self::buttons::Buttons;
pub use self::initialization_flags::Flags;
pub use self::initialization_flags::Format;
pub use self::number::Number;
pub use self::numericselector::NumericSelector;
