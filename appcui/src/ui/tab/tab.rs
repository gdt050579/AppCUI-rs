use crate::prelude::*;
use crate::ui::tab::Type;

#[CustomControl(overwrite=OnPaint, internal=true)]
pub struct Tab {
    tab_type: Type,
    tab_width: u8,
}

impl Tab {
    fn update_margins(&mut self) {
        match self.tab_type {
            Type::Hidden => self.base.set_margins(0, 0, 0, 0),
            Type::OnTop => self.base.set_margins(0, 1, 0, 0),
            Type::OnBottom => self.base.set_margins(0, 0, 0, 1),
            Type::OnLeft => self.base.set_margins(self.tab_width, 0, 0, 0),
            Type::List => {
                let idx = self.base.focused_child_index.index();
                let cnt = self.base.children.len();
                if idx < cnt {
                    self.base.set_margins(0, 1 + idx as u8, 0, (cnt - (idx + 1)) as u8);
                } else {
                    self.base.set_margins(0, 0, 0, 0);
                }
            }
        }
    }
    fn mouse_position_to_index(&self, x: i32, y: i32) -> Option<usize> {
        let count = self.base.children.len();
        if count == 0 {
            return None;
        }
        match self.tab_type {
            Type::Hidden => None,
            Type::OnTop => {
                if (y != 0) || (x < 1) {
                    return None;
                }
                let idx = (x as usize - 1) / ((self.tab_width as usize) + 1usize);
                if idx >= count {
                    return None;
                }
                Some(idx)
            }
            Type::OnBottom => {
                if (y != self.size().height as i32 - 1) || (x < 1) {
                    return None;
                }
                let idx = (x as usize - 1) / ((self.tab_width as usize) + 1usize);
                if idx >= count {
                    return None;
                }
                Some(idx)
            }
            Type::OnLeft => {
                if (x < 0) || (x > self.tab_width as i32) || (y < 1) {
                    return None;
                }
                let idx = y as usize - 1;
                if idx >= count {
                    return None;
                }
                Some(idx)
            }
            Type::List => {
                if y < 0 {
                    return None;
                }
                let fc = self.base.focused_child_index.index();
                // check top allignament
                if y as usize <= fc {
                    return Some(y as usize);
                }
                if fc >= count {
                    return None;
                }
                // check bottom allignament
                let bottom_index = (count - fc) as i32;
                let h = self.size().height as i32;
                if h < bottom_index {
                    return None;
                }
                if y >= (h - bottom_index) && (y < h) {
                    Some(fc + 1 + ((h - bottom_index) as usize))
                } else {
                    None
                }
            }
        }
    }
}

impl OnPaint for Tab {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}
