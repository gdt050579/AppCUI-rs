//! A minimal editor UI control scaffold.
//!
//! This control currently provides baseline event hooks for keyboard, mouse,
//! and paint handling.

mod editor;
mod chunk;
mod line_fragment;
mod line;
#[cfg(test)]
mod tests;

pub use self::editor::Editor;
pub(super) use self::line_fragment::LineFragment;
