use std::sync::atomic::AtomicPtr;

use super::Column;
use crate::graphics::*;
use crate::system::*;
use crate::ui::ControlBase;

pub struct ColumnsHeader {
    columns: Vec<Column>,
}
impl ColumnsHeader {
    pub fn add(&mut self, column: Column) {
        self.columns.push(column);
        if self.columns.len() == 1 {
            self.columns[0].x = 0;
        } else {
            let last = self.columns.len() - 1;
            self.columns[last].x = self.columns[last - 1].x + 1 + self.columns[last - 1].width as i32;
        }
    }
    pub fn paint(&self, surface: &mut Surface, theme: &Theme, control: &ControlBase) {
        let (text,hotkey,symbol) = match () {
            _ if !control.is_active() => (theme.header.text.inactive,theme.header.hotkey.inactive,theme.header.symbol.inactive),
            _ if control.has_focus() => (theme.header.text.focused,theme.header.hotkey.focused,theme.header.symbol.focused),
            _ => (theme.header.text.normal,theme.header.hotkey.normal,theme.header.symbol.normal)
        };
        // first draw an empty header
        let width = control.size().width as i32;
        surface.fill_horizontal_line(0, 0, width, Character::with_attributes(' ', text));
        for c in &self.columns {
            let r = c.x + c.width as i32;
            if (r<0) || (c.x >= width) || (c.width==0) {
                continue;
            }
            c.paint(surface, text, hotkey);            
        }
    }
}
