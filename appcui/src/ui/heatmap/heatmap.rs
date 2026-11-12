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
    min_value: f64,
    max_value: f64,
}

#[allow(private_bounds)]
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
            min_value: 0.0,
            max_value: 0.0,
        };
        me.set_size_bounds(1, 1, u16::MAX, u16::MAX);
        me
    }
    /// Returns the cell at `row` and `column`, or `None` if the position is empty or outside the heat map.
    pub fn cell(&self, row: u32, column: u32) -> Option<&Cell<T>> {
        self.index(row, column).and_then(|idx| self.cells[idx].as_ref())
    }

    /// Sets the cell at `row` and `column`. A number can be passed directly
    /// (for example `heatmap.set(2, 3, 17)`).
    ///
    /// A position outside the heat map is ignored. If the new value does not change the
    /// minimum or the maximum, only this cell is repainted; otherwise every cell is.
    pub fn set(&mut self, row: u32, column: u32, cell: impl Into<Cell<T>>) {
        self.replace(row, column, Some(cell.into()));
    }

    /// Empties the cell at `row` and `column`. A position outside the heat map is ignored.
    pub fn clear(&mut self, row: u32, column: u32) {
        self.replace(row, column, None);
    }

    /// Empties every cell.
    pub fn clear_all(&mut self) {
        self.cells.iter_mut().for_each(|c| *c = None);
        self.refresh_all();
    }

    /// Sets cells in row-major order, starting with `(0,0)`: the first row from left to
    /// right, then the second row, and so on.
    ///
    /// Returns the number of cells that were set. Values beyond the last cell are not read,
    /// so an endless iterator is allowed. If there are fewer values than cells, the remaining
    /// cells keep their current content (call [`clear_all`](Self::clear_all) first for an empty map).
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(4, 3), heatmap::Flags::None);
    /// let written = heatmap.set_values([1, 5, 9, 2, 7, 3, 8, 4, 6, 0, 1, 2]);
    /// assert_eq!(written, 12);
    /// ```
    pub fn set_values<C: Into<Cell<T>>>(&mut self, values: impl IntoIterator<Item = C>) -> usize {
        let count = self.cells.len();
        let mut written = 0;
        for (slot, value) in self.cells.iter_mut().zip(values.into_iter().take(count)) {
            *slot = Some(value.into());
            written += 1;
        }
        if written > 0 {
            self.refresh_all();
        }
        written
    }

    /// Sets every cell from a callback that receives `(row, column)`. Returning `None`
    /// leaves that position empty.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(10, 10), heatmap::Flags::None);
    /// heatmap.fill_with(|row, column| Some(row * column));
    /// ```
    pub fn fill_with<C: Into<Cell<T>>>(&mut self, mut f: impl FnMut(u32, u32) -> Option<C>) {
        let width = self.size.width;
        for (idx, slot) in self.cells.iter_mut().enumerate() {
            let (row, column) = ((idx / width as usize) as u32, (idx % width as usize) as u32);
            *slot = f(row, column).map(Into::into);
        }
        self.refresh_all();
    }

    /// Calls a callback for every position with `(row, column, &mut cell)`, so cells can be
    /// changed, added (`*cell = Some(..)`) or removed (`*cell = None`) in place.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(10, 10), heatmap::Flags::None);
    /// heatmap.fill_with(|row, column| Some(row + column));
    /// // empty the diagonal
    /// heatmap.update_with(|row, column, cell| {
    ///     if row == column {
    ///         *cell = None;
    ///     }
    /// });
    /// ```
    pub fn update_with(&mut self, mut f: impl FnMut(u32, u32, &mut Option<Cell<T>>)) {
        let width = self.size.width;
        for (idx, slot) in self.cells.iter_mut().enumerate() {
            f((idx / width as usize) as u32, (idx % width as usize) as u32, slot);
        }
        self.refresh_all();
    }

    #[inline(always)]
    fn index(&self, row: u32, column: u32) -> Option<usize> {
        if row < self.size.height && column < self.size.width {
            Some(row as usize * self.size.width as usize + column as usize)
        } else {
            None
        }
    }

    // Replaces one cell and repaints as little as possible.
    fn replace(&mut self, row: u32, column: u32, new: Option<Cell<T>>) {
        let Some(idx) = self.index(row, column) else {
            return;
        };
        let old_value = self.cells[idx].as_ref().map(|c| c.value.to_f64());
        let new_value = new.as_ref().map(|c| c.value.to_f64());
        self.cells[idx] = new;

        // The range can only change if the old value was an extreme (it may have been the
        // only one) or the new value falls outside the current range. NaN never counts.
        let old_was_extreme = old_value.is_some_and(|v| v == self.min_value || v == self.max_value);
        let new_outside = new_value.is_some_and(|v| v < self.min_value || v > self.max_value);
        // (an empty map has the range [0, 0], so a first value is handled by `new_outside`)
        if old_was_extreme || new_outside {
            let (min, max) = (self.min_value, self.max_value);
            self.update_min_max();
            if min != self.min_value || max != self.max_value {
                self.update_view();
                return;
            }
        }
        self.repaint_cell(row, column);
    }

    fn refresh_all(&mut self) {
        self.update_min_max();
        self.update_view();
    }

    fn update_min_max(&mut self) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for cell in self.cells.iter().flatten() {
            let value = cell.value.to_f64();
            if !value.is_nan() {
                min = min.min(value);
                max = max.max(value);
            }
        }
        if min > max {
            // no cells (or only NaN values)
            min = 0.0;
            max = 0.0;
        }
        self.min_value = min;
        self.max_value = max;
    }

    fn paint_params(&self) -> PaintParams {
        PaintParams {
            x: 0,
            y: 0,
            size: self.cell_size,
            percentage: 0,
            default_attr: self.default_attr.unwrap_or(CellAttribute::Fixed(self.theme().chart.bar[0])),
            default_fill: self.default_fill.unwrap_or(CellFill::Shades),
        }
    }

    // Repaints one cell on the cached surface, if it is visible.
    fn repaint_cell(&mut self, row: u32, column: u32) {
        let x = column as i64 * self.cell_size.width as i64 - self.left_scroll as i64;
        let y = row as i64 * self.cell_size.height as i64 - self.top_scroll as i64;
        let view = self.surface.size();
        if x >= view.width as i64 || y >= view.height as i64 || x + self.cell_size.width as i64 <= 0 || y + self.cell_size.height as i64 <= 0 {
            return;
        }
        let (x, y) = (x as i32, y as i32);
        let background = Character::with_attributes(' ', self.theme().chart.background);
        for line in 0..self.cell_size.height as i32 {
            self.surface.fill_horizontal_line_with_size(x, y + line, self.cell_size.width, background);
        }
        let mut params = self.paint_params();
        params.x = x;
        params.y = y;
        self.paint_cell(row, column, &mut params);
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
        // a column past the last one would otherwise wrap to the next row
        let Some(idx) = self.index(row, column) else {
            return;
        };
        let cell = &self.cells[idx];
        if cell.is_none() {
            return;
        }
        let cell = cell.as_ref().unwrap();
        let value = cell.value.to_f64();
        let percentage = (((value - self.min_value) / (self.max_value - self.min_value)) * 100.0).round() as u8;
        params.percentage = percentage;
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
        // loop over the visible area (the control), not over the grid size
        let view = self.surface.size();
        let height = view.height as i32;
        let width = view.width as i32;
        let mut row = start_row;
        let mut params = self.paint_params();
        params.y = start_y;
        while params.y < height && row < self.size.height {
            let mut column = start_column;
            params.x = start_x;
            while params.x < width && column < self.size.width {
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
