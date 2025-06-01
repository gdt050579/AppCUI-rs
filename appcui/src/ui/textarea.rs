//! A textarea UI control for multi-line text input and editing.
//!
//! The TextArea control provides a scrollable field for entering and editing larger amounts of text.
//! It supports word wrapping, text selection, and various keyboard shortcuts for text manipulation.

pub mod events;
mod initialization_flags;
#[cfg(test)]
mod tests;
mod textarea;

pub use self::initialization_flags::Flags;
pub use self::textarea::TextArea;
