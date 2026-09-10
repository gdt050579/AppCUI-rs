use super::{bar::Bar, Flags};
use crate::prelude::*;
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct BarLayour {
    x: i32,
    h: u16,
    digits: u8,
}
struct BarWithLayout<T: Number + 'static> {
    bar: Bar<T>,
    layout: BarLayour,
}

#[CustomControl(overwrite=OnPaint+OnResize, internal=true)]
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
    bars_width: u32,
    left_scroll: i32,
    first_visible_bar: u32,
    left_margin: u32,
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
            bars_width: 0,
            left_scroll: 0,
            first_visible_bar: 0,
            left_margin: 0,
        }
    }
    pub fn add_bar<B>(&mut self, bar: B)
    where
        B: Into<Bar<T>>,
    {
        self.bars.push(BarWithLayout {
            bar: bar.into(),
            layout: BarLayour { x: 0, h: 0, digits: 0 },
        });
        self.update_bars_layout();
    }
    pub fn add_bars<B>(&mut self, bars: impl IntoIterator<Item = B>)
    where
        B: Into<Bar<T>>,
    {
        self.bars.extend(bars.into_iter().map(|bar| BarWithLayout {
            bar: bar.into(),
            layout: BarLayour { x: 0, h: 0, digits: 0 },
        }));
        self.update_bars_layout();
    }
    fn update_bars_layout(&mut self) {
        if self.bars.is_empty() {
            self.bars_width = 0;
            return;
        }
        let mut x = 0;
        let mut v_max = f64::MIN;
        let mut v_min = f64::MAX;
        let height = self.size().height as i32;
        for bar in self.bars.iter_mut() {
            x += bar.bar.spacing as i32;
            bar.layout.x = x;
            bar.layout.h = 0;
            x += bar.bar.thickness as i32;
            let value = bar.bar.value.to_f64();
            v_max = v_max.max(value);
            v_min = v_min.min(value);
        }
        if v_max > v_min {
            let dif = v_max - v_min;
            let visible_space = self.size().height.saturating_sub(2) as f64;
            for bar in self.bars.iter_mut() {
                let h = (bar.bar.value.to_f64() - v_min) / dif * visible_space;
                bar.layout.digits = (h.fract() * 100.0) as u8;
                bar.layout.h = h as u16;
            }
        }
        self.bars_width = x as u32 + self.bars[0].bar.spacing as u32;
    }
}

impl<T> OnPaint for VBarChart<T>
where
    T: Number + 'static,
{
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(char!("' ',white,black"));
        let len = self.bars.len();
        let mut start = self.first_visible_bar as usize;
        let width = self.size().width as i32;
        let y = self.size().height as i32 - 2;
        while start < len {
            let x = self.bars[start].layout.x - self.left_scroll + self.left_margin as i32;
            if x >= width { break; }
            let bar = &self.bars[start];
            let h = bar.layout.h;
            let d = bar.layout.digits;
            bar.bar.paint_vertical(surface, charattr!("white,red"), x, y, h, d);
            start += 1;
        }
    }
}

impl<T> OnResize for VBarChart<T>
where
    T: Number + 'static,
{
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        self.update_bars_layout();
    }
}
