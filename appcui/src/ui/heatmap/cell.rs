use crate::{
    graphics::{CharAttribute, Character, Color, Rect, Size, Surface},
    prelude::Number,
};

pub(crate) struct PaintParams {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) size: Size,
    pub(crate) percentage: u8,
    pub(crate) default_attr: CellAttribute,
    pub(crate) default_fill: CellFill,
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum CellAttribute {
    Fixed(CharAttribute),
    Range(CharAttribute, CharAttribute),
}
/// How a heat map cell is drawn.
///
/// [`Shades`](Self::Shades) uses the cell's place between the smallest and the largest value
/// to pick a character: a space, `░`, `▒`, `▓`, or `█`. [`Custom`](Self::Custom) repeats one
/// character and ignores that place.
///
/// A cell with no fill of its own uses the heat map's default fill. With no default fill,
/// the heat map uses [`Shades`](Self::Shades).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellFill {
    /// Shade characters chosen from the cell's value.
    Shades,
    /// Repeats this character for the whole cell.
    Custom(char),
}
pub struct Cell<T: Number> {
    pub(crate) value: T,
    pub(crate) label: String,
    pub(crate) attr: Option<CellAttribute>,
    pub(crate) fill: Option<CellFill>,
}
impl<T: Number> Cell<T> {
    const SHADES: &[char] = &[' ', '░', '▒', '▓', '█'];
    pub fn value(&self) -> T {
        self.value
    }
    /// This cell's fill, or `None` when the cell uses the heat map's default fill.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(4, 3), heatmap::Flags::None);
    /// heatmap.set(1, 2, 10);
    /// heatmap.update_cell(1, 2, |cell| cell.set_fill(heatmap::CellFill::Shades));
    /// let fill = heatmap.cell(1, 2).and_then(|cell| cell.fill());
    /// assert_eq!(fill, Some(heatmap::CellFill::Shades));
    /// ```
    pub fn fill(&self) -> Option<CellFill> {
        self.fill
    }
    /// Sets this cell's fill, replacing the heat map's default fill for this cell only.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(4, 3), heatmap::Flags::None);
    /// heatmap.set(1, 2, 10);
    /// heatmap.update_cell(1, 2, |cell| cell.set_fill(heatmap::CellFill::Custom('*')));
    /// ```
    pub fn set_fill(&mut self, fill: CellFill) {
        self.fill = Some(fill);
    }
    /// Removes this cell's fill so the cell uses the heat map's default fill.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(4, 3), heatmap::Flags::None);
    /// heatmap.set(1, 2, 10);
    /// heatmap.update_cell(1, 2, |cell| {
    ///     cell.set_fill(heatmap::CellFill::Custom('*'));
    ///     cell.clear_fill();
    /// });
    /// ```
    pub fn clear_fill(&mut self) {
        self.fill = None;
    }
    /// The label shown by tooltip formats that include a label.
    pub fn label(&self) -> &str {
        &self.label
    }
    /// Sets the label shown by tooltip formats that include a label.
    ///
    /// An empty label is allowed.
    pub fn set_label(&mut self, label: &str) {
        self.label.clear();
        self.label.push_str(label);
    }
    // where percentage is a value between 0 and 100
    pub(super) fn paint(&self, surface: &mut Surface, params: &PaintParams) {
        let attr = self.compute_attr(params);
        let fill = self.compute_fill(params);
        let ch = Character::with_attributes(fill, attr);
        let r = Rect::with_size(params.x, params.y, params.size.width as u16, params.size.height as u16);
        surface.fill_rect(r, ch);
    }
    fn compute_fill(&self, params: &PaintParams) -> char {
        let fill = self.fill.unwrap_or(params.default_fill);
        match fill {
            CellFill::Shades => Self::percentage_to_char(Self::SHADES, params.percentage),
            CellFill::Custom(ch) => ch,
        }
    }
    fn compute_attr(&self, params: &PaintParams) -> CharAttribute {
        let attr = self.attr.as_ref().unwrap_or(&params.default_attr);
        match attr {
            CellAttribute::Fixed(attr) => *attr,
            CellAttribute::Range(min, max) => Self::attribute_from_range(*min, *max, params.percentage),
        }
    }
    fn attribute_from_range(start: CharAttribute, end: CharAttribute, percentage: u8) -> CharAttribute {
        let fore = Self::color_from_range(start.foreground, end.foreground, percentage);
        let back = Self::color_from_range(start.background, end.background, percentage);
        if percentage <= 50 {
            CharAttribute::new(fore, back, start.flags)
        } else {
            CharAttribute::new(fore, back, end.flags)
        }
    }
    fn blend(from: u8, to: u8, percentage: u32) -> u8 {
        let (from, to) = (from as u32, to as u32);
        if to >= from {
            (from + (to - from) * percentage / 100) as u8
        } else {
            (from - (from - to) * percentage / 100) as u8
        }
    }
    fn color_from_range(start: Color, end: Color, percentage: u8) -> Color {
        let percentage = percentage.min(100) as u32;
        match (start.to_rgb(), end.to_rgb()) {
            (Some(start_rgb), Some(end_rgb)) => {
                let r = Self::blend(start_rgb.0, end_rgb.0, percentage);
                let g = Self::blend(start_rgb.1, end_rgb.1, percentage);
                let b = Self::blend(start_rgb.2, end_rgb.2, percentage);
                Color::from_rgb(r, g, b)
            }
            _ => {
                if percentage <= 50 {
                    start
                } else {
                    end
                }
            }
        }
    }
    fn percentage_to_char(list: &[char], proc: u8) -> char {
        let len = list.len();
        let csz = (100_usize / len).max(1);
        let index = (proc as usize / csz).min(len - 1);
        list[index]
    }
}

/// A cell with this value and the heat map's default fill and attribute.
impl<T: Number> From<T> for Cell<T> {
    fn from(value: T) -> Self {
        Self {
            value,
            label: String::new(),
            attr: None,
            fill: None,
        }
    }
}
