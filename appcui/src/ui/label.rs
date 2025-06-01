//! A label UI control for displaying non-editable text.
//!
//! The Label control provides a simple way to show static text in the interface.
//! It supports single or multi-line text with various formatting options.

mod label;
#[cfg(test)]
mod tests;

pub use self::label::Label;