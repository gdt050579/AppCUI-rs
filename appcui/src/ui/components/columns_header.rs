use super::Column;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;
use crate::ui::ControlBase;

#[derive(Copy, Clone, Default, PartialEq)]
enum SelectedComponent {
    #[default]
    None,
    Header(usize),
    Column(usize),
}
pub struct ColumnsHeader {
    columns: Vec<Column>,
    hovered: SelectedComponent,
    selected: SelectedComponent,
    repaint: bool,
}
impl ColumnsHeader {
    pub fn with_capacity(capacity: usize) -> ColumnsHeader {
        ColumnsHeader {
            columns: Vec::with_capacity(capacity),
            hovered: SelectedComponent::None,
            selected: SelectedComponent::None,
            repaint: false,
        }
    }
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
        let is_active = control.is_active();
        let (text, hotkey, symbol) = match () {
            _ if !is_active => (theme.header.text.inactive, theme.header.hotkey.inactive, theme.header.symbol.inactive),
            _ if control.has_focus() => (theme.header.text.focused, theme.header.hotkey.focused, theme.header.symbol.focused),
            _ => (theme.header.text.normal, theme.header.hotkey.normal, theme.header.symbol.normal),
        };
        // first draw an empty header
        let width = control.size().width as i32;
        let hovered_index = match self.hovered {
            SelectedComponent::Header(index) => index,
            _ => usize::MAX,
        };
        surface.fill_horizontal_line(0, 0, width, Character::with_attributes(' ', text));
        for (index,c) in self.columns.iter().enumerate() {
            let r = c.x + c.width as i32;
            if (r < 0) || (c.x >= width) || (c.width == 0) {
                continue;
            }
            if is_active {
                if index == hovered_index {
                    c.paint(surface, theme.header.text.hovered, theme.header.hotkey.hovered, true);
                } else {
                    c.paint(surface, text, hotkey, false);
                }
            } else {
                c.paint(surface, text, hotkey, false);
            }
        }
    }
    pub fn paint_columns(&self, surface: &mut Surface, theme: &Theme, control: &ControlBase) {
        let attr = match () {
            _ if !control.is_active() => theme.lines.inactive,
            _ if control.has_focus() => theme.lines.focused,
            _ => theme.lines.normal,
        };
        let sz = control.size();
        let width = sz.width as i32;
        let height = sz.height as i32;
        for c in &self.columns {
            let r = c.x + c.width as i32;
            if (r < 0) || (c.x >= width) || (c.width == 0) {
                continue;
            }
            surface.draw_vertical_line(r, 0, height, LineType::Single, attr);
        }
    }
    fn mouse_to_state(&self, x: i32, y: i32) -> SelectedComponent {
        if y == 0 {
            // headers and columns
            for (index, c) in self.columns.iter().enumerate() {
                let r = c.x + c.width as i32;
                if x == r {
                    return SelectedComponent::Column(index);
                }
                if x >= c.x && x < r {
                    return SelectedComponent::Header(index);
                }
            }
        } else {
            // only column is possible

            for (index, c) in self.columns.iter().enumerate() {
                let r = c.x + c.width as i32;
                if x == r {
                    return SelectedComponent::Column(index);
                }
            }
        }
        SelectedComponent::None
    }
    pub fn process_mouse_event(&mut self, event: &MouseEvent) -> bool {
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                self.repaint = self.hovered != SelectedComponent::None;
                self.hovered = SelectedComponent::None;
                return false;
            }
            MouseEvent::Over(p) => {
                let status = self.mouse_to_state(p.x, p.y);
                self.repaint = status != self.hovered;
                self.hovered = status;
                return status != SelectedComponent::None;
            }
            MouseEvent::Pressed(_) => todo!(),
            MouseEvent::Released(_) => todo!(),
            MouseEvent::DoubleClick(_) => todo!(),
            MouseEvent::Drag(_) => todo!(),
            MouseEvent::Wheel(_) => todo!(),
        }
    }
}
