use std::cmp::min;
use std::string;
use std::thread::current;

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

#[CustomControl(overwrite: [OnPaint, OnMouseEvent, OnResize], internal=true)]
pub struct NumericSlider<T>
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
    m: usize, //max size la valori
    nr_val: u32, //cate valori am in interval
    sec_dim : f32, // dimensiunea unei sectiuni
    y: f32, // dimensiunea secventei m + spatiu + m + spatiu +...+ m in size
    o: u32, // padding-ul necesar pentru prima sectiune
    p: u32, // padding-ul necesar pentru restul sectiunilor
    values_string: String
}
impl<T> NumericSlider<T>
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

    pub fn new(min: T, max: T, step: T, value: T, format: Format, layout: Layout, flags: Flags) -> Self {
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
            values_string: String::new()
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

        self.y = (self.bound + 1) as f32
          / (self.m + 1) as f32;

        self.nr_val = ((self.max - self.min) / self.step).cast_to_u32() + 1;
        self.sec_dim = (self.bound as f32) / ((self.nr_val - 1) as f32);
        self.p = (self.sec_dim - self.m as f32) as u32;
        self.o = (self.sec_dim - (self.m as f32 + ((self.m / 2) as f32))) as u32;

        if self.p >= 1 && self.o >= 1 {
            self.ok_step = self.step; //am suficient spatiu
        }
        else {
            self.ok_step = (self.max - self.min) / T::cast_float_number(self.y as f64);
            self.nr_val = min(((self.max - self.min) / self.ok_step).cast_to_u32() + 1, self.y as u32);
            self.sec_dim = (self.bound as f32) / ((self.nr_val - 1) as f32);
            self.p = (self.sec_dim - self.m as f32) as u32;
            self.o = (self.sec_dim - (self.m as f32 + ((self.m / 2) as f32))) as u32;
        }

        self.values_string.clear();
        let mut current_value: T = self.min;

        // prima oara trebuie sa adun o
        current_value.write_to_string(&mut string_buffer, self.format);
        self.values_string.push_str(&string_buffer);
        self.values_string.push_str(&Self::get_n_spaces((self.o as usize + self.m - string_buffer.len()) as u32));

        current_value = current_value + self.ok_step;


        for i in 1..self.nr_val - 1 {
            current_value.write_to_string(&mut string_buffer, self.format);
            let space_debt: u32 = (self.m - string_buffer.len()) as u32;
            string_buffer = Self::get_n_spaces(space_debt / 2 + space_debt % 2) + &string_buffer + &Self::get_n_spaces(space_debt / 2);
            
            self.values_string.push_str(&string_buffer);

            if i != self.nr_val - 2{
                self.values_string.push_str(&Self::get_n_spaces(self.p));
            }

            current_value = current_value + self.ok_step;
        }
        self.values_string.push_str(&Self::get_n_spaces(self.o));
        
        current_value.write_to_string(&mut string_buffer, self.format);
        let space_debt: u32 = (self.m - string_buffer.len()) as u32 + (self.m % 2 == 0) as u32;
        string_buffer = Self::get_n_spaces(space_debt) + &string_buffer;
        self.values_string.push_str(&string_buffer);


        //println!("max = {}, min = {}, step = {}, size = {}, m = {}, y = {}, nr_val = {}, sec_dim = {}, p = {}, o = {}, ok_step = {}", self.max, self.min, self.step, self.bound, self.m, self.y, self.nr_val, self.sec_dim, self.p, self.o, self.ok_step);
        //panic!("test");
    }

    fn get_charset_based_on_theme(&self, theme: &Theme) -> CharSet {
        let mut set: CharSet = CharSet {
            start_char: Character::new(
                SpecialChar::BoxTopLeftCornerSingleLine,
                theme.border.normal.foreground,
                theme.border.normal.background,
                theme.lines.normal.flags,
            ),
            separator: Character::new(
                SpecialChar::BoxHorizontalSingleLine,
                theme.border.normal.foreground,
                theme.border.normal.background,
                theme.lines.normal.flags,
            ),
            value_indicator: Character::new(
                SpecialChar::SingleLineDownT,
                theme.border.normal.foreground,
                theme.border.normal.background,
                theme.lines.normal.flags,
            ),
            end_char: Character::new(
                SpecialChar::BoxTopRightCornerSingleLine,
                theme.border.normal.foreground,
                theme.border.normal.background,
                theme.lines.normal.flags,
            ),
            selected_value_indicator: Character::new(
                SpecialChar::TriangleDown,
                theme.lines.pressed_or_selectd.foreground,
                theme.lines.normal.background,
                theme.lines.normal.flags,
            ),
        };
        if self.flags.contains(Flags::SingleLine | Flags::HorizontalSlider) {
            return set;
        }
        panic!("Invalid flags received for character set!");
    }

    pub fn set_selected_value(&mut self, value: T) {
        self.value = value;
    }
    pub fn get_selected_value(&self) -> T {
        self.value
    }

    fn update_cursor_pos(&mut self, x: i32){
        self.poz_triunghi = (x / self.sec_dim as i32) * self.sec_dim as i32;
        self.value = self.min + self.ok_step * Number::cast_signed_number((x / self.sec_dim as i32) as i128);
    }
}
impl<T> OnPaint for NumericSlider<T>
where
    T: Number + 'static,
{

    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let current_character_set: CharSet = self.get_charset_based_on_theme(theme);
        surface.write_char(self.poz_triunghi, 0, current_character_set.selected_value_indicator);
        surface.write_string(0, 2, &self.values_string, theme.text.normal, false);

        surface.write_string(3, 0, &self.value.to_string(), theme.text.normal, false);

        //desenez marginea pentru min si max
        surface.write_char(0, 1, current_character_set.start_char);
        surface.write_char(((self.nr_val - 1) * self.sec_dim as u32) as i32, 1, current_character_set.end_char);

        let mut index: i32 = 1;
        while index < ((self.nr_val - 1) as i32 * self.sec_dim as i32) {
            if index % self.sec_dim as i32 == 0 {
                surface.write_char(index, 1, current_character_set.value_indicator);
            }
            else {
                surface.write_char(index, 1, current_character_set.separator);
            }
            index += 1;
        }
    }
}

impl<T> OnMouseEvent for NumericSlider<T>
where
    T: Number + 'static,
{
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        match _event {
            MouseEvent::Enter => return EventProcessStatus::Ignored,
            MouseEvent::Leave => return EventProcessStatus::Ignored,
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
                self.update_cursor_pos(mouse_event_data.x);
                return EventProcessStatus::Processed;
            }
        };
    }
}

impl<T> OnResize for NumericSlider<T>
where
    T: Number + 'static,
{
    fn on_resize(&mut self, old_size: Size, new_size: Size) {
        self.bound = {
            if self.flags.contains(Flags::HorizontalSlider) {
                new_size.width as i32
            } else {
                new_size.height as i32
            }
        };
        self.compute_math_fields();

        self.poz_triunghi = (((self.value - self.min) / self.ok_step).cast_to_u32() * self.sec_dim as u32) as i32;
        self.value = Self::to_interval(self.min + self.ok_step * ((self.value - self.min) / self.ok_step), self.min, self.max);
    }
}
