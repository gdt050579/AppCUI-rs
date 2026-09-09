use super::{Flags, bar::Bar};
use crate::prelude::*;
use std::marker::PhantomData;

struct BarWithLayout<T: Number + 'static> {
    bar: Bar<T>,
    x: i32,
    y: i32,
    h: u32,
}


#[CustomControl(overwrite=OnPaint, internal=true)]
/// A vertical bar chart for a numeric series of type `T`.
///
/// `VBarChart` displays one vertical bar per value. Visual options are controlled
/// by [`Flags`].
pub struct VBarChart<T>
where
    T: Number + 'static,
{
    flags: Flags,
    _phantom: PhantomData<T>,
    bars: Vec<BarWithLayout<T>>,
}

impl<T> VBarChart<T>
where
    T: Number + 'static,
{
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled),
            flags,
            _phantom: PhantomData,
            bars: Vec::new(),
        }
    }
    pub fn add_bar<B>(&mut self, bar: B) where B: Into<Bar<T>> {
        self.bars.push(BarWithLayout {
            bar: bar.into(),
            x: 0,
            y: 0,
            h: 0,
        });
        self.update_bars_layout();
    }
    pub fn add_bars<B>(&mut self, bars: impl IntoIterator<Item=B>) where B: Into<Bar<T>> {
        self.bars.extend(bars.into_iter().map(|bar| BarWithLayout {
            bar: bar.into(),
            x: 0,
            y: 0,
            h: 0,
        }));
        self.update_bars_layout();
    }
    fn update_bars_layout(&mut self) {
        let mut x = 0;
        let mut v_max = f64::MIN;
        let mut v_min = f64::MAX;
        for bar in self.bars.iter_mut() {
            x += bar.bar.spacing as i32;
            bar.x = x;
            x += bar.bar.thickness as i32;
            let value = bar.bar.value.to_f64();
            v_max = v_max.max(value);
            v_min = v_min.min(value);
        }
    }
}

impl<T> OnPaint for VBarChart<T>
where
    T: Number + 'static,
{
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}
