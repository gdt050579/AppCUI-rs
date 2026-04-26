//! A minimal editor UI control scaffold.
//!
//! This control currently provides baseline event hooks for keyboard, mouse,
//! and paint handling.

mod editor;
mod string_pool;
mod line_chunk_splitter;
mod line_fragment;
mod line;
#[cfg(test)]
mod tests;

pub use self::editor::Editor;
pub(self) use self::line_fragment::LineFragment;
pub(self) use self::line_chunk_splitter::LineChunkSplitter;
pub(self) use self::string_pool::StringPool;
