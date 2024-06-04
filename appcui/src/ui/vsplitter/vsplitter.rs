use crate::prelude::*;
use super::SplitterPanel;

#[CustomControl(overwrite=OnPaint + OnKeyPressed + OnMouseEvent + OnResize + OnFocus, internal = true)]
pub struct VSplitter {
    left: Handle<SplitterPanel>,
    right: Handle<SplitterPanel>,
}
impl VSplitter {

}
impl OnPaint for VSplitter {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}
impl OnKeyPressed for VSplitter {
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl OnMouseEvent for VSplitter {
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl OnResize for VSplitter {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {}
}
impl OnFocus for VSplitter {
    fn on_focus(&mut self) {}

    fn on_lose_focus(&mut self) {}
} 