#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum Color {
    /// <table><tr><td style="background-color: #000000; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    #[default]
    Black = 0x00,

    /// <table><tr><td style="background-color: #000080; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    DarkBlue = 0x01,

    /// <table><tr><td style="background-color: #008000; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    DarkGreen = 0x02,

    /// <table><tr><td style="background-color: #008080; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Teal = 0x03,

    /// <table><tr><td style="background-color: #800000; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    DarkRed = 0x04,

    /// <table><tr><td style="background-color: #800080; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Magenta = 0x05,

    /// <table><tr><td style="background-color: #808000; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Olive = 0x06,

    /// <table><tr><td style="background-color: #C0C0C0; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Silver = 0x07,

    /// <table><tr><td style="background-color: #808080; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Gray = 0x08,

    /// <table><tr><td style="background-color: #0000FF; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Blue = 0x09,

    /// <table><tr><td style="background-color: #00FF00; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Green = 0x0A,

    /// <table><tr><td style="background-color: #00FFFF; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Aqua = 0x0B,

    /// <table><tr><td style="background-color: #FF0000; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Red = 0x0C,

    /// <table><tr><td style="background-color: #FF00FF; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Pink = 0x0D,

    /// <table><tr><td style="background-color: #FFFF00; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    Yellow = 0x0E,

    /// <table><tr><td style="background-color: #FFFFFF; width: 20px; height: 20px; border: 1px solid #000;"></td></tr></table>
    White = 0x0F,

    /// a marker for transparent color
    Transparent = 0x10,
}
impl Color {
    pub fn from_value(value: i32) -> Option<Color> {
        match value {
            0 => Some(Color::Black),
            1 => Some(Color::DarkBlue),
            2 => Some(Color::DarkGreen),
            3 => Some(Color::Teal),
            4 => Some(Color::DarkRed),
            5 => Some(Color::Magenta),
            6 => Some(Color::Olive),
            7 => Some(Color::Silver),
            8 => Some(Color::Gray),
            9 => Some(Color::Blue),
            10 => Some(Color::Green),
            11 => Some(Color::Aqua),
            12 => Some(Color::Red),
            13 => Some(Color::Pink),
            14 => Some(Color::Yellow),
            15 => Some(Color::White),
            16 => Some(Color::Transparent),
            _ => None,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Color::Black => "Black",
            Color::DarkBlue => "DarkBlue",
            Color::DarkGreen => "DarkGreen",
            Color::Teal => "Teal",
            Color::DarkRed => "DarkRed",
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

    pub fn to_rgba(&self) -> [f32; 4] {
        match self {
            Color::Black => [0.0, 0.0, 0.0, 1.0],
            Color::DarkBlue => [0.0, 0.0, 127.5, 1.0],
            Color::DarkGreen => [0.0, 127.5, 0.0, 1.0],
            Color::Teal => [0.0, 127.5, 127.5, 1.0],
            Color::DarkRed => [127.5, 0.0, 0.0, 1.0],
            Color::Magenta => [127.5, 0.0, 127.5, 1.0],
            Color::Olive => [127.5, 127.5, 0.0, 1.0],
            Color::Silver => [191.25, 191.25, 191.25, 1.0],
            Color::Gray => [127.5, 127.5, 127.5, 1.0],
            Color::Blue => [0.0, 0.0, 255.0, 1.0],
            Color::Green => [0.0, 255.0, 0.0, 1.0],
            Color::Aqua => [0.0, 255.0, 255.0, 1.0],
            Color::Red => [255.0, 0.0, 0.0, 1.0],
            Color::Pink => [255.0, 0.0, 255.0, 1.0],
            Color::Yellow => [255.0, 255.0, 0.0, 1.0],
            Color::White => [255.0, 255.0, 255.0, 1.0],
            Color::Transparent => [0.0, 0.0, 0.0, 0.0],
        }
    }
}
