//! Types shared by [`VBarChart`](crate::ui::vbarchart::VBarChart) and [`HBarChart`](crate::ui::hbarchart::HBarChart).
//!
//! A [`Bar`] stores a numeric value and an optional label, color, thickness,
//! spacing, and [`BarDrawMode`]. Build one with [`BarBuilder`], or convert a
//! number through [`From`]. [`BarSpan`] labels a run of bars: [`VBarChart`](crate::ui::vbarchart::VBarChart)
//! draws that label under the bars, and [`HBarChart`](crate::ui::hbarchart::HBarChart) draws it beside them.
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
/// [`Solid`](Self::Solid) is the default. Line caps follow the bar: a horizontal
/// stroke on a [`VBarChart`](crate::ui::vbarchart::VBarChart), a vertical stroke
/// on an [`HBarChart`](crate::ui::hbarchart::HBarChart).
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
/// [`VBarChart`](crate::ui::vbarchart::VBarChart) and [`HBarChart`](crate::ui::hbarchart::HBarChart)
/// draw the same 3-by-2 block of cells. [`RoundSquare`](Self::RoundSquare) is the default.
/// The bar thickness around that marker differs: see [`BarDrawMode::LargePoint`].
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
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// How a [`VBarChart`](crate::ui::vbarchart::VBarChart) or [`HBarChart`](crate::ui::hbarchart::HBarChart) draws a bar.
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
    /// Draws a 3-by-2 marker at the bar's value.
    ///
    /// Thickness is 3 on a [`VBarChart`](crate::ui::vbarchart::VBarChart) and 2 on an
    /// [`HBarChart`](crate::ui::hbarchart::HBarChart).
    LargePoint(BarLargePointType),
    /// Draws a cap across the tip of the bar. Thickness is at least 1.
    Cap(BarCapType),
}

impl BarDrawMode {
    fn thickness_range(&self, vertical: bool) -> (u8, u8) {
        match self {
            BarDrawMode::Fill(_) => (1, u8::MAX),
            BarDrawMode::Line(_) => (1, 1),
            BarDrawMode::Rectangle(_) => (2, u8::MAX),
            BarDrawMode::FilledRectangle(_) => (3, u8::MAX),
            BarDrawMode::Point(_) => (1, 1),
            BarDrawMode::LargePoint(_) => if vertical { (3, 3) } else { (2,2) },
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
/// One value in a [`VBarChart`](crate::ui::vbarchart::VBarChart) or [`HBarChart`](crate::ui::hbarchart::HBarChart).
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
    pub(crate) vertical: bool,
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
    /// Returns the label displayed for this bar.
    ///
    /// A [`VBarChart`](crate::ui::vbarchart::VBarChart) can show it under the bar.
    /// An [`HBarChart`](crate::ui::hbarchart::HBarChart) can show it beside the bar.
    /// Both charts include it in the hover tooltip when it is not empty.
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
    fn paint_horizontal_line(&self, surface: &mut Surface, line_type: LineType, attr: CharAttribute, layout: &BarLayout) {
        let (right_cap, left_cap, zero) = match line_type {
            LineType::Single => ('┤', '├', '│'),
            LineType::Double => ('╣', '╠', '║'),
            LineType::SingleThick => ('┫', '┣', '┃'),
            LineType::Border => ('█', '█', '┃'),
            LineType::Ascii => ('|', '|', '|'),
            LineType::AsciiRound => ('|', '|', '|'),
            LineType::SingleRound => ('┤', '├', '│'),
            LineType::Braille => ('⣿', '⣿', '⡇'),
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
            surface.draw_vertical_line((r.left() - 1).max(0), r.top(), r.bottom(), line_type, attr);
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
    fn paint_vertical_point(&self, surface: &mut Surface, bar_point_type: BarPointType, attr: CharAttribute, layout: &BarLayout) {
        let point = self.point_vertical(layout);
        surface.write_char(point.x, point.y, bar_point_type.character(attr));
    }
    #[inline(always)]
    fn paint_horizontal_point(&self, surface: &mut Surface, bar_point_type: BarPointType, attr: CharAttribute, layout: &BarLayout) {
        let point = self.point_horizontal(layout);
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
    fn paint_horizontal_large_point(&self, surface: &mut Surface, bar_point_type: BarLargePointType, attr: CharAttribute, layout: &BarLayout) {
        let point = self.point_horizontal(layout);
        let chars = bar_point_type.characters();
        surface.write_char(point.x - 1, point.y, Character::with_attributes(chars[0], attr));
        surface.write_char(point.x + 0, point.y, Character::with_attributes(chars[1], attr));
        surface.write_char(point.x + 1, point.y, Character::with_attributes(chars[2], attr));
        surface.write_char(point.x - 1, point.y + 1, Character::with_attributes(chars[3], attr));
        surface.write_char(point.x + 0, point.y + 1, Character::with_attributes(chars[4], attr));
        surface.write_char(point.x + 1, point.y + 1, Character::with_attributes(chars[5], attr));
    }    
    #[inline(always)]
    fn paint_vertical_cap(&self, surface: &mut Surface, cap_type: BarCapType, attr: CharAttribute, layout: &BarLayout, defaults: &BarDefaults) {
        let point = self.point_vertical(layout);
        surface.fill_horizontal_line_with_size(point.x, point.y, self.actual_thickness(defaults) as u32, cap_type.character(attr));
    }
    #[inline(always)]
    fn paint_horizontal_cap(&self, surface: &mut Surface, cap_type: BarCapType, attr: CharAttribute, layout: &BarLayout, defaults: &BarDefaults) {
        let point = self.point_horizontal(layout);
        surface.fill_vertical_line_with_size(
            point.x,
            point.y,
            self.actual_thickness(defaults) as u32,
            cap_type.vertical_character(attr),
        );
    }    
    #[inline(always)]
    pub(crate) fn actual_thickness(&self, defaults: &BarDefaults) -> u8 {
        let mode = self.draw_mode.unwrap_or(defaults.draw_mode);
        let (min, max) = mode.thickness_range(defaults.vertical);
        self.thickness.unwrap_or(defaults.thickness).clamp(min, max)
    }
    #[inline(always)]
    pub(crate) fn rect_vertical(&self, layout: &BarLayout, defaults: &BarDefaults) -> Rect {
        let thickness = self.actual_thickness(defaults) as u16;
        let abs_h = layout.length.unsigned_abs();
        let top = if layout.length > 0 {
            layout.y + 1 - layout.length as i32
        } else {
            layout.y + 1
        };
        Rect::with_size(layout.x, top, thickness, abs_h)
    }
    #[inline(always)]
    pub(crate) fn rect_horizontal(&self, layout: &BarLayout, defaults: &BarDefaults) -> Rect {
        let thickness = self.actual_thickness(defaults) as u16;
        let abs_w = layout.length.unsigned_abs();
        let left = if layout.length <= 0 {
            layout.x + 1 + layout.length as i32
        } else {
            layout.x + 1
        };
        Rect::with_size(left, layout.y, abs_w, thickness)
    }
    pub(crate) fn point_vertical(&self, layout: &BarLayout) -> Point {
        if layout.length > 0 {
            Point::new(layout.x, layout.y - layout.length as i32 + 1)
        } else {
            Point::new(layout.x, layout.y + layout.length as i32)
        }
    }
    pub(crate) fn point_horizontal(&self, layout: &BarLayout) -> Point {
        if layout.length > 0 {
            Point::new(layout.x + layout.length as i32 - 1, layout.y)
        } else {
            Point::new(layout.x + layout.length as i32, layout.y)
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
/// A label that covers a run of consecutive bars.
///
/// A [`VBarChart`](crate::ui::vbarchart::VBarChart) draws the label under the bars.
/// An [`HBarChart`](crate::ui::hbarchart::HBarChart) draws it beside them.
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
pub(crate)struct BarWithLayout<T: Number + 'static> {
    pub(crate) bar: Bar<T>,
    pub(crate) pos: i32,
    pub(crate) len: i16,
}
impl<T> BarWithLayout<T>
where
    T: Number + 'static,
{
    #[inline(always)]
    pub(crate) fn new(bar: Bar<T>) -> Self {
        Self { bar, pos: 0, len: 0 }
    }
}

/// A mutable view of the bars stored in a [`VBarChart`](crate::ui::vbarchart::VBarChart) or an [`HBarChart`](crate::ui::hbarchart::HBarChart).
///
/// `Bars` is passed to [`VBarChart::update_bars`](crate::ui::vbarchart::VBarChart::update_bars) or
/// [`HBarChart::update_bars`](crate::ui::hbarchart::HBarChart::update_bars) so several inserts, deletes,
/// and in-place edits can be applied before the chart relayouts and repaints once.
///
/// Custom label spans use bar indices. Inserting or deleting bars may require updating those spans afterwards.
pub struct Bars<'a, T>
where
    T: Number + 'static,
{
    pub(crate) inner: &'a mut Vec<BarWithLayout<T>>,
}
impl<'a, T> Bars<'a, T>
where
    T: Number + 'static,
{
    /// Returns the number of bars in the series.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Returns `true` if the series contains no bars.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Returns an immutable reference to the bar at `index`, or `None` if out of range.
    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&Bar<T>> {
        self.inner.get(index).map(|item| &item.bar)
    }
    /// Returns a mutable reference to the bar at `index`, or `None` if out of range.
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Bar<T>> {
        self.inner.get_mut(index).map(|item| &mut item.bar)
    }
    /// Appends a bar at the end of the series.
    #[inline(always)]
    pub fn add<B>(&mut self, bar: B)
    where
        B: Into<Bar<T>>,
    {
        self.inner.push(BarWithLayout::new(bar.into()));
    }
    /// Appends several bars at the end of the series.
    pub fn add_bars<B>(&mut self, bars: impl IntoIterator<Item = B>)
    where
        B: Into<Bar<T>>,
    {
        self.inner.extend(bars.into_iter().map(|bar| BarWithLayout::new(bar.into())));
    }
    /// Inserts a bar at `index`. Returns `false` if `index` is greater than [`len`](Self::len).
    pub fn insert<B>(&mut self, index: usize, bar: B) -> bool
    where
        B: Into<Bar<T>>,
    {
        if index > self.inner.len() {
            return false;
        }
        self.inner.insert(index, BarWithLayout::new(bar.into()));
        true
    }
    /// Removes the bar at `index` and returns it, or `None` if out of range.
    pub fn delete(&mut self, index: usize) -> Option<Bar<T>> {
        if index >= self.inner.len() {
            return None;
        }
        Some(self.inner.remove(index).bar)
    }
    /// Replaces the bar at `index` and returns the previous bar, or `None` if out of range.
    pub fn set<B>(&mut self, index: usize, bar: B) -> Option<Bar<T>>
    where
        B: Into<Bar<T>>,
    {
        let slot = self.inner.get_mut(index)?;
        Some(std::mem::replace(&mut slot.bar, bar.into()))
    }
    /// Removes all bars from the series.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    /// Iterates over the bars in the series.
    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &Bar<T>> {
        self.inner.iter().map(|item| &item.bar)
    }
    /// Iterates mutably over the bars in the series.
    #[inline(always)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Bar<T>> {
        self.inner.iter_mut().map(|item| &mut item.bar)
    }
}
/// How [`VBarChart`](crate::ui::vbarchart::VBarChart) and [`HBarChart`](crate::ui::hbarchart::HBarChart) map bar values onto the plot.
///
/// On a vertical chart the plot runs from bottom to top. On a horizontal chart it runs from left to right.
pub enum BarScale<T: Number + 'static> {
    /// Draws every bar from zero. The visible range includes zero and every bar value.
    FromZero,
    /// Draws every bar from zero, and expands the range so that it covers `min`, `max`, and every bar value.
    FromZeroMinRange {
        /// Lowest value that must remain inside the scale.
        min: T,
        /// Highest value that must remain inside the scale.
        max: T,
    },
    /// Stretches the smallest and largest bar values across the full plot.
    ///
    /// When every value is equal, each bar is drawn at half the plot size: half the height
    /// on a [`VBarChart`](crate::ui::vbarchart::VBarChart), half the length on an
    /// [`HBarChart`](crate::ui::hbarchart::HBarChart).
    FitData,
    /// Uses a fixed range. Values outside it are drawn at the corresponding edge of the plot.
    ///
    /// When `min` is not lower than `max`, every bar is drawn at half the plot size.
    Fixed {
        /// Lowest value in the range.
        ///
        /// This is the bottom of a [`VBarChart`](crate::ui::vbarchart::VBarChart) plot and the
        /// left end of an [`HBarChart`](crate::ui::hbarchart::HBarChart) plot.
        min: T,
        /// Highest value in the range.
        ///
        /// This is the top of a [`VBarChart`](crate::ui::vbarchart::VBarChart) plot and the
        /// right end of an [`HBarChart`](crate::ui::hbarchart::HBarChart) plot.
        max: T,
    },
}