//! A textfield UI control for single-line text input and editing.
//!
//! The TextField control provides a way for users to enter and edit text.
//! It supports features like text selection, clipboard operations, and input validation.

mod char_class;
pub mod events;
mod initialization_flags;
pub mod selection;
#[cfg(test)]
mod tests;
mod textfield;

pub(crate) use self::char_class::CharClass;
pub use self::initialization_flags::Flags;
use self::selection::Selection;
pub use self::textfield::TextField;
