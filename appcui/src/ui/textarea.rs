//! A textarea UI control for multi-line text input and editing.
//!
//! The TextArea control provides a scrollable field for entering and editing larger amounts of text.
//! It supports word wrapping, text selection, and various keyboard shortcuts for text manipulation.

mod textarea;
mod initialization_flags;
pub mod events;
#[cfg(test)]
mod tests;

pub use self::textarea::TextArea;
pub use self::initialization_flags::Flags;