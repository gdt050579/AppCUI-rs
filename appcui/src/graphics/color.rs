#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    Black = 0x00,
    DarkBlue = 0x01,
    DarkGreen = 0x02,
    Teal = 0x03,
    DarkRed = 0x04,
    Magenta = 0x05,
    Olive = 0x06,
    Silver = 0x07,
    Gray = 0x08,
    Blue = 0x09,
    Green = 0x0A,
    Aqua = 0x0B,
    Red = 0x0C,
    Pink = 0x0D,
    Yellow = 0x0E,
    White = 0x0F,
    Transparent = 0x10,
}
impl Color {
    pub fn get_name(&self)->&str {
        match self {
            Color::Black => "Black",
            Color::DarkBlue => "DarkBlue",
            Color::DarkGreen => "DarkGreen",
            Color::Teal => "Teal",
            Color::DarkRed => "DarkRead",
            Color::Magenta => "Magenta",
            Color::Olive => "Olive",
            Color::Silver => "Silver",
            Color::Gray => "Gray",
            Color::Blue => "Blue",
            Color::Green => "Green",
            Color::Aqua => "Aqua",
            Color::Red => "Red",
            Color::Pink => "Pink",
            Color::Yellow => "Yellow",
            Color::White => "White",
            Color::Transparent => "Transparent",
        }
    }
}