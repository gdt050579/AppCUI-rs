#[repr(u8)]
#[derive(Copy,Clone,PartialEq)]
pub (super) enum MenuItemType {
    Command,
    Check,
    Radio,
    Line,
    SubMenu
}