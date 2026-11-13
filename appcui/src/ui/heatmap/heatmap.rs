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
        let mut params = self.paint_params();
        self.paint_cell(row, column, &mut params);
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

    #[inline(always)]
    fn margin(&self) -> u32 {
        u32::from(self.flags.contains(Flags::Margins))
    }
    #[inline(always)]
    fn content_width(&self) -> u64 {
        self.size.width as u64 * self.cell_size.width as u64 + self.margin() as u64 * 2
    }
    #[inline(always)]
    fn content_height(&self) -> u64 {
        self.size.height as u64 * self.cell_size.height as u64 + self.margin() as u64 * 2
    }
    fn paint_cell(&mut self, row: u32, column: u32, params: &mut PaintParams) {
        // a column past the last one would otherwise wrap to the next row
        let Some(idx) = self.index(row, column) else {
            return;
        };
        let margin = self.margin() as i64;
        let x = column as i64 * self.cell_size.width as i64 - self.left_scroll as i64 + margin;
        let y = row as i64 * self.cell_size.height as i64 - self.top_scroll as i64 + margin;
        let view = self.surface.size();
        if x >= view.width as i64 || y >= view.height as i64 || x + self.cell_size.width as i64 <= 0 || y + self.cell_size.height as i64 <= 0 {
            return;
        }
        params.x = x as i32;
        params.y = y as i32;
        let background = Character::with_attributes(' ', self.theme().chart.background);
        for line in 0..self.cell_size.height as i32 {
            self.surface.fill_horizontal_line_with_size(params.x, params.y + line, self.cell_size.width, background);
        }
        if let Some(cell) = self.cells[idx].as_ref() {
            let value = cell.value.to_f64();
            params.percentage = (((value - self.min_value) / (self.max_value - self.min_value)) * 100.0).round() as u8;
            cell.paint(&mut self.surface, params);
        }
    }
    // Furthest offset that does not reveal empty space past the image.
    fn clamp_scroll(offset: u32, content: u64, view: u32) -> u32 {
        let max = if view == 0 || content <= view as u64 {
            0
        } else {
            (content - view as u64).min(u32::MAX as u64) as u32
        };
        offset.min(max)
    }
    fn scroll_to(&mut self, left: u32, top: u32) {
        let view = self.size();
        self.left_scroll = Self::clamp_scroll(left, self.content_width(), view.width);
        self.top_scroll = Self::clamp_scroll(top, self.content_height(), view.height);
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
        // only the rows and columns that are (at least partly) visible
        let view = self.surface.size();
        let (cw, ch) = (self.cell_size.width as u64, self.cell_size.height as u64);
        if cw == 0 || ch == 0 {
            return;
        }
        let margin = self.margin() as u64;
        let first_column = ((self.left_scroll as u64).saturating_sub(margin) / cw) as u32;
        let first_row = ((self.top_scroll as u64).saturating_sub(margin) / ch) as u32;
        let end_column = (self.left_scroll as u64 + view.width as u64).saturating_sub(margin).div_ceil(cw).min(self.size.width as u64) as u32;
        let end_row = (self.top_scroll as u64 + view.height as u64).saturating_sub(margin).div_ceil(ch).min(self.size.height as u64) as u32;
        let mut params = self.paint_params();
        for row in first_row..end_row {
            for column in first_column..end_column {
                self.paint_cell(row, column, &mut params);
            }
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
        // the view changed, so a previous offset may now sit past the image
        self.scroll_to(self.left_scroll, self.top_scroll);
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
        if self.scrollbars.process_mouse_event(event) {
            self.scroll_to(self.scrollbars.horizontal_index() as u32, self.scrollbars.vertical_index() as u32);
            return EventProcessStatus::Processed;
        }
        match event {
            MouseEvent::Wheel(direction) => {
                match direction {
                    MouseWheelDirection::Left => self.scroll_by_cells(-1, 0),
                    MouseWheelDirection::Right => self.scroll_by_cells(1, 0),
                    MouseWheelDirection::Up => self.scroll_by_cells(0, -1),
                    MouseWheelDirection::Down => self.scroll_by_cells(0, 1),
                }
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        if self.scrollbars.should_repaint() {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}
