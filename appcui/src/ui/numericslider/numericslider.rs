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
        };
        control.set_size_bounds(2, 3, u16::MAX, 3);
        control
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

    fn get_values_count_with_custom_step(&self, step: T) -> u32{
        return ((self.max - self.min) / step).cast_to_u32() + 1;
    }

    fn find_ok_step(&self, bound: i32) -> T {
        let mut max_value = String::new();
        self.max.write_to_string(&mut max_value, self.format);

        let size_per_entry = max_value.len() + 1; // las un spatiu intre numere
        let mut current_step = self.step;
        let mut total_size = self.get_values_count_with_custom_step(current_step) * size_per_entry as u32;

        if total_size <= bound as u32 {
            return self.step;
        }

        while total_size > bound as u32 && current_step < self.max {
            current_step = current_step + T::one();
            total_size = self.get_values_count_with_custom_step(current_step) * size_per_entry as u32;
        }

        current_step
    }
}
impl<T> OnPaint for NumericSlider<T>
where
    T: Number + 'static,
{
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        //surface.clear(Character::new(SpecialChar::SingleLineDownT, Color::Blue, Color::Black, CharFlags::None));
        surface.clear(char!("' ',red,black"));
        let current_character_set: CharSet = self.get_charset_based_on_theme(theme);
        let bound: i32 = {
            if (self.flags.contains(Flags::HorizontalSlider)) {
                surface.size().width as i32
            } else {
                surface.size().height as i32
            }
        };

        let mut string_buffer = String::with_capacity(32);

        self.max.write_to_string(&mut string_buffer, self.format);
        let mut max_size: usize = string_buffer.len();
        let ok_step = self.find_ok_step(bound);

        let mut selected_value_as_str: String = String::new();
        self.value.write_to_string(&mut selected_value_as_str, self.format);

        //printez valorile
        // x = coloana
        // y = rand
        let mut current_value = self.min;

        let mut value_X = 0;
        let value_Y = 2;
        let mut last_column = 0;
        let mut first_column = 0;
        // for value in values.iter() {
        //     surface.write_string(value_X, value_Y, value, theme.text.normal, false);

        //     let indicator_pos_X = value_X + (value.len() as i32) / 2;
        //     if index == 0 {
        //         surface.write_char(indicator_pos_X, value_Y - 1, current_character_set.start_char);
        //         first_column = indicator_pos_X;
        //     } else if index == ((values.len() - 1) as i32) {
        //         surface.write_char(indicator_pos_X, value_Y - 1, current_character_set.end_char);
        //         last_column = indicator_pos_X;
        //     } else {
        //         surface.write_char(indicator_pos_X, value_Y - 1, current_character_set.value_indicator);
        //     }

        //     if (*value == selected_value_as_str) {
        //         surface.write_char(indicator_pos_X, value_Y - 2, current_character_set.selected_value_indicator);
        //     }

        //     value_X = value_X + value.len() as i32 + ((max_size - value.len() + 2) as i32);
        //     index += 1;
        // }

        while current_value < self.max {
            current_value.write_to_string(&mut string_buffer, self.format);

            surface.write_string(value_X, value_Y, &string_buffer, theme.text.normal, false);

            let indicator_pos_X = value_X + (string_buffer.len() as i32) / 2;
            if current_value == self.min {
                surface.write_char(indicator_pos_X, value_Y - 1, current_character_set.start_char);
                first_column = indicator_pos_X;
            } else if (current_value + ok_step) >= self.max {
                surface.write_char(indicator_pos_X, value_Y - 1, current_character_set.end_char);
                last_column = indicator_pos_X;
            } else {
                surface.write_char(indicator_pos_X, value_Y - 1, current_character_set.value_indicator);
            }

            if *string_buffer == selected_value_as_str {
                surface.write_char(indicator_pos_X, value_Y - 2, current_character_set.selected_value_indicator);
            }

            value_X = value_X + string_buffer.len() as i32 + ((max_size - string_buffer.len() + 1) as i32);
            current_value = current_value + ok_step;
        }

        for i in first_column+1..last_column {
            let mut write_separator = false;
            match surface.get(i, value_Y - 1) {
                Some(v) => {
                    if v.code == current_character_set.start_char.code
                        || v.code == current_character_set.end_char.code
                        || v.code == current_character_set.value_indicator.code
                    {
                        write_separator = false;
                    } else {
                        write_separator = true;
                    }
                }
                None => write_separator = true,
            }
            if write_separator {
                surface.write_char(i, value_Y - 1, current_character_set.separator);
            }
        }
    }
}

impl<T> OnMouseEvent for NumericSlider<T>
where
    T: Number + 'static,
{
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

impl<T> OnResize for NumericSlider<T>
where
    T: Number + 'static,
{
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {}
}
