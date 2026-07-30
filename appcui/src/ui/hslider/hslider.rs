use std::ops::{Div, Mul};

use crate::prelude::*;
use crate::ui::hslider::{events::EventData, Flags, Type};
use crate::ui::numericselector::Format;

#[CustomControl(overwrite=OnPaint+OnDefaultAction+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct HSlider<T>
where
    T: Number + 'static,
{
    value: T,
    min: T,
    max: T,
    step: T,
    page_step: T,
    tick_value: u32,
    hslider_type: Type,
    marker_pos: u32,
    buffer_value: String,
    buffer_value_max_len: u32,
    hslider_width: u32,
    hslider_pos: u32,
    format: Format,
    flags: Flags,
    pressed: bool,
}

impl<T> HSlider<T>
where
    T: Number + 'static,
{
    pub fn new(min: T, max: T, step: T, hslider_type: Type, hslider_falgs: Flags, layout: Layout) -> Self {
        Self::inner_create(min, max, step, hslider_type, hslider_falgs, layout, StatusFlags::None)
    }
    fn inner_create(min: T, max: T, step: T, hslider_type: Type, hslider_falgs: Flags, layout: Layout, status: StatusFlags) -> Self {
        let mut hslider = HSlider {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | status),
            value: min,
            min: min,
            max: max,
            step: step,
            page_step: step,
            tick_value: 0,
            hslider_type: hslider_type,
            marker_pos: 0,
            flags: hslider_falgs,
            pressed: false,
            buffer_value: String::new(),
            format: Format::Decimal,
            hslider_width: 0,
            hslider_pos: 0,
            buffer_value_max_len: 0,
        };
        hslider.update_layout();
        hslider.set_buffer_value();
        hslider.set_size_bounds(1, 1, u16::MAX, 1);
        hslider
    }

    pub fn set_ticks(&mut self, val: u32) {
        self.tick_value = val;
    }

    fn set_buffer_value(&mut self) {
        self.value.write_to_string(&mut self.buffer_value, self.format);
    }

    fn update_layout(&mut self) {
        self.hslider_pos = 0;
        self.hslider_width = self.size().width;
        if self.flags.contains(Flags::ShowValue) || self.flags.contains(Flags::ValueAsMarker) {
            let mut b = String::new();
            self.min.write_to_string(&mut b, self.format);
            let mut val = b.len();
            self.max.write_to_string(&mut b, self.format);
            val = val.max(b.len());
            if self.flags.contains(Flags::ShowValue) {
                self.hslider_width = self.hslider_width.saturating_sub(val as u32 + 1);
            }
            self.buffer_value_max_len = val as u32;
        }
        if self.hslider_type.char_set().left_cap.is_some() {
            self.hslider_pos += 1;
            self.hslider_width = self.hslider_width.saturating_sub(1);
        }
        if self.hslider_type.char_set().right_cap.is_some() {
            self.hslider_width = self.hslider_width.saturating_sub(1);
        }

        let (lo, hi) = self.marker_bounds();
        self.marker_pos = self.marker_pos.clamp(lo, hi);
    }

    fn marker_width(&self) -> u32 {
        if self.flags.contains(Flags::ValueAsMarker) {
            self.buffer_value_max_len.max(1)
        } else {
            1
        }
    }

    fn marker_bounds(&self) -> (u32, u32) {
        let val1 = if self.hslider_type.char_set().left_marker.is_some() { 1 } else { 0 };
        let val2 = self.marker_width() + if self.hslider_type.char_set().right_marker.is_some() { 1 } else { 0 };
        let lo = self.hslider_pos + val1;
        let hi = (self.hslider_pos + self.hslider_width).saturating_sub(val2);
        (lo.min(hi), hi)
    }

    fn add_step_value(&mut self) {
        self.value = self.value + self.step;
        if self.value > self.max {
            self.value = self.max;
        }
        self.marker_pos = self.get_marker_x_position();
        self.set_buffer_value();
    }

    fn sub_step_value(&mut self) {
        self.value = self.value - self.step;
        if self.value < self.min {
            self.value = self.min;
        }
        self.marker_pos = self.get_marker_x_position();
        self.set_buffer_value();
    }

    fn set_marker_position_from_x(&mut self, x: i32) {
        let (lo, hi) = self.marker_bounds();
        let span = hi.saturating_sub(lo);
        let min = self.min.to_f64();
        let max = self.max.to_f64();

        let mw = self.marker_width();
        let x_anchor = x - (mw as i32) / 2;
        let clamped = (x_anchor.max(0) as u32).clamp(lo, hi);

        let raw = if span == 0 {
            min
        } else {
            let rel = (clamped - lo) as f64;
            min + (rel / span as f64) * (max - min)
        };

        let step = self.step.to_f64();
        let snapped = if step > 0.0 {
            (min + ((raw - min) / step).round() * step).clamp(min, max)
        } else {
            raw.clamp(min, max)
        };

        self.value = T::from_f64(snapped);
        self.marker_pos = self.get_marker_x_position();
        self.set_buffer_value();
    }

    fn get_marker_x_position(&self) -> u32 {
        let (lo, hi) = self.marker_bounds();
        let span = hi.saturating_sub(lo);
        if span == 0 {
            return lo;
        }
        let value = self.value.to_f64();
        let min = self.min.to_f64();
        let max = self.max.to_f64();
        if max - min == 0.0 {
            return lo;
        }
        let rel = ((value - min) / (max - min) * span as f64).round().clamp(0.0, span as f64) as u32;
        lo + rel
    }

    fn paint_standard(&self, surface: &mut Surface, theme: &Theme) {
        let attr = match () {
            _ if !self.is_enabled() => theme.text.inactive,
            _ if self.has_focus() => theme.text.focused,
            _ if self.is_mouse_over() => theme.text.hovered,
            _ => theme.text.normal,
        };

        //I will modify them later, I will take them from the specific hslider theme
        let left_marker_line_attr = CharAttribute::new(Color::Green, Color::Transparent, CharFlags::None);
        let right_marker_line_attr = CharAttribute::new(Color::Green, Color::Transparent, CharFlags::None);
        let tick_attr = CharAttribute::new(Color::Green, Color::Transparent, CharFlags::None);
        let left_tick_attr = CharAttribute::new(Color::Green, Color::Transparent, CharFlags::None);
        let right_tick_attr = CharAttribute::new(Color::Green, Color::Transparent, CharFlags::None);
        let marker_attr = CharAttribute::new(Color::Green, Color::Transparent, CharFlags::None);
        let left_marker_attr = CharAttribute::new(Color::Green, Color::Transparent, CharFlags::None);
        let right_marker_attr = CharAttribute::new(Color::Green, Color::Transparent, CharFlags::None);
        let left_cap_attr = CharAttribute::new(Color::Green, Color::Transparent, CharFlags::None);
        let right_cap_attr = CharAttribute::new(Color::Green, Color::Transparent, CharFlags::None);
        let text_attr = attr;

        let mw = self.marker_width();

        let left_marker_line = Character::with_attributes(self.hslider_type.char_set().left_marker_line, left_marker_line_attr);
        let size = self.marker_pos.saturating_sub(self.hslider_pos);
        surface.fill_horizontal_line_with_size(self.hslider_pos as i32, 0, size, left_marker_line);

        let right_marker_line = Character::with_attributes(self.hslider_type.char_set().right_marker_line, right_marker_line_attr);
        let right_start = self.marker_pos + mw;
        let size = (self.hslider_pos + self.hslider_width).saturating_sub(right_start);
        surface.fill_horizontal_line_with_size(right_start as i32, 0, size, right_marker_line);

        if self.flags.contains(Flags::Ticks) {
            let tick = Character::with_attributes(self.hslider_type.char_set().tick, tick_attr);
            let left_tick = Character::with_attributes(self.hslider_type.char_set().left_tick, left_tick_attr);
            let right_tick = Character::with_attributes(self.hslider_type.char_set().right_tick, right_tick_attr);

            let last = self.hslider_width.saturating_sub(1);

            if self.tick_value == 1 {
                surface.write_char(self.hslider_pos as i32, 0, left_tick);
            } else if self.tick_value >= 2 {
                let n = self.tick_value - 1;
                for k in 0..=n {
                    let offset = ((k as f64 / n as f64) * last as f64).round() as u32;
                    let ch = if k == 0 {
                        left_tick
                    } else if k == n {
                        right_tick
                    } else {
                        tick
                    };
                    surface.write_char((self.hslider_pos + offset) as i32, 0, ch);
                }
            }
        }

        if let Some(code) = self.hslider_type.char_set().left_marker {
            let left_marker = Character::with_attributes(code, left_marker_attr);
            surface.write_char(self.marker_pos.saturating_sub(1) as i32, 0, left_marker);
        }

        if let Some(code) = self.hslider_type.char_set().right_marker {
            let right_marker = Character::with_attributes(code, right_marker_attr);
            surface.write_char((self.marker_pos + mw) as i32, 0, right_marker);
        }

        let marker = Character::with_attributes(self.hslider_type.char_set().marker, marker_attr);
        if self.flags.contains(Flags::ValueAsMarker) {
            surface.write_string(
                self.marker_pos as i32 + self.buffer_value_max_len.saturating_sub(self.buffer_value.len() as u32) as i32,
                0,
                &self.buffer_value,
                marker_attr,
                false,
            );
        } else {
            surface.write_char(self.marker_pos as i32, 0, marker);
        }

        if let Some(code) = self.hslider_type.char_set().left_cap {
            let left_cap = Character::with_attributes(code, left_cap_attr);
            surface.write_char(self.hslider_pos.saturating_sub(1) as i32, 0, left_cap);
        }

        if let Some(code) = self.hslider_type.char_set().right_cap {
            let right_cap = Character::with_attributes(code, right_cap_attr);
            surface.write_char((self.hslider_pos + self.hslider_width) as i32, 0, right_cap);
        }

        if self.flags.contains(Flags::ShowValue) {
            surface.write_string(
                (self.hslider_pos + self.hslider_width + self.buffer_value_max_len.saturating_sub(self.buffer_value.len() as u32)) as i32 + 2,
                0,
                &self.buffer_value,
                text_attr,
                false,
            );
        }
    }

    fn paint_progress_bar(&self, surface: &mut Surface, theme: &Theme) {
        // let attr = match () {
        //     _ if !self.is_enabled() => theme.text.inactive,
        //     _ if self.has_focus() => theme.text.focused,
        //     _ if self.is_mouse_over() => theme.text.hovered,
        //     _ => theme.text.normal,
        // };

        let attr1 = CharAttribute::new(Color::Transparent, Color::Green, CharFlags::None);
        let attr2 = CharAttribute::new(Color::Transparent, Color::Gray, CharFlags::None);
        let attr_text = CharAttribute::new(Color::Black, Color::Transparent, CharFlags::None);

        let width = self.size().width;
        let size1 = self.marker_pos + 1;
        let size2 = width.saturating_sub(self.marker_pos);

        surface.fill_horizontal_line_with_size(0, 0, size1, Character::with_attributes(' ', attr1));

        surface.fill_horizontal_line_with_size(size1 as i32, 0, size2, Character::with_attributes(' ', attr2));

        let pos = (size1 as i32 - self.buffer_value.len() as i32).max(0);
        let format = TextFormatBuilder::new()
            .position(pos, 0)
            .attribute(attr_text)
            .align(TextAlignment::Left)
            .chars_count(self.buffer_value.chars().count() as u16)
            .wrap_type(WrapType::SingleLineWrap(width as u16))
            .build();

        surface.write_text(&self.buffer_value, &format);
    }
}

impl<T> OnDefaultAction for HSlider<T>
where
    T: Number + 'static,
{
    fn on_default_action(&mut self) {
        // self.raise_event(ControlEvent {
        //     emitter: self.handle,
        //     receiver: self.event_processor,
        //     data: ControlEventData::HyperLink(EventData),
        // });
    }
}
impl<T> OnKeyPressed for HSlider<T>
where
    T: Number + 'static,
{
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Left") => {
                self.sub_step_value();
                EventProcessStatus::Processed
            }
            key!("Right") => {
                self.add_step_value();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl<T> OnPaint for HSlider<T>
where
    T: Number + 'static,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        match &self.hslider_type {
            Type::Standard => self.paint_standard(surface, theme),
            Type::ProgressBar => {} // not yet
            Type::Inline => self.paint_standard(surface, theme),
        }
    }
}
impl<T> OnMouseEvent for HSlider<T>
where
    T: Number + 'static,
{
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Pressed(mouse) => {
                self.pressed = true;
                if self.flags.contains(Flags::ShowTooltip) {
                    self.show_tooltip(&self.buffer_value);
                }
                self.set_marker_position_from_x(mouse.x);
                EventProcessStatus::Processed
            }
            MouseEvent::Released(_) => {
                self.pressed = false;
                self.hide_tooltip();
                EventProcessStatus::Processed
            }
            MouseEvent::Drag(mouse) => {
                self.set_marker_position_from_x(mouse.x);
                if self.flags.contains(Flags::ShowTooltip) {
                    self.show_tooltip(&self.buffer_value);
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Enter => {
                if self.flags.contains(Flags::ShowTooltip) {
                    self.show_tooltip(&self.buffer_value);
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Leave => {
                self.hide_tooltip();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl<T> OnResize for HSlider<T>
where
    T: Number + 'static,
{
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        self.update_layout();
    }
}
