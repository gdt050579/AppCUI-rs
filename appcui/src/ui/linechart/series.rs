use super::LineMode;
use crate::graphics::CharAttribute;
use crate::ui::common::Number;
use crate::ui::components::{Bar, BarWithLayout};

/// One series of a [`struct@super::LineChart`]: a label, an optional line style and color,
/// and a list of points.
///
/// Every point is a [`Bar`], so it can carry its own color and marker
/// ([`BarDrawMode::Point`](crate::ui::components::BarDrawMode::Point) or
/// [`BarDrawMode::LargePoint`](crate::ui::components::BarDrawMode::LargePoint)).
/// The label of a point is a note about that value only (for example `"peak"`): it is shown in
/// the tooltip of the point. The names of the X positions are set on the chart instead, with
/// [`LineChart::set_xaxis_labels`](super::LineChart::set_xaxis_labels).
/// A point's thickness and spacing are ignored: all series share the same columns,
/// so the column width comes from the markers and the spacing from the chart default.
///
/// The color of the series is used for its line, and for every point that has no color of its own.
/// When the series has no color, a theme color is chosen from the series index.
///
/// # Example
/// ```rust
/// use appcui::prelude::*;
///
/// let mut s = linechart::Series::with_values("CPU", [10, 14, 9]);
/// s.add(22);
/// s.set_line(Some(linechart::LineMode::Braille));
/// assert_eq!(s.len(), 4);
/// assert_eq!(s.get(3).map(|b| b.value()), Some(22));
/// ```
pub struct Series<T>
where
    T: Number + 'static,
{
    pub(super) data: Vec<BarWithLayout<T>>,
    label: String,
    line: Option<LineMode>,
    attr: Option<CharAttribute>,
}

impl<T> Series<T>
where
    T: Number + 'static,
{
    /// Creates an empty series with the given label. Line style and color use the chart defaults.
    pub fn new(label: &str) -> Self {
        Self {
            data: Vec::new(),
            label: label.to_string(),
            line: None,
            attr: None,
        }
    }

    /// Creates a series with the given label and points.
    ///
    /// Each item may be a value of type `T` or anything that converts into a [`Bar`].
    pub fn with_values<B>(label: &str, values: impl IntoIterator<Item = B>) -> Self
    where
        B: Into<Bar<T>>,
    {
        let mut s = Self::new(label);
        s.add_values(values);
        s
    }

    /// Returns the label of the series (shown in tooltips).
    #[inline(always)]
    pub fn label(&self) -> &str {
        &self.label
    }
    /// Sets the label of the series.
    #[inline(always)]
    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label.clear();
        self.label.push_str(label);
        self
    }

    /// Returns how the points of this series are connected, or `None` if the chart default is used.
    #[inline(always)]
    pub fn line(&self) -> Option<LineMode> {
        self.line
    }
    /// Sets how the points of this series are connected. `None` uses the chart default.
    #[inline(always)]
    pub fn set_line(&mut self, line: Option<LineMode>) -> &mut Self {
        self.line = line;
        self
    }

    /// Returns the color of the series, or `None` if a theme color is used.
    #[inline(always)]
    pub fn attr(&self) -> Option<CharAttribute> {
        self.attr
    }
    /// Sets the color of the series. `None` uses a theme color chosen from the series index.
    #[inline(always)]
    pub fn set_attr(&mut self, attr: Option<CharAttribute>) -> &mut Self {
        self.attr = attr;
        self
    }

    /// Returns the number of points in the series.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Returns `true` if the series has no points.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Returns the point at `index`, or `None` if out of range.
    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&Bar<T>> {
        self.data.get(index).map(|item| &item.bar)
    }
    /// Returns a mutable reference to the point at `index`, or `None` if out of range.
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Bar<T>> {
        self.data.get_mut(index).map(|item| &mut item.bar)
    }
    /// Iterates over the points of the series.
    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &Bar<T>> {
        self.data.iter().map(|item| &item.bar)
    }
    /// Iterates mutably over the points of the series.
    #[inline(always)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Bar<T>> {
        self.data.iter_mut().map(|item| &mut item.bar)
    }

    /// Appends one point at the end of the series.
    #[inline(always)]
    pub fn add<B>(&mut self, value: B)
    where
        B: Into<Bar<T>>,
    {
        self.data.push(BarWithLayout::new(value.into()));
    }
    /// Appends several points at the end of the series.
    pub fn add_values<B>(&mut self, values: impl IntoIterator<Item = B>)
    where
        B: Into<Bar<T>>,
    {
        self.data.extend(values.into_iter().map(|v| BarWithLayout::new(v.into())));
    }
    /// Inserts a point at `index`. Returns `false` if `index` is greater than [`len`](Self::len).
    pub fn insert<B>(&mut self, index: usize, value: B) -> bool
    where
        B: Into<Bar<T>>,
    {
        if index > self.data.len() {
            return false;
        }
        self.data.insert(index, BarWithLayout::new(value.into()));
        true
    }
    /// Replaces the point at `index` and returns the previous one, or `None` if out of range.
    pub fn set<B>(&mut self, index: usize, value: B) -> Option<Bar<T>>
    where
        B: Into<Bar<T>>,
    {
        let slot = self.data.get_mut(index)?;
        Some(std::mem::replace(&mut slot.bar, value.into()))
    }
    /// Removes the point at `index` and returns it, or `None` if out of range.
    pub fn delete(&mut self, index: usize) -> Option<Bar<T>> {
        if index >= self.data.len() {
            return None;
        }
        Some(self.data.remove(index).bar)
    }
    /// Removes the first `count` points (or all of them if `count` is larger than the series).
    ///
    /// Useful for a rolling window: add new samples at the end and drop the oldest ones.
    pub fn remove_first(&mut self, count: usize) {
        let count = count.min(self.data.len());
        self.data.drain(..count);
    }
    /// Removes all points from the series. The label, line style, and color are kept.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

/// Builds a [`Series`] in one expression.
///
/// # Example
/// ```rust
/// use appcui::prelude::*;
///
/// let s = linechart::SeriesBuilder::new("RAM")
///     .line(linechart::LineMode::Line(LineType::Double))
///     .attr(charattr!("green"))
///     .values([40, 42, 41])
///     .build();
/// assert_eq!(s.label(), "RAM");
/// assert_eq!(s.len(), 3);
/// ```
pub struct SeriesBuilder<T>
where
    T: Number + 'static,
{
    series: Series<T>,
}

impl<T> SeriesBuilder<T>
where
    T: Number + 'static,
{
    /// Starts a series with the given label. Line style and color stay unset (chart defaults).
    pub fn new(label: &str) -> Self {
        Self { series: Series::new(label) }
    }
    /// Sets how the points of the series are connected.
    pub fn line(mut self, line: LineMode) -> Self {
        self.series.line = Some(line);
        self
    }
    /// Sets the color of the series.
    pub fn attr(mut self, attr: CharAttribute) -> Self {
        self.series.attr = Some(attr);
        self
    }
    /// Appends points to the series.
    pub fn values<B>(mut self, values: impl IntoIterator<Item = B>) -> Self
    where
        B: Into<Bar<T>>,
    {
        self.series.add_values(values);
        self
    }
    /// Returns the finished series.
    pub fn build(self) -> Series<T> {
        self.series
    }
}

impl<T> From<SeriesBuilder<T>> for Series<T>
where
    T: Number + 'static,
{
    fn from(builder: SeriesBuilder<T>) -> Self {
        builder.build()
    }
}
