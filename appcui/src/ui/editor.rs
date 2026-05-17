//! A minimal editor UI control scaffold.
//!
//! This control currently provides baseline event hooks for keyboard, mouse,
//! and paint handling.

mod editor;
mod line;
mod view_port;
mod document;
mod char_class;
mod selection;
mod initialization_flags;
pub mod events;
#[cfg(test)]
mod tests;

pub(self) use self::document::Document;
pub(self) use self::document::PosInfo;
pub(self) use self::char_class::CharClass;
pub(self) use self::selection::Selection;
pub(self) use self::line::Line;
pub(self) use self::line::LineCharIndex;
pub(self) use self::line::VisualPos;
pub(self) use self::view_port::ViewPort;
pub(self) use self::view_port::VisualRowRef;
pub use self::editor::Editor;
pub use self::initialization_flags::Flags;