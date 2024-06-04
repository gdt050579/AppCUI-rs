use crate::prelude::*;
use super::SplitterPanel;

#[CustomControl(overwrite=OnPaint + OnKeyPressed + OnMouseEvent + OnResize + OnFocus, internal = true)]
pub struct Splitter {
    h_1: Handle<SplitterPanel>,
    h_2: Handle<SplitterPanel>,
}
impl Splitter {

}
impl OnPaint for Splitter {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}
impl OnKeyPressed for Splitter {
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl OnMouseEvent for Splitter {
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl OnResize for Splitter {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {}
}
impl OnFocus for Splitter {
    fn on_focus(&mut self) {}

    fn on_lose_focus(&mut self) {}
} 