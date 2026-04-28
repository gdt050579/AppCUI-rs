//! A minimal editor UI control scaffold.
//!
//! This control currently provides baseline event hooks for keyboard, mouse,
//! and paint handling.

mod editor;
mod document;
mod string_pool;
mod line_chunk_splitter;
mod line_fragment;
mod line;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub(self) use self::line_fragment::LineFragment;
pub(self) use self::line_chunk_splitter::LineChunkSplitter;
pub(self) use self::string_pool::StringPool;
pub(self) use self::document::Document;

pub use self::editor::Editor;
pub use self::initialization_flags::Flags;