//! A markdown UI control for composing and editing formatted text content.
//!
//! The MarkdownComposer control provides an editable text area for writing
//! markdown-formatted text, with live inline highlighting as you type.
//! It supports word wrapping, cursor navigation, and real-time parsing of
//! markdown markers such as emphasis (`**bold**`, `_italic_`) and emoji
//! shortcodes (`:cat:`).

mod markdown_composer;
mod initialization_flags;
pub mod events;
mod parser;

pub use self::markdown_composer::MarkdownComposer;
pub use self::initialization_flags::Flags;
pub use self::parser::{Parser, Span, SpanType};