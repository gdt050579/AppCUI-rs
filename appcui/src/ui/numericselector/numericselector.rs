use std::fmt::Display;
use std::ops::{Add, Sub};

use super::events::EventData;
use super::Buttons;
use super::Flags;
use crate::prelude::*;
use std::fmt::Write;

pub trait Numeric: Add<Output = Self> + Sub<Output = Self> + Copy + Clone + PartialOrd + PartialEq + Display {}

impl Numeric for i8 {}
impl Numeric for i16 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for i128 {}
impl Numeric for u8 {}
impl Numeric for u16 {}
impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for u128 {}
impl Numeric for usize {}
impl Numeric for isize {}
impl Numeric for f32 {}
impl Numeric for f64 {}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct NumericSelector<T>
where
    T: Numeric + 'static,
{
    value: T,
    min: T,
    max: T,
    step: T,
    flags: Flags,
    buttons: Buttons,
    txt: String,
    txtlen: u8,
}
impl<T> NumericSelector<T>
where
    T: Numeric + 'static,
{
    fn to_interval(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }
    pub fn new(value: T, min: T, max: T, step: T, layout: Layout, flags: Flags) -> Self {
        let v_min = if min < max { min } else { max };
        let v_max = if max > min { max } else { min };
        let v = Self::to_interval(value, v_min, v_max);
        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            min: v_min,
            max: v_max,
            step,
            value: v,
            flags,
            txt: String::with_capacity(16),
            txtlen: 0,
            buttons: Buttons::new(),
        };
        obj.buttons.update_width(obj.size().width as u16);
        if flags.contains(Flags::HideButtons) {
            obj.set_size_bounds(3, 1, u16::MAX, 1);
        } else {
            obj.set_size_bounds(12, 1, u16::MAX, 1);
        }
        obj.update_string_representation();
        obj.update_button_status();
        obj
    }
    #[inline(always)]
    pub fn value(&self) -> T {
        self.value.clone()
    }
    pub fn set_value(&mut self, value: T) {
        self.value = Self::to_interval(value, self.min, self.max);
        self.update_button_status();
        self.update_string_representation()
    }
    #[inline(always)]
    fn update_button_status(&mut self) {
        self.buttons.disable_buttons(self.value == self.min, self.value == self.max);
    }
    fn update_string_representation(&mut self) {
        self.txt.clear();
        write!(self.txt, "{}", self.value).unwrap();
        self.txtlen = self.txt.len() as u8;
    }
    fn increment(&mut self) {
        let mut new_value = self.value + self.step;
        if new_value < self.value {
            // overflow
            new_value = self.max;
        }
        new_value = Self::to_interval(new_value, self.min, self.max);
        if new_value != self.value {
            self.value = new_value;
            self.update_button_status();
            self.update_string_representation();
            self.emit_on_selection_changed_event();
        }
    }
    fn decrement(&mut self) {
        let mut new_value = self.value - self.step;
        if new_value > self.value {
            // underflow
            new_value = self.min;
        }
        new_value = Self::to_interval(new_value, self.min, self.max);
        if new_value != self.value {
            self.value = new_value;
            self.update_button_status();
            self.update_string_representation();
            self.emit_on_selection_changed_event();
        }
    }

    fn emit_on_selection_changed_event(&mut self) {
        // self.raise_event(ControlEvent {
        //     emitter: self.handle,
        //     receiver: self.event_processor,
        //     data: ControlEventData::Selector(EventData {
        //         type_id: std::any::TypeId::of::<T>(),
        //     }),
        // });
    }
}
impl<T> OnPaint for NumericSelector<T>
where
    T: Numeric + 'static,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let has_buttons = !self.flags.contains(Flags::HideButtons);
        if has_buttons {
            self.buttons.paint(surface, theme, self.is_enabled());
        }
        let attr = match () {
            _ if !self.is_enabled() => theme.editor.inactive,
            _ if self.has_focus() => theme.editor.focused,
            _ if self.is_mouse_over() => theme.editor.hovered,
            _ => theme.editor.normal,
        };
        let w = self.size().width as i32;
        let (l, r) = if has_buttons { (4, w - 6) } else { (0, w - 1) };
        surface.fill_horizontal_line(l, 0, r, Character::with_attributes(' ', attr));
        if l + 2 <= r {
            let mut format = TextFormat::new((l + r) / 2, 0, attr, TextAlignament::Center, false);
            format.chars_count = Some(self.txtlen as u16);
            format.width = Some((r - (l + 1)) as u16);
            surface.write_text(&self.txt, &format);
        }
    }
}
impl<T> OnKeyPressed for NumericSelector<T>
where
    T: Numeric + 'static,
{
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        match key.value() {
            key!("Up") | key!("Right") => {
                self.increment();
                return EventProcessStatus::Processed;
            }
            key!("Down") | key!("Left") => {
                self.decrement();
                return EventProcessStatus::Processed;
            }
            key!("Home") => {
                self.set_value(self.min);
                return EventProcessStatus::Processed;
            }
            key!("End") => {
                self.set_value(self.max);
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        EventProcessStatus::Ignored
    }
}
impl<T> OnMouseEvent for NumericSelector<T>
where
    T: Numeric + 'static,
{
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        let mut response = EventProcessStatus::Ignored;
        // process buttons if visible
        if !self.flags.contains(Flags::HideButtons) {
            let bres = self.buttons.on_mouse_event(event);

            if bres.repaint {
                response = EventProcessStatus::Processed;
            }
            if bres.click_on_add {
                self.increment();
                response = EventProcessStatus::Processed;
            }
            if bres.click_on_sub {
                self.decrement();
                response = EventProcessStatus::Processed;
            }
            if !bres.forward_to_control {
                return response;
            }
        }
        // do other processing here
        response
    }
}
impl<T> OnResize for NumericSelector<T>
where
    T: Numeric + 'static,
{
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.buttons.update_width(new_size.width as u16);
    }
}
