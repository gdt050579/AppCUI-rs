//! A markdown UI control for composing and editing formatted text content.
//!
//! The MarkdownComposer control provides an editable text area for writing
//! markdown-formatted text, with live inline highlighting as you type.
//! It supports word wrapping, vertical scrolling, mouse and keyboard selection,
//! clipboard operations, and real-time parsing of markdown markers such as
//! emphasis (`**bold**`, `_italic_`), inline code (`` `code` ``) and fenced
//! code blocks (` ```block``` `).
//!
//! Line prefixes are rendered as their formatted counterparts: `*` and `-`
//! become bullet points, `1.` starts an ordered list, and `>` draws a quote
//! bar along the left edge. Wrapped lines are indented to align with the text
//! of the item they continue, rather than with the margin. Links and email
//! addresses are detected and highlighted automatically.
//!
//! The markers themselves can be either hidden, so that only the formatted
//! result is visible, or displayed alongside it while editing.

mod markdown_composer;
mod initialization_flags;
pub mod events;
mod parser;

pub use self::markdown_composer::MarkdownComposer;
pub use self::initialization_flags::Flags;
pub use self::parser::Parser;