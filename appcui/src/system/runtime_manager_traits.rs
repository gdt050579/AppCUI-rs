use crate::ui::common::*;
use crate::ui::common::traits::*;
use crate::ui::common::control_manager::ParentLayout;
use crate::ui::menu::Menu;
use crate::input::*;
use crate::terminals::*;
use super::Handle;
use super::Theme;

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
pub(crate) trait MouseMethods {
    fn coordinates_to_child_control(&mut self, handle: Handle<UIElement>, x: i32, y: i32, ignore_expanded: bool) -> Handle<UIElement>;
    fn coordinates_to_control(&mut self, x: i32, y: i32, ignore_expanded: bool) -> Handle<UIElement>;
    fn process_menu_and_cmdbar_mousemove(&mut self, x: i32, y: i32) -> bool;
    fn process_menu_mouse_click(&mut self, handle: Handle<Menu>, x: i32, y: i32);
    fn process_mousewheel_event(&mut self, event: MouseWheelEvent);
    fn process_mousedrag(&mut self, handle: Handle<UIElement>, event: MouseMoveEvent);
    fn process_mousemove(&mut self, event: MouseMoveEvent);
    fn process_mousemove_event(&mut self, event: MouseMoveEvent);
    fn process_mousebuttondown_event(&mut self, event: MouseButtonDownEvent);
    fn process_mousebuttonup_event(&mut self, event: MouseButtonUpEvent);
    fn process_mouse_dblclick_event(&mut self, _event: MouseDoubleClickEvent);
}

pub(crate) trait ThemeMethods {
    fn update_theme(&mut self);
    fn update_theme_for_control(&mut self, handle: Handle<UIElement>);
    fn theme(&self) -> &Theme;
    fn set_theme(&mut self, theme: Theme);
}