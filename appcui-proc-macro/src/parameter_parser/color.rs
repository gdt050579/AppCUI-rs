#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub(crate) enum Color {
    Black,
    DarkBlue,
    DarkGreen,
    Teal,
    DarkRed,
    Magenta,
    Olive,
    Silver,
    Gray,
    Blue,
    Green,
    Aqua,
    Red,
    Pink,
    Yellow,
    White,
    Transparent,
}
impl Color {
    pub(super) fn from_hash(hash: u64) -> Option<Color> {
        return None;
        // let entry_index = (hash % 95) as usize;
        // if HASH_COLISION_VALIDATOR[entry_index] != hash {
        //     return None;
        // }
        // return HASH_TO_ALIGNAMENT[entry_index];
    }
    pub(crate) fn get_name(&self) -> &'static str {
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
}