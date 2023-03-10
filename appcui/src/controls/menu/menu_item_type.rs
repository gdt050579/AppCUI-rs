#[repr(u8)]
#[derive(Copy,Clone)]
pub (super) enum MenuItemType {
    Command,
    Check,
    Radio,
    Line,
    SubMenu
}