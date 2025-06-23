//! A horizontal splitter UI control for dividing space between components.
//!
//! The HSplitter control provides a horizontally resizable divider between two panels.
//! It allows users to adjust the relative sizes of components by dragging the divider.

mod hsplitter;
mod initialization_flags;
mod splitter_panel;
#[cfg(test)]
mod tests;

use self::splitter_panel::SplitterPanel;

pub use self::hsplitter::HSplitter;
pub use self::initialization_flags::ResizeBehavior;
pub use self::initialization_flags::Panel;

