//! A minimal editor UI control scaffold.
//!
//! This control currently provides baseline event hooks for keyboard, mouse,
//! and paint handling.

mod editor;
#[cfg(test)]
mod tests;

pub use self::editor::Editor;
