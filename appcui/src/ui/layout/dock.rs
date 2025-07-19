#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Dock {
    Left = 0,
    Right = 1,
    Top = 2,
    Bottom = 3,
    Fill = 4,
}

static HASH_TO_DOCK: [Option<Dock>; 19] = [
    None,
    None,
    Some(Dock::Fill),
    None,
    Some(Dock::Right),
    None,
    None,
    None,
    Some(Dock::Bottom),
    Some(Dock::Fill),
    Some(Dock::Right),
    None,
    Some(Dock::Bottom),
    Some(Dock::Left),
    None,
    Some(Dock::Top),
    Some(Dock::Top),
    Some(Dock::Left),
    None,
];

static HASH_COLISION_VALIDATOR: [u64; 19] = [
    0x0,
    0x0,
    0xAAD01878F02A7608,
    0x0,
    0xAF63EF4C86020CD5,
    0x0,
    0x0,
    0x0,
    0xAF63DF4C8601F1A5,
    0xAF63DB4C8601EAD9,
    0x76AAAA535714D805,
    0x0,
    0xE117B24625D0110A,
    0x24B070ADA2041CB0,
    0x0,
    0xAF63E94C860202A3,
    0x56F9BC194465A83C,
    0xAF63E14C8601F50B,
    0x0,
];

impl Dock {
    pub(super) fn from_hash(hash: u64) -> Option<Dock> {
        let entry_index = (hash % 19) as usize;
        if HASH_COLISION_VALIDATOR[entry_index] != hash {
            return None;
        }
        return HASH_TO_DOCK[entry_index];
    }
    pub fn get_name(&self) -> &'static str {
        match self {
            Dock::Left => "Left",
            Dock::Right => "Right",
            Dock::Top => "Top",
            Dock::Bottom => "Bottom",
            Dock::Fill => "Fill",
        }
    }
}
