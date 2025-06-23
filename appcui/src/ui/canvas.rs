//! A canvas UI control for custom drawing and direct surface manipulation.
//!
//! The Canvas control provides a blank area where applications can perform custom
//! drawing operations. It gives direct access to a surface for character-level drawing.

mod canvas;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub use self::canvas::Canvas;
pub use self::initialization_flags::Flags;