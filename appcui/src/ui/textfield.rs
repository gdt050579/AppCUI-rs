//! A textfield UI control for single-line text input and editing.
//!
//! The TextField control provides a way for users to enter and edit text.
//! It supports features like text selection, clipboard operations, and input validation.

mod textfield;
mod initialization_flags;
mod char_class;
pub mod selection;
pub mod events;
#[cfg(test)]
mod tests;

use self::selection::Selection;
pub use self::textfield::TextField;
pub use self::initialization_flags::Flags;
pub(crate) use self::char_class::CharClass;