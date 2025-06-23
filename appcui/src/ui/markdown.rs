//! A markdown UI control for displaying formatted text content.
//!
//! The Markdown control provides a way to render and display markdown-formatted text.
//! It supports standard markdown syntax including headers, lists, links, and emphasis.

mod markdown;
pub mod events;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub use self::markdown::Markdown;
pub use self::initialization_flags::Flags;