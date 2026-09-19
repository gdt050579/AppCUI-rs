use flat_string::FlatString;

use super::{
    bar::{Bar, BarDefaults, BarDrawMode, BarLayout},
    BarScale, BarSpan, Flags, XAxisLabelFormat,
};
use crate::{prelude::*, ui::vbarchart::XAxisLabelMode};

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
    bottom_value: f64,
    bottom_step: f64,
}
struct XAxis {
    label_format: XAxisLabelFormat,
    spans: Vec<BarSpan>,
}

const INT_FORMAT: FormatNumber = FormatNumber::new(10).group(3, b',');

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
    bars: Vec<BarWithLayout<T>>,
    bars_width: u32,
    left_scroll: i32,
    first_visible_bar: u32,
    yaxis: YAxis,
    scale: BarScale<T>,
    number_format: FormatNumber,
    defaults: BarDefaults,
    surface: Surface,
    use_theme_colors_for_bars: bool,
    xaxis: XAxis,
}

impl<T> VBarChart<T>
where
    T: Number + 'static,
{
    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled),
            flags,
            bars: Vec::new(),
            bars_width: 0,
            left_scroll: 0,
            first_visible_bar: 0,
            yaxis: YAxis {
                width: 6,
                step: 3,
                zero: 0,
                bottom_value: 0.0,
                bottom_step: 0.0,
            },
            defaults: BarDefaults {
                attr: CharAttribute::default(),
                thickness: 1,
                spacing: 1,
                draw_mode: BarDrawMode::Normal,
            },
            xaxis: XAxis {
                label_format: XAxisLabelFormat::None,
                spans: Vec::new(),
            },
            scale: BarScale::FromZero,
            number_format: if T::is_float() {
                FormatNumber::new(10).decimals(2)
            } else {
                FormatNumber::new(10).group(3, b',')
            },
            surface: Surface::new(1, 1),
            use_theme_colors_for_bars: true,
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
        self.repaint_surface();
    }
    pub fn add_bars<B>(&mut self, bars: impl IntoIterator<Item = B>)
    where
        B: Into<Bar<T>>,
    {
        self.bars.extend(bars.into_iter().map(|bar| BarWithLayout {
            bar: bar.into(),
            layout: BarLayour { x: 0, h: 0, digits: 0 },
        }));
        self.repaint_surface();
    }
    pub fn set_bars_scale(&mut self, scale: BarScale<T>) {
        self.scale = scale;
        self.repaint_surface();
    }
    pub fn set_number_format(&mut self, format: FormatNumber) {
        self.number_format = format;
    }
    pub fn set_default_bar_width(&mut self, width: u8) {
        self.defaults.thickness = width.max(1);
        self.repaint_surface();
    }
    pub fn set_default_bar_spacing(&mut self, spacing: u8) {
        self.defaults.spacing = spacing;
        self.repaint_surface();
    }
    pub fn set_default_bar_drawmode(&mut self, draw_mode: BarDrawMode, attr: CharAttribute) {
        self.defaults.draw_mode = draw_mode;
        self.defaults.attr = attr;
        self.use_theme_colors_for_bars = false;
        self.repaint_surface();
    }
    pub fn set_xaxis_label_mode(&mut self, xaxis: XAxisLabelMode) {
        match xaxis {
            XAxisLabelMode::None => self.xaxis.label_format = XAxisLabelFormat::None,
            XAxisLabelMode::Index { start } => self.xaxis.label_format = XAxisLabelFormat::Index { start },
            XAxisLabelMode::BarLabels => self.xaxis.label_format = XAxisLabelFormat::BarLabels,
            XAxisLabelMode::Custom(spans) => {
                self.xaxis.label_format = XAxisLabelFormat::Custom;
                self.xaxis.spans.clear();
                self.xaxis.spans.extend(spans);
                self.xaxis.spans.sort_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)));
                let spans = &mut self.xaxis.spans;
                let mut write = 0usize;
                for read in 0..spans.len() {
                    let overlaps = write > 0 && spans[read].start <= spans[write - 1].end;
                    if !overlaps {
                        if read != write {
                            spans.swap(read, write);
                        }
                        write += 1;
                    }
                }
                spans.truncate(write);
            }
        }
        self.repaint_surface();
    }
    #[inline(always)]
    fn visible_height(&self) -> u32 {
        // one space from the top and the size of the height
        self.size().height.saturating_sub(self.xaxis.label_format.height() as u32 + 1)
    }
    #[inline(always)]
    fn x_axis_left_margin(&self) -> i32 {
        if self.yaxis.width > 0 {
            self.yaxis.width as i32 + 2
        } else {
            0
        }
    }
    fn update_yaxis_scale(&mut self, bottom_value: f64, top_value: f64) {
        let height = self.visible_height() as f64;
        self.yaxis.bottom_value = bottom_value;
        self.yaxis.bottom_step = if height > 0.0 {
            (bottom_value - top_value) / height * (self.yaxis.step as f64)
        } else {
            0.0
        };
    }
    fn update_bars_height_from_zero(&mut self, min: f64, max: f64) {
        let lo = min.min(0.0);
        let hi = max.max(0.0);
        let total = hi - lo;
        self.update_yaxis_scale(lo, hi);
        if total > 0.0 {
            let height = self.visible_height() as f64;
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
    fn update_bars_height_fit_data(&mut self, min: f64, max: f64) {
        let height = self.visible_height() as f64;
        let range = max - min;
        self.yaxis.zero = 0;
        self.update_yaxis_scale(min, max);

        if range > 0.0 {
            for bar in self.bars.iter_mut() {
                let v = bar.bar.value.to_f64();
                let h = (v - min) / range * height;
                bar.layout.digits = (h.fract() * 100.0) as u8;
                bar.layout.h = h.trunc() as i16;
            }
        } else {
            let uniform = (height * 0.5).trunc() as i16;
            for bar in self.bars.iter_mut() {
                bar.layout.digits = 0;
                bar.layout.h = uniform;
            }
        }
    }
    fn update_bars_height_fixed(&mut self, min: f64, max: f64) {
        let height = self.visible_height() as f64;
        let total = max - min;
        self.update_yaxis_scale(min, max);

        if total <= 0.0 {
            self.yaxis.zero = 0;
            let uniform = (height * 0.5).trunc() as i16;
            for bar in self.bars.iter_mut() {
                bar.layout.h = uniform;
                bar.layout.digits = 0;
            }
            return;
        }

        let zero_cell = (0.0 - min) / total * height;
        let zero_vis = zero_cell.clamp(0.0, height); // visible baseline row
        self.yaxis.zero = zero_vis.round() as i32;

        for bar in self.bars.iter_mut() {
            let v = bar.bar.value.to_f64();
            let tip = ((v - min) / total * height).clamp(0.0, height);
            let h = tip - zero_vis;
            bar.layout.digits = (h.abs().fract() * 100.0) as u8;
            bar.layout.h = h.trunc() as i16;
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
            x += bar.bar.spacing.unwrap_or(self.defaults.spacing) as i32;
            bar.layout.x = x;
            bar.layout.h = 0;
            x += bar.bar.actual_thickness(&self.defaults) as i32;
            let value = bar.bar.value.to_f64();
            v_max = v_max.max(value);
            v_min = v_min.min(value);
        }
        self.bars_width = x as u32 + self.bars[0].bar.spacing.unwrap_or(self.defaults.spacing) as u32;
        match self.scale {
            BarScale::FromZero => self.update_bars_height_from_zero(v_min, v_max),
            BarScale::FromZeroMinRange { min, max } => self.update_bars_height_from_zero(min.to_f64().min(v_min), max.to_f64().max(v_max)),
            BarScale::FitData => self.update_bars_height_fit_data(v_min, v_max),
            BarScale::Fixed { min, max } => self.update_bars_height_fixed(min.to_f64(), max.to_f64()),
        }
    }
    fn repaint_surface(&mut self) {
        self.update_bars_layout();
        self.surface.reset_clip();
        self.surface.clear(char!("' ',white,black"));
        // mereu in ordinea asta - Y, X (X va suprascrie o parte de la Y)
        self.paint_yaxis(charattr!("gray,black"));
        self.paint_xaxis(charattr!("gray,black"));
        let len = self.bars.len();
        let mut start = self.first_visible_bar as usize;
        let width = self.size().width as i32;
        let plot_bottom = self.size().height as i32 - (self.xaxis.label_format.height() as i32 + 1);
        let left_margin = self.x_axis_left_margin();
        let mut layout = BarLayout::default();
        layout.y = plot_bottom - self.yaxis.zero;
        let mut defaults = self.defaults;
        if self.use_theme_colors_for_bars {
            defaults.attr = charattr!("red");
        }
        self.surface.set_relative_clip(left_margin, 0, width, plot_bottom);
        while start < len {
            layout.x = self.bars[start].layout.x - self.left_scroll + left_margin;
            if layout.x >= width {
                break;
            }
            let bar = &self.bars[start];
            layout.length = bar.layout.h;
            layout.digits = bar.layout.digits;
            bar.bar.paint_vertical(&mut self.surface, &layout, &defaults);
            start += 1;
        }
    }
    fn paint_yaxis(&mut self, attr: CharAttribute) {
        let bottom = self.size().height as i32 - if self.xaxis.label_format.is_none() { 1 } else { 2 };
        if self.yaxis.width > 0 {
            self.surface
                .draw_vertical_line(self.yaxis.width as i32 + 1, 0, bottom, LineType::Single, attr);
            if !self.xaxis.label_format.is_none() {
                self.surface.write_char(
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
            let format = &self.number_format;
            let mut buffer: [u8; 32] = [0u8; 32];
            let mut y = bottom;
            let mut bottom_value = self.yaxis.bottom_value;
            while y >= 0 {
                self.surface.fill_horizontal_line(x_poz, y, right, ch);
                if let Some(value) = format.write_float(bottom_value, &mut buffer) {
                    self.surface.write_ascii(x_poz - value.len() as i32 - 1, y, value.as_bytes(), attr, false);
                }
                y -= self.yaxis.step as i32;
                bottom_value -= self.yaxis.bottom_step;
            }
        }
    }
    fn paint_xaxis(&mut self, attr: CharAttribute) {
        if self.xaxis.label_format.is_none() {
            return;
        }
        let left = if self.yaxis.width > 0 { self.yaxis.width as i32 + 2 } else { 0 };
        let right = self.size().width as i32;
        let y = self.size().height as i32 - 2;
        self.surface.draw_horizontal_line(left, y, right, LineType::Single, attr);

        match self.xaxis.label_format {
            XAxisLabelFormat::None => (),
            XAxisLabelFormat::Index { start } => self.paint_xaxis_index(start, attr),
            XAxisLabelFormat::BarLabels => todo!(),
            XAxisLabelFormat::Custom => todo!(),
        }
        match self.xaxis.label_format {
            XAxisLabelFormat::None => (),
            XAxisLabelFormat::Index { start } => self.paint_xaxis_index(start, attr),
            XAxisLabelFormat::BarLabels => self.paint_xaxis_bar_labels(attr),
            XAxisLabelFormat::Custom => todo!(),
        }
    }
    fn print_label(&mut self, x: i32, y: i32, start_bar_index: usize, end_bar_index: usize, label: &str, attr: CharAttribute) {
        let last_bar = &self.bars[end_bar_index];
        let first_bar = &self.bars[start_bar_index];
        let width = (last_bar.bar.actual_thickness(&self.defaults) as i32) + last_bar.layout.x - first_bar.layout.x;
        let left_space = first_bar.bar.spacing.unwrap_or(self.defaults.spacing) as i32;
        let right_space = if end_bar_index + 1 < self.bars.len() {
            self.bars[end_bar_index + 1].bar.spacing.unwrap_or(self.defaults.spacing) as i32
        } else {
            left_space
        };
        // pentru impare - 5 -> 5/2 - (1-5 & 1) = 2 - 0 = 2;
        // pentru pare - 6 => 6/2 - (1-6 & 1) = 3 - 1 = 2;
        let left = left_space / 2 - (1 - left_space & 1);
        let right = right_space / 2 - (1 - right_space & 1);
        let total_space = left + width + right;
        if total_space < 1 {
            return;
        }
        let (text, count, truncated) = {
            let mut count = 0usize;
            let mut end = label.len();
            let mut truncated = false;
            for (byte_idx, _) in label.char_indices() {
                if count == total_space as usize {
                    end = byte_idx;
                    truncated = true;
                    break;
                }
                count += 1;
            }
            (&label[..end], count, truncated)
        };
        let x = x - left;
        if truncated {
            self.surface.write_string(x, y, text, attr, false);
            self.surface.write_char(
                x + total_space - 1,
                y,
                Character::with_attributes(SpecialChar::ThreePointsHorizontal, attr),
            );
        } else {
            self.surface.write_string(x + (total_space - count as i32) / 2, y, text, attr, false);
        }
    }
    fn paint_xaxis_index(&mut self, start: i32, attr: CharAttribute) {
        let mut temp: [u8; 16] = [0u8; 16];
        let mut idx_start = self.first_visible_bar as usize;
        let mut bar_idx = (start as i64) + self.first_visible_bar as i64;
        let len = self.bars.len();
        let width = self.size().width as i32;
        let left_margin = self.x_axis_left_margin();
        let y = self.size().height as i32 - 1;
        while idx_start < len {
            let bar = &self.bars[idx_start];
            let x = bar.layout.x - self.left_scroll + left_margin;
            if x >= width {
                break;
            }
            if let Some(result) = INT_FORMAT.write_number(bar_idx, &mut temp) {
                self.print_label(x, y, idx_start, idx_start, &result, attr);
            }
            idx_start += 1;
            bar_idx += 1;
        }
    }
    fn paint_xaxis_bar_labels(&mut self, attr: CharAttribute) {
        let mut temp_str: FlatString<254> = FlatString::new();
        let mut idx_start = self.first_visible_bar as usize;
        let len = self.bars.len();
        let width = self.size().width as i32;
        let left_margin = self.x_axis_left_margin();
        let y = self.size().height as i32 - 1;
        while idx_start < len {
            let bar = &self.bars[idx_start];
            let x = bar.layout.x - self.left_scroll + left_margin;
            if x >= width {
                break;
            }
            if !bar.bar.label.is_empty() {
                temp_str.set(bar.bar.label.as_str());
                self.print_label(x, y, idx_start, idx_start, temp_str.as_str(), attr);
            }
            idx_start += 1;
        }
    }    
}

impl<T> OnPaint for VBarChart<T>
where
    T: Number + 'static,
{
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.draw_surface(0, 0, &self.surface);
    }
}

impl<T> OnResize for VBarChart<T>
where
    T: Number + 'static,
{
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.surface.resize(new_size);
        self.repaint_surface();
    }
}
