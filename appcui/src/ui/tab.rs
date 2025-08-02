//! A tab UI control that organizes content into multiple selectable pages.
//!
//! The Tab control provides a way to display multiple content areas in the same space.
//! It uses tabs to switch between different views while only showing one view at a time.

mod tab;
mod tabpage;
mod initialization_flags;
pub mod events;
#[cfg(test)]
mod tests;

use self::tabpage::TabPage;
pub use self::tab::Tab;
pub use self::initialization_flags::Type;
pub use self::initialization_flags::Flags;