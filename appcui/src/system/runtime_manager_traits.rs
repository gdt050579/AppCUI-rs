use crate::ui::common::*;
use crate::ui::common::traits::*;
use crate::ui::common::control_manager::ParentLayout;
use crate::ui::menu::Menu;
use crate::input::*;
use super::timer::TimerManager;
use super::Handle;
use super::Theme;
use super::KeyPressedEvent;
use super::MouseButtonDownEvent;
use super::MouseWheelEvent;
use super::MouseMoveEvent;
use super::MouseDoubleClickEvent;
use super::MouseButtonUpEvent;

pub(crate) trait LayoutMethods {
    fn update_control_layout(&mut self, handle: Handle<()>, parent_layout: &ParentLayout);
    fn recompute_layouts(&mut self);
    fn request_recompute_layout(&mut self);
}
pub(crate) trait PaintMethods {
    fn paint(&mut self);
    fn paint_control(&mut self, handle: Handle<()>);
    fn paint_menu(&mut self, handle: Handle<Menu>, activ: bool);
    fn request_repaint(&mut self);
}
 
pub(crate) trait KeyboardMethods {
    fn process_key_modifier_changed_event(&mut self, new_state: KeyModifier);
    fn process_keypressed_event(&mut self, event: KeyPressedEvent);
    fn process_control_keypressed_event(&mut self, handle: Handle<()>, key: Key, character: char) -> EventProcessStatus;
}
pub(crate) trait MouseMethods {
    fn coordinates_to_child_control(&mut self, handle: Handle<()>, x: i32, y: i32, ignore_expanded: bool) -> Handle<()>;
    fn coordinates_to_control(&mut self, x: i32, y: i32, ignore_expanded: bool) -> Handle<()>;
    fn process_menu_and_cmdbar_mousemove(&mut self, x: i32, y: i32) -> bool;
    fn process_menu_mouse_click(&mut self, handle: Handle<Menu>, x: i32, y: i32) -> bool;
    fn process_mousewheel_event(&mut self, event: MouseWheelEvent);
    fn process_mousedrag(&mut self, handle: Handle<()>, event: MouseMoveEvent);
    fn process_mousemove(&mut self, event: MouseMoveEvent);
    fn process_mousemove_event(&mut self, event: MouseMoveEvent);
    fn process_mousebuttondown_event(&mut self, event: MouseButtonDownEvent);
    fn process_mousebuttonup_event(&mut self, event: MouseButtonUpEvent);
    fn process_mouse_dblclick_event(&mut self, _event: MouseDoubleClickEvent);
}

pub(crate) trait ThemeMethods {
    fn update_theme(&mut self);
    fn update_theme_for_control(&mut self, handle: Handle<()>);
    fn theme(&self) -> &Theme;
    fn set_theme(&mut self, theme: Theme);
}

pub(crate) trait TimerMethods {
    fn request_timer_threads_update(&mut self);
    fn get_timer_manager(&mut self) -> &mut TimerManager;
    fn process_timer_tick_update_event(&mut self, id: u8, tick: u64);
    fn process_timer_paused_event(&mut self, id: u8, tick: u64);
    fn process_timer_start_event(&mut self, id: u8, tick: u64);
    fn timer_id_to_control(&mut self, id: u8) -> Option<&mut ControlManager>;
}

pub(crate) trait BackgroundTaskMethods {
    fn background_task_handle_to_control(&mut self, backgoundtask_handle: Handle<()>) -> Option<&mut ControlManager>;
    fn on_start(&mut self, handle: Handle<()>);
    fn on_notify(&mut self, handle: Handle<()>);
    fn on_finish(&mut self, handle: Handle<()>);
    fn on_query(&mut self, handle: Handle<()>);
}