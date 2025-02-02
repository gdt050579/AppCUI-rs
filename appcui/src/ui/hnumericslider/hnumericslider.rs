use std::cmp::min;
use std::string;
use std::thread::current;

use super::events::EventData;
use super::initialization_flags::Flags;
use crate::prelude::*;
use crate::ui::common::number::Format;
use crate::ui::common::Number;

struct CharSet {
    start_char: Character,
    separator: Character,
    value_indicator: Character,
    end_char: Character,
    selected_value_indicator: Character,
}

#[CustomControl(overwrite: [OnPaint, OnMouseEvent, OnResize, OnKeyPressed], internal=true)]
pub struct HNumericSlider<T>
where
    T: Number + 'static,
{
    flags: Flags,
    min: T,
    max: T,
    step: T,
    value: T,
    format: Format,
    bound: i32,
    ok_step: T,
    max_size_per_entry: usize,
    last_pressed_coods: Point,
    poz_triunghi: i32,
    // pentru mate
    m: usize,        //max size la valori
    nr_val: u32,     //cate valori am in interval
    sec_dim: f32,    // dimensiunea unei sectiuni
    y: f32,          // dimensiunea secventei m + spatiu + m + spatiu +...+ m in size
    o: u32,          // padding-ul necesar pentru prima sectiune
    p: u32,          // padding-ul necesar pentru restul sectiunilor
    computed_max: T, // pentru cazul in care nu pot ajunge la max cu step-ul curent
    values_string: String,
}
impl<T> HNumericSlider<T>
where
    T: Number + 'static,
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

    fn get_n_spaces(value: u32) -> String {
        let mut str = String::with_capacity(value as usize);
        while str.len() < value as usize {
            str.push(' ');
        }
        str
    }

    pub fn new(value: T, min: T, max: T, step: T, layout: Layout, flags: Flags, format: Format) -> Self {
        if step.is_zero() {
            panic!("Step can't be 0 for NumericSlider");
        }
        if min >= max {
            panic!("Min value can't be greater or equal to the Max value");
        }
        let clamped_value = Self::to_interval(value, min, max);
        let mut control = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            min,
            max,
            step,
            value: clamped_value,
            format,
            bound: 0,
            ok_step: T::ONE,
            max_size_per_entry: 0,
            poz_triunghi: 0,
            last_pressed_coods: Point::new(-1, -1),
            m: 0,
            nr_val: 0,
            sec_dim: 0.0,
            y: 0.0,
            o: 0,
            p: 0,
            values_string: String::new(),
            computed_max: max,
        };
        control.set_size_bounds(2, 3, u16::MAX, 3);
        control
    }

    fn compute_math_fields(&mut self) {
        let mut string_buffer = String::new();
        self.max.write_to_string(&mut string_buffer, self.format);
        self.m = string_buffer.len();
        self.min.write_to_string(&mut string_buffer, self.format);
        if string_buffer.len() > self.m {
            self.m = string_buffer.len();
        };

        self.y = (self.bound + 1) as f32 / (self.m + 1) as f32;

        self.nr_val = ((self.max - self.min) / self.step).cast_to_u32() + 1;
        self.sec_dim = (self.bound as f32) / ((self.nr_val - 1) as f32);
        self.p = (self.sec_dim - self.m as f32) as u32;
        self.o = (self.sec_dim - (self.m as f32 + ((self.m / 2) as f32))) as u32;

        if self.p >= 1 && self.o >= 1 {
            self.ok_step = self.step; //am suficient spatiu
        } else {
            let w = self.bound;
            let k = self.m;
            self.nr_val = ((w + 1) / (k as i32 + 1)).max(2) as u32;
            self.ok_step = (self.max - self.min) / T::cast_float_number((self.nr_val - 1) as f64);
            let p2 = (w - (self.nr_val * (k as u32)) as i32) / ((self.nr_val - 1) as i32);
            self.sec_dim = (self.bound as f32) / ((self.nr_val - 1) as f32);
            self.p = p2 as u32;
            self.o = p2 as u32;
        }

        self.values_string.clear();
        let mut current_value: T = self.min;

        // prima oara trebuie sa adun o
        current_value.write_to_string(&mut string_buffer, self.format);
        self.values_string.push_str(&string_buffer);
        self.values_string
            .push_str(&Self::get_n_spaces((self.o as usize + self.m - string_buffer.len()) as u32));

        current_value = current_value + self.ok_step;

        for i in 1..self.nr_val - 1 {
            current_value.write_to_string(&mut string_buffer, self.format);
            let space_debt: u32 = (self.m - string_buffer.len()) as u32;
            self.values_string.push_str(&format!(
                "{}{}{}",
                Self::get_n_spaces(space_debt / 2 + space_debt % 2),
                string_buffer,
                Self::get_n_spaces(space_debt / 2)
            ));

            if i != self.nr_val - 2 {
                self.values_string.push_str(&Self::get_n_spaces(self.p));
            }

            current_value = current_value + self.ok_step;
        }
        if self.nr_val - 1 != 1 {
            self.values_string.push_str(&Self::get_n_spaces(self.o));
        }

        current_value.write_to_string(&mut string_buffer, self.format);
        let space_debt: u32 = (self.m - string_buffer.len()) as u32 + (self.m % 2 == 0) as u32;
        self.values_string
            .push_str(&format!("{}{}", Self::get_n_spaces(space_debt), string_buffer));

        self.computed_max = current_value;
    }

    fn get_charset_based_on_theme(&self, theme: &Theme) -> CharSet {
        let (attr_line, attr_triangle) = match () {
            _ if !self.is_enabled() => (theme.text.inactive, theme.text.inactive),
            _ if self.has_focus() => (theme.text.focused, theme.text.focused),
            _ if self.is_mouse_over() => (theme.text.hovered, theme.text.hovered),
            _ => (theme.text.normal, theme.text.normal),
        };
        let down = !self.flags.contains(Flags::OnTop);
        CharSet {
            start_char: Character::with_attributes(
                if down {
                    SpecialChar::BoxTopLeftCornerSingleLine
                } else {
                    SpecialChar::BoxBottomLeftCornerSingleLine
                },
                attr_line,
            ),
            separator: Character::with_attributes(
                SpecialChar::BoxHorizontalSingleLine, // ramane acelasi pe up
                attr_line,
            ),
            value_indicator: Character::with_attributes(
                if down {
                    SpecialChar::SingleLineDownT
                } else {
                    SpecialChar::BoxMidleBottom
                }, //up e BoxMidleBottom
                attr_line,
            ),
            end_char: Character::with_attributes(
                if down {
                    SpecialChar::BoxTopRightCornerSingleLine
                } else {
                    SpecialChar::BoxBottomRightCornerSingleLine
                }, // BoxBottomRightCornerSingleLine
                attr_line,
            ),
            selected_value_indicator: Character::with_attributes(
                if down { SpecialChar::TriangleDown } else { SpecialChar::TriangleUp }, // TriangleUp
                attr_triangle,
            ),
        }
    }

    pub fn set_selected_value(&mut self, value: T) {
        self.value = value;
    }
    pub fn get_selected_value(&self) -> T {
        self.value
    }

    fn update_cursor_pos(&mut self, x: i32) {
        self.poz_triunghi = ((x / self.sec_dim as i32) * self.sec_dim as i32).min(self.bound - 1);
        self.value = self.min + self.ok_step * Number::cast_signed_number((x / self.sec_dim as i32) as i128);
    }

    fn emit_changed_event(&self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::HNumericSlider(EventData {
                type_id: std::any::TypeId::of::<T>(),
            }),
        });
    }
}
impl<T> OnPaint for HNumericSlider<T>
where
    T: Number + 'static,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let current_character_set: CharSet = self.get_charset_based_on_theme(theme);

        let attr_text = match () {
            _ if !self.is_enabled() => theme.text.inactive,
            _ if self.has_focus() => theme.text.focused,
            _ if self.is_mouse_over() => theme.text.hovered,
            _ => theme.text.normal,
        };

        let y_separators = 1; // mereu in centru
        let mut y_values = 2;
        let mut y_selector = 0;
        if self.flags.contains(Flags::OnTop) {
            y_values = 0;
            y_selector = 2;
        }

        surface.write_char(self.poz_triunghi, y_selector, current_character_set.selected_value_indicator);
        surface.write_string(0, y_values, &self.values_string, attr_text, false);

        //desenez marginea pentru min si max
        surface.write_char(0, y_separators, current_character_set.start_char);
        surface.write_char(
            (((self.nr_val - 1) * self.sec_dim as u32) as i32).min(self.bound - 1),
            y_separators,
            current_character_set.end_char,
        );

        let mut index: i32 = 1;
        while index < ((self.nr_val - 1) as i32 * self.sec_dim as i32).min(self.bound - 1) {
            if index % self.sec_dim as i32 == 0 {
                surface.write_char(index, y_separators, current_character_set.value_indicator);
            } else {
                surface.write_char(index, y_separators, current_character_set.separator);
            }
            index += 1;
        }
    }
}

impl<T> OnMouseEvent for HNumericSlider<T>
where
    T: Number + 'static,
{
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        match _event {
            MouseEvent::Enter => return EventProcessStatus::Processed,
            MouseEvent::Leave => return EventProcessStatus::Processed,
            MouseEvent::Over(point) => return EventProcessStatus::Ignored,
            MouseEvent::Released(mouse_event_data) => return EventProcessStatus::Ignored,
            MouseEvent::DoubleClick(mouse_event_data) => return EventProcessStatus::Ignored,
            MouseEvent::Wheel(mouse_wheel_direction) => return EventProcessStatus::Ignored,
            MouseEvent::Pressed(mouse_event_data) | MouseEvent::Drag(mouse_event_data) => {
                if mouse_event_data.button != MouseButton::Left {
                    return EventProcessStatus::Ignored;
                }
                self.last_pressed_coods.x = mouse_event_data.x;
                self.last_pressed_coods.y = mouse_event_data.y;
                self.update_cursor_pos(mouse_event_data.x.clamp(0, self.bound - 1));
                return EventProcessStatus::Processed;
            }
        };
    }
}

impl<T> OnKeyPressed for HNumericSlider<T>
where
    T: Number + 'static,
{
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        match _key.code {
            KeyCode::Left => {
                self.value = Self::to_interval(self.value - self.ok_step, self.min, self.computed_max);
                self.poz_triunghi = (((self.value - self.min) / self.ok_step).cast_to_u32() * self.sec_dim as u32) as i32;
                self.emit_changed_event();
                return EventProcessStatus::Processed;
            }
            KeyCode::Right => {
                self.value = Self::to_interval(self.value + self.ok_step, self.min, self.computed_max);
                self.poz_triunghi = (((self.value - self.min) / self.ok_step).cast_to_u32() * self.sec_dim as u32) as i32;
                self.emit_changed_event();
                return EventProcessStatus::Processed;
            }
            KeyCode::Home => {
                self.value = self.min;
                self.poz_triunghi = 0;
                self.emit_changed_event();
                return EventProcessStatus::Processed;
            }
            KeyCode::End => {
                self.value = self.computed_max;
                self.poz_triunghi = (((self.value - self.min) / self.ok_step).cast_to_u32() * self.sec_dim as u32) as i32;
                self.emit_changed_event();
                return EventProcessStatus::Processed;
            }
            _ => return EventProcessStatus::Ignored,
        };
    }
}

// implement update_value care emite si event
impl<T> OnResize for HNumericSlider<T>
where
    T: Number + 'static,
{
    fn on_resize(&mut self, old_size: Size, new_size: Size) {
        self.bound = new_size.width as i32;
        self.compute_math_fields();

        self.poz_triunghi = (((self.value - self.min) / self.ok_step).cast_to_u32() * self.sec_dim as u32) as i32;
        let temp = self.value;
        self.value = Self::to_interval(
            self.min + self.ok_step * ((self.value - self.min) / self.ok_step),
            self.min,
            self.computed_max,
        );
        if temp != self.value {
            self.emit_changed_event();
        }
    }
}
