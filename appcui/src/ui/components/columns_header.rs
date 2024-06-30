use super::Column;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;
use crate::ui::ControlBase;

#[derive(Copy, Clone, Default, PartialEq)]
enum SelectedComponent {
    #[default]
    None,
    Header(u16),
    Column(u16),
}
#[derive(Copy, Clone, PartialEq)]
pub enum ColumnsHeaderAction {
    None,
    Repaint,
    ResizeColumn,
    Sort((u16,bool)),
    AutoResize(u16),
}
impl ColumnsHeaderAction {
    #[inline(always)]
    pub fn should_repaint(&self) -> bool {
        match self {
            ColumnsHeaderAction::None => false,
            _ => true,
        }
    }
    #[inline(always)]
    pub fn is_processed(&self) -> bool {
        match self {
            ColumnsHeaderAction::None => false,
            ColumnsHeaderAction::Repaint => false,
            _ => true,
        }
    }
}
pub struct ColumnsHeader {
    columns: Vec<Column>,
    hovered: SelectedComponent,
    selected_column_index: u16,
    sort_ascendent: bool,
    selected_column_line_index: u16,
    width: u32,
}
impl ColumnsHeader {
    pub fn with_capacity(capacity: usize) -> ColumnsHeader {
        ColumnsHeader {
            columns: Vec::with_capacity(capacity),
            hovered: SelectedComponent::None,
            selected_column_index: u16::MAX,
            selected_column_line_index: u16::MAX,
            sort_ascendent: true,
            width: 0,
        }
    }
    pub fn add(&mut self, column: Column) {
        self.columns.push(column);
        if self.columns.len() == 1 {
            self.columns[0].x = 0;
            self.width = (self.columns[0].width as u32) + 1;
        } else {
            let last = self.columns.len() - 1;
            self.columns[last].x = self.columns[last - 1].x + 1 + self.columns[last - 1].width as i32;
            self.width += (self.columns[last - 1].width as u32) + 1;
        }
    }
    #[inline(always)]
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn paint(&self, surface: &mut Surface, theme: &Theme, control: &ControlBase) {
        let is_active = control.is_active();
        let (text, hotkey) = match () {
            _ if !is_active => (theme.header.text.inactive, theme.header.hotkey.inactive),
            _ if control.has_focus() => (theme.header.text.focused, theme.header.hotkey.focused),
            _ => (theme.header.text.normal, theme.header.hotkey.normal),
        };
        // first draw an empty header
        let width = control.size().width as i32;
        let hovered_index = match self.hovered {
            SelectedComponent::Header(index) => index as usize,
            _ => usize::MAX,
        };
        surface.fill_horizontal_line(0, 0, width, Character::with_attributes(' ', text));
        for (index, c) in self.columns.iter().enumerate() {
            let r = c.x + c.width as i32;
            if (r < 0) || (c.x >= width) || (c.width == 0) {
                continue;
            }
            if is_active {
                if index == hovered_index {
                    c.paint(surface, theme.header.text.hovered, theme.header.hotkey.hovered, true);
                    if index == self.selected_column_index as usize {
                        surface.write_char(
                            r - 1,
                            0,
                            Character::with_attributes(
                                if self.sort_ascendent {
                                    SpecialChar::ArrowUp
                                } else {
                                    SpecialChar::ArrowDown
                                },
                                theme.header.hotkey.hovered,
                            ),
                        );
                    }
                } else if index == self.selected_column_index as usize {
                    c.paint(
                        surface,
                        theme.header.text.pressed_or_selectd,
                        theme.header.hotkey.pressed_or_selectd,
                        true,
                    );
                    surface.write_char(
                        r - 1,
                        0,
                        Character::with_attributes(
                            if self.sort_ascendent {
                                SpecialChar::ArrowUp
                            } else {
                                SpecialChar::ArrowDown
                            },
                            theme.header.hotkey.pressed_or_selectd,
                        ),
                    );
                } else {
                    c.paint(surface, text, hotkey, false);
                }
            } else {
                c.paint(surface, text, hotkey, false);
            }
        }
    }
    pub fn paint_columns(&self, surface: &mut Surface, theme: &Theme, control: &ControlBase) {
        let is_active = control.is_active();
        let attr = match () {
            _ if !is_active => theme.lines.inactive,
            _ if control.has_focus() => theme.lines.focused,
            _ => theme.lines.normal,
        };
        let sz = control.size();
        let width = sz.width as i32;
        let height = sz.height.saturating_sub(1) as i32;
        let hovered_index = match self.hovered {
            SelectedComponent::Column(index) => index as usize,
            _ => usize::MAX,
        };
        for (index, c) in self.columns.iter().enumerate() {
            let r = c.x + c.width as i32;
            if (r < 0) || (c.x >= width) || (c.width == 0) {
                continue;
            }
            if is_active {
                if index == self.selected_column_line_index as usize {
                    surface.draw_vertical_line(r, 0, height, LineType::Single, theme.lines.pressed_or_selectd);
                } else if index == hovered_index {
                    surface.draw_vertical_line(r, 0, height, LineType::Single, theme.lines.hovered);
                } else {
                    surface.draw_vertical_line(r, 0, height, LineType::Single, attr);
                }
            } else {
                surface.draw_vertical_line(r, 0, height, LineType::Single, attr);
            }
        }
    }
    fn update_column_positions(&mut self, start: i32) {
        if self.columns.len() == 0 {
            return;
        }
        let mut pos = start;
        self.width = 0;
        for c in self.columns.iter_mut() {
            c.x = pos;
            pos += 1 + c.width as i32;
            self.width += (c.width as u32) + 1;
        }
    }
    fn mouse_to_state(&self, x: i32, y: i32) -> SelectedComponent {
        if y == 0 {
            // headers and columns
            for (index, c) in self.columns.iter().enumerate() {
                let r = c.x + c.width as i32;
                if x == r {
                    return SelectedComponent::Column(index as u16);
                }
                if x >= c.x && x < r {
                    return SelectedComponent::Header(index as u16);
                }
            }
        } else {
            // only column is possible

            for (index, c) in self.columns.iter().enumerate() {
                let r = c.x + c.width as i32;
                if x == r {
                    return SelectedComponent::Column(index as u16);
                }
            }
        }
        SelectedComponent::None
    }
    pub fn process_mouse_event(&mut self, event: &MouseEvent) -> ColumnsHeaderAction {
        match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                if self.hovered != SelectedComponent::None {
                    self.hovered = SelectedComponent::None;
                    ColumnsHeaderAction::Repaint
                } else {
                    ColumnsHeaderAction::None
                }
            }
            MouseEvent::Over(p) => {
                let status = self.mouse_to_state(p.x, p.y);
                if status != self.hovered {
                    self.hovered = status;
                    ColumnsHeaderAction::Repaint
                } else {
                    ColumnsHeaderAction::None
                }
            }
            MouseEvent::Pressed(ev) => {
                let status = self.mouse_to_state(ev.x, ev.y);
                match status {
                    SelectedComponent::Header(index) => {
                        if self.selected_column_index == index {
                            self.sort_ascendent = !self.sort_ascendent;
                        } else {
                            self.selected_column_index = index;
                            self.sort_ascendent = true;
                        }
                        ColumnsHeaderAction::Sort((index,self.sort_ascendent))
                    }
                    SelectedComponent::Column(index) => {
                        self.selected_column_line_index = index;
                        ColumnsHeaderAction::ResizeColumn
                    }
                    _ => ColumnsHeaderAction::None,
                }
            }
            MouseEvent::Released(ev) => {
                if self.selected_column_line_index != u16::MAX {
                    let c = &mut self.columns[self.selected_column_line_index as usize];
                    c.width = (ev.x - c.x).clamp(0, 255) as u8;
                    self.update_column_positions(self.columns[0].x);
                    self.selected_column_line_index = u16::MAX;
                    ColumnsHeaderAction::ResizeColumn
                } else {
                    ColumnsHeaderAction::None
                }
            }
            MouseEvent::DoubleClick(ev) => {
                let status = self.mouse_to_state(ev.x, ev.y);
                if let SelectedComponent::Column(index) = status {
                    ColumnsHeaderAction::AutoResize(index)
                } else {
                    ColumnsHeaderAction::None
                }
            }
            MouseEvent::Drag(ev) => {
                if self.selected_column_line_index != u16::MAX {
                    let c = &mut self.columns[self.selected_column_line_index as usize];
                    c.width = (ev.x - c.x).clamp(0, 255) as u8;
                    self.update_column_positions(self.columns[0].x);
                    ColumnsHeaderAction::ResizeColumn
                } else {
                    ColumnsHeaderAction::None
                }
            }
            MouseEvent::Wheel(_) => ColumnsHeaderAction::None,
        }
    }
}
