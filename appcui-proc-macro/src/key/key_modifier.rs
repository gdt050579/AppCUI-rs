#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub(crate) enum KeyModifier {
    None = 0,
    Alt = 1,
    Shift = 2,
    Ctrl = 3,
}

static HASH_TO_ALIGNAMENT: [Option<KeyModifier>; 10] = [
    Some(KeyModifier::Alt),
    None,
    Some(KeyModifier::Ctrl),
    Some(KeyModifier::Shift),
    Some(KeyModifier::None),
    None,
    None,
    None,
    None,
    Some(KeyModifier::None),
];

static HASH_COLISION_VALIDATOR: [u64; 10] = [
    0xE6F0A3190519E83C,
    0x0,
    0x45253F90A9043CC4,
    0x297A5749140AC887,
    0x904FEFB3D01CB2AE,
    0x0,
    0x0,
    0x0,
    0x0,
    0x3C0D17BAD169557B,
];

impl KeyModifier {
    pub(super) fn from_hash(hash: u64) -> Option<KeyModifier> {
        let entry_index = (hash % 10) as usize;
        if HASH_COLISION_VALIDATOR[entry_index] != hash {
            return None;
        }
        HASH_TO_ALIGNAMENT[entry_index]
    }
    pub fn _get_name(&self) -> &'static str {
        match self {
            KeyModifier::None => "None",
            KeyModifier::Alt => "Alt",
            KeyModifier::Shift => "Shift",
            KeyModifier::Ctrl => "Ctrl",
        }
    }
}
