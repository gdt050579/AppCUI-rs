//! A panel UI control that serves as a container for organizing other controls.
//!
//! The Panel control provides a rectangular area for grouping related UI elements.
//! It can have optional borders and title, and manages the layout of its child controls.

mod initialization_flags;
mod panel;
#[cfg(test)]
mod tests;

pub use self::initialization_flags::Type;
pub use self::panel::Panel;
