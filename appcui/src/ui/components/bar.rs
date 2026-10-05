//! Types that describe one bar in a chart.
//!
//! A [`Bar`] stores a numeric value and an optional label, color, thickness,
//! spacing, and [`BarDrawMode`]. Build one with [`BarBuilder`], or convert a
//! number through [`From`]. [`BarSpan`] places a label under a run of bars.
//!
//! [`BarFillType`], [`BarCapType`], [`BarPointType`], and [`BarLargePointType`]
//! choose the character or marker used by a draw mode.

use crate::graphics::{LineType, Point, Size};
use flat_string::FlatString;

use crate::{
    graphics::{CharAttribute, Character, Rect, SpecialChar, Surface},
    ui::common::Number,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
/// Character repeated to fill a bar drawn with [`BarDrawMode::Fill`].
///
/// [`Solid`](Self::Solid) is the default.
pub enum BarFillType {
    /// U+2588 — full block.
    #[default]
    Solid,
    /// U+2593 ▓ — dense shade.
    Shade75,
    /// U+2592 ▒ — medium shade.
    Shade50,
    /// U+2591 ░ — light shade.
    Shade25,
    /// 2x4 braille cells used as a fill.
    Braille,
    /// 2x4 checkerboard pattern.
    Checkerboard,
    /// U+253C ┼ — light grid/lattice.
    Grid,
    /// U+256C ╬ — double-line grid, heavier than [`Grid`](Self::Grid).
    GridDouble,
    /// U+2573 ╳ — diagonal cross-hatch.
    CrossHatch,
    /// ┇ — dashed line.
    Dashed,
    /// U+2571 ╱ — forward diagonal (rising left-to-right).
    DiagonalUp,
    /// U+2572 ╲ — backward diagonal (falling left-to-right).
    DiagonalDown,
    /// U+2587 ▇ — near-solid block with a thin notch (gap) at the top of each cell.
    Notched,
    /// Fill with an arbitrary character.
    Custom(char),
}
impl BarFillType {
    #[inline(always)]
    pub(crate) fn character(&self, attr: CharAttribute) -> Character {
        match self {
            BarFillType::Solid => Character::with_attributes(SpecialChar::Block100, attr),
            BarFillType::Shade75 => Character::with_attributes(SpecialChar::Block75, attr),
            BarFillType::Shade50 => Character::with_attributes(SpecialChar::Block50, attr),
            BarFillType::Shade25 => Character::with_attributes(SpecialChar::Block25, attr),
            BarFillType::Braille => Character::with_attributes('\u{28FF}', attr),
            BarFillType::Checkerboard => Character::with_attributes('\u{259A}', attr),
            BarFillType::Grid => Character::with_attributes('\u{253C}', attr),
            BarFillType::GridDouble => Character::with_attributes('\u{256C}', attr),
            BarFillType::CrossHatch => Character::with_attributes('\u{2573}', attr),
            BarFillType::Dashed => Character::with_attributes('┇', attr),
            BarFillType::DiagonalUp => Character::with_attributes('\u{2571}', attr),
            BarFillType::DiagonalDown => Character::with_attributes('\u{2572}', attr),
            BarFillType::Notched => Character::with_attributes('\u{2587}', attr),
            BarFillType::Custom(ch) => Character::with_attributes(*ch, attr),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
/// Character drawn across the tip of a bar that uses [`BarDrawMode::Cap`].
///
/// [`Solid`](Self::Solid) is the default.
pub enum BarCapType {
    /// U+2588 — full block.
    #[default]
    Solid,
    /// U+2593 ▓ — dense shade.
    Shade75,
    /// U+2592 ▒ — medium shade.
    Shade50,
    /// U+2591 ░ — light shade.
    Shade25,
    /// 2x4 braille cells used as a fill.
    Braille,
    /// Single line.
    SingleLine,
    /// Double line.
    DoubleLine,
    /// Single thick line.
    SingleThickLine,
    /// Fill with an arbitrary character.
    Custom(char),
}
impl BarCapType {
    #[inline(always)]
    pub(crate) fn character(&self, attr: CharAttribute) -> Character {
        match self {
            BarCapType::Solid => Character::with_attributes(SpecialChar::Block100, attr),
            BarCapType::Shade75 => Character::with_attributes(SpecialChar::Block75, attr),
            BarCapType::Shade50 => Character::with_attributes(SpecialChar::Block50, attr),
            BarCapType::Shade25 => Character::with_attributes(SpecialChar::Block25, attr),
            BarCapType::Braille => Character::with_attributes('\u{28FF}', attr),
            BarCapType::SingleLine => Character::with_attributes(SpecialChar::BoxHorizontalSingleLine, attr),
            BarCapType::DoubleLine => Character::with_attributes(SpecialChar::BoxHorizontalDoubleLine, attr),
            BarCapType::SingleThickLine => Character::with_attributes('\u{2501}', attr),
            BarCapType::Custom(ch) => Character::with_attributes(*ch, attr),
        }
    }
    /// Cap character drawn as a vertical stroke across a horizontal bar.
    fn vertical_character(&self, attr: CharAttribute) -> Character {
        match self {
            BarCapType::SingleLine => Character::with_attributes(SpecialChar::BoxVerticalSingleLine, attr),
            BarCapType::DoubleLine => Character::with_attributes(SpecialChar::BoxVerticalDoubleLine, attr),
            BarCapType::SingleThickLine => Character::with_attributes('\u{2503}', attr),
            _ => self.character(attr),
        }
    }
}

/// One-cell marker drawn by [`BarDrawMode::Point`].
///
/// [`Bullet`](Self::Bullet) is the default.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BarPointType {
    /// ● — default marker.
    #[default]
    Bullet,
    /// ◆ — diamond.
    Diamond,
    /// ■ — square.
    Square,
    /// An arbitrary marker character.
    Custom(char),
}
impl BarPointType {
    #[inline(always)]
    pub(crate) fn character(&self, attr: CharAttribute) -> Character {
        match self {
            BarPointType::Bullet => Character::with_attributes('●', attr),
            BarPointType::Diamond => Character::with_attributes('◆', attr),
            BarPointType::Square => Character::with_attributes('■', attr),
            BarPointType::Custom(ch) => Character::with_attributes(*ch, attr),
        }
    }
}

/// Marker drawn by [`BarDrawMode::LargePoint`].
///
/// The marker occupies a 3-by-2 block of cells. [`RoundSquare`](Self::RoundSquare) is the default.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BarLargePointType {
    /// Rounded single-line rectangle (`╭─╮` over `╰─╯`).
    #[default]
    RoundSquare,
    /// Single-line rectangle (`┌─┐` over `└─┘`).
    Square,
    /// Double-line rectangle (`╔═╗` over `╚═╝`).
    DoubleLineSquare,
    /// Thick-line rectangle (`┏━┓` over `┗━┛`).
    ThickSquare,
    /// Approximate circle drawn in the 3-by-2 block.
    Circle,
    /// Diamond drawn in the 3-by-2 block.
    Diamond,
    /// Fills the 3-by-2 block with an arbitrary character.
    Custom(char),
}
impl BarLargePointType {
    pub(crate) fn characters(&self) -> [char; 6] {
        match self {
            BarLargePointType::RoundSquare => ['╭', '─', '╮', '╰', '─', '╯'],
            BarLargePointType::Square => ['┌', '─', '┐', '└', '─', '┘'],
            BarLargePointType::DoubleLineSquare => ['╔', '═', '╗', '╚', '═', '╝'],
            BarLargePointType::ThickSquare => ['┏', '━', '┓', '┗', '━', '┛'],
            BarLargePointType::Circle => ['▞', '▀', '▚', '▚', '▄', '▞'],
            BarLargePointType::Diamond => ['╱', '╲', ' ', '╲', '╱', ' '],
            BarLargePointType::Custom(ch) => [*ch, *ch, *ch, *ch, *ch, *ch],
        }
    }
    /// 2-by-3 marker used when the bar is horizontal. Cells are row-major.
    fn characters_horizontal(&self) -> [char; 6] {
        match self {
            BarLargePointType::RoundSquare => ['╭', '╮', '│', '│', '╰', '╯'],
            BarLargePointType::Square => ['┌', '┐', '│', '│', '└', '┘'],
            BarLargePointType::DoubleLineSquare => ['╔', '╗', '║', '║', '╚', '╝'],
            BarLargePointType::ThickSquare => ['┏', '┓', '┃', '┃', '┗', '┛'],
            BarLargePointType::Circle => ['▞', '▚', '▀', '▀', '▚', '▞'],
            BarLargePointType::Diamond => ['╱', '╲', ' ', ' ', '╲', '╱'],
            BarLargePointType::Custom(ch) => [*ch, *ch, *ch, *ch, *ch, *ch],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// How a bar is drawn.
///
/// Some modes force the thickness into a fixed range. A thickness outside that
/// range is clamped when the bar is painted. [`Fill`](Self::Fill) with
/// [`BarFillType::Solid`] is the default.
pub enum BarDrawMode {
    /// Fills the bar with a repeated character. Thickness is at least 1.
    Fill(BarFillType),
    /// Draws a line of the given [`LineType`]. Thickness is always 1.
    Line(LineType),
    /// Draws an empty rectangle of the given [`LineType`]. Thickness is at least 2.
    Rectangle(LineType),
    /// Draws a rectangle of the given [`LineType`], filled with a solid block. Thickness is at least 3.
    FilledRectangle(LineType),
    /// Draws a one-cell marker at the bar's value. Thickness is always 1.
    Point(BarPointType),
    /// Draws a 3-by-2 marker at the bar's value. Thickness is always 3.
    LargePoint(BarLargePointType),
    /// Draws a cap across the tip of the bar. Thickness is at least 1.
    Cap(BarCapType),
}

impl BarDrawMode {
    fn thickness_range(&self) -> (u8, u8) {
        match self {
            BarDrawMode::Fill(_) => (1, u8::MAX),
            BarDrawMode::Line(_) => (1, 1),
            BarDrawMode::Rectangle(_) => (2, u8::MAX),
            BarDrawMode::FilledRectangle(_) => (3, u8::MAX),
            BarDrawMode::Point(_) => (1, 1),
            BarDrawMode::LargePoint(_) => (3, 3),
            BarDrawMode::Cap(_) => (1, u8::MAX),
        }
    }
}
impl Default for BarDrawMode {
    fn default() -> Self {
        BarDrawMode::Fill(BarFillType::Solid)
    }
}

#[derive(Clone, Debug)]
/// One value in a bar chart.
///
/// A bar stores its value and label. Color, thickness, spacing, and draw mode
/// are optional: a field left unset uses the chart default. A value of type `T`
/// converts into a bar through [`From`].
pub struct Bar<T: Number + 'static> {
    pub(crate) value: T,
    pub(crate) label: String,
    pub(crate) attr: Option<CharAttribute>,
    pub(crate) thickness: Option<u8>,
    pub(crate) spacing: Option<u8>,
    pub(crate) draw_mode: Option<BarDrawMode>,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct BarDefaults {
    pub(crate) attr: CharAttribute,
    pub(crate) thickness: u8,
    pub(crate) spacing: u8,
    pub(crate) draw_mode: BarDrawMode,
}
#[derive(Copy, Clone, Default)]
pub(crate) struct BarLayout {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) length: i16,
    pub(crate) surface_size: Size,
}

impl<T: Number + 'static> Bar<T> {
    /// Returns the numeric value of this bar.
    #[inline(always)]
    pub fn value(&self) -> T {
        self.value
    }
    /// Sets the numeric value of this bar.
    #[inline(always)]
    pub fn set_value(&mut self, value: T) -> &mut Self {
        self.value = value;
        self
    }
    /// Returns the label displayed for this bar (for example on the X axis).
    #[inline(always)]
    pub fn label(&self) -> &str {
        &self.label
    }
    /// Sets the label displayed for this bar.
    #[inline(always)]
    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label.clear();
        self.label.push_str(label);
        self
    }
    /// Returns the character attribute of this bar, or `None` if the chart default is used.
    #[inline(always)]
    pub fn attr(&self) -> Option<CharAttribute> {
        self.attr
    }
    /// Sets the character attribute of this bar.
    #[inline(always)]
    pub fn set_attr(&mut self, attr: CharAttribute) -> &mut Self {
        self.attr = Some(attr);
        self
    }
    /// Clears the bar-specific attribute so the chart default is used.
    #[inline(always)]
    pub fn clear_attr(&mut self) -> &mut Self {
        self.attr = None;
        self
    }
    /// Returns the thickness of this bar, or `None` if the chart default is used.
    #[inline(always)]
    pub fn thickness(&self) -> Option<u8> {
        self.thickness
    }
    /// Sets the thickness of this bar, in cells.
    ///
    /// The draw mode may clamp this value when the bar is painted.
    #[inline(always)]
    pub fn set_thickness(&mut self, thickness: u8) -> &mut Self {
        self.thickness = Some(thickness);
        self
    }
    /// Clears the bar-specific thickness so the chart default is used.
    #[inline(always)]
    pub fn clear_thickness(&mut self) -> &mut Self {
        self.thickness = None;
        self
    }
    /// Returns the spacing before this bar, or `None` if the chart default is used.
    #[inline(always)]
    pub fn spacing(&self) -> Option<u8> {
        self.spacing
    }
    /// Sets the spacing before this bar.
    #[inline(always)]
    pub fn set_spacing(&mut self, spacing: u8) -> &mut Self {
        self.spacing = Some(spacing);
        self
    }
    /// Clears the bar-specific spacing so the chart default is used.
    #[inline(always)]
    pub fn clear_spacing(&mut self) -> &mut Self {
        self.spacing = None;
        self
    }
    /// Returns the draw mode of this bar, or `None` if the chart default is used.
    #[inline(always)]
    pub fn draw_mode(&self) -> Option<BarDrawMode> {
        self.draw_mode
    }
    /// Sets the draw mode of this bar.
    #[inline(always)]
    pub fn set_draw_mode(&mut self, draw_mode: BarDrawMode) -> &mut Self {
        self.draw_mode = Some(draw_mode);
        self
    }
    /// Clears the bar-specific draw mode so the chart default is used.
    #[inline(always)]
    pub fn clear_draw_mode(&mut self) -> &mut Self {
        self.draw_mode = None;
        self
    }
    #[inline(always)]
    fn paint_vertical_fill(&self, surface: &mut Surface, c: Character, layout: &BarLayout, defaults: &BarDefaults) {
        if layout.length == 0 {
            let ch = Character::new('_', c.foreground, c.background, c.flags);
            surface.fill_horizontal_line_with_size(layout.x, layout.y, self.actual_thickness(defaults) as u32, ch);
        } else {
            surface.fill_rect(self.rect_vertical(layout, defaults), c);
        }
    }

    #[inline(always)]
    fn paint_vertical_line(&self, surface: &mut Surface, line_type: LineType, attr: CharAttribute, layout: &BarLayout) {
        let (upper, bottom, zero) = match line_type {
            LineType::Single => ('┬', '┴', '─'),
            LineType::Double => ('╦', '╩', '═'),
            LineType::SingleThick => ('┳', '┻', '━'),
            LineType::Border => ('▄', '▀', '━'),
            LineType::Ascii => ('-', '-', '-'),
            LineType::AsciiRound => ('-', '-', '-'),
            LineType::SingleRound => ('┬', '┴', '─'),
            LineType::Braille => ('⣶', '⠿', '⠶'),
        };
        if layout.length > 0 {
            surface.draw_vertical_line_with_size(layout.x, layout.y - layout.length as i32 + 1, layout.length as u32, line_type, attr);
            surface.write_char(layout.x, layout.y, Character::with_attributes(bottom, attr));
        } else if layout.length < 0 {
            surface.draw_vertical_line_with_size(layout.x, layout.y, layout.length.unsigned_abs() as u32, line_type, attr);
            surface.write_char(layout.x, layout.y, Character::with_attributes(upper, attr));
        } else {
            surface.write_char(layout.x, layout.y, Character::with_attributes(zero, attr));
        }
    }
    #[inline(always)]
    fn paint_vertical_rect(
        &self,
        surface: &mut Surface,
        line_type: LineType,
        attr: CharAttribute,
        layout: &BarLayout,
        defaults: &BarDefaults,
        fill: bool,
    ) {
        let mut r = self.rect_vertical(layout, defaults);
        if layout.length == 0 {
            let y = r.top().min(layout.surface_size.height as i32 - 1);
            surface.draw_horizontal_line(r.left(), y, r.right(), line_type, attr);
        } else {
            if (layout.length > 0) && (r.bottom() as u32) + 1 < layout.surface_size.height {
                r.set_bottom(r.bottom() + 1, false);
            }
            if fill {
                surface.fill_rect(r, Character::with_attributes(SpecialChar::Block100, attr));
            }
            surface.draw_rect(r, line_type, attr);
        }
    }
    #[inline(always)]
    fn paint_vertical_point(&self, surface: &mut Surface, bar_point_type: BarPointType, attr: CharAttribute, layout: &BarLayout) {
        let point = self.point_vertical(layout);
        surface.write_char(point.x, point.y, bar_point_type.character(attr));
    }
    #[inline(always)]
    fn paint_vertical_large_point(&self, surface: &mut Surface, bar_point_type: BarLargePointType, attr: CharAttribute, layout: &BarLayout) {
        let point = self.point_vertical(layout);
        let chars = bar_point_type.characters();
        surface.write_char(point.x, point.y, Character::with_attributes(chars[0], attr));
        surface.write_char(point.x + 1, point.y, Character::with_attributes(chars[1], attr));
        surface.write_char(point.x + 2, point.y, Character::with_attributes(chars[2], attr));
        surface.write_char(point.x, point.y + 1, Character::with_attributes(chars[3], attr));
        surface.write_char(point.x + 1, point.y + 1, Character::with_attributes(chars[4], attr));
        surface.write_char(point.x + 2, point.y + 1, Character::with_attributes(chars[5], attr));
    }
    #[inline(always)]
    fn paint_vertical_cap(&self, surface: &mut Surface, cap_type: BarCapType, attr: CharAttribute, layout: &BarLayout, defaults: &BarDefaults) {
        let point = self.point_vertical(layout);
        surface.fill_horizontal_line_with_size(point.x, point.y, self.actual_thickness(defaults) as u32, cap_type.character(attr));
    }
    #[inline(always)]
    pub(crate) fn actual_thickness(&self, defaults: &BarDefaults) -> u8 {
        let mode = self.draw_mode.unwrap_or(defaults.draw_mode);
        let (min, max) = mode.thickness_range();
        self.thickness.unwrap_or(defaults.thickness).clamp(min, max)
    }
    #[inline(always)]
    pub(crate) fn rect_vertical(&self, layout: &BarLayout, defaults: &BarDefaults) -> Rect {
        let thickness = self.actual_thickness(defaults) as u32;
        let abs_h = layout.length.unsigned_abs();
        let top = if layout.length > 0 {
            layout.y + 1 - layout.length as i32
        } else {
            layout.y + 1
        };
        Rect::with_size(layout.x, top, thickness as u16, abs_h)
    }
    pub(crate) fn point_vertical(&self, layout: &BarLayout) -> Point {
        if layout.length > 0 {
            Point::new(layout.x, layout.y - layout.length as i32 + 1)
        } else {
            Point::new(layout.x, layout.y + layout.length as i32)
        }
    }
    pub(crate) fn paint_vertical(&self, surface: &mut Surface, layout: &BarLayout, defaults: &BarDefaults) {
        let mode = self.draw_mode.unwrap_or(defaults.draw_mode);
        let attr = self.attr.unwrap_or(defaults.attr);
        match mode {
            BarDrawMode::Fill(fill_type) => self.paint_vertical_fill(surface, fill_type.character(attr), layout, defaults),
            BarDrawMode::Line(line_type) => self.paint_vertical_line(surface, line_type, attr, layout),
            BarDrawMode::FilledRectangle(line_type) => self.paint_vertical_rect(surface, line_type, attr, layout, defaults, true),
            BarDrawMode::Rectangle(line_type) => self.paint_vertical_rect(surface, line_type, attr, layout, defaults, false),
            BarDrawMode::Point(point_type) => self.paint_vertical_point(surface, point_type, attr, layout),
            BarDrawMode::LargePoint(point_type) => self.paint_vertical_large_point(surface, point_type, attr, layout),
            BarDrawMode::Cap(cap_type) => self.paint_vertical_cap(surface, cap_type, attr, layout, defaults),
        }
    }

    /// `layout.x` is the baseline column and `layout.y` is the top of the bar.
    /// A positive length grows to the right; a negative length grows to the left.
    #[inline(always)]
    fn paint_horizontal_fill(&self, surface: &mut Surface, c: Character, layout: &BarLayout, defaults: &BarDefaults) {
        if layout.length == 0 {
            let ch = Character::new('|', c.foreground, c.background, c.flags);
            surface.fill_vertical_line_with_size(layout.x, layout.y, self.actual_thickness(defaults) as u32, ch);
        } else {
            surface.fill_rect(self.rect_horizontal(layout, defaults), c);
        }
    }
    #[inline(always)]
    fn paint_horizontal_line(&self, surface: &mut Surface, line_type: LineType, attr: CharAttribute, layout: &BarLayout) {
        let (right_cap, left_cap, zero) = match line_type {
            LineType::Single => ('┤', '├', '│'),
            LineType::Double => ('╣', '╠', '║'),
            LineType::SingleThick => ('┫', '┣', '┃'),
            LineType::Border => ('▐', '▌', '┃'),
            LineType::Ascii => ('|', '|', '|'),
            LineType::AsciiRound => ('|', '|', '|'),
            LineType::SingleRound => ('┤', '├', '│'),
            LineType::Braille => ('⠸', '⠇', '⡇'),
        };
        if layout.length > 0 {
            surface.draw_horizontal_line_with_size(layout.x, layout.y, layout.length as u32, line_type, attr);
            surface.write_char(layout.x, layout.y, Character::with_attributes(left_cap, attr));
        } else if layout.length < 0 {
            let abs = layout.length.unsigned_abs() as u32;
            surface.draw_horizontal_line_with_size(layout.x - abs as i32 + 1, layout.y, abs, line_type, attr);
            surface.write_char(layout.x, layout.y, Character::with_attributes(right_cap, attr));
        } else {
            surface.write_char(layout.x, layout.y, Character::with_attributes(zero, attr));
        }
    }
    #[inline(always)]
    fn paint_horizontal_rect(
        &self,
        surface: &mut Surface,
        line_type: LineType,
        attr: CharAttribute,
        layout: &BarLayout,
        defaults: &BarDefaults,
        fill: bool,
    ) {
        let mut r = self.rect_horizontal(layout, defaults);
        if layout.length == 0 {
            let x = r.left().min(layout.surface_size.width as i32 - 1);
            surface.draw_vertical_line(x, r.top(), r.bottom(), line_type, attr);
        } else {
            if (layout.length > 0) && r.left() > 0 {
                r.set_left(r.left() - 1, false);
            }
            if fill {
                surface.fill_rect(r, Character::with_attributes(SpecialChar::Block100, attr));
            }
            surface.draw_rect(r, line_type, attr);
        }
    }
    #[inline(always)]
    fn paint_horizontal_point(&self, surface: &mut Surface, bar_point_type: BarPointType, attr: CharAttribute, layout: &BarLayout) {
        let point = self.point_horizontal(layout);
        surface.write_char(point.x, point.y, bar_point_type.character(attr));
    }
    #[inline(always)]
    fn paint_horizontal_large_point(&self, surface: &mut Surface, bar_point_type: BarLargePointType, attr: CharAttribute, layout: &BarLayout) {
        let point = self.point_horizontal(layout);
        let chars = bar_point_type.characters_horizontal();
        let x = if layout.length > 0 { point.x - 1 } else { point.x };
        surface.write_char(x, point.y, Character::with_attributes(chars[0], attr));
        surface.write_char(x + 1, point.y, Character::with_attributes(chars[1], attr));
        surface.write_char(x, point.y + 1, Character::with_attributes(chars[2], attr));
        surface.write_char(x + 1, point.y + 1, Character::with_attributes(chars[3], attr));
        surface.write_char(x, point.y + 2, Character::with_attributes(chars[4], attr));
        surface.write_char(x + 1, point.y + 2, Character::with_attributes(chars[5], attr));
    }
    #[inline(always)]
    fn paint_horizontal_cap(&self, surface: &mut Surface, cap_type: BarCapType, attr: CharAttribute, layout: &BarLayout, defaults: &BarDefaults) {
        let point = self.point_horizontal(layout);
        surface.fill_vertical_line_with_size(point.x, point.y, self.actual_thickness(defaults) as u32, cap_type.vertical_character(attr));
    }
    #[inline(always)]
    pub(crate) fn rect_horizontal(&self, layout: &BarLayout, defaults: &BarDefaults) -> Rect {
        let thickness = self.actual_thickness(defaults) as u16;
        let abs_w = layout.length.unsigned_abs();
        let left = if layout.length > 0 { layout.x } else { layout.x - abs_w as i32 };
        Rect::with_size(left, layout.y, abs_w, thickness)
    }
    pub(crate) fn point_horizontal(&self, layout: &BarLayout) -> Point {
        if layout.length > 0 {
            Point::new(layout.x + layout.length as i32 - 1, layout.y)
        } else {
            Point::new(layout.x + layout.length as i32, layout.y)
        }
    }
    pub(crate) fn paint_horizontal(&self, surface: &mut Surface, layout: &BarLayout, defaults: &BarDefaults) {
        let mode = self.draw_mode.unwrap_or(defaults.draw_mode);
        let attr = self.attr.unwrap_or(defaults.attr);
        match mode {
            BarDrawMode::Fill(fill_type) => self.paint_horizontal_fill(surface, fill_type.character(attr), layout, defaults),
            BarDrawMode::Line(line_type) => self.paint_horizontal_line(surface, line_type, attr, layout),
            BarDrawMode::FilledRectangle(line_type) => self.paint_horizontal_rect(surface, line_type, attr, layout, defaults, true),
            BarDrawMode::Rectangle(line_type) => self.paint_horizontal_rect(surface, line_type, attr, layout, defaults, false),
            BarDrawMode::Point(point_type) => self.paint_horizontal_point(surface, point_type, attr, layout),
            BarDrawMode::LargePoint(point_type) => self.paint_horizontal_large_point(surface, point_type, attr, layout),
            BarDrawMode::Cap(cap_type) => self.paint_horizontal_cap(surface, cap_type, attr, layout, defaults),
        }
    }
}

/// Builds a [`Bar`] by setting only the properties that should differ from the chart defaults.
///
/// # Example
/// ```rust
/// use appcui::prelude::*;
///
/// let bar = BarBuilder::new(10)
///     .label("Jun")
///     .thickness(4)
///     .draw_mode(BarDrawMode::Fill(BarFillType::Shade50))
///     .build();
/// assert_eq!(bar.value(), 10);
/// assert_eq!(bar.label(), "Jun");
/// ```
pub struct BarBuilder<T: Number + 'static> {
    bar: Bar<T>,
}
impl<T: Number + 'static> BarBuilder<T> {
    /// Starts a bar with `value`. Color, thickness, spacing, label, and draw mode stay unset.
    pub fn new(value: T) -> Self {
        Self {
            bar: Bar {
                value,
                attr: None,
                label: String::new(),
                thickness: None,
                spacing: None,
                draw_mode: None,
            },
        }
    }
    /// Sets the character attribute of the bar.
    pub fn attr(mut self, attr: CharAttribute) -> Self {
        self.bar.attr = Some(attr);
        self
    }
    /// Sets the label displayed for the bar.
    pub fn label(mut self, label: &str) -> Self {
        self.bar.label = label.to_string();
        self
    }
    /// Sets the thickness of the bar, in cells.
    ///
    /// The draw mode may clamp this value when the bar is painted.
    pub fn thickness(mut self, thickness: u8) -> Self {
        self.bar.thickness = Some(thickness);
        self
    }
    /// Sets the gap, in cells, before the bar.
    pub fn spacing(mut self, spacing: u8) -> Self {
        self.bar.spacing = Some(spacing);
        self
    }
    /// Sets the draw mode of the bar.
    pub fn draw_mode(mut self, draw_mode: BarDrawMode) -> Self {
        self.bar.draw_mode = Some(draw_mode);
        self
    }
    /// Returns the finished bar.
    pub fn build(self) -> Bar<T> {
        self.bar
    }
}

impl<T> From<T> for Bar<T>
where
    T: Number + 'static,
{
    fn from(value: T) -> Self {
        BarBuilder::new(value).build()
    }
}
impl<T> From<&T> for Bar<T>
where
    T: Number + 'static,
{
    fn from(value: &T) -> Self {
        (*value).into()
    }
}

#[derive(Copy, Clone, Debug)]
/// A label that covers a run of consecutive bars on a chart's X axis.
///
/// The label is stored in a 22-character buffer.
pub struct BarSpan {
    pub(crate) start: u32,
    pub(crate) end: u32,
    pub(crate) label: FlatString<22>,
}
impl BarSpan {
    /// Creates a span that starts at bar `start` and covers `count` bars.
    ///
    /// A `count` below 1 is treated as 1.
    pub fn new(start: u32, count: u32, label: &str) -> Self {
        Self {
            start,
            end: start.saturating_add(count.max(1) - 1),
            label: FlatString::from_str(label),
        }
    }
}
