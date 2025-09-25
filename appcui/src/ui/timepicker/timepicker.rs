use super::{events::EventData, Flags};
use crate::input::KeyCode;
use crate::prelude::*;
use crate::ui::common::{ControlEvent, ControlEventData};

#[derive(Clone, Copy, PartialEq)]
enum TimeComponent {
    Hour,
    Minute,
    Second,
    AmPm,
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnFocus, internal=true)]
pub struct TimePicker {
    hour: u8,
    minute: u8,
    second: u8,
    is_pm: bool,
    flags: Flags,
    selected_component: TimeComponent,
    input_buffer: String,
    input_timeout: u32,
}

impl TimePicker {
    pub fn new(hour: u8, minute: u8, second: u8, layout: Layout, flags: Flags) -> Self {
        let hour = hour.min(23);
        let minute = minute.min(59);
        let second = second.min(59);

        let (display_hour, is_pm) = if flags.contains(Flags::AMPM) {
            if hour == 0 {
                (12, false)
            } else if hour <= 12 {
                (hour, hour == 12)
            } else {
                (hour - 12, true)
            }
        } else {
            (hour, false)
        };

        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            hour: display_hour,
            minute,
            second,
            is_pm,
            flags,
            selected_component: TimeComponent::Hour,
            input_buffer: String::new(),
            input_timeout: 0,
        };

        let min_width = 5 + if flags.contains(Flags::AMPM) { 3 } else { 0 } + if flags.contains(Flags::Seconds) { 3 } else { 0 };
        obj.set_size_bounds(min_width, 1, u16::MAX, 1);
        obj
    }

    fn raise_time_changed_event(&mut self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::TimePicker(EventData {}),
        });
    }

    /// Gets the current time in 24-hour format
    pub fn get_time(&self) -> (u8, u8, u8) {
        let hour_24 = if self.flags.contains(Flags::AMPM) {
            if self.hour == 12 {
                if self.is_pm {
                    12
                } else {
                    0
                }
            } else {
                if self.is_pm {
                    self.hour + 12
                } else {
                    self.hour
                }
            }
        } else {
            self.hour
        };
        (hour_24, self.minute, self.second)
    }

    /// Sets the time in 24-hour format
    pub fn set_time(&mut self, hour: u8, minute: u8, second: u8) {
        let hour = hour.min(23);
        let minute = minute.min(59);
        let second = second.min(59);

        let (display_hour, is_pm) = if self.flags.contains(Flags::AMPM) {
            if hour == 0 {
                (12, false)
            } else if hour <= 12 {
                (hour, hour == 12)
            } else {
                (hour - 12, true)
            }
        } else {
            (hour, false)
        };

        self.hour = display_hour;
        self.minute = minute;
        self.second = second;
        self.is_pm = is_pm;

        self.raise_time_changed_event();
    }

    fn get_component_range(&self, component: TimeComponent) -> (u8, u8) {
        match component {
            TimeComponent::Hour => {
                if self.flags.contains(Flags::AMPM) {
                    (1, 12)
                } else {
                    (0, 23)
                }
            }
            TimeComponent::Minute => (0, 59),
            TimeComponent::Second => (0, 59),
            TimeComponent::AmPm => (0, 1),
        }
    }

    fn mouse_pos_to_component(&self, x: i32, y: i32) -> Option<TimeComponent> {
        if y != 0 {
            return None;
        }
        match x {
            1 | 2 => Some(TimeComponent::Hour),
            4 | 5 => Some(TimeComponent::Minute),
            7 | 8 => {
                if self.flags.contains_one(Flags::Seconds) {
                    Some(TimeComponent::Second)
                } else if self.flags.contains_one(Flags::AMPM) {
                    Some(TimeComponent::AmPm)
                } else {
                    None
                }
            }
            10 | 11 => {
                if self.flags & (Flags::Seconds | Flags::AMPM) == (Flags::Seconds | Flags::AMPM) {
                    Some(TimeComponent::AmPm)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn increment_component(&mut self, component: TimeComponent, amount: i32) {
        let (min_val, max_val) = self.get_component_range(component);

        match component {
            TimeComponent::Hour => {
                let new_val = ((self.hour as i32 + amount - min_val as i32).rem_euclid((max_val - min_val + 1) as i32) + min_val as i32) as u8;
                self.hour = new_val;
            }
            TimeComponent::Minute => {
                let new_val = ((self.minute as i32 + amount).rem_euclid(60)) as u8;
                self.minute = new_val;
            }
            TimeComponent::Second => {
                let new_val = ((self.second as i32 + amount).rem_euclid(60)) as u8;
                self.second = new_val;
            }
            TimeComponent::AmPm => {
                self.is_pm = !self.is_pm;
            }
        }

        self.raise_time_changed_event();
    }

    fn process_digit_input(&mut self, digit: char) {
        if !digit.is_ascii_digit() {
            return;
        }

        self.input_buffer.push(digit);
        self.input_timeout = 100; // Reset timeout

        let digit_val = digit.to_digit(10).unwrap() as u8;
        let (min_val, max_val) = self.get_component_range(self.selected_component);

        match self.selected_component {
            TimeComponent::Hour => {
                if self.input_buffer.len() == 1 {
                    // First digit
                    if self.flags.contains(Flags::AMPM) {
                        if digit_val >= 1 && digit_val <= 1 {
                            // Could be 10, 11, 12
                            self.hour = digit_val;
                        } else if digit_val >= 2 && digit_val <= 9 {
                            // Single digit hour (2-9)
                            self.hour = digit_val;
                            self.input_buffer.clear();
                            self.raise_time_changed_event();
                        }
                    } else {
                        if digit_val <= 2 {
                            // Could be 20-23
                            self.hour = digit_val;
                        } else {
                            // Single digit hour (3-9)
                            self.hour = digit_val;
                            self.input_buffer.clear();
                            self.raise_time_changed_event();
                        }
                    }
                } else if self.input_buffer.len() == 2 {
                    // Second digit
                    let first_digit = self.input_buffer.chars().nth(0).unwrap().to_digit(10).unwrap() as u8;
                    let new_hour = first_digit * 10 + digit_val;

                    if new_hour >= min_val && new_hour <= max_val {
                        self.hour = new_hour;
                        self.input_buffer.clear();
                        self.raise_time_changed_event();
                    }
                }
            }
            TimeComponent::Minute | TimeComponent::Second => {
                let current_val = if self.selected_component == TimeComponent::Minute {
                    &mut self.minute
                } else {
                    &mut self.second
                };

                if self.input_buffer.len() == 1 {
                    if digit_val <= 5 {
                        // Could be 50-59
                        *current_val = digit_val;
                    } else {
                        // Single digit (6-9)
                        *current_val = digit_val;
                        self.input_buffer.clear();
                        self.raise_time_changed_event();
                    }
                } else if self.input_buffer.len() == 2 {
                    let first_digit = self.input_buffer.chars().nth(0).unwrap().to_digit(10).unwrap() as u8;
                    let new_val = first_digit * 10 + digit_val;

                    if new_val <= 59 {
                        *current_val = new_val;
                        self.input_buffer.clear();
                        self.raise_time_changed_event();
                    }
                }
            }
            TimeComponent::AmPm => {
                // Toggle AM/PM on any digit
                self.is_pm = !self.is_pm;
                self.input_buffer.clear();
                self.raise_time_changed_event();
            }
        }
    }

    fn move_to_next_component(&mut self) {
        self.selected_component = match self.selected_component {
            TimeComponent::Hour => TimeComponent::Minute,
            TimeComponent::Minute => {
                if self.flags.contains(Flags::Seconds) {
                    TimeComponent::Second
                } else if self.flags.contains(Flags::AMPM) {
                    TimeComponent::AmPm
                } else {
                    TimeComponent::Hour
                }
            }
            TimeComponent::Second => {
                if self.flags.contains(Flags::AMPM) {
                    TimeComponent::AmPm
                } else {
                    TimeComponent::Hour
                }
            }
            TimeComponent::AmPm => TimeComponent::Hour,
        };
        self.input_buffer.clear();
    }

    fn move_to_prev_component(&mut self) {
        self.selected_component = match self.selected_component {
            TimeComponent::Hour => {
                if self.flags.contains(Flags::AMPM) {
                    TimeComponent::AmPm
                } else if self.flags.contains(Flags::Seconds) {
                    TimeComponent::Second
                } else {
                    TimeComponent::Minute
                }
            }
            TimeComponent::Minute => TimeComponent::Hour,
            TimeComponent::Second => TimeComponent::Minute,
            TimeComponent::AmPm => {
                if self.flags.contains(Flags::Seconds) {
                    TimeComponent::Second
                } else {
                    TimeComponent::Minute
                }
            }
        };
        self.input_buffer.clear();
    }
    #[inline(always)]
    fn paint_number(&self, surface: &mut Surface, x: i32, num: u8, attr: CharAttribute, show_cursor: bool) {
        let buf: [u8; 2] = [48 + num / 10, 48 + num % 10];
        surface.write_ascii(x, 0, &buf, attr, false);
    }
}

impl OnPaint for TimePicker {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let attr = match () {
            _ if !self.is_enabled() => theme.editor.inactive,
            _ if self.has_focus() => theme.editor.focused,
            _ if self.is_mouse_over() => theme.editor.hovered,
            _ => theme.editor.normal,
        };
        surface.clear(Character::with_attributes(' ', attr));

        let attr_selected = theme.editor.pressed_or_selectd;
        let has_focus = self.has_focus();
        let sep = Character::with_attributes(':', theme.editor.inactive);

        let mut x = 1;

        // hour
        self.paint_number(
            surface,
            x,
            self.hour,
            if has_focus && self.selected_component == TimeComponent::Hour {
                attr_selected
            } else {
                attr
            },
            false,
        );
        surface.write_char(x + 2, 0, sep);
        x += 3;

        // minute
        self.paint_number(
            surface,
            x,
            self.minute,
            if has_focus && self.selected_component == TimeComponent::Minute {
                attr_selected
            } else {
                attr
            },
            false,
        );
        surface.write_char(x + 2, 0, sep);
        x += 3;

        // second
        if self.flags.contains(Flags::Seconds) {
            self.paint_number(
                surface,
                x,
                self.second,
                if has_focus && self.selected_component == TimeComponent::Second {
                    attr_selected
                } else {
                    attr
                },
                false,
            );
            x += 2;
        }
        if self.flags.contains(Flags::AMPM) {
            x += 1;
            let attr = if has_focus && self.selected_component == TimeComponent::AmPm {
                theme.editor.pressed_or_selectd
            } else {
                attr
            };
            if self.hour > 12 {
                surface.write_ascii(x, 0, b"PM", attr, false);
            } else {
                surface.write_ascii(x, 0, b"AM", attr, false);
            }
        }
    }
}

impl OnKeyPressed for TimePicker {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        if !self.is_enabled() {
            return EventProcessStatus::Ignored;
        }

        self.input_timeout = self.input_timeout.saturating_sub(1);
        if self.input_timeout == 0 {
            self.input_buffer.clear();
        }

        match key.code {
            KeyCode::Up => {
                self.increment_component(self.selected_component, 1);
                EventProcessStatus::Processed
            }
            KeyCode::Down => {
                self.increment_component(self.selected_component, -1);
                EventProcessStatus::Processed
            }
            KeyCode::PageUp => {
                let increment = match self.selected_component {
                    TimeComponent::Hour => 6,
                    TimeComponent::Minute | TimeComponent::Second => 10,
                    TimeComponent::AmPm => 1,
                };
                self.increment_component(self.selected_component, increment);
                EventProcessStatus::Processed
            }
            KeyCode::PageDown => {
                let increment = match self.selected_component {
                    TimeComponent::Hour => 6,
                    TimeComponent::Minute | TimeComponent::Second => 10,
                    TimeComponent::AmPm => 1,
                };
                self.increment_component(self.selected_component, -increment);
                EventProcessStatus::Processed
            }
            KeyCode::Left => {
                self.move_to_prev_component();
                EventProcessStatus::Processed
            }
            KeyCode::Right | KeyCode::Tab => {
                self.move_to_next_component();
                EventProcessStatus::Processed
            }
            _ => {
                if _character.is_ascii_digit() {
                    self.process_digit_input(_character);
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
        }
    }
}

impl OnMouseEvent for TimePicker {
    fn on_mouse_event(&mut self, mouse_event: &MouseEvent) -> EventProcessStatus {
        if !self.is_enabled() {
            return EventProcessStatus::Ignored;
        }

        match mouse_event {
            MouseEvent::Pressed(data) => {
                if let Some(new_pos) = self.mouse_pos_to_component(data.x, data.y) {
                    self.selected_component = new_pos;
                }
                return EventProcessStatus::Processed;
            }
            _ => {}
        }

        EventProcessStatus::Ignored
    }
}

impl OnFocus for TimePicker {
    fn on_focus(&mut self) {
        self.input_buffer.clear();
    }
}
