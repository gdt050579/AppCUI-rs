//! A horizontal line UI control for creating visual separations between components.
//!
//! The HLine control provides a simple horizontal divider for organizing interface elements.
//! It can be styled with different line characters and colors to match the application theme.

mod hline;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub use self::initialization_flags::Flags;
pub use self::hline::HLine;