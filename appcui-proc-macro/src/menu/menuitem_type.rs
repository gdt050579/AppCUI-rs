#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub(crate) enum MenuItemType {
    Command = 0,
    CheckBox = 1,
    SingleChoice = 2,
    SubMenu = 3,
    Separator = 4,
}

static HASH_TO_ALIGNAMENT: [Option<MenuItemType>; 45] = [
    Some(MenuItemType::Separator),
    Some(MenuItemType::SingleChoice),
    Some(MenuItemType::SubMenu),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(MenuItemType::Command),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(MenuItemType::CheckBox),
    None,
    Some(MenuItemType::SingleChoice),
    None,
    Some(MenuItemType::SingleChoice),
    None,
    Some(MenuItemType::Separator),
    Some(MenuItemType::SubMenu),
    None,
    None,
    Some(MenuItemType::SingleChoice),
    None,
    Some(MenuItemType::CheckBox),
    None,
    Some(MenuItemType::CheckBox),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(MenuItemType::SubMenu),
    Some(MenuItemType::Command),
    None,
    None,
    Some(MenuItemType::Separator),
];

static HASH_COLISION_VALIDATOR: [u64; 45] = [
    0x823B8B195CE214EF,
    0x722B5C4B1B0B8CBD,
    0x61C886171CC4D9CA,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x66655DCA2C3E8A12,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x830CF17637260A67,
    0x0,
    0x51D04A40A1499DDE,
    0x0,
    0xD5DC35DCB7E762A9,
    0x0,
    0xA9568C2DDFE55A5C,
    0x42FF01A2E311EE9A,
    0x0,
    0x0,
    0x8813D411D1692D14,
    0x0,
    0x26653529B19C5158,
    0x0,
    0xCBF29CE484222325,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x0,
    0x591EC3D12BE5802F,
    0xF60BF7190D078D0B,
    0x0,
    0x0,
    0xBF4BA5AD694F5907,
];

impl MenuItemType {
    pub(super) fn from_hash(hash: u64) -> Option<MenuItemType> {
        let entry_index = (hash % 45) as usize;
        if HASH_COLISION_VALIDATOR[entry_index] != hash {
            return None;
        }
        return HASH_TO_ALIGNAMENT[entry_index];
    }
    pub fn _get_name(&self) -> &'static str {
        match self {
            MenuItemType::Command => "Command",
            MenuItemType::CheckBox => "CheckBox",
            MenuItemType::SingleChoice => "SingleChoice",
            MenuItemType::SubMenu => "SubMenu",
            MenuItemType::Separator => "Line",
        }
    }
}
