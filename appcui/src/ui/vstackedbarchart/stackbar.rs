use crate::graphics::{CharAttribute, LineType};
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

#[derive(Copy, Clone, Default)]
pub(crate) struct SegmentLayout {
    pub(crate) offset: i16,
    pub(crate) len: i16,
}

pub(crate) struct StackWithLayout<T: Number + 'static> {
    pub(crate) stack: BarsStack<T>,
    pub(crate) pos: i32,
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
            layout: Vec::new(),
        }
    }
}
