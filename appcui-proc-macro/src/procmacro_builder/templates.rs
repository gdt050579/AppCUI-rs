// pub(crate) static IMPORTS_old: &str = "
// use $(ROOT)::ui::*;
// use $(ROOT)::ui::common::traits::*;
// use $(ROOT)::ui::button::events::ButtonEvents;
// use $(ROOT)::ui::checkbox::events::CheckBoxEvents;
// use $(ROOT)::ui::window::events::WindowEvents;
// use $(ROOT)::ui::window::events::ToolBarEvents;
// use $(ROOT)::ui::command_bar::events::CommandBarEvents;
// use $(ROOT)::ui::menu::events::MenuEvents;
// use $(ROOT)::ui::menu::*;
// use $(ROOT)::graphics::*;
// use $(ROOT)::system::*;
// use $(ROOT)::input::*;
// ";
pub(crate) static IMPORTS: &str = "
use $(ROOT)::prelude::*;
";

pub(crate) static IMPORTS_INTERNAL: &str = "
use crate::utils::*;
use crate::ui::common::*;
";

pub(crate) static DEREF_TRAIT: &str = "
impl std::ops::Deref for $(STRUCT_NAME) {
    type Target = $(BASE);
    fn deref(&self) -> &Self::Target { return &self.base; }
}
impl std::ops::DerefMut for $(STRUCT_NAME) {
    fn deref_mut(&mut self) -> &mut Self::Target { return &mut self.base; }
}
";

pub(crate) static ON_PAINT_TRAIT: &str = "
impl OnPaint for $(STRUCT_NAME) {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme)  { self.base.on_paint(surface, theme); }
}
";

pub(crate) static ON_KEY_PRESSED_TRAIT: &str = "
impl OnKeyPressed for $(STRUCT_NAME) {
    fn on_key_pressed(&mut self, key: Key, character: char)->EventProcessStatus { return self.base.on_key_pressed(key, character); }
}
";

pub(crate) static ON_MOUSE_EVENT_TRAIT: &str = "
impl OnMouseEvent for $(STRUCT_NAME) {
    fn on_mouse_event(&mut self, event: &MouseEvent)->EventProcessStatus { return self.base.on_mouse_event(event); }
}
";

pub(crate) static ON_DEFAULT_ACTION_TRAIT: &str = "
impl OnDefaultAction for $(STRUCT_NAME) {
    fn on_default_action(&mut self){ self.base.on_default_action(); }
}
";

pub(crate) static ON_RESIZE_TRAIT: &str = "
impl OnResize for $(STRUCT_NAME) {
    fn on_resize(&mut self, old: Size, new: Size)  { self.base.on_resize(old, new); }
}
";

pub(crate) static ON_FOCUS_TRAIT: &str = "
impl OnFocus for $(STRUCT_NAME) {
    fn on_focus(&mut self)  { self.base.on_focus(); }
    fn on_lose_focus(&mut self)  { self.base.on_lose_focus(); }
}
";

pub(crate) static ON_WINDOW_REGISTERED_TRAIT: &str = "
impl OnWindowRegistered for $(STRUCT_NAME) {
    fn on_registered(&mut self)  { self.base.on_registered(); }
}
";