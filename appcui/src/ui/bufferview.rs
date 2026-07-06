//! A bufferview UI control for displaying and navigating a buffer of data.
//!
//! The BufferView control provides a scrollable view over an in-memory buffer.
//! This is currently a skeleton implementation.

pub mod events;
mod bufferview;
mod initialization_flags;
mod format;
mod output_buffer;
mod interval;
mod codepage;
mod selection;
mod search_parser;
mod buffer;
#[cfg(test)]
mod tests;

use self::output_buffer::OutputBuffer;
use self::interval::IntervalSet;
use self::interval::Segment;
use self::selection::Selection;
use self::buffer::Buffer;

pub use self::bufferview::BufferView;
pub use self::initialization_flags::Flags;
pub use self::format::ColumnsCount;
pub use self::format::Endian;
pub use self::format::DataRepresentationFormat;
pub use self::format::HexFormat;
pub use self::format::IntFormat;
pub use self::format::UIntFormat;
pub use self::format::FloatFormat;
pub use self::format::OffsetFormat;
pub use self::interval::Interval;
pub use self::codepage::Codepage;
pub use self::buffer::BufferAccess;
