//! A markdown UI control for displaying formatted text content.
//!
//! The Markdown control provides a way to render and display markdown-formatted text.
//! It supports standard markdown syntax including headers, lists, links, and emphasis.

pub mod events;
mod initialization_flags;
mod markdown;
#[cfg(test)]
mod tests;

pub use self::initialization_flags::Flags;
pub use self::markdown::Markdown;
