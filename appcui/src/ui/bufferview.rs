//! A bufferview UI control for displaying and navigating a buffer of data.
//!
//! The BufferView control provides a scrollable view over an in-memory buffer.
//! This is currently a skeleton implementation.

mod bufferview;
mod initialization_flags;
mod format;
mod output_buffer;
#[cfg(test)]
mod tests;

use self::output_buffer::OutputBuffer;
use self::format::Representation;

pub use self::bufferview::BufferView;
pub use self::initialization_flags::BufferAccess;
pub use self::initialization_flags::Flags;
pub use self::format::ColumnsCount;
pub use self::format::Endian;
pub use self::format::Format;
pub use self::format::BytesCount;
