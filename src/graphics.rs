mod character;
mod color;
mod char_attribute;
mod surface;
mod line_type;
mod size;
mod clip_area;
#[cfg(test)]
mod tests;


pub use self::character::Character;
pub use self::character::SpecialChar;
pub use self::color::Color;
pub use self::char_attribute::CharFlags;
pub use self::char_attribute::CharAttribute;
pub use self::line_type::LineType;
pub use self::surface::Surface;
pub use self::size::Size;
use self::clip_area::ClipArea;
