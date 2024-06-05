use super::Flags;
use super::SplitterPanel;
use crate::prelude::*;
use crate::ui::layout::Dimension;

#[CustomControl(overwrite=OnPaint + OnKeyPressed + OnMouseEvent + OnResize + OnFocus, internal = true)]
pub struct VSplitter {
    left: Handle<SplitterPanel>,
    right: Handle<SplitterPanel>,
    pos: Dimension,
    flags: Flags,
}
impl VSplitter {
    pub fn new<T>(pos: T, layout: Layout, flags: Flags) -> Self
    where
        Dimension: From<T>,
    {
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            left: Handle::None,
            right: Handle::None,
            pos: pos.into(),
            flags,
        };
        obj.set_size_bounds(3, 1, u16::MAX, u16::MAX);
        obj.left = obj.add_child(SplitterPanel::new());
        obj.right = obj.add_child(SplitterPanel::new());
        obj
    }
    fn update_panel_sizes(&mut self, new_size: Size) {
        let w = self.pos.absolute_size(new_size.width as u16);
        let h = new_size.height as u16;
        let h1 = self.left;
        let h2 = self.right;
        let rm = RuntimeManager::get();
        if let Some(p1) = rm.get_control_mut(h1) {
            p1.set_position(0, 0);
            if w > 2 {
                p1.set_size(w - 1, h);
                p1.set_visible(true);
            } else {
                p1.set_size(0, h);
                p1.set_visible(false);
            }
        }
        if let Some(p2) = rm.get_control_mut(h2) {
            p2.set_position(w as i32 + 1, 0);
            if (w as i32 + 1) < new_size.width as i32 {
                p2.set_size(new_size.width as u16 - w - 1, h);
                p2.set_visible(true);
            } else {
                p2.set_size(0, h);
                p2.set_visible(false);
            }
        }
    }
}
impl OnPaint for VSplitter {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let (col_line, col_b1, col_b2) = if !self.is_enabled() {
            (theme.lines.inactive, theme.symbol.inactive, theme.symbol.inactive)
        } else {
            (theme.lines.normal, theme.symbol.arrows, theme.symbol.arrows)
        };
        let sz = self.size();
        let x = self.pos.absolute_size(sz.width as u16) as i32;
        surface.draw_vertical_line_with_size(x, 0, sz.height, LineType::Single, col_line);
    }
}
impl OnKeyPressed for VSplitter {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Ctrl+Alt+Left") => {
                let sz = self.size();
                self.pos.decrement(sz.width as u16, true);
                self.update_panel_sizes(sz);
                EventProcessStatus::Processed
            }
            key!("Ctrl+Alt+Right") => {
                let sz = self.size();
                self.pos.increment(sz.width as u16, true);
                self.update_panel_sizes(sz);
                EventProcessStatus::Processed
            }

            _ => EventProcessStatus::Ignored,
        }
    }
}
impl OnMouseEvent for VSplitter {
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl OnResize for VSplitter {
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        // recompute the position of the splitter

        // set the size of panels
        self.update_panel_sizes(new_size);
    }
}
impl OnFocus for VSplitter {
    fn on_focus(&mut self) {}

    fn on_lose_focus(&mut self) {}
}
