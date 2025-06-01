//! A colorpicker UI control for selecting colors from a predefined palette.
//!
//! The ColorPicker control provides an interface for choosing colors for text and backgrounds.
//! It supports both foreground and background color selection with previews.

mod colorpicker;
pub mod events;
#[cfg(test)]
mod tests;

pub use self::colorpicker::ColorPicker;
