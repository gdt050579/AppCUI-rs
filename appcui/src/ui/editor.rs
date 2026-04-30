//! A minimal editor UI control scaffold.
//!
//! This control currently provides baseline event hooks for keyboard, mouse,
//! and paint handling.

mod editor;
mod document;
mod char_class;
mod selection;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub(self) use self::document::Document;
pub(self) use self::char_class::CharClass;
pub(self) use self::selection::Selection;

pub use self::editor::Editor;
pub use self::initialization_flags::Flags;