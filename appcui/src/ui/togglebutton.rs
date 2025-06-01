//! A togglebutton UI control that can be switched between pressed and unpressed states.
//!
//! The ToggleButton control provides a button that maintains its state when clicked.
//! It visually indicates its current state and can be used for binary on/off choices.

mod togglebutton;
mod initialization_flags;
pub mod events;
#[cfg(test)]
mod tests;

pub use self::togglebutton::ToggleButton;
pub use self::initialization_flags::Type;