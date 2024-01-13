use crate::ui::common::*;
use crate::ui::common::traits::*;
use crate::ui::common::control_manager::ParentLayout;
use crate::ui::menu::Menu;
use crate::input::*;
use crate::terminals::*;
use super::Handle;

pub(crate) trait LayoutMethods {
    fn update_control_layout(&mut self, handle: Handle<UIElement>, parent_layout: &ParentLayout);
    fn recompute_layouts(&mut self);
    fn request_recompute_layout(&mut self);
}
pub(crate) trait PaintMethods {
    fn paint(&mut self);
    fn paint_control(&mut self, handle: Handle<UIElement>);
    fn paint_menu(&mut self, handle: Handle<Menu>, activ: bool);
    fn request_repaint(&mut self);
}

pub(crate) trait KeyboardMethods {
    fn process_key_modifier_changed_event(&mut self, new_state: KeyModifier);
    fn process_keypressed_event(&mut self, event: KeyPressedEvent);
    fn process_control_keypressed_event(&mut self, handle: Handle<UIElement>, key: Key, character: char) -> EventProcessStatus;
}