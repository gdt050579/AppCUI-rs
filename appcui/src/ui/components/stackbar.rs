//! Types shared by [`VStackedBarChart`](crate::ui::vstackedbarchart::VStackedBarChart).
//!
//! A [`StackBar`] is one segment of a [`BarsStack`]. Positive segments stack upward from zero
//! and negative segments stack downward, in the order they were added. Build a segment with
//! [`StackBarBuilder`], or convert a number through [`From`]. A stack stores those segments,
//! an optional label, and optional thickness and spacing. Build one with [`BarsStackBuilder`],
//! or convert a sequence of numbers through [`From`].
//!
//! [`StackBarDrawMode`] chooses how a segment is painted. [`StackBarScale`] chooses how the
//! chart maps stack totals onto the plot.

use crate::graphics::{CharAttribute, Character, LineType, Rect, SpecialChar, Surface};
use crate::ui::common::Number;
use crate::ui::components::BarFillType;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// How a [`VStackedBarChart`](crate::ui::vstackedbarchart::VStackedBarChart) draws one segment of a stack.
///
/// A stack is as wide as the thickest requirement among its segments. A requested thickness
/// below that minimum is raised when the stack is laid out. [`Fill`](Self::Fill) with
/// [`BarFillType::Solid`] is the default.
pub enum StackBarDrawMode {
    /// Fills the segment with a repeated character. Thickness is at least 1.
    Fill(BarFillType),
    /// Draws an empty rectangle of the given [`LineType`]. Thickness is at least 2.
    Rectangle(LineType),
    /// Draws a rectangle of the given [`LineType`], filled with a solid block. Thickness is at least 3.
    FilledRectangle(LineType),
}

impl StackBarDrawMode {
    #[inline(always)]
    pub(crate) fn min_thickness(&self) -> u8 {
        match self {
            StackBarDrawMode::Fill(_) => 1,
            StackBarDrawMode::Rectangle(_) => 2,
            StackBarDrawMode::FilledRectangle(_) => 3,
        }
    }
}

impl Default for StackBarDrawMode {
    /// Returns [`StackBarDrawMode::Fill`] with [`BarFillType::Solid`].
    fn default() -> Self {
        StackBarDrawMode::Fill(BarFillType::Solid)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
/// How a [`VStackedBarChart`](crate::ui::vstackedbarchart::VStackedBarChart) maps stack totals onto the plot.
///
/// Positive segments grow upward from zero and negative segments grow downward. The plot runs
/// from bottom to top. A new chart uses [`FromZero`](Self::FromZero).
pub enum StackBarScale<T: Number + 'static> {
    /// Draws every stack from zero. The visible range includes zero and every stack's upward and downward totals.
    ///
    /// The upward total is the sum of a stack's positive segments. The downward total is the sum of its negative segments.
    FromZero,
    /// Draws every stack from zero, and expands the range so that it covers `min`, `max`, and every stack total.
    ///
    /// A non-finite `min` or `max` is treated as zero.
    FromZeroMinRange {
        /// Lowest value that must remain inside the scale.
        min: T,
        /// Highest value that must remain inside the scale.
        max: T,
    },
    /// Stretches the smallest and largest upward and downward stack totals across the full plot.
    ///
    /// The upward total is the sum of a stack's positive segments. The downward total is the sum
    /// of its negative segments. When those totals have no span, segments are not drawn.
    FitData,
    /// Uses a fixed range. Totals outside it are drawn at the corresponding edge of the plot.
    ///
    /// `min` is the bottom of the plot and `max` is the top. When `min` is not lower than `max`,
    /// or either bound is non-finite, segments are not drawn.
    Fixed {
        /// Lowest value in the range. This is the bottom of the plot.
        min: T,
        /// Highest value in the range. This is the top of the plot.
        max: T,
    },
    /// Scales each stack so the sum of the absolute values of its segments is 100%.
    ///
    /// The plot runs from the largest downward share to the largest upward share. A stack of
    /// only positive values fills from 0% to 100%. A stack with both signs splits that 100%
    /// above and below zero.
    Percentage,
}

#[derive(Clone, Debug)]
/// One segment in a [`BarsStack`].
///
/// A segment stores its value and label. Color and draw mode are optional: a field left unset
/// uses the chart default. Segments are stacked in the order they were added. Positive values
/// pile upward from zero, each one on top of the previous positive total. Negative values pile
/// downward the same way. A value of zero, or a non-finite value, occupies no cells. A value of
/// type `T` converts into a segment through [`From`].
///
/// The label is included in the hover tooltip when it is not empty.
pub struct StackBar<T: Number + 'static> {
    pub(crate) value: T,
    pub(crate) label: String,
    pub(crate) attr: Option<CharAttribute>,
    pub(crate) draw_mode: Option<StackBarDrawMode>,
}

impl<T: Number + 'static> StackBar<T> {
    /// Returns the numeric value of this segment.
    #[inline(always)]
    pub fn value(&self) -> T {
        self.value
    }
    /// Sets the numeric value of this segment.
    #[inline(always)]
    pub fn set_value(&mut self, value: T) -> &mut Self {
        self.value = value;
        self
    }
    /// Returns the label displayed for this segment.
    ///
    /// A [`VStackedBarChart`](crate::ui::vstackedbarchart::VStackedBarChart) includes it in the
    /// hover tooltip when it is not empty.
    #[inline(always)]
    pub fn label(&self) -> &str {
        &self.label
    }
    /// Sets the label displayed for this segment.
    #[inline(always)]
    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label.clear();
        self.label.push_str(label);
        self
    }
    /// Returns the character attribute of this segment, or `None` if the chart default is used.
    #[inline(always)]
    pub fn attr(&self) -> Option<CharAttribute> {
        self.attr
    }
    /// Sets the character attribute of this segment.
    #[inline(always)]
    pub fn set_attr(&mut self, attr: CharAttribute) -> &mut Self {
        self.attr = Some(attr);
        self
    }
    /// Clears the segment-specific attribute so the chart default is used.
    #[inline(always)]
    pub fn clear_attr(&mut self) -> &mut Self {
        self.attr = None;
        self
    }
    /// Returns the draw mode of this segment, or `None` if the chart default is used.
    #[inline(always)]
    pub fn draw_mode(&self) -> Option<StackBarDrawMode> {
        self.draw_mode
    }
    /// Sets the draw mode of this segment.
    ///
    /// The mode can raise the thickness of the whole stack when the stack is laid out.
    #[inline(always)]
    pub fn set_draw_mode(&mut self, draw_mode: StackBarDrawMode) -> &mut Self {
        self.draw_mode = Some(draw_mode);
        self
    }
    /// Clears the segment-specific draw mode so the chart default is used.
    #[inline(always)]
    pub fn clear_draw_mode(&mut self) -> &mut Self {
        self.draw_mode = None;
        self
    }
}

/// Builds a [`StackBar`] by setting only the properties that should differ from the chart defaults.
///
/// # Example
/// ```rust
/// use appcui::prelude::*;
///
/// let bar = StackBarBuilder::new(10)
///     .label("Jun")
///     .attr(CharAttribute::with_fore_color(Color::Yellow))
///     .draw_mode(StackBarDrawMode::Fill(BarFillType::Shade50))
///     .build();
/// assert_eq!(bar.value(), 10);
/// assert_eq!(bar.label(), "Jun");
/// assert!(bar.attr().is_some());
/// ```
pub struct StackBarBuilder<T: Number + 'static> {
    bar: StackBar<T>,
}

impl<T: Number + 'static> StackBarBuilder<T> {
    /// Starts a segment with `value`. Color, label, and draw mode stay unset.
    pub fn new(value: T) -> Self {
        Self {
            bar: StackBar {
                value,
                label: String::new(),
                attr: None,
                draw_mode: None,
            },
        }
    }
    /// Sets the label displayed for the segment.
    pub fn label(mut self, label: &str) -> Self {
        self.bar.label = label.to_string();
        self
    }
    /// Sets the character attribute of the segment.
    pub fn attr(mut self, attr: CharAttribute) -> Self {
        self.bar.attr = Some(attr);
        self
    }
    /// Sets the draw mode of the segment.
    ///
    /// The mode can raise the thickness of the whole stack when the stack is laid out.
    pub fn draw_mode(mut self, draw_mode: StackBarDrawMode) -> Self {
        self.bar.draw_mode = Some(draw_mode);
        self
    }
    /// Returns the finished segment.
    pub fn build(self) -> StackBar<T> {
        self.bar
    }
}

impl<T> From<T> for StackBar<T>
where
    T: Number + 'static,
{
    /// Creates a segment that stores `value`. Label, color, and draw mode stay unset.
    fn from(value: T) -> Self {
        StackBarBuilder::new(value).build()
    }
}

impl<T> From<&T> for StackBar<T>
where
    T: Number + 'static,
{
    /// Creates a segment from a borrowed value. Label, color, and draw mode stay unset.
    fn from(value: &T) -> Self {
        (*value).into()
    }
}

#[derive(Clone, Debug)]
/// One column of a [`VStackedBarChart`](crate::ui::vstackedbarchart::VStackedBarChart).
///
/// Positive [`StackBar`] values pile upward from zero and negative values pile downward.
/// Thickness and spacing are optional and apply to the whole column: a field left unset uses
/// the chart default. The thickness is shared by every segment and is raised when a segment's
/// draw mode needs more cells. A sequence of numbers converts into a stack through [`From`].
/// [`BarsStack::new`] and [`Default`] produce an empty stack.
///
/// The label is drawn under the column when the chart shows stack labels, and it is included
/// in the hover tooltip when it is not empty.
pub struct BarsStack<T: Number + 'static> {
    pub(crate) bars: Vec<StackBar<T>>,
    pub(crate) label: String,
    pub(crate) thickness: Option<u8>,
    pub(crate) spacing: Option<u8>,
    pub(crate) computed_thickness: u8,
}

impl<T: Number + 'static> BarsStack<T> {
    /// Creates an empty stack with no label and no thickness or spacing override.
    pub fn new() -> Self {
        Self {
            bars: Vec::new(),
            label: String::new(),
            thickness: None,
            spacing: None,
            computed_thickness: 0,
        }
    }
    /// Appends one segment and returns this stack.
    ///
    /// `bar` can be a [`StackBar`] or a number, which converts into a segment.
    pub fn add_bar<B>(&mut self, bar: B) -> &mut Self
    where
        B: Into<StackBar<T>>,
    {
        self.bars.push(bar.into());
        self
    }
    /// Appends every segment from `bars` and returns this stack.
    ///
    /// Each item can be a [`StackBar`] or a number, which converts into a segment.
    pub fn add_bars<B>(&mut self, bars: impl IntoIterator<Item = B>) -> &mut Self
    where
        B: Into<StackBar<T>>,
    {
        self.bars.extend(bars.into_iter().map(|bar| bar.into()));
        self
    }
    /// Returns the number of segments in this stack.
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.bars.len()
    }
    /// Returns the segment at `index`, or `None` when `index` is past the last segment.
    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&StackBar<T>> {
        self.bars.get(index)
    }
    /// Returns a mutable reference to the segment at `index`, or `None` when `index` is past the last segment.
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut StackBar<T>> {
        self.bars.get_mut(index)
    }
    /// Returns the label displayed for this stack.
    ///
    /// A [`VStackedBarChart`](crate::ui::vstackedbarchart::VStackedBarChart) can show it under
    /// the column. The hover tooltip includes it when it is not empty.
    #[inline(always)]
    pub fn label(&self) -> &str {
        &self.label
    }
    /// Sets the label displayed for this stack.
    #[inline(always)]
    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label.clear();
        self.label.push_str(label);
        self
    }
    /// Returns the thickness of this stack, in cells, or `None` if the chart default is used.
    #[inline(always)]
    pub fn thickness(&self) -> Option<u8> {
        self.thickness
    }
    /// Sets the thickness of this stack, in cells.
    ///
    /// Every segment uses this width. A value below 1 is raised to 1 when the stack is laid out.
    /// A draw mode may raise it further: [`StackBarDrawMode::Rectangle`] needs at least 2 cells
    /// and [`StackBarDrawMode::FilledRectangle`] needs at least 3. The widest requirement among
    /// the segments is used.
    #[inline(always)]
    pub fn set_thickness(&mut self, thickness: u8) -> &mut Self {
        self.thickness = Some(thickness);
        self
    }
    /// Clears the stack-specific thickness so the chart default is used.
    ///
    /// A draw mode may still raise that default when the stack is laid out.
    #[inline(always)]
    pub fn clear_thickness(&mut self) -> &mut Self {
        self.thickness = None;
        self
    }
    /// Returns the spacing before this stack, in cells, or `None` if the chart default is used.
    #[inline(always)]
    pub fn spacing(&self) -> Option<u8> {
        self.spacing
    }
    /// Sets the gap, in cells, before this stack.
    #[inline(always)]
    pub fn set_spacing(&mut self, spacing: u8) -> &mut Self {
        self.spacing = Some(spacing);
        self
    }
    /// Clears the stack-specific spacing so the chart default is used.
    #[inline(always)]
    pub fn clear_spacing(&mut self) -> &mut Self {
        self.spacing = None;
        self
    }
    pub(crate) fn update_actual_thickness(&mut self, default_thickness: u8, default_draw_mode: StackBarDrawMode) {
        let requested = self.thickness.unwrap_or(default_thickness).max(1);
        let required = self
            .bars
            .iter()
            .map(|bar| bar.draw_mode.unwrap_or(default_draw_mode).min_thickness())
            .max()
            .unwrap_or(1);
        self.computed_thickness = requested.max(required);
    }
    #[inline(always)]
    pub(crate) fn actual_thickness(&self) -> u8 {
        self.computed_thickness
    }
    #[inline(always)]
    pub(crate) fn actual_spacing(&self, default_spacing: u8) -> u8 {
        self.spacing.unwrap_or(default_spacing)
    }
}

impl<T: Number + 'static> Default for BarsStack<T> {
    /// Returns an empty stack. Equivalent to [`BarsStack::new`].
    fn default() -> Self {
        Self::new()
    }
}

/// Builds a [`BarsStack`] by setting only the properties that should differ from the chart defaults.
///
/// # Example
/// ```rust
/// use appcui::prelude::*;
///
/// let stack = BarsStackBuilder::<i32>::new()
///     .label("North")
///     .bar(10)
///     .bar(StackBarBuilder::new(-4).label("returns").build())
///     .thickness(3)
///     .spacing(1)
///     .build();
/// assert_eq!(stack.count(), 2);
/// assert_eq!(stack.label(), "North");
/// assert_eq!(stack.get(1).unwrap().value(), -4);
/// ```
pub struct BarsStackBuilder<T: Number + 'static> {
    stack: BarsStack<T>,
}

impl<T: Number + 'static> BarsStackBuilder<T> {
    /// Starts an empty stack. Label, thickness, and spacing stay unset.
    pub fn new() -> Self {
        Self { stack: BarsStack::new() }
    }
    /// Appends one segment.
    ///
    /// `bar` can be a [`StackBar`] or a number, which converts into a segment.
    pub fn bar<B>(mut self, bar: B) -> Self
    where
        B: Into<StackBar<T>>,
    {
        self.stack.bars.push(bar.into());
        self
    }
    /// Appends every segment from `bars`.
    ///
    /// Each item can be a [`StackBar`] or a number, which converts into a segment.
    pub fn bars<B>(mut self, bars: impl IntoIterator<Item = B>) -> Self
    where
        B: Into<StackBar<T>>,
    {
        self.stack.bars.extend(bars.into_iter().map(|bar| bar.into()));
        self
    }
    /// Sets the label displayed for the stack.
    pub fn label(mut self, label: &str) -> Self {
        self.stack.label = label.to_string();
        self
    }
    /// Sets the thickness of the stack, in cells.
    ///
    /// Every segment uses this width. A value below 1 is raised to 1 when the stack is laid out.
    /// A draw mode may raise it further: [`StackBarDrawMode::Rectangle`] needs at least 2 cells
    /// and [`StackBarDrawMode::FilledRectangle`] needs at least 3. The widest requirement among
    /// the segments is used.
    pub fn thickness(mut self, thickness: u8) -> Self {
        self.stack.thickness = Some(thickness);
        self
    }
    /// Sets the gap, in cells, before the stack.
    pub fn spacing(mut self, spacing: u8) -> Self {
        self.stack.spacing = Some(spacing);
        self
    }
    /// Returns the finished stack.
    pub fn build(self) -> BarsStack<T> {
        self.stack
    }
}

impl<T: Number + 'static> Default for BarsStackBuilder<T> {
    /// Returns an empty builder. Equivalent to [`BarsStackBuilder::new`].
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<&[T]> for BarsStack<T>
where
    T: Number + 'static,
{
    /// Creates a stack with one segment per value. Label, thickness, and spacing stay unset.
    fn from(values: &[T]) -> Self {
        let mut stack = BarsStack::new();
        stack.add_bars(values);
        stack
    }
}

impl<T> From<Vec<T>> for BarsStack<T>
where
    T: Number + 'static,
{
    /// Creates a stack with one segment per value. Label, thickness, and spacing stay unset.
    fn from(values: Vec<T>) -> Self {
        let mut stack = BarsStack::new();
        stack.add_bars(values);
        stack
    }
}

impl<T> From<&Vec<T>> for BarsStack<T>
where
    T: Number + 'static,
{
    /// Creates a stack with one segment per value. Label, thickness, and spacing stay unset.
    fn from(values: &Vec<T>) -> Self {
        values.as_slice().into()
    }
}

impl<T, const N: usize> From<[T; N]> for BarsStack<T>
where
    T: Number + 'static,
{
    /// Creates a stack with one segment per value. Label, thickness, and spacing stay unset.
    fn from(values: [T; N]) -> Self {
        let mut stack = BarsStack::new();
        stack.add_bars(values);
        stack
    }
}

impl<T, const N: usize> From<&[T; N]> for BarsStack<T>
where
    T: Number + 'static,
{
    /// Creates a stack with one segment per value. Label, thickness, and spacing stay unset.
    fn from(values: &[T; N]) -> Self {
        values.as_slice().into()
    }
}

impl<T> From<&BarsStack<T>> for BarsStack<T>
where
    T: Number + 'static,
{
    /// Creates a stack by cloning `stack`.
    fn from(stack: &BarsStack<T>) -> Self {
        stack.clone()
    }
}

pub(crate) fn segment_rows(y: i32, len: i16) -> Option<(i32, i32)> {
    if len > 0 {
        Some((y - len as i32 + 1, y))
    } else if len < 0 {
        Some((y + 1, y + len.unsigned_abs() as i32))
    } else {
        None
    }
}

#[derive(Copy, Clone, Default)]
pub(crate) struct SegmentLayout {
    pub(crate) offset: i16,
    pub(crate) len: i16,
}

pub(crate) struct StackWithLayout<T: Number + 'static> {
    pub(crate) stack: BarsStack<T>,
    pub(crate) pos: i32,
    pub(crate) positive_sum: f64,
    pub(crate) negative_sum: f64,
    pub(crate) layout: Vec<SegmentLayout>,
}

impl<T> StackWithLayout<T>
where
    T: Number + 'static,
{
    #[inline(always)]
    pub(crate) fn new(stack: BarsStack<T>) -> Self {
        Self {
            stack,
            pos: 0,
            positive_sum: 0.0,
            negative_sum: 0.0,
            layout: Vec::new(),
        }
    }
    pub(crate) fn update_sums(&mut self) {
        let mut positive = 0.0f64;
        let mut negative = 0.0f64;
        for bar in self.stack.bars.iter() {
            let value = bar.value.to_f64();
            if !value.is_finite() {
                continue;
            }
            if value > 0.0 {
                positive += value;
            } else if value < 0.0 {
                negative += value;
            }
        }
        self.positive_sum = positive;
        self.negative_sum = negative;
    }
    #[inline(always)]
    pub(crate) fn abs_total(&self) -> f64 {
        self.positive_sum - self.negative_sum
    }
    #[inline(always)]
    pub(crate) fn has_positive(&self) -> bool {
        self.positive_sum > 0.0
    }
    #[inline(always)]
    pub(crate) fn has_negative(&self) -> bool {
        self.negative_sum < 0.0
    }
    pub(crate) fn update_segments(&mut self, mapping: &ValueMapping, percentage: bool) {
        let total = self.abs_total();
        let to_domain = |cumulative: f64| -> f64 {
            if !percentage {
                cumulative
            } else if total > 0.0 {
                cumulative / total * 100.0
            } else {
                0.0
            }
        };
        let to_i16 = |value: i32| -> i16 { value.clamp(i16::MIN as i32, i16::MAX as i32) as i16 };
        self.layout.clear();
        let mut positive = 0.0f64;
        let mut negative = 0.0f64;
        for bar in self.stack.bars.iter() {
            let value = bar.value.to_f64();
            let segment = if !value.is_finite() || value == 0.0 {
                SegmentLayout::default()
            } else if value > 0.0 {
                let start = mapping.offset(to_domain(positive));
                positive += value;
                let end = mapping.offset(to_domain(positive));
                SegmentLayout {
                    offset: to_i16(start),
                    len: to_i16(end - start),
                }
            } else {
                let start = mapping.offset(to_domain(negative));
                negative += value;
                let end = mapping.offset(to_domain(negative));
                SegmentLayout {
                    offset: to_i16(start),
                    len: to_i16(end - start),
                }
            };
            self.layout.push(segment);
        }
    }
}

pub(crate) fn paint_segment(surface: &mut Surface, x: i32, y: i32, thickness: u8, len: i16, mode: StackBarDrawMode, attr: CharAttribute) {
    let height = len.unsigned_abs();
    if height == 0 || thickness == 0 {
        return;
    }
    let top = if len > 0 { y + 1 - len as i32 } else { y + 1 };
    let rect = Rect::with_size(x, top, thickness as u16, height);
    match mode {
        StackBarDrawMode::Fill(fill_type) => surface.fill_rect(rect, fill_type.character(attr)),
        StackBarDrawMode::Rectangle(line_type) => paint_rectangle(surface, rect, line_type, attr, false),
        StackBarDrawMode::FilledRectangle(line_type) => paint_rectangle(surface, rect, line_type, attr, true),
    }
}

fn paint_rectangle(surface: &mut Surface, rect: Rect, line_type: LineType, attr: CharAttribute, fill: bool) {
    if rect.bottom() <= rect.top() {
        surface.draw_horizontal_line(rect.left(), rect.top(), rect.right(), line_type, attr);
        return;
    }
    if fill {
        surface.fill_rect(rect, Character::with_attributes(SpecialChar::Block100, attr));
    }
    surface.draw_rect(rect, line_type, attr);
}

pub(crate) fn resolve_attr(bar: Option<CharAttribute>, default: Option<CharAttribute>, theme: CharAttribute) -> CharAttribute {
    bar.or(default).unwrap_or(theme)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ValueMapping {
    lo: f64,
    scale: f64,
    height: u32,
    zero: u32,
}

impl ValueMapping {
    pub(crate) fn new(lo: f64, hi: f64, height: u32) -> Self {
        let lo = if lo.is_finite() { lo } else { 0.0 };
        let scale = if height > 0 && hi.is_finite() && hi > lo && (hi - lo).is_finite() {
            height as f64 / (hi - lo)
        } else {
            0.0
        };
        let mut mapping = Self { lo, scale, height, zero: 0 };
        mapping.zero = mapping.position(0.0);
        mapping
    }
    #[inline(always)]
    #[cfg(test)]
    pub(crate) fn height(&self) -> u32 {
        self.height
    }
    #[inline(always)]
    pub(crate) fn zero(&self) -> u32 {
        self.zero
    }
    #[inline(always)]
    #[cfg(test)]
    pub(crate) fn above(&self) -> u32 {
        self.height - self.zero
    }
    #[inline(always)]
    #[cfg(test)]
    pub(crate) fn below(&self) -> u32 {
        self.zero
    }
    #[inline(always)]
    pub(crate) fn bottom_value(&self) -> f64 {
        self.lo
    }
    #[inline(always)]
    pub(crate) fn value_per_cell(&self) -> f64 {
        if self.scale > 0.0 {
            1.0 / self.scale
        } else {
            0.0
        }
    }
    pub(crate) fn position(&self, value: f64) -> u32 {
        let p = (value - self.lo) * self.scale;
        if p.is_nan() {
            return self.zero;
        }
        p.round().clamp(0.0, self.height as f64) as u32
    }
    pub(crate) fn offset(&self, value: f64) -> i32 {
        self.position(value) as i32 - self.zero as i32
    }
}
