use super::events::{EventData, EventType};
use super::{Cell, CellAttribute, CellFill, Flags, PaintParams, TooltipFormat};
use crate::prelude::*;
use crate::utils::TempString;
use std::any::TypeId;

#[CustomControl(overwrite = OnPaint+OnResize+OnMouseEvent+OnKeyPressed, internal = true)]
/// A heat map control.
///
/// Hovering a cell shows a tooltip. See [`TooltipFormat`](super::TooltipFormat).
/// Clicking a cell selects it. Clicking an empty cell, or the margin around the grid,
/// clears the selection. The selection stays visible when the heat map loses the focus:
/// the cell is outlined and, with [`Flags::DimCellsOnSelection`](super::Flags::DimCellsOnSelection),
/// the others are dimmed. With [`Flags::Margins`](super::Flags::Margins) the outline is a rectangle
/// around the cell. Without margins, a one-row cell is marked with brackets, a two-row cell with
/// corners, and a taller cell with a rectangle. A disabled heat map is drawn with the inactive
/// color and shows no selection.
///
/// A cell is 2 characters wide and 1 tall unless [`set_cell_size`](Self::set_cell_size) changes it.
/// [`set_heatmap_size`](Self::set_heatmap_size) changes how many columns and rows the grid has.
///
/// When the heat map has the focus, the arrow keys move the selection. Empty cells are
/// skipped, and the selection wraps around. `Left` and `Right` walk the grid in row order.
/// `Up` and `Down` walk it in column order. `Escape` clears the selection.
///
/// | Key      | Action                                      |
/// | -------- | ------------------------------------------- |
/// | `Right`  | selects the next cell in the row            |
/// | `Left`   | selects the previous cell in the row        |
/// | `Down`   | selects the next cell in the column         |
/// | `Up`     | selects the previous cell in the column     |
/// | `Escape` | clears the selection                        |
///
/// `Ctrl` with an arrow key scrolls the heat map by one cell and does not change the selection.
/// Selecting a cell or clearing the selection this way raises the events of
/// [`GenericHeatMapEvents`](super::events::GenericHeatMapEvents). With the `#[Window]` macro,
/// use `events = HeatMapEvents<T>`.
pub struct HeatMap<T>
where
    T: Number + 'static,
{
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
    tooltip_format: TooltipFormat,
    hovered: Option<(u32, u32)>,
    selected: Option<(u32, u32)>,
    min_value: f64,
    max_value: f64,
}

#[allow(private_bounds)]
impl<T: Number + 'static> HeatMap<T> {
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
            tooltip_format: TooltipFormat::LabelAndValue,
            hovered: None,
            selected: None,
            min_value: 0.0,
            max_value: 0.0,
        };
        me.set_size_bounds(1, 1, u16::MAX, u16::MAX);
        me
    }

    /// The width and height, in characters, of one cell.
    ///
    /// The default is 2 by 1.
    pub fn cell_size(&self) -> Size {
        self.cell_size
    }

    /// Sets the width and height, in characters, of one cell, and repaints the heat map.
    ///
    /// A width or height of 0 is ignored. The scroll position is kept and clamped so the
    /// view does not run past the end of the map. The selection mark follows the cell
    /// height: one row uses brackets, two rows use corners, and a taller cell uses a rectangle.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(10, 10), heatmap::Flags::None);
    /// heatmap.set_cell_size(Size::new(3, 2));
    /// assert_eq!(heatmap.cell_size(), Size::new(3, 2));
    /// ```
    pub fn set_cell_size(&mut self, size: Size) {
        if size.width == 0 || size.height == 0 || size == self.cell_size {
            return;
        }
        self.cell_size = size;
        self.scroll_to(self.left_scroll, self.top_scroll);
        self.refresh_hovered_tooltip();
    }

    /// The number of columns (`width`) and rows (`height`) in the grid.
    pub fn heatmap_size(&self) -> Size {
        self.size
    }

    /// Sets the number of columns (`width`) and rows (`height`), and repaints the heat map.
    ///
    /// Cells that still fit stay at the same row and column, including their value, label
    /// and fill. New positions are empty. Cells outside the new grid are removed. A selected
    /// or hovered cell that no longer fits is cleared. This does not raise
    /// [`on_clear_selection`](super::events::GenericHeatMapEvents::on_clear_selection).
    /// A width or height of 0 is ignored. The scroll position is kept and clamped so the
    /// view does not run past the end of the map.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(10, 10), heatmap::Flags::None);
    /// heatmap.set(1, 2, 7);
    /// heatmap.set_heatmap_size(Size::new(4, 3));
    /// assert_eq!(heatmap.heatmap_size(), Size::new(4, 3));
    /// assert_eq!(heatmap.cell(1, 2).map(|cell| cell.value()), Some(7));
    /// ```
    pub fn set_heatmap_size(&mut self, size: Size) {
        if size.width == 0 || size.height == 0 || size == self.size {
            return;
        }
        let count = size.width as usize * size.height as usize;
        let mut next = Vec::with_capacity(count);
        next.resize_with(count, || None);
        let rows = self.size.height.min(size.height);
        let columns = self.size.width.min(size.width);
        for row in 0..rows {
            for column in 0..columns {
                let old_index = row as usize * self.size.width as usize + column as usize;
                let new_index = row as usize * size.width as usize + column as usize;
                if let Some(slot) = self.cells.get_mut(old_index) {
                    next[new_index] = slot.take();
                }
            }
        }
        self.cells = next;
        self.size = size;
        if self.selected.is_some_and(|(row, column)| self.index(row, column).is_none()) {
            self.selected = None;
        }
        if self.hovered.is_some_and(|(row, column)| self.index(row, column).is_none()) {
            self.clear_hover();
        }
        self.update_min_max();
        self.scroll_to(self.left_scroll, self.top_scroll);
        self.refresh_hovered_tooltip();
    }

    /// The default attribute, or `None` when cells use the theme color.
    ///
    /// `None` means the default was never set, or was removed with [`clear_default_attr`](Self::clear_default_attr).
    /// Cells are then drawn with the theme's first chart color.
    pub fn default_attr(&self) -> Option<CellAttribute> {
        self.default_attr
    }

    /// Sets the attribute used by cells that do not have their own, and repaints the heat map.
    ///
    /// A cell that has an attribute ([`Cell::set_attr`](super::Cell::set_attr)) keeps it.
    /// [`CellAttribute::Fixed`](super::CellAttribute::Fixed) uses one color.
    /// [`CellAttribute::Range`](super::CellAttribute::Range) blends two colors from the smallest
    /// value to the largest.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(10, 10), heatmap::Flags::None);
    /// heatmap.fill_with(|row, column| Some(row + column));
    /// let attr = heatmap::CellAttribute::Fixed(CharAttribute::with_color(Color::Red, Color::Black));
    /// heatmap.set_default_attr(attr);
    /// assert_eq!(heatmap.default_attr(), Some(attr));
    /// ```
    pub fn set_default_attr(&mut self, attr: CellAttribute) {
        if self.default_attr == Some(attr) {
            return;
        }
        self.default_attr = Some(attr);
        self.update_view();
    }

    /// Removes the default attribute and repaints the heat map.
    ///
    /// Cells without their own attribute are drawn with the theme's first chart color.
    /// Cells that have an attribute are left as they are.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(10, 10), heatmap::Flags::None);
    /// heatmap.set_default_attr(heatmap::CellAttribute::Fixed(CharAttribute::with_color(Color::Red, Color::Black)));
    /// heatmap.clear_default_attr();
    /// assert_eq!(heatmap.default_attr(), None);
    /// ```
    pub fn clear_default_attr(&mut self) {
        if self.default_attr.is_none() {
            return;
        }
        self.default_attr = None;
        self.update_view();
    }

    /// Returns the cell at `row` and `column`, or `None` if the position is empty or outside the heat map.
    pub fn cell(&self, row: u32, column: u32) -> Option<&Cell<T>> {
        self.index(row, column).and_then(|idx| self.cells[idx].as_ref())
    }

    /// Sets the cell at `row` and `column`. A number can be passed directly
    /// (for example `heatmap.set(2, 3, 17)`). A [`CellBuilder`](super::CellBuilder) can be passed as well.
    ///
    /// A position outside the heat map is ignored. If the new value does not change the
    /// minimum or the maximum, only this cell is repainted; otherwise every cell is.
    pub fn set(&mut self, row: u32, column: u32, cell: impl Into<Cell<T>>) {
        self.replace(row, column, Some(cell.into()));
    }

    /// Empties the cell at `row` and `column`. A position outside the heat map is ignored.
    ///
    /// If that cell was selected, the selection is cleared as well. This does not raise
    /// [`on_clear_selection`](super::events::GenericHeatMapEvents::on_clear_selection).
    pub fn clear_cell(&mut self, row: u32, column: u32) {
        self.replace(row, column, None);
        if self.selected == Some((row, column)) {
            self.selected = None;
        }
    }

    /// Empties every cell.
    ///
    /// The selection is cleared as well. This does not raise
    /// [`on_clear_selection`](super::events::GenericHeatMapEvents::on_clear_selection).
    pub fn clear_all(&mut self) {
        self.cells.iter_mut().for_each(|c| *c = None);
        self.selected = None;
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

    /// Changes the cell at `row` and `column`.
    ///
    /// The callback receives that cell, so it can change the label, the fill, or the attribute. A position
    /// outside the heat map, or an empty cell, is ignored and the callback is not called.
    /// If the value changes the minimum or the maximum, every cell is repainted; otherwise
    /// only this cell is. The hover tooltip is updated when it is showing this cell.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(10, 10), heatmap::Flags::None);
    /// heatmap.set(2, 3, 17);
    /// heatmap.update_cell(2, 3, |cell| {
    ///     cell.set_label("Peak");
    /// });
    /// ```
    pub fn update_cell(&mut self, row: u32, column: u32, f: impl FnOnce(&mut Cell<T>)) {
        let Some(idx) = self.index(row, column) else {
            return;
        };
        let Some(cell) = self.cells[idx].as_mut() else {
            return;
        };
        let old_value = cell.value.to_f64();
        f(cell);
        let new_value = self.cells[idx].as_ref().map(|cell| cell.value.to_f64());
        self.refresh_cell(row, column, Some(old_value), new_value);
    }

    /// Sets what the hover tooltip of a cell shows.
    ///
    /// The default is [`TooltipFormat::LabelAndValue`](super::TooltipFormat::LabelAndValue).
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(10, 10), heatmap::Flags::None);
    /// heatmap.set_tooltip_format(heatmap::TooltipFormat::Value);
    /// ```
    pub fn set_tooltip_format(&mut self, format: TooltipFormat) {
        if self.tooltip_format == format {
            return;
        }
        self.tooltip_format = format;
        self.update_tooltip();
    }

    /// Returns what the hover tooltip of a cell shows.
    ///
    /// The default is [`TooltipFormat::LabelAndValue`](super::TooltipFormat::LabelAndValue).
    pub fn tooltip_format(&self) -> TooltipFormat {
        self.tooltip_format
    }

    /// Sets the fill used by cells that do not have their own, and repaints the heat map.
    ///
    /// A cell that has a fill ([`Cell::set_fill`](super::Cell::set_fill)) keeps it.
    /// [`CellFill::Shades`](super::CellFill::Shades) draws each cell with a shade chosen from
    /// its value. [`CellFill::Custom`](super::CellFill::Custom) repeats one character.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(10, 10), heatmap::Flags::None);
    /// heatmap.fill_with(|row, column| Some(row + column));
    /// heatmap.set_default_fill(heatmap::CellFill::Custom('#'));
    /// ```
    pub fn set_default_fill(&mut self, fill: CellFill) {
        if self.default_fill == Some(fill) {
            return;
        }
        self.default_fill = Some(fill);
        self.update_view();
    }

    /// The default fill, or `None` when cells use the built-in shades.
    ///
    /// `None` and [`CellFill::Shades`](super::CellFill::Shades) draw the same characters.
    /// `None` means the default was never set, or was removed with [`clear_default_fill`](Self::clear_default_fill).
    pub fn default_fill(&self) -> Option<CellFill> {
        self.default_fill
    }

    /// Removes the default fill and repaints the heat map.
    ///
    /// Cells without their own fill are drawn with shades. Cells that have a fill are left as they are.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(10, 10), heatmap::Flags::None);
    /// heatmap.set_default_fill(heatmap::CellFill::Custom('#'));
    /// heatmap.clear_default_fill();
    /// ```
    pub fn clear_default_fill(&mut self) {
        if self.default_fill.is_none() {
            return;
        }
        self.default_fill = None;
        self.update_view();
    }

    /// Returns the selected cell as `(row, column)`, or `None` when nothing is selected.
    pub fn selected_cell(&self) -> Option<(u32, u32)> {
        self.selected
    }

    /// Selects the cell at `(row, column)`, or clears the selection when `cell` is `None`.
    ///
    /// A position outside the heat map is ignored. An empty cell can be selected. The cell
    /// is outlined and, with [`Flags::DimCellsOnSelection`](super::Flags::DimCellsOnSelection),
    /// the others are dimmed. The outline stays when the heat map loses the focus.
    ///
    /// This does not raise [`GenericHeatMapEvents`](super::events::GenericHeatMapEvents).
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(4, 3), heatmap::Flags::None);
    /// heatmap.set_values([1, 5, 9, 2, 7, 3, 8, 4, 6, 0, 1, 2]);
    /// heatmap.set_selected_cell(Some((1, 2)));
    /// heatmap.set_selected_cell(None);
    /// ```
    pub fn set_selected_cell(&mut self, cell: Option<(u32, u32)>) {
        let Some((row, column)) = cell else {
            self.selected = None;
            return;
        };
        if self.index(row, column).is_none() {
            return;
        }
        self.selected = Some((row, column));
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
        self.refresh_cell(row, column, old_value, new_value);
    }

    // Repaints after one cell's value may have changed. The range can only change if the old
    // value was an extreme (it may have been the only one) or the new value falls outside the
    // current range. NaN never counts. An empty map has the range [0, 0], so a first value is
    // handled by `new_outside`.
    fn refresh_cell(&mut self, row: u32, column: u32, old_value: Option<f64>, new_value: Option<f64>) {
        let old_was_extreme = old_value.is_some_and(|v| v == self.min_value || v == self.max_value);
        let new_outside = new_value.is_some_and(|v| v < self.min_value || v > self.max_value);
        if old_was_extreme || new_outside {
            let (min, max) = (self.min_value, self.max_value);
            self.update_min_max();
            if min != self.min_value || max != self.max_value {
                self.update_view();
                self.refresh_hovered_tooltip();
                return;
            }
        }
        let mut params = self.paint_params();
        self.paint_cell(row, column, &mut params);
        if self.hovered == Some((row, column)) {
            self.update_tooltip();
        }
    }

    fn refresh_all(&mut self) {
        self.update_min_max();
        self.update_view();
        self.refresh_hovered_tooltip();
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
            self.surface
                .fill_horizontal_line_with_size(params.x, params.y + line, self.cell_size.width, background);
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
        let left = Self::clamp_scroll(left, self.content_width(), view.width);
        let top = Self::clamp_scroll(top, self.content_height(), view.height);
        let moved = left != self.left_scroll || top != self.top_scroll;
        self.left_scroll = left;
        self.top_scroll = top;
        self.update_view();
        if self.flags.contains(Flags::ScrollBars) {
            self.scrollbars.set_indexes(self.left_scroll as u64, self.top_scroll as u64);
        }
        if moved {
            self.clear_hover();
        }
    }
    fn cell_at(&self, x: i32, y: i32) -> Option<(u32, u32)> {
        let (cw, ch) = (self.cell_size.width as i64, self.cell_size.height as i64);
        if cw <= 0 || ch <= 0 {
            return None;
        }
        let margin = self.margin() as i64;
        let content_x = x as i64 + self.left_scroll as i64 - margin;
        let content_y = y as i64 + self.top_scroll as i64 - margin;
        if content_x < 0 || content_y < 0 {
            return None;
        }
        let column = (content_x / cw) as u32;
        let row = (content_y / ch) as u32;
        if row >= self.size.height || column >= self.size.width {
            return None;
        }
        Some((row, column))
    }
    fn percentage(&self, value: f64) -> u8 {
        if !value.is_finite() {
            return 0;
        }
        let span = self.max_value - self.min_value;
        if span == 0.0 {
            return 100;
        }
        (((value - self.min_value) / span) * 100.0).round().clamp(0.0, 100.0) as u8
    }
    fn tooltip_text(&self, cell: &Cell<T>) -> TempString<128> {
        let mut text = TempString::<128>::new("");
        let label = cell.label();
        match self.tooltip_format {
            TooltipFormat::Value => {
                let _ = write!(text, "{}", cell.value);
            }
            TooltipFormat::Label => {
                let _ = write!(text, "{label}");
            }
            TooltipFormat::Percentage => {
                let _ = write!(text, "{}%", self.percentage(cell.value.to_f64()));
            }
            TooltipFormat::LabelAndValue => {
                if !label.is_empty() {
                    let _ = write!(text, "{label}: ");
                }
                let _ = write!(text, "{}", cell.value);
            }
            TooltipFormat::LabelAndPercentage => {
                if !label.is_empty() {
                    let _ = write!(text, "{label}: ");
                }
                let _ = write!(text, "{}%", self.percentage(cell.value.to_f64()));
            }
        }
        text
    }
    fn update_tooltip(&self) {
        let Some((row, column)) = self.hovered else {
            self.hide_tooltip();
            return;
        };
        let Some(cell) = self.cell(row, column) else {
            self.hide_tooltip();
            return;
        };
        let text = self.tooltip_text(cell);
        if text.is_empty() {
            self.hide_tooltip();
            return;
        }
        let margin = self.margin() as i32;
        let x = column as i32 * self.cell_size.width as i32 - self.left_scroll as i32 + margin;
        let y = row as i32 * self.cell_size.height as i32 - self.top_scroll as i32 + margin;
        let rect = Rect::with_size(x, y, self.cell_size.width as u16, self.cell_size.height as u16);
        self.show_tooltip_on_rect(text.as_str(), &rect);
    }
    fn refresh_hovered_tooltip(&self) {
        if self.hovered.is_some() {
            self.update_tooltip();
        }
    }
    fn clear_hover(&mut self) {
        if self.hovered.take().is_some() {
            self.hide_tooltip();
        }
    }
    fn hover_at(&mut self, x: i32, y: i32) {
        let cell = self.cell_at(x, y).filter(|&(row, column)| self.cell(row, column).is_some());
        if cell == self.hovered {
            return;
        }
        self.hovered = cell;
        self.update_tooltip();
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
    fn cell_view_rect(&self, row: u32, column: u32) -> Rect {
        let margin = self.margin() as i64;
        let x = column as i64 * self.cell_size.width as i64 - self.left_scroll as i64 + margin;
        let y = row as i64 * self.cell_size.height as i64 - self.top_scroll as i64 + margin;
        Rect::with_size(x as i32, y as i32, self.cell_size.width as u16, self.cell_size.height as u16)
    }
    fn raise_selection_event(&self, cell: Option<(u32, u32)>) {
        let event_type = match cell {
            Some((row, column)) => EventType::CellSelected { row, column },
            None => EventType::ClearSelection,
        };
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::HeatMap(EventData {
                type_id: TypeId::of::<T>(),
                event_type,
            }),
        });
    }
    fn select_cell_from_user(&mut self, cell: Option<(u32, u32)>) -> bool {
        let cell = cell.filter(|&(row, column)| self.index(row, column).is_some());
        if self.selected == cell {
            return false;
        }
        self.selected = cell;
        self.raise_selection_event(cell);
        true
    }
    // The next filled cell after the selection (or before it), wrapping around.
    // Horizontal moves walk the rows; vertical moves walk the columns. Empty cells are skipped.
    // With nothing selected, a forward move starts at the first filled cell and a backward move at the last.
    fn neighbor(&self, vertical: bool, forward: bool) -> Option<(u32, u32)> {
        let width = self.size.width as usize;
        let height = self.size.height as usize;
        if width == 0 || height == 0 || self.cells.is_empty() {
            return None;
        }
        let count = width * height;
        let index_of = |row: u32, column: u32| -> usize {
            if vertical {
                column as usize * height + row as usize
            } else {
                row as usize * width + column as usize
            }
        };
        let cell_of = |index: usize| -> (u32, u32) {
            if vertical {
                ((index % height) as u32, (index / height) as u32)
            } else {
                ((index / width) as u32, (index % width) as u32)
            }
        };
        let start = match self.selected {
            Some((row, column)) => index_of(row, column),
            None => {
                if forward {
                    count - 1
                } else {
                    0
                }
            }
        };
        let mut index = start;
        for _ in 0..count {
            index = if forward {
                if index + 1 >= count {
                    0
                } else {
                    index + 1
                }
            } else if index == 0 {
                count - 1
            } else {
                index - 1
            };
            let (row, column) = cell_of(index);
            if self.cell(row, column).is_some() {
                return Some((row, column));
            }
        }
        None
    }
    fn move_selection(&mut self, vertical: bool, forward: bool) {
        match self.neighbor(vertical, forward) {
            Some((row, column)) => {
                self.select_cell_from_user(Some((row, column)));
                self.reveal_cell(row, column);
            }
            None => {
                self.select_cell_from_user(None);
            }
        }
    }
    fn reveal_cell(&mut self, row: u32, column: u32) {
        let (cw, ch) = (self.cell_size.width, self.cell_size.height);
        if cw == 0 || ch == 0 {
            return;
        }
        let view = self.size();
        if view.width == 0 || view.height == 0 {
            return;
        }
        let margin = self.margin();
        let cell_left = column.saturating_mul(cw).saturating_add(margin);
        let cell_top = row.saturating_mul(ch).saturating_add(margin);
        let cell_right = cell_left.saturating_add(cw);
        let cell_bottom = cell_top.saturating_add(ch);
        let mut left = self.left_scroll;
        let mut top = self.top_scroll;
        if cell_left < left {
            left = cell_left;
        } else if cell_right > left.saturating_add(view.width) {
            left = cell_right.saturating_sub(view.width);
        }
        if cell_top < top {
            top = cell_top;
        } else if cell_bottom > top.saturating_add(view.height) {
            top = cell_bottom.saturating_sub(view.height);
        }
        if left != self.left_scroll || top != self.top_scroll {
            self.scroll_to(left, top);
        }
    }
    fn paint_selection_border(&self, surface: &mut Surface, content: Rect, attr: CharAttribute) {
        let view = surface.size();
        let view_w = view.width as i32;
        let view_h = view.height as i32;
        if view_w <= 0 || view_h <= 0 || self.cell_size.width == 0 {
            return;
        }
        if content.right() < 0 || content.bottom() < 0 || content.left() >= view_w || content.top() >= view_h {
            return;
        }
        let right_most = view_w - 1;
        let bottom_most = view_h - 1;
        // the margin is a free character around the grid, so the border can be a full rectangle
        if self.flags.contains(Flags::Margins) {
            let r = Rect::new(content.left() - 1, content.top() - 1, content.right() + 1, content.bottom() + 1);
            surface.draw_rect(r, LineType::Single, attr);
            return;
        }
        let left = (content.left() - 1).clamp(0, right_most);
        let right = (content.right() + 1).clamp(0, right_most);
        match self.cell_size.height {
            0 => {}
            1 => {
                let y = content.top();
                if (0..=bottom_most).contains(&y) {
                    surface.write_char(left, y, Character::with_attributes('[', attr));
                    surface.write_char(right, y, Character::with_attributes(']', attr));
                }
            }
            2 => {
                let y0 = content.top();
                let y1 = content.bottom();
                if (0..=bottom_most).contains(&y0) {
                    surface.write_char(left, y0, Character::with_attributes('┌', attr));
                    surface.write_char(right, y0, Character::with_attributes('┐', attr));
                }
                if (0..=bottom_most).contains(&y1) {
                    surface.write_char(left, y1, Character::with_attributes('└', attr));
                    surface.write_char(right, y1, Character::with_attributes('┘', attr));
                }
            }
            _ => {
                let top = content.top().clamp(0, bottom_most);
                let bottom = content.bottom().clamp(0, bottom_most);
                surface.draw_rect(Rect::new(left, top, right, bottom), LineType::Single, attr);
            }
        }
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
        let end_column = (self.left_scroll as u64 + view.width as u64)
            .saturating_sub(margin)
            .div_ceil(cw)
            .min(self.size.width as u64) as u32;
        let end_row = (self.top_scroll as u64 + view.height as u64)
            .saturating_sub(margin)
            .div_ceil(ch)
            .min(self.size.height as u64) as u32;
        let mut params = self.paint_params();
        for row in first_row..end_row {
            for column in first_column..end_column {
                self.paint_cell(row, column, &mut params);
            }
        }
    }
}

impl<T: Number + 'static> OnPaint for HeatMap<T> {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if self.has_focus() && self.flags.contains(Flags::ScrollBars) {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        if !self.is_enabled() {
            let attr = theme.chart.inactive;
            surface.draw_surface_with_transform(0, 0, &self.surface, |ch, _| Some(Character::with_attributes(ch.code, attr)));
            return;
        }
        // the selection stays visible after the heat map loses the focus, as in the usage chart
        let Some((row, column)) = self.selected else {
            surface.draw_surface(0, 0, &self.surface);
            return;
        };
        if self.index(row, column).is_none() {
            surface.draw_surface(0, 0, &self.surface);
            return;
        }
        let content = self.cell_view_rect(row, column);
        if self.flags.contains(Flags::DimCellsOnSelection) {
            let attr = theme.chart.inactive;
            surface.draw_surface_with_transform(0, 0, &self.surface, |ch, p| {
                if content.contains(p) {
                    Some(ch)
                } else {
                    Some(Character::with_attributes(ch.code, attr))
                }
            });
        } else {
            surface.draw_surface(0, 0, &self.surface);
        }
        self.paint_selection_border(surface, content, theme.chart.selection_border);
    }
}

impl<T: Number + 'static> OnResize for HeatMap<T> {
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.surface.resize(new_size);
        if self.flags.contains(Flags::ScrollBars) {
            self.scrollbars.resize(self.content_width(), self.content_height(), &self.base);
        }
        // the view changed, so a previous offset may now sit past the image
        self.scroll_to(self.left_scroll, self.top_scroll);
    }
}

impl<T: Number + 'static> OnKeyPressed for HeatMap<T> {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Ctrl+Left") => self.scroll_by_cells(-1, 0),
            key!("Ctrl+Right") => self.scroll_by_cells(1, 0),
            key!("Ctrl+Up") => self.scroll_by_cells(0, -1),
            key!("Ctrl+Down") => self.scroll_by_cells(0, 1),
            key!("Ctrl+Home") => self.scroll_to(0, 0),
            key!("Ctrl+End") => self.scroll_to(u32::MAX, u32::MAX),
            key!("Left") => self.move_selection(false, false),
            key!("Right") => self.move_selection(false, true),
            key!("Up") => self.move_selection(true, false),
            key!("Down") => self.move_selection(true, true),
            key!("Escape") => {
                if self.select_cell_from_user(None) {
                    return EventProcessStatus::Processed;
                }
                return EventProcessStatus::Ignored;
            }
            _ => return EventProcessStatus::Ignored,
        }
        EventProcessStatus::Processed
    }
}

impl<T: Number + 'static> OnMouseEvent for HeatMap<T> {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.scrollbars.process_mouse_event(event) {
            self.scroll_to(self.scrollbars.horizontal_index() as u32, self.scrollbars.vertical_index() as u32);
            return EventProcessStatus::Processed;
        }
        let response = match event {
            MouseEvent::Enter | MouseEvent::Leave => {
                self.clear_hover();
                EventProcessStatus::Processed
            }
            MouseEvent::Over(point) => {
                let previous = self.hovered;
                self.hover_at(point.x, point.y);
                if self.hovered == previous {
                    EventProcessStatus::Ignored
                } else {
                    EventProcessStatus::Processed
                }
            }
            MouseEvent::Wheel(direction) => {
                match direction {
                    MouseWheelDirection::Left => self.scroll_by_cells(-1, 0),
                    MouseWheelDirection::Right => self.scroll_by_cells(1, 0),
                    MouseWheelDirection::Up => self.scroll_by_cells(0, -1),
                    MouseWheelDirection::Down => self.scroll_by_cells(0, 1),
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Pressed(data) => {
                let cell = self.cell_at(data.x, data.y).filter(|&(row, column)| self.cell(row, column).is_some());
                if self.select_cell_from_user(cell) {
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            _ => EventProcessStatus::Ignored,
        };
        if self.scrollbars.should_repaint() {
            EventProcessStatus::Processed
        } else {
            response
        }
    }
}
