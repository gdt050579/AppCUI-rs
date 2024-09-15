#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub(crate) enum TextAlignament {
    Left = 0,
    Right = 1,
    Center = 2,
}

static HASH_TO_ALIGNAMENT: [Option<TextAlignament>; 9] = [
    Some(TextAlignament::Center),
    None,
    Some(TextAlignament::Right),
    Some(TextAlignament::Left),
    None,
    Some(TextAlignament::Center),
    Some(TextAlignament::Left),
    None,
    Some(TextAlignament::Right),
];

static HASH_COLISION_VALIDATOR: [u64; 9] = [
    0xAF63DE4C8601EFF2,
    0x0,
    0x76AAAA535714D805,
    0xAF63E14C8601F50B,
    0x0,
    0x6F4B7EC4DCAA8AC4,
    0x24B070ADA2041CB0,
    0x0,
    0xAF63EF4C86020CD5,
];

impl TextAlignament {
    pub(super) fn from_hash(hash: u64) -> Option<TextAlignament> {
        let entry_index = (hash % 9) as usize;
        if HASH_COLISION_VALIDATOR[entry_index] != hash {
            return None;
        }
        HASH_TO_ALIGNAMENT[entry_index]
    }
    pub fn get_name(&self) -> &'static str {
        match self {
            TextAlignament::Left => "Left",
            TextAlignament::Right => "Right",
            TextAlignament::Center => "Center",
        }
    }
}
