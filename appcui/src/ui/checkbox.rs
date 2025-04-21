//! A checkbox UI control that allows toggling between checked and unchecked states.
//!
//! The CheckBox control provides a selectable option with a label that can be toggled on or off.
//! It supports different visual styles and can be configured to raise events when its state changes.

mod checkbox;
mod initialization_flags;
pub mod events;
#[cfg(test)]
mod tests;
pub use self::checkbox::CheckBox;
pub use self::initialization_flags::Type;



