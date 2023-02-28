mod character;
mod color;
mod char_attribute;
mod surface;
mod line_type;
mod size;
mod clip_area;
mod rect;
mod point;
mod cursor;
mod pixel;
mod image;
mod text_format;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod surface_tester;

pub use self::character::Character;
pub use self::character::SpecialChar;
pub use self::color::Color;
pub use self::char_attribute::CharFlags;
pub use self::char_attribute::CharAttribute;
pub use self::line_type::LineType;
pub use self::surface::Surface;
pub use self::surface::ImageRenderingMethod;
pub use self::surface::ImageScaleMethod;
pub use self::size::Size;
pub use self::rect::Rect;
pub use self::cursor::Cursor;
pub use self::pixel::Pixel;
pub use self::image::Image;
pub use self::text_format::TextAlignament;
pub use self::text_format::TextFormat;
pub use self::text_format::TextWrap;
pub (crate) use self::clip_area::ClipArea;
pub (crate) use self::point::Point;

#[cfg(test)]
pub (crate) use self::surface_tester::SurfaceTester;
