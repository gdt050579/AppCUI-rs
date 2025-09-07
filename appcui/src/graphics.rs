//! This module contains the graphics-related types and functions such as:
//! * `Color`
//! * `Point`
//! * `Rect`
//! * `Size`
//! * `Surface`

mod char_attribute;
mod character;
mod clip_area;
mod color;
mod cursor;
pub mod image;
mod line_type;
mod point;
mod rect;
mod size;
mod surface;
mod text_format;
mod orthogonal_direction;

#[cfg(test)]
mod surface_tester;
#[cfg(test)]
mod tests;

pub use self::char_attribute::CharAttribute;
pub use self::char_attribute::CharFlags;
pub use self::character::Character;
pub use self::character::SpecialChar;
pub(crate) use self::clip_area::ClipArea;
pub use self::color::Color;
pub(crate) use self::cursor::Cursor;
pub use self::image::Image;
pub use self::image::*;
pub use self::line_type::LineType;
pub use self::point::Point;
pub use self::rect::Rect;
pub use self::rect::RectAlignment;
pub use self::size::Size;
pub use self::surface::Surface;
pub use self::text_format::TextAlignment;
pub use self::text_format::TextFormat;
pub use self::text_format::TextFormatBuilder;
pub use self::text_format::WrapType;
pub use self::orthogonal_direction::OrthogonalDirection;

#[cfg(test)]
pub(crate) use self::surface_tester::SurfaceTester;
