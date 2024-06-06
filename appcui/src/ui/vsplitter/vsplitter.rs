use self::layout::Dimension;

use super::Flags;
use super::SplitterPanel;
use crate::prelude::*;
use crate::ui::layout::Coordonate;

#[CustomControl(overwrite=OnPaint + OnKeyPressed + OnMouseEvent + OnResize + OnFocus, internal = true)]
pub struct VSplitter {
    left: Handle<SplitterPanel>,
    right: Handle<SplitterPanel>,
    min_left: Dimension,
    min_right: Dimension,
    pos: Coordonate,
    flags: Flags,
}
impl VSplitter {
    pub fn new<T>(pos: T, layout: Layout, flags: Flags) -> Self
    where
        Coordonate: From<T>,
    {
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            left: Handle::None,
            right: Handle::None,
            pos: pos.into(),
            min_left: Dimension::Percentage(0),
            min_right: Dimension::Percentage(0),
            flags,
        };
        obj.set_size_bounds(3, 1, u16::MAX, u16::MAX);
        obj.left = obj.add_child(SplitterPanel::new());
        obj.right = obj.add_child(SplitterPanel::new());
        obj
    }
    pub fn set_left_min_size<T>(&mut self, min_size: T)
    where
        Dimension: From<T>,
    {
        self.min_left = min_size.into();
    }
    pub fn set_right_min_size<T>(&mut self, min_size: T)
    where
        Dimension: From<T>,
    {
        self.min_right = min_size.into();
    }
    pub fn set_position<T>(&mut self, pos: T)
    where
        Coordonate: From<T>,
    {
        self.set_position_with_coordonate(pos.into());
    }
    fn set_position_with_coordonate(&mut self, pos: Coordonate) {
        let w = self.size().width as u16;
        self.pos = pos;
        let mut abs_value = self.pos.absolute(w);
        let min_left_margin = self.min_left.absolute(w);
        let min_right_margin = self.min_right.absolute(w);
        if abs_value >= (w as i32 - min_right_margin as i32) {
            abs_value = w as i32 - min_right_margin as i32 - 1;
        }
        abs_value = abs_value.max(min_left_margin as i32);
        self.pos.update_with_absolute_value(abs_value as i16, w);
        self.update_panel_sizes(self.size());
    }
    fn update_panel_sizes(&mut self, new_size: Size) {
        let w = self.pos.absolute(new_size.width as u16).max(0) as u16;
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
        let x = self.pos.absolute(sz.width as u16) as i32;
        surface.draw_vertical_line_with_size(x, 0, sz.height, LineType::Single, col_line);
    }
}
impl OnKeyPressed for VSplitter {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Ctrl+Alt+Left") => {
                let sz = self.size();
                if sz.width > 0 {
                    self.set_position_with_coordonate(Coordonate::Absolute((self.pos.absolute(sz.width as u16) - 1) as i16));
                }
                EventProcessStatus::Processed
            }
            key!("Ctrl+Alt+Right") => {
                let sz = self.size();
                if sz.width > 0 {
                    self.set_position_with_coordonate(Coordonate::Absolute((self.pos.absolute(sz.width as u16) + 1) as i16));
                }
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
