use super::column::*;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;
use crate::ui::ControlBase;
use AppCUIProcMacro::*;

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
    UpdateScroll,
    Processed,
    Sort((u16, bool)),
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
    left_scroll: u32,
    freez_columns: u16,
    control_size: Size,
    mouse_capture: bool,
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
            left_scroll: 0,
            freez_columns: 0,
            control_size: Size::new(0, 0),
            mouse_capture: false,
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
    #[inline(always)]
    pub fn frozen_columns(&self) -> u16 {
        self.freez_columns
    }
    pub fn set_frozen_columns(&mut self, count: u16) {
        if count as usize >= self.columns.len() {
            self.freez_columns = 0;
            return;
        }
        self.freez_columns = count;
        self.update_column_positions(0);
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
        surface.fill_horizontal_line(0, 0, width - 1, Character::with_attributes(' ', text));
        let freez_clip_left = if self.freez_columns == 0 {
            0
        } else {
            let c = &self.columns[self.freez_columns as usize - 1];
            c.x + c.width as i32 + 1
        };
        for (index, c) in self.columns.iter().enumerate() {
            let r = c.x + c.width as i32;
            if (r < 0) || (c.x >= width) || (c.width == 0) {
                continue;
            }
            if index >= self.freez_columns as usize {
                surface.set_relative_clip(c.x.max(freez_clip_left), 0, r.max(freez_clip_left), 0);
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
        surface.reset_clip();
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
        let frozen_column_index = if self.freez_columns == 0 {
            usize::MAX
        } else {
            self.freez_columns as usize - 1
        };
        let freez_clip_left = if self.freez_columns == 0 {
            0
        } else {
            let c = &self.columns[self.freez_columns as usize - 1];
            c.x + c.width as i32 + 1
        };
        for (index, c) in self.columns.iter().enumerate() {
            let r = c.x + c.width as i32;
            if (r < 0) || (c.x >= width) || (c.width == 0) {
                continue;
            }
            if (index > frozen_column_index) && (r < freez_clip_left) {
                continue;
            }
            if is_active {
                let line_type = if index == frozen_column_index {
                    LineType::Double
                } else {
                    LineType::Single
                };
                if index == self.selected_column_line_index as usize {
                    surface.draw_vertical_line(r, 0, height, line_type, theme.lines.pressed_or_selectd);
                } else if index == hovered_index {
                    surface.draw_vertical_line(r, 0, height, line_type, theme.lines.hovered);
                } else {
                    surface.draw_vertical_line(r, 0, height, line_type, attr);
                }
            } else {
                surface.draw_vertical_line(r, 0, height, LineType::Single, attr);
            }
        }
    }
    fn update_column_positions(&mut self, start: i32) {
        if self.columns.len() == 0 {
            self.left_scroll = 0;
            return;
        }
        if self.freez_columns == 0 {
            let mut pos = start;
            self.width = 0;
            for c in self.columns.iter_mut() {
                c.x = pos;
                pos += 1 + c.width as i32;
                self.width += 1 + c.width as u32;
            }
        } else {
            let mut pos = 0;
            let fc = self.freez_columns as usize;
            self.width = 0;
            //let mut right_most = 0;
            for (index, c) in self.columns.iter_mut().enumerate() {
                if index < fc {
                    c.x = pos;
                } else {
                    c.x = pos + start;
                }
                //right_most = (c.x + c.width as i32 + 1).max(right_most);
                pos += 1 + c.width as i32;
                self.width += 1 + c.width as u32;
            }
        }
        // if (self.left_scroll > 0) && (self.left_scroll < self.width) {
        //     let right_most = self.width - self.left_scroll;
        //     if right_most <= self.control_size.width {
        //         self.scroll_to(self.control_size.width - right_most);
        //     }
        // }
    }
    #[inline(always)]
    pub fn scroll_pos(&self) -> u32 {
        self.left_scroll
    }
    pub fn scroll_to(&mut self, pos: u32) {
        self.left_scroll = if self.control_size.width >= self.width {
            0
        } else if pos + self.control_size.width >= self.width {
            self.width - self.control_size.width
        } else {
            pos
        };
        self.update_column_positions(-(self.left_scroll as i32));
    }
    pub fn resize(&mut self, new_size: Size) {
        self.control_size = new_size;
        self.scroll_to(self.left_scroll);
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
    fn ensure_visible(&mut self, index: u16, favor_right_align: bool) {
        if (index as usize) >= self.columns.len() {
            return;
        }
        let c = &self.columns[index as usize];
        let left = c.x;
        let right = left + c.width as i32;
        // if the entire column is visible --> do nothing
        if (left >= 0) && (right < self.control_size.width as i32) {
            return;
        }
        // if the column can fit into the view, try to position the scroll acordimgly
        let position_from_right = if (c.width as u32) < self.control_size.width {
            right >= self.control_size.width as i32
        } else {
            favor_right_align
        };
        let new_view_pos = if position_from_right {
            (self.control_size.width as i32) - (c.width as i32) - 1
        } else {
            0
        };
        let dif = new_view_pos - left;
        let new_scroll = (self.left_scroll as i32 - dif).max(0) as u32;
        //println!("Ensure visible: left:{} , right:{} , dif:{} , new_scroll: {}, current_scroll: {}", left, right, dif, new_scroll,self.left_scroll);
        self.scroll_to(new_scroll);
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
                        self.ensure_visible(self.selected_column_index, false);
                        self.mouse_capture = true;
                        ColumnsHeaderAction::Sort((index, self.sort_ascendent))
                    }
                    SelectedComponent::Column(index) => {
                        self.mouse_capture = true;
                        self.selected_column_line_index = index;
                        ColumnsHeaderAction::ResizeColumn
                    }
                    _ => ColumnsHeaderAction::None,
                }
            }
            MouseEvent::Released(ev) => {
                if self.mouse_capture && self.selected_column_line_index != u16::MAX {
                    let c = &mut self.columns[self.selected_column_line_index as usize];
                    c.width = (ev.x - c.x).clamp(0, 255) as u8;
                    self.update_column_positions(self.columns[0].x);
                    self.selected_column_line_index = u16::MAX;
                    self.mouse_capture = false;
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
                if self.mouse_capture && self.selected_column_line_index != u16::MAX {
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
    pub fn process_key_pressed(&mut self, key: Key) -> ColumnsHeaderAction {
        if self.selected_column_line_index != u16::MAX {
            match key.value() {
                key!("Left") => {
                    let c = &mut self.columns[self.selected_column_line_index as usize];
                    c.width = c.width.saturating_sub(1);
                    self.update_column_positions(self.columns[0].x);
                    self.ensure_visible(self.selected_column_line_index, true);
                    ColumnsHeaderAction::ResizeColumn
                }
                key!("Right") => {
                    let c = &mut self.columns[self.selected_column_line_index as usize];
                    c.width = c.width.saturating_add(1);
                    self.update_column_positions(self.columns[0].x);
                    self.ensure_visible(self.selected_column_line_index, true);
                    ColumnsHeaderAction::ResizeColumn
                }
                key!("Ctrl+Left") => {
                    self.selected_column_line_index = self.selected_column_line_index.saturating_sub(1);
                    self.ensure_visible(self.selected_column_line_index, true);
                    ColumnsHeaderAction::UpdateScroll
                }
                key!("Ctrl+Right") => {
                    if self.columns.len() > 0 {
                        self.selected_column_line_index = (self.selected_column_line_index + 1).min((self.columns.len() - 1) as u16);
                        self.ensure_visible(self.selected_column_line_index, true);
                    }
                    ColumnsHeaderAction::UpdateScroll
                }
                key!("Escape") => {
                    self.selected_column_line_index = u16::MAX;
                    ColumnsHeaderAction::Processed
                }
                _ => {
                    self.selected_column_line_index = u16::MAX;
                    ColumnsHeaderAction::Repaint     
                }
            }
        } else {
            match key.value() {
                key!("Left") => {
                    self.scroll_to(self.left_scroll.saturating_sub(1));
                    ColumnsHeaderAction::UpdateScroll
                }
                key!("Right") => {
                    self.scroll_to(self.left_scroll.saturating_add(1));
                    ColumnsHeaderAction::UpdateScroll
                }
                _ => ColumnsHeaderAction::None,
            }
        }
    }
    pub fn enter_resize_mode(&mut self) {
        self.selected_column_line_index = 0;
        self.ensure_visible(0, true);
    }
    #[inline(always)]
    pub fn columns(&self) -> &Vec<Column> {
        &self.columns
    }
}
