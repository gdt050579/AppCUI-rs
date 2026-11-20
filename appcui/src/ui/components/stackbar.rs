use crate::graphics::{CharAttribute, Character, LineType, Rect, SpecialChar, Surface};
use crate::ui::common::Number;
use crate::ui::components::BarFillType;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StackBarDrawMode {
    Fill(BarFillType),
    Rectangle(LineType),
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
    fn default() -> Self {
        StackBarDrawMode::Fill(BarFillType::Solid)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StackBarScale<T: Number + 'static> {
    FromZero,
    FromZeroMinRange { min: T, max: T },
    FitData,
    Fixed { min: T, max: T },
    Percentage,
}

#[derive(Clone, Debug)]
pub struct StackBar<T: Number + 'static> {
    pub(crate) value: T,
    pub(crate) label: String,
    pub(crate) attr: Option<CharAttribute>,
    pub(crate) draw_mode: Option<StackBarDrawMode>,
}

impl<T: Number + 'static> StackBar<T> {
    #[inline(always)]
    pub fn value(&self) -> T {
        self.value
    }
    #[inline(always)]
    pub fn set_value(&mut self, value: T) -> &mut Self {
        self.value = value;
        self
    }
    #[inline(always)]
    pub fn label(&self) -> &str {
        &self.label
    }
    #[inline(always)]
    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label.clear();
        self.label.push_str(label);
        self
    }
    #[inline(always)]
    pub fn attr(&self) -> Option<CharAttribute> {
        self.attr
    }
    #[inline(always)]
    pub fn set_attr(&mut self, attr: CharAttribute) -> &mut Self {
        self.attr = Some(attr);
        self
    }
    #[inline(always)]
    pub fn clear_attr(&mut self) -> &mut Self {
        self.attr = None;
        self
    }
    #[inline(always)]
    pub fn draw_mode(&self) -> Option<StackBarDrawMode> {
        self.draw_mode
    }
    #[inline(always)]
    pub fn set_draw_mode(&mut self, draw_mode: StackBarDrawMode) -> &mut Self {
        self.draw_mode = Some(draw_mode);
        self
    }
    #[inline(always)]
    pub fn clear_draw_mode(&mut self) -> &mut Self {
        self.draw_mode = None;
        self
    }
}

pub struct StackBarBuilder<T: Number + 'static> {
    bar: StackBar<T>,
}

impl<T: Number + 'static> StackBarBuilder<T> {
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
    pub fn label(mut self, label: &str) -> Self {
        self.bar.label = label.to_string();
        self
    }
    pub fn attr(mut self, attr: CharAttribute) -> Self {
        self.bar.attr = Some(attr);
        self
    }
    pub fn draw_mode(mut self, draw_mode: StackBarDrawMode) -> Self {
        self.bar.draw_mode = Some(draw_mode);
        self
    }
    pub fn build(self) -> StackBar<T> {
        self.bar
    }
}

impl<T> From<T> for StackBar<T>
where
    T: Number + 'static,
{
    fn from(value: T) -> Self {
        StackBarBuilder::new(value).build()
    }
}

impl<T> From<&T> for StackBar<T>
where
    T: Number + 'static,
{
    fn from(value: &T) -> Self {
        (*value).into()
    }
}

#[derive(Clone, Debug)]
pub struct BarsStack<T: Number + 'static> {
    pub(crate) bars: Vec<StackBar<T>>,
    pub(crate) label: String,
    pub(crate) thickness: Option<u8>,
    pub(crate) spacing: Option<u8>,
    pub(crate) computed_thickness: u8,
}

impl<T: Number + 'static> BarsStack<T> {
    pub fn new() -> Self {
        Self {
            bars: Vec::new(),
            label: String::new(),
            thickness: None,
            spacing: None,
            computed_thickness: 0,
        }
    }
    pub fn add_bar<B>(&mut self, bar: B) -> &mut Self
    where
        B: Into<StackBar<T>>,
    {
        self.bars.push(bar.into());
        self
    }
    pub fn add_bars<B>(&mut self, bars: impl IntoIterator<Item = B>) -> &mut Self
    where
        B: Into<StackBar<T>>,
    {
        self.bars.extend(bars.into_iter().map(|bar| bar.into()));
        self
    }
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.bars.len()
    }
    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&StackBar<T>> {
        self.bars.get(index)
    }
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut StackBar<T>> {
        self.bars.get_mut(index)
    }
    #[inline(always)]
    pub fn label(&self) -> &str {
        &self.label
    }
    #[inline(always)]
    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label.clear();
        self.label.push_str(label);
        self
    }
    #[inline(always)]
    pub fn thickness(&self) -> Option<u8> {
        self.thickness
    }
    #[inline(always)]
    pub fn set_thickness(&mut self, thickness: u8) -> &mut Self {
        self.thickness = Some(thickness);
        self
    }
    #[inline(always)]
    pub fn clear_thickness(&mut self) -> &mut Self {
        self.thickness = None;
        self
    }
    #[inline(always)]
    pub fn spacing(&self) -> Option<u8> {
        self.spacing
    }
    #[inline(always)]
    pub fn set_spacing(&mut self, spacing: u8) -> &mut Self {
        self.spacing = Some(spacing);
        self
    }
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
    fn default() -> Self {
        Self::new()
    }
}

pub struct BarsStackBuilder<T: Number + 'static> {
    stack: BarsStack<T>,
}

impl<T: Number + 'static> BarsStackBuilder<T> {
    pub fn new() -> Self {
        Self { stack: BarsStack::new() }
    }
    pub fn bar<B>(mut self, bar: B) -> Self
    where
        B: Into<StackBar<T>>,
    {
        self.stack.bars.push(bar.into());
        self
    }
    pub fn bars<B>(mut self, bars: impl IntoIterator<Item = B>) -> Self
    where
        B: Into<StackBar<T>>,
    {
        self.stack.bars.extend(bars.into_iter().map(|bar| bar.into()));
        self
    }
    pub fn label(mut self, label: &str) -> Self {
        self.stack.label = label.to_string();
        self
    }
    pub fn thickness(mut self, thickness: u8) -> Self {
        self.stack.thickness = Some(thickness);
        self
    }
    pub fn spacing(mut self, spacing: u8) -> Self {
        self.stack.spacing = Some(spacing);
        self
    }
    pub fn build(self) -> BarsStack<T> {
        self.stack
    }
}

impl<T: Number + 'static> Default for BarsStackBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<&[T]> for BarsStack<T>
where
    T: Number + 'static,
{
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
    fn from(values: &Vec<T>) -> Self {
        values.as_slice().into()
    }
}

impl<T, const N: usize> From<[T; N]> for BarsStack<T>
where
    T: Number + 'static,
{
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
    fn from(values: &[T; N]) -> Self {
        values.as_slice().into()
    }
}

impl<T> From<&BarsStack<T>> for BarsStack<T>
where
    T: Number + 'static,
{
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
