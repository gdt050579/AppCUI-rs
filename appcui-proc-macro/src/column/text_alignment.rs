#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub(crate) enum TextAlignment {
    Left = 0,
    Right = 1,
    Center = 2,
}

static HASH_TO_ALIGNAMENT: [Option<TextAlignment>; 9] = [
    Some(TextAlignment::Center),
    None,
    Some(TextAlignment::Right),
    Some(TextAlignment::Left),
    None,
    Some(TextAlignment::Center),
    Some(TextAlignment::Left),
    None,
    Some(TextAlignment::Right),
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

impl TextAlignment {
    pub(super) fn from_hash(hash: u64) -> Option<TextAlignment> {
        let entry_index = (hash % 9) as usize;
        if HASH_COLISION_VALIDATOR[entry_index] != hash {
            return None;
        }
        HASH_TO_ALIGNAMENT[entry_index]
    }
    pub fn get_name(&self) -> &'static str {
        match self {
            TextAlignment::Left => "Left",
            TextAlignment::Right => "Right",
            TextAlignment::Center => "Center",
        }
    }
}
