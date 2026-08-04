use crate::prelude::*;
use crate::ui::hslider::{events::EventData, Flags, Type};
use crate::ui::numericselector::Format;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct HSlider<T>
where
    T: Number + 'static,
{
    value: T,
    min: T,
    max: T,
    step: T,
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
    tick_pos: Vec<u16>,
    tick_index: usize,
}

impl<T> HSlider<T>
where
    T: Number + 'static,
{
    /// Creates a horizontal slider bounded by `min` and `max`, with the given `step`, visual `hslider_type` and `flags`.
    /// The initial value is set to `min`.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let slider = HSlider::new(0, 10, 1, hslider::Type::Standard, layout!("x:1,y:1,w:20,h:1"), hslider::Flags::None);
    /// ```
    pub fn new(min: T, max: T, step: T, hslider_type: Type, layout: Layout, hslider_falgs: Flags) -> Self {
        Self::inner_create(min, max, step, hslider_type, hslider_falgs, layout, StatusFlags::None)
    }
    fn inner_create(min: T, max: T, step: T, hslider_type: Type, hslider_falgs: Flags, layout: Layout, status: StatusFlags) -> Self {
        let mut hslider = HSlider {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | status),
            value: min,
            min,
            max,
            step,
            tick_value: 0,
            hslider_type,
            marker_pos: 0,
            flags: hslider_falgs,
            pressed: false,
            buffer_value: String::new(),
            format: Format::Decimal,
            hslider_width: 0,
            hslider_pos: 0,
            buffer_value_max_len: 0,
            tick_pos: Vec::new(),
            tick_index: 0,
        };
        hslider.update_metrics();
        hslider.set_buffer_value();
        hslider.set_size_bounds(1, 1, u16::MAX, 1);
        hslider
    }

    /// Sets the number of tick marks displayed along the slider.
    /// The ticks are only shown when the `Ticks` flag is set and `val` is at least 2.
    pub fn set_ticks(&mut self, val: u32) {
        self.tick_value = val;
        self.rebuild_ticks();
        self.sync_tick_index();
        self.marker_pos = self.get_marker_x_position();
        self.request_update();
    }

    /// Sets the current value of the slider.
    /// The value is clamped to the `[min, max]` range, and the marker is repositioned accordingly.
    pub fn set_value(&mut self, value: T) {
        let v = value.to_f64().clamp(self.min.to_f64(), self.max.to_f64());
        self.value = T::from_f64(v);
        self.sync_tick_index();
        self.marker_pos = self.get_marker_x_position();
        self.set_buffer_value();
        self.request_update();
        self.emit_value_changed();
    }

    /// Sets the lower bound of the slider.
    /// If the current value falls below the new minimum, it is clamped up to it.
    pub fn set_min(&mut self, min: T) {
        self.min = min;
        let v = self.value.to_f64().clamp(self.min.to_f64(), self.max.to_f64());
        self.value = T::from_f64(v);
        self.update_metrics();
        self.sync_tick_index();
        self.marker_pos = self.get_marker_x_position();
        self.set_buffer_value();
        self.request_update();
        self.emit_value_changed();
    }

    /// Sets the upper bound of the slider.
    /// If the current value exceeds the new maximum, it is clamped down to it.
    pub fn set_max(&mut self, max: T) {
        self.max = max;
        let v = self.value.to_f64().clamp(self.min.to_f64(), self.max.to_f64());
        self.value = T::from_f64(v);
        self.update_metrics();
        self.sync_tick_index();
        self.marker_pos = self.get_marker_x_position();
        self.set_buffer_value();
        self.request_update();
        self.emit_value_changed();
    }

    /// Sets the increment used when the slider value changes by one step.
    pub fn set_step(&mut self, step: T) {
        self.step = step;
    }

    /// Returns the number of tick marks configured for the slider.
    pub fn ticks(&self) -> u32 {
        self.tick_value
    }

    /// Returns the current value of the slider.
    pub fn value(&self) -> T {
        self.value
    }

    /// Returns the lower bound of the slider.
    pub fn min(&self) -> T {
        self.min
    }

    /// Returns the upper bound of the slider.
    pub fn max(&self) -> T {
        self.max
    }

    /// Returns the increment used when the slider value changes by one step.
    pub fn step(&self) -> T {
        self.step
    }

    fn rebuild_ticks(&mut self) {
        self.tick_pos.clear();
        let (lo, hi) = self.track_bounds();
        match self.tick_value {
            0 | 1 => {}
            _ => {
                let n = (self.tick_value - 1) as u64;
                let span = hi.saturating_sub(lo) as u64;
                for k in 0..=n {
                    let x = lo + ((k * span + n / 2) / n) as u32;
                    self.tick_pos.push(x as u16);
                }
            }
        }
        self.tick_index = self.tick_index.min(self.tick_pos.len().saturating_sub(1));
    }

    fn ticks_active(&self) -> bool {
        self.flags.contains(Flags::Ticks) && self.tick_pos.len() >= 2
    }

    fn tick_index_from_x(&self, x: u32) -> usize {
        let mut best = 0usize;
        let mut best_dist = u32::MAX;
        for (idx, &tick_x) in self.tick_pos.iter().enumerate() {
            let dist = (tick_x as u32).abs_diff(x);
            if dist < best_dist || (dist == best_dist && idx == self.tick_index) {
                best_dist = dist;
                best = idx;
            }
        }
        best
    }

    fn sync_tick_index(&mut self) {
        if self.ticks_active() {
            self.tick_index = self.value_to_tick_index(self.value);
            self.value = self.index_to_value(self.tick_index);
        } else {
            self.tick_index = 0;
        }
    }

    fn value_to_tick_index(&self, value: T) -> usize {
        if self.tick_value < 2 {
            return 0;
        }
        let min = self.min.to_f64();
        let max = self.max.to_f64();
        if max - min == 0.0 {
            return 0;
        }
        let n = (self.tick_value - 1) as f64;
        let seg = (max - min) / n;
        (((value.to_f64() - min) / seg).round() as i64).clamp(0, (self.tick_value - 1) as i64) as usize
    }

    fn index_to_value(&self, idx: usize) -> T {
        if self.tick_value < 2 {
            return self.min;
        }
        let n = (self.tick_value - 1) as usize;
        if idx == 0 {
            return self.min;
        }
        if idx >= n {
            return self.max;
        }
        let min = self.min.to_f64();
        let max = self.max.to_f64();
        T::from_f64(min + idx as f64 * (max - min) / n as f64)
    }

    fn set_buffer_value(&mut self) {
        self.value.write_to_string(&mut self.buffer_value, self.format);
    }

    fn update_metrics(&mut self) {
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

        self.rebuild_ticks();

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

    fn track_bounds(&self) -> (u32, u32) {
        let val1 = if self.hslider_type.char_set().left_marker.is_some() { 1 } else { 0 };
        let val2 = 1 + if self.hslider_type.char_set().right_marker.is_some() { 1 } else { 0 };
        let lo = self.hslider_pos + val1;
        let hi = (self.hslider_pos + self.hslider_width).saturating_sub(val2);
        (lo.min(hi), hi)
    }

    fn marker_bounds(&self) -> (u32, u32) {
        let (lo, hi) = self.track_bounds();
        (lo, hi.saturating_sub(self.marker_width() - 1).max(lo))
    }

    fn add_step_value(&mut self) {
        let old = self.value;
        if self.ticks_active() {
            if self.tick_index + 1 >= self.tick_pos.len() {
                return;
            }
            self.tick_index += 1;
            self.value = self.index_to_value(self.tick_index);
        } else {
            let new_val = T::from_f64((self.value.to_f64() + self.step.to_f64()).min(self.max.to_f64()));
            if self.value == new_val {
                return;
            }
            self.value = new_val;
        }
        self.marker_pos = self.get_marker_x_position();
        self.set_buffer_value();
        self.request_update();
        if self.value != old {
            self.emit_value_changed();
        }
    }

    fn sub_step_value(&mut self) {
        let old = self.value;
        if self.ticks_active() {
            if self.tick_index == 0 {
                return;
            }
            self.tick_index -= 1;
            self.value = self.index_to_value(self.tick_index);
        } else {
            let new_val = T::from_f64((self.value.to_f64() - self.step.to_f64()).max(self.min.to_f64()));
            if self.value == new_val {
                return;
            }
            self.value = new_val;
        }
        self.marker_pos = self.get_marker_x_position();
        self.set_buffer_value();
        self.request_update();
        if self.value != old {
            self.emit_value_changed();
        }
    }

    fn set_marker_position_from_x(&mut self, x: i32) {
        let (lo, hi) = self.marker_bounds();
        let mw = self.marker_width();

        let changed = if self.ticks_active() {
            let (tlo, thi) = self.track_bounds();
            let idx = self.tick_index_from_x((x.max(0) as u32).clamp(tlo, thi));
            self.tick_index = idx;
            let new_val = self.index_to_value(idx);
            let changed = new_val != self.value;
            self.value = new_val;
            changed
        } else {
            let clamped = ((x - (mw as i32) / 2).max(0) as u32).clamp(lo, hi);
            let span = hi.saturating_sub(lo);
            let min = self.min.to_f64();
            let max = self.max.to_f64();
            let raw = if span == 0 {
                min
            } else {
                min + ((clamped - lo) as f64 / span as f64) * (max - min)
            };
            let step = self.step.to_f64();
            let snapped = if step > 0.0 {
                (min + ((raw - min) / step).round() * step).clamp(min, max)
            } else {
                raw.clamp(min, max)
            };
            let new_val = T::from_f64(snapped);
            let changed = new_val != self.value;
            self.value = new_val;
            changed
        };

        self.marker_pos = self.get_marker_x_position();
        self.set_buffer_value();
        self.request_update();
        if changed {
            self.emit_value_changed();
        }
    }

    fn get_marker_x_position(&self) -> u32 {
        if self.ticks_active() {
            let tick_x = self.tick_pos[self.tick_index.min(self.tick_pos.len() - 1)] as u32;
            let (lo, hi) = self.marker_bounds();
            return tick_x.saturating_sub(self.marker_width() / 2).clamp(lo, hi);
        }
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
        let hslider_theme = &theme.hslider;

        let left_marker_line_attr = hslider_theme.before_line;
        let right_marker_line_attr = hslider_theme.after_line;

        let left_tick_attr = hslider_theme.before_line;
        let right_tick_attr = hslider_theme.after_line;

        let marker_attr = match () {
            _ if !self.is_enabled() => hslider_theme.marker.inactive,
            _ if self.pressed => hslider_theme.marker.pressed_or_selected,
            _ if self.has_focus() => hslider_theme.marker.focused,
            _ if self.is_mouse_over() => hslider_theme.marker.hovered,
            _ => hslider_theme.marker.normal,
        };
        let left_marker_attr = match () {
            _ if !self.is_enabled() => hslider_theme.marker_border.inactive,
            _ if self.pressed => hslider_theme.marker_border.pressed_or_selected,
            _ if self.has_focus() => hslider_theme.marker_border.focused,
            _ if self.is_mouse_over() => hslider_theme.marker_border.hovered,
            _ => hslider_theme.marker_border.normal,
        };
        let right_marker_attr = left_marker_attr;

        let left_cap_attr = hslider_theme.cap;
        let right_cap_attr = hslider_theme.cap;

        let text_attr = match () {
            _ if !self.is_enabled() => theme.text.inactive,
            _ if self.has_focus() => theme.text.focused,
            _ if self.is_mouse_over() => theme.text.hovered,
            _ => theme.text.normal,
        };

        let mw = self.marker_width();

        let left_marker_line = Character::with_attributes(self.hslider_type.char_set().left_marker_line, left_marker_line_attr);
        let size = self.marker_pos.saturating_sub(self.hslider_pos);
        surface.fill_horizontal_line_with_size(self.hslider_pos as i32, 0, size, left_marker_line);

        let right_marker_line = Character::with_attributes(self.hslider_type.char_set().right_marker_line, right_marker_line_attr);
        let right_start = self.marker_pos + mw;
        let size = (self.hslider_pos + self.hslider_width).saturating_sub(right_start);
        surface.fill_horizontal_line_with_size(right_start as i32, 0, size, right_marker_line);

        if self.flags.contains(Flags::Ticks) {
            let marker_start = self.marker_pos;
            let marker_end = self.marker_pos + mw;

            let left_tick = Character::with_attributes(self.hslider_type.char_set().left_tick, left_tick_attr);
            let right_tick = Character::with_attributes(self.hslider_type.char_set().right_tick, right_tick_attr);
            let tick_left = Character::with_attributes(self.hslider_type.char_set().tick, left_marker_line_attr);
            let tick_right = Character::with_attributes(self.hslider_type.char_set().tick, right_marker_line_attr);

            let draw_tick = |surface: &mut Surface, x: u32, ch| {
                if x < marker_start || x >= marker_end {
                    surface.write_char(x as i32, 0, ch);
                }
            };

            let n = self.tick_pos.len();
            for (k, &xp) in self.tick_pos.iter().enumerate() {
                if k == 0 || k + 1 == n {
                    continue;
                }
                let x = xp as u32;
                let ch = if x < marker_start { tick_left } else { tick_right };
                draw_tick(surface, x, ch);
            }
            if n >= 2 {
                draw_tick(surface, self.tick_pos[0] as u32, left_tick);
                draw_tick(surface, self.tick_pos[n - 1] as u32, right_tick);
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
                text_attr,
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
            let cap = if self.hslider_type.char_set().right_cap.is_some() { 1 } else { 0 };
            let pad = self.buffer_value_max_len.saturating_sub(self.buffer_value.len() as u32);
            let text_x = (self.hslider_pos + self.hslider_width + pad) as i32 + cap + 1;
            surface.write_string(text_x, 0, &self.buffer_value, text_attr, false);
        }
    }

    fn emit_value_changed(&mut self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::HSliderEvents(EventData {
                type_id: std::any::TypeId::of::<T>(),
            }),
        });
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
                self.hide_tooltip();
                EventProcessStatus::Processed
            }
            key!("Right") => {
                self.add_step_value();
                self.hide_tooltip();
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
        self.paint_standard(surface, theme);
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
                self.set_marker_position_from_x(mouse.x);
                self.show_tooltip(&self.buffer_value);
                EventProcessStatus::Processed
            }
            MouseEvent::Released(_) => {
                self.pressed = false;
                self.hide_tooltip();
                self.request_update();
                EventProcessStatus::Processed
            }
            MouseEvent::Drag(mouse) => {
                self.set_marker_position_from_x(mouse.x);
                self.show_tooltip(&self.buffer_value);
                EventProcessStatus::Processed
            }
            MouseEvent::Enter => {
                self.show_tooltip(&self.buffer_value);
                EventProcessStatus::Processed
            }
            MouseEvent::Leave => {
                self.pressed = false;
                self.hide_tooltip();
                self.request_update();
                EventProcessStatus::Processed
            }
            MouseEvent::Wheel(dir) => {
                match dir {
                    MouseWheelDirection::Down | MouseWheelDirection::Right => {
                        self.sub_step_value();
                        self.show_tooltip(&self.buffer_value);
                    }
                    MouseWheelDirection::Up | MouseWheelDirection::Left => {
                        self.add_step_value();
                        self.show_tooltip(&self.buffer_value);
                    }
                }
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
        self.update_metrics();
        self.marker_pos = self.get_marker_x_position();
    }
}
