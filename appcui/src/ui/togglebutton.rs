//! A togglebutton UI control that can be switched between pressed and unpressed states.
//!
//! The ToggleButton control provides a button that maintains its state when clicked.
//! It visually indicates its current state and can be used for binary on/off choices.

pub mod events;
mod initialization_flags;
#[cfg(test)]
mod tests;
mod togglebutton;

pub use self::initialization_flags::Type;
pub use self::togglebutton::ToggleButton;
