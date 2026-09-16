use super::{bar::Bar, BarScale, Flags};
use crate::prelude::*;
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct BarLayour {
    x: i32,
    h: i16,
    digits: u8,
}
struct BarWithLayout<T: Number + 'static> {
    bar: Bar<T>,
    layout: BarLayour,
}
struct YAxis {
    width: u8,
    step: u8,
    zero: i32,
    percentage: bool,
}
struct XAxis {
    enabled: bool,
}

const FORMAT_FLOAT: FormatNumber = FormatNumber::new(10).decimals(2);
const FORMAT_PERCENTAGE: FormatNumber = FormatNumber::new(10).decimals(2).suffix("%");
const FORMAT_INTEGER: FormatNumber = FormatNumber::new(10).group(3, b',');

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
    yaxis: YAxis,
    xaxis: XAxis,
    scale: BarScale<T>,
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
            yaxis: YAxis {
                width: 6,
                step: 3,
                zero: 0,
                percentage: false,
            },
            xaxis: XAxis { enabled: true },
            scale: BarScale::Auto,
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
    pub fn set_bars_scale(&mut self, scale: BarScale<T>) {
        self.scale = scale;
        self.update_bars_layout();
    }
    fn update_bars_height(&mut self, min: f64, max: f64) {
        let height = self.size().height;
        if max > min {
            let dif = max - min;
            let visible_space = self.size().height.saturating_sub(2) as f64;
            for bar in self.bars.iter_mut() {
                let h = (bar.bar.value.to_f64() - min) / dif * visible_space;
                bar.layout.digits = (h.fract() * 100.0) as u8;
                bar.layout.h = h as i16;
            }
        } else {
        }
    }
    fn update_bars_height_from_zero(&mut self, min: f64, max: f64) {
        let lo = min.min(0.0);
        let hi = max.max(0.0);
        let total = hi - lo;
        if total > 0.0 {
            let height = self.size().height.saturating_sub(3) as f64;
            let cells_below = (-lo / total * height).round();
            let cells_above = height - cells_below;
            self.yaxis.zero = cells_below as i32;
            for bar in self.bars.iter_mut() {
                let v = bar.bar.value.to_f64();
                let h = if v >= 0.0 {
                    if hi > 0.0 {
                        v / hi * cells_above
                    } else {
                        0.0
                    }
                } else {
                    -(v / lo * cells_below)
                };
                bar.layout.digits = (h.abs().fract() * 100.0) as u8;
                bar.layout.h = h.round() as i16;
            }
        }
    }
    fn update_bars_layout(&mut self) {
        if self.bars.is_empty() {
            self.bars_width = 0;
            return;
        }
        let mut x = 0;
        let mut v_max = f64::MIN;
        let mut v_min = f64::MAX;

        for bar in self.bars.iter_mut() {
            x += bar.bar.spacing as i32;
            bar.layout.x = x;
            bar.layout.h = 0;
            x += bar.bar.thickness as i32;
            let value = bar.bar.value.to_f64();
            v_max = v_max.max(value);
            v_min = v_min.min(value);
        }
        self.bars_width = x as u32 + self.bars[0].bar.spacing as u32;
        match self.scale {
            BarScale::Auto => {
                let visible_space = self.size().height.saturating_sub(4) as f64;
                let dif = v_max - v_min;
                let car_scale = (dif / visible_space as f64).max(1.0);
                self.update_bars_height(v_min - car_scale, v_max + car_scale);
            }
            BarScale::FromZero => self.update_bars_height_from_zero(v_min, v_max),
            BarScale::FitData => {
                let visible_space = self.size().height.saturating_sub(4) as f64;
                let dif = v_max - v_min;
                let car_scale = (dif / visible_space as f64).max(1.0);
                self.update_bars_height(v_min - car_scale, v_max + car_scale);
            }
            BarScale::Fixed { min, max } => self.update_bars_height(min.to_f64(), max.to_f64()),
        }
    }
    fn paint_yaxis(&self, surface: &mut Surface, attr: CharAttribute) {
        let bottom = self.size().height as i32 - if self.xaxis.enabled { 2 } else { 1 };
        if self.yaxis.width > 0 {
            surface.draw_vertical_line(self.yaxis.width as i32 + 1, 0, bottom, LineType::Single, attr);
            if self.xaxis.enabled {
                surface.write_char(
                    self.yaxis.width as i32 + 1,
                    bottom,
                    Character::with_attributes(SpecialChar::BoxBottomLeftCornerSingleLine, attr),
                );
            }
        }
        if self.yaxis.step > 0 {
            let x_poz = if self.yaxis.width > 0 { self.yaxis.width as i32 + 2 } else { 0 };
            let right = self.size().width as i32;
            let ch = Character::with_attributes('┈', attr);
            let format = &FORMAT_FLOAT;
            let mut buffer: [u8; 32] = [0u8; 32];
            let mut y = bottom;
            while y >= 0 {
                surface.fill_horizontal_line(x_poz, y, right, ch);
                if let Some(value) = format.write_float(y as f64, &mut buffer) {
                    surface.write_ascii(x_poz - value.len() as i32 - 1, y, value.as_bytes(), attr, false);
                }
                y -= self.yaxis.step as i32;
            }
        }
    }
    fn paint_xaxis(&self, surface: &mut Surface, attr: CharAttribute) {
        if !self.xaxis.enabled {
            return;
        }
        let left = if self.yaxis.width > 0 { self.yaxis.width as i32 + 2 } else { 0 };
        let right = self.size().width as i32;
        let y = self.size().height as i32 - 2;
        surface.draw_horizontal_line(left, y, right, LineType::Single, attr);
    }
    fn paint_axis(&self, surface: &mut Surface, attr: CharAttribute) {
        // mereu in ordinea asta - Y, X (X va suprascrie o parte de la Y)
        self.paint_yaxis(surface, attr);
        self.paint_xaxis(surface, attr);
    }
}

impl<T> OnPaint for VBarChart<T>
where
    T: Number + 'static,
{
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(char!("' ',white,black"));
        self.paint_axis(surface, charattr!("gray,black"));
        let len = self.bars.len();
        let mut start = self.first_visible_bar as usize;
        let width = self.size().width as i32;
        let plot_bottom = self.size().height as i32 - if self.xaxis.enabled { 3 } else { 1 };
        let y = plot_bottom - self.yaxis.zero;
        let left_margin = if self.yaxis.width > 0 { self.yaxis.width as i32 + 2 } else { 0 };
        surface.set_relative_clip(left_margin, 0, width, plot_bottom);
        while start < len {
            let x = self.bars[start].layout.x - self.left_scroll + left_margin;
            if x >= width {
                break;
            }
            let bar = &self.bars[start];
            let h = bar.layout.h;
            let d = bar.layout.digits;
            bar.bar.paint_vertical(surface, charattr!("red"), x, y, h, d);
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
