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
/// The colors a heat map cell is drawn with.
///
/// [`Fixed`](Self::Fixed) uses one [`CharAttribute`] for every value.
/// [`Range`](Self::Range) blends two attributes by the cell's place between the
/// smallest and the largest value: the first at 0% and the second at 100%.
///
/// A cell with no attribute of its own uses the heat map's default attribute.
/// With no default attribute, the heat map uses the theme's first chart color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellAttribute {
    /// One attribute for every value.
    Fixed(CharAttribute),
    /// Blends from the first attribute to the second as the value goes from the minimum to the maximum.
    Range(CharAttribute, CharAttribute),
}
/// How a heat map cell is drawn.
///
/// Each variant except [`Custom`](Self::Custom) uses the cell's place between the
/// smallest and the largest value to pick a character from its list, from a space at
/// 0% to the last character at 100%. [`Custom`](Self::Custom) repeats one character
/// and ignores that place.
///
/// A cell with no fill of its own uses the heat map's default fill. With no default fill,
/// the heat map uses [`Shades`](Self::Shades).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellFill {
    /// Space, `░`, `▒`, `▓`, `█`.
    Shades,
    /// Space, `·`, `•`, `●`.
    Dots,
    /// Space, `.`, `:`, `-`, `=`, `+`, `x`, `#`, `%`, `@`.
    ASCII,
    /// Space, `·`, `∘`, `○`, `◎`, `●`.
    Circles,
    /// Space, `▫`, `□`, `▪`, `■`.
    Squares,
    /// Space, `◇`, `◈`, `◆`.
    Diamonds,
    /// Space, `·`, `+`, `✚`.
    Crosses,
    /// Space, `✧`, `✦`, `✶`, `✹`.
    Stars,
    /// Space, `✢`, `✣`, `✤`, `✥`.
    Asterisks,
    /// Space, `❀`, `❁`, `✿`.
    Flowers,
    /// Space, `✛`, `✜`, `✚`.
    Pluses,
    /// Repeats this character for the whole cell.
    Custom(char),
}
/// One cell of a [`struct@super::HeatMap`].
///
/// A cell stores a value, a label, and an optional fill and attribute. A number becomes
/// a cell with that value, an empty label, and the heat map's default fill and attribute.
/// Build a cell that sets a label, fill, or attribute with [`CellBuilder`].
pub struct Cell<T: Number> {
    pub(crate) value: T,
    pub(crate) label: String,
    pub(crate) attr: Option<CellAttribute>,
    pub(crate) fill: Option<CellFill>,
}
impl<T: Number> Cell<T> {
    const SHADES: &[char] = &[' ', '░', '▒', '▓', '█'];
    const DOTS: &[char] = &[' ', '·', '•', '●'];
    const ASCII: &[char] = &[' ', '.', ':', '-', '=', '+', 'x', '#', '%', '@'];
    const CIRCLES: &[char] = &[' ', '·', '∘', '○', '◎', '●'];
    const SQUARES: &[char] = &[' ', '▫', '□', '▪', '■'];
    const DIAMONDS: &[char] = &[' ', '◇', '◈', '◆'];
    const CROSSES: &[char] = &[' ', '·', '+', '✚'];
    const STARS: &[char] = &[' ', '✧', '✦', '✶', '✹'];
    const ASTERISKS: &[char] = &[' ', '✢', '✣', '✤', '✥'];
    const FLOWERS: &[char] = &[' ', '❀', '❁', '✿'];
    const PLUSES: &[char] = &[' ', '✛', '✜', '✚'];
    /// The number stored in this cell.
    ///
    /// The heat map uses it for the fill character, the color blend, and the tooltip.
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
    /// This cell's attribute, or `None` when the cell uses the heat map's default attribute.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(4, 3), heatmap::Flags::None);
    /// heatmap.set(1, 2, 10);
    /// let attr = heatmap::CellAttribute::Fixed(CharAttribute::with_color(Color::Red, Color::Black));
    /// heatmap.update_cell(1, 2, |cell| cell.set_attr(attr));
    /// assert_eq!(heatmap.cell(1, 2).and_then(|cell| cell.attr()), Some(attr));
    /// ```
    pub fn attr(&self) -> Option<CellAttribute> {
        self.attr
    }
    /// Sets this cell's attribute, replacing the heat map's default attribute for this cell only.
    ///
    /// [`CellAttribute::Fixed`](super::CellAttribute::Fixed) uses one color.
    /// [`CellAttribute::Range`](super::CellAttribute::Range) blends two colors from this cell's
    /// place between the smallest and the largest value.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(4, 3), heatmap::Flags::None);
    /// heatmap.set(1, 2, 10);
    /// heatmap.update_cell(1, 2, |cell| {
    ///     cell.set_attr(heatmap::CellAttribute::Fixed(CharAttribute::with_color(Color::Red, Color::Black)));
    /// });
    /// ```
    pub fn set_attr(&mut self, attr: CellAttribute) {
        self.attr = Some(attr);
    }
    /// Removes this cell's attribute so the cell uses the heat map's default attribute.
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut heatmap: HeatMap<u32> = HeatMap::new(layout!("d:f"), Size::new(4, 3), heatmap::Flags::None);
    /// heatmap.set(1, 2, 10);
    /// heatmap.update_cell(1, 2, |cell| {
    ///     cell.set_attr(heatmap::CellAttribute::Fixed(CharAttribute::with_color(Color::Red, Color::Black)));
    ///     cell.clear_attr();
    /// });
    /// ```
    pub fn clear_attr(&mut self) {
        self.attr = None;
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
            CellFill::Dots => Self::percentage_to_char(Self::DOTS, params.percentage),
            CellFill::ASCII => Self::percentage_to_char(Self::ASCII, params.percentage),
            CellFill::Circles => Self::percentage_to_char(Self::CIRCLES, params.percentage),
            CellFill::Squares => Self::percentage_to_char(Self::SQUARES, params.percentage),
            CellFill::Diamonds => Self::percentage_to_char(Self::DIAMONDS, params.percentage),
            CellFill::Crosses => Self::percentage_to_char(Self::CROSSES, params.percentage),
            CellFill::Stars => Self::percentage_to_char(Self::STARS, params.percentage),
            CellFill::Asterisks => Self::percentage_to_char(Self::ASTERISKS, params.percentage),
            CellFill::Flowers => Self::percentage_to_char(Self::FLOWERS, params.percentage),
            CellFill::Pluses => Self::percentage_to_char(Self::PLUSES, params.percentage),
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

/// Builds a [`Cell`] by setting only the properties that should differ from the heat map defaults.
///
/// # Example
/// ```rust
/// use appcui::prelude::*;
///
/// let cell = heatmap::CellBuilder::new(10)
///     .label("Peak")
///     .fill(heatmap::CellFill::Dots)
///     .attr(heatmap::CellAttribute::Fixed(CharAttribute::with_color(Color::Red, Color::Black)))
///     .build();
/// assert_eq!(cell.value(), 10);
/// assert_eq!(cell.label(), "Peak");
/// assert_eq!(cell.fill(), Some(heatmap::CellFill::Dots));
/// ```
pub struct CellBuilder<T: Number> {
    cell: Cell<T>,
}
impl<T: Number> CellBuilder<T> {
    /// Starts a cell with `value`. The label is empty, and the fill and attribute stay unset.
    pub fn new(value: T) -> Self {
        Self {
            cell: Cell {
                value,
                label: String::new(),
                attr: None,
                fill: None,
            },
        }
    }
    /// Sets the label shown by tooltip formats that include a label.
    ///
    /// An empty label is allowed.
    pub fn label(mut self, label: &str) -> Self {
        self.cell.label = label.to_string();
        self
    }
    /// Sets this cell's fill, replacing the heat map's default fill for this cell only.
    pub fn fill(mut self, fill: CellFill) -> Self {
        self.cell.fill = Some(fill);
        self
    }
    /// Sets this cell's attribute, replacing the heat map's default attribute for this cell only.
    pub fn attr(mut self, attr: CellAttribute) -> Self {
        self.cell.attr = Some(attr);
        self
    }
    /// Returns the finished cell.
    pub fn build(self) -> Cell<T> {
        self.cell
    }
}

/// The cell produced by this builder.
impl<T: Number> From<CellBuilder<T>> for Cell<T> {
    fn from(builder: CellBuilder<T>) -> Self {
        builder.build()
    }
}

/// A cell with this value and the heat map's default fill and attribute.
impl<T: Number> From<T> for Cell<T> {
    fn from(value: T) -> Self {
        CellBuilder::new(value).build()
    }
}
