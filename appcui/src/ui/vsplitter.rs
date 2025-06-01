//! A vertical splitter UI control for dividing space between components.
//!
//! The VSplitter control provides a vertically resizable divider between two panels.
//! It allows users to adjust the relative sizes of components by dragging the divider.

mod vsplitter;
mod initialization_flags;
mod splitter_panel;
#[cfg(test)]
mod tests;

use self::splitter_panel::SplitterPanel;

pub use self::vsplitter::VSplitter;
pub use self::initialization_flags::ResizeBehavior;
pub use self::initialization_flags::Panel;

