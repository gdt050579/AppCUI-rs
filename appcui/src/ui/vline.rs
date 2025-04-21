//! A vertical line UI control for creating visual separations between components.
//!
//! The VLine control provides a simple vertical divider for organizing interface elements.
//! It can be styled with different line characters and colors to match the application theme.

mod vline;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub use self::initialization_flags::Flags;
pub use self::vline::VLine;