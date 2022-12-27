use super::CharAttribute;

#[repr(u8)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum TextAlignament {
    Left,
    Center,
    Right
}

pub struct TextFormat {    
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    char_attr: CharAttribute,
    hotkey_attr: CharAttribute,    
    align: TextAlignament,
}
