use crate::graphics::CharAttribute;
#[derive(Default)]
pub struct ControlCharAttributesState {
    pub normal: CharAttribute,
    pub focused: CharAttribute,
    pub hovered: CharAttribute,
    pub inactive: CharAttribute,
    pub pressed_or_selectd: CharAttribute,
}