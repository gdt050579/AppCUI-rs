use super::{Cell, CellAttribute, CellFill, Flags, PaintParams};
use crate::prelude::*;

#[CustomControl(overwrite = OnPaint+OnResize+OnMouseEvent+OnKeyPressed, internal = true)]
/// A heat map control.
pub struct HeatMap<T> where T: Number {
    flags: Flags,
    cells: Vec<Option<Cell<T>>>,
    size: Size,
    cell_size: Size,
    surface: Surface,
    scrollbars: ScrollBars,
    left_scroll: u32,
    top_scroll: u32,
    default_attr: Option<CellAttribute>,
    default_fill: Option<CellFill>,
}

impl<T: Number> HeatMap<T> {
    /// Creates a heat map with the given layout and flags.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let heatmap = HeatMap::new(layout!("d:f"), Size::new(10, 10), heatmap::Flags::None);
    /// ```
    pub fn new(layout: Layout, size: Size, flags: Flags) -> Self {
        let count = size.width as usize * size.height as usize;
        let mut v = Vec::with_capacity(count.max(16));
        for _ in 0..count {
            v.push(None);
        }
        let has_scrollbars = flags.contains(Flags::ScrollBars);
        let extra = if has_scrollbars {
            StatusFlags::IncreaseBottomMarginOnFocus | StatusFlags::IncreaseRightMarginOnFocus
        } else {
            StatusFlags::None
        };
        let mut me = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | extra),
            flags,
            cells: v,
            cell_size: Size::new(2, 1),
            size,
            surface: Surface::new(1, 1),
            scrollbars: ScrollBars::new(has_scrollbars),
            left_scroll: 0,
            top_scroll: 0,
            default_attr: None,
            default_fill: None,
        };
        me.set_size_bounds(1, 1, u16::MAX, u16::MAX);
        me
    }
    pub fn cell(&self, row: u32, column: u32) -> Option<&Cell<T>> {
        self.cells
            .get(row as usize * self.size.width as usize + column as usize)
            .and_then(|cell| cell.as_ref())
    }
    #[inline(always)]
    fn content_width(&self) -> u64 {
        self.size.width as u64 * self.cell_size.width as u64
    }
    #[inline(always)]
    fn content_height(&self) -> u64 {
        self.size.height as u64 * self.cell_size.height as u64
    }
    fn paint_cell(&mut self, row: u32, column: u32, params: &mut PaintParams) {
        let idx = row as usize * self.size.width as usize + column as usize;
        if idx >= self.cells.len() {
            return;
        }
        let cell = &self.cells[idx];
        if cell.is_none() {
            return;
        }
        let cell = cell.as_ref().unwrap();
        cell.paint(&mut self.surface, params);
    }
    fn scroll_to(&mut self, left: u32, top: u32) {
        let max_left = self.size.width.saturating_sub(1).saturating_mul(self.cell_size.width);
        let max_top = self.size.height.saturating_sub(1).saturating_mul(self.cell_size.height);
        self.left_scroll = left.min(max_left);
        self.top_scroll = top.min(max_top);
        self.update_view();
        if self.flags.contains(Flags::ScrollBars) {
            self.scrollbars.set_indexes(self.left_scroll as u64, self.top_scroll as u64);
        }
    }
    fn scroll_by_cells(&mut self, columns: i32, rows: i32) {
        let left = if columns < 0 {
            self.left_scroll
                .saturating_sub(self.cell_size.width.saturating_mul(columns.unsigned_abs()))
        } else {
            self.left_scroll.saturating_add(self.cell_size.width.saturating_mul(columns as u32))
        };
        let top = if rows < 0 {
            self.top_scroll.saturating_sub(self.cell_size.height.saturating_mul(rows.unsigned_abs()))
        } else {
            self.top_scroll.saturating_add(self.cell_size.height.saturating_mul(rows as u32))
        };
        self.scroll_to(left, top);
    }
    fn update_view(&mut self) {
        self.surface.clear(Character::with_attributes(' ', self.theme().chart.background));
        if self.flags.contains(Flags::ScrollBars) {
            self.scrollbars.update(self.content_width(), self.content_height(), self.size());
        }
        let start_column = self.left_scroll / self.cell_size.width;
        let start_row = self.top_scroll / self.cell_size.height;
        let start_x = (start_column * self.cell_size.width) as i32 - self.left_scroll as i32;
        let start_y = (start_row * self.cell_size.height) as i32 - self.top_scroll as i32;
        let height = self.size.height as i32;
        let width = self.size.width as i32;
        let mut row = start_row;
        let mut params = PaintParams {
            x: start_x,
            y: start_y,
            size: self.cell_size,
            percentage: 0,
            default_attr: self.default_attr.unwrap_or(CellAttribute::Fixed(self.theme().chart.bar[0])),
            default_fill: self.default_fill.unwrap_or(CellFill::Shades),
        };
        while params.y < height {
            let mut column = start_column;
            while params.x < width {
                self.paint_cell(row, column, &mut params);
                params.x += self.cell_size.width as i32;
                column += 1;
            }
            params.y += self.cell_size.height as i32;
            row += 1;
        }
    }
}

impl<T: Number> OnPaint for HeatMap<T> {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if self.has_focus() && self.flags.contains(Flags::ScrollBars) {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        surface.draw_surface(0, 0, &self.surface);
    }
}

impl<T: Number> OnResize for HeatMap<T> {
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.surface.resize(new_size);
        if self.flags.contains(Flags::ScrollBars) {
            self.scrollbars.resize(self.content_width(), self.content_height(), &self.base);
        }
        self.update_view();
    }
}

impl<T: Number> OnKeyPressed for HeatMap<T> {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Ctrl+Left") => self.scroll_by_cells(-1, 0),
            key!("Ctrl+Right") => self.scroll_by_cells(1, 0),
            key!("Ctrl+Up") => self.scroll_by_cells(0, -1),
            key!("Ctrl+Down") => self.scroll_by_cells(0, 1),
            key!("Ctrl+Home") => self.scroll_to(0, 0),
            key!("Ctrl+End") => self.scroll_to(u32::MAX, u32::MAX),
            _ => return EventProcessStatus::Ignored,
        }
        EventProcessStatus::Processed
    }
}

impl<T: Number> OnMouseEvent for HeatMap<T> {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if !self.flags.contains(Flags::ScrollBars) {
            return EventProcessStatus::Ignored;
        }
        if self.scrollbars.process_mouse_event(event) {
            self.left_scroll = self.scrollbars.horizontal_index() as u32;
            self.top_scroll = self.scrollbars.vertical_index() as u32;
            self.update_view();
            EventProcessStatus::Processed
        } else if self.scrollbars.should_repaint() {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}
