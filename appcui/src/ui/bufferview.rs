//! A bufferview UI control for displaying and navigating a buffer of data.
//!
//! The BufferView control provides a scrollable view over an in-memory buffer.
//! This is currently a skeleton implementation.

mod bufferview;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub use self::bufferview::BufferView;
pub use self::initialization_flags::BufferAccess;
pub use self::initialization_flags::Flags;
