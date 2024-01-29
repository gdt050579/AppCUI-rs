pub(super) enum MenuItemType {
    Command,
    CheckBox,
    SingleChoice,
    Line,
    SubMenu
}
impl MenuItemType {
    pub(super) fn from(text: &str)->Option<MenuItemType> {
        None
    }
}