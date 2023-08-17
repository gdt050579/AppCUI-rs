use crate::graphics::CharAttribute;

pub(super) struct PaintData {
    pub(super) focused: bool,
    pub(super) current: bool,
    pub(super) maximized: bool,
    pub(super) is_current_item_pressed: bool,
    pub(super) sep_attr: CharAttribute,
}