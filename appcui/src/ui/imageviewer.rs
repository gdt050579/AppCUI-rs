//! An imageviewer UI control for displaying and manipulating images.
//!
//! The ImageViewer control provides functionality to display images within the terminal.
//! It supports various operations like zooming, panning, and different rendering modes.

mod imageviewer;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub use self::imageviewer::ImageViewer;
pub use self::initialization_flags::Flags;