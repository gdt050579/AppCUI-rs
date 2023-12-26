use crate::ui::common::UIElement;
use crate::ui::common::control_manager::ParentLayout;
use crate::ui::menu::Menu;
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