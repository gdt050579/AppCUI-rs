use crate::graphics::CharAttribute;
/// Character attributes for the common visual states of a control.
///
/// Stores the [`CharAttribute`] used when a control is normal, focused, hovered,
/// inactive, or pressed/selected.
#[derive(Default, Clone, Copy)]
pub struct ControlCharAttributesState {
    pub normal: CharAttribute,
    pub focused: CharAttribute,
    pub hovered: CharAttribute,
    pub inactive: CharAttribute,
    pub pressed_or_selected: CharAttribute,
}
