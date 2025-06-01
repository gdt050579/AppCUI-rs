//! A tab UI control that organizes content into multiple selectable pages.
//!
//! The Tab control provides a way to display multiple content areas in the same space.
//! It uses tabs to switch between different views while only showing one view at a time.

mod initialization_flags;
mod tab;
mod tabpage;
#[cfg(test)]
mod tests;

pub use self::initialization_flags::Flags;
pub use self::initialization_flags::Type;
pub use self::tab::Tab;
use self::tabpage::TabPage;
