use super::{
    events::{EventData, TimePickerEventsType},
    Flags,
};
use crate::prelude::*;
use crate::ui::common::{ControlEvent, ControlEventData};
use crate::input::KeyCode;

#[derive(Clone, Copy, PartialEq)]
enum TimeComponent {
    Hour,
    Minute,
    Second,
    AmPm,
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize+OnFocus, internal=true)]
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
    /// Creates a new TimePicker control with the specified time, layout and flags.
    /// The flags can be a combination of the following values:
    /// * `Flags::AMPM` - if set, the time picker will use 12-hour format with AM/PM
    /// * `Flags::Seconds` - if set, the time picker will show and allow editing seconds
    ///
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut timepicker = TimePicker::new(14, 30, 0, 
    ///                                     layout!("x:1,y:1,w:10,h:1"),
    ///                                     timepicker::Flags::AMPM | timepicker::Flags::Seconds);
    /// ```
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
        
        let min_width = if flags.contains(Flags::AMPM) {
            if flags.contains(Flags::Seconds) { 11 } else { 8 }  // HH:MM:SS AM or HH:MM AM
        } else {
            if flags.contains(Flags::Seconds) { 8 } else { 5 }   // HH:MM:SS or HH:MM
        };
        
        obj.set_size_bounds(min_width, 1, u16::MAX, 1);
        obj
    }

    /// Gets the current time in 24-hour format
    pub fn get_time(&self) -> (u8, u8, u8) {
        let hour_24 = if self.flags.contains(Flags::AMPM) {
            if self.hour == 12 {
                if self.is_pm { 12 } else { 0 }
            } else {
                if self.is_pm { self.hour + 12 } else { self.hour }
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
        
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::TimePicker(EventData {
                evtype: TimePickerEventsType::OnTimeChanged,
            }),
        });
    }

    fn format_time(&self) -> String {
        if self.flags.contains(Flags::Seconds) {
            if self.flags.contains(Flags::AMPM) {
                format!("{:02}:{:02}:{:02} {}", 
                    self.hour, self.minute, self.second,
                    if self.is_pm { "PM" } else { "AM" })
            } else {
                format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
            }
        } else {
            if self.flags.contains(Flags::AMPM) {
                format!("{:02}:{:02} {}", 
                    self.hour, self.minute,
                    if self.is_pm { "PM" } else { "AM" })
            } else {
                format!("{:02}:{:02}", self.hour, self.minute)
            }
        }
    }

    fn get_component_range(&self, component: TimeComponent) -> (u8, u8) {
        match component {
            TimeComponent::Hour => {
                if self.flags.contains(Flags::AMPM) {
                    (1, 12)
                } else {
                    (0, 23)
                }
            },
            TimeComponent::Minute => (0, 59),
            TimeComponent::Second => (0, 59),
            TimeComponent::AmPm => (0, 1),
        }
    }

    fn get_component_positions(&self) -> Vec<(TimeComponent, usize, usize)> {
        let mut positions = Vec::new();
        
        // Hour: positions 0-1
        positions.push((TimeComponent::Hour, 0, 2));
        
        // Minute: positions 3-4
        positions.push((TimeComponent::Minute, 3, 2));
        
        // Second: positions 6-7 (if enabled)
        if self.flags.contains(Flags::Seconds) {
            positions.push((TimeComponent::Second, 6, 2));
        }
        
        // AM/PM: positions after seconds or minutes
        if self.flags.contains(Flags::AMPM) {
            let start_pos = if self.flags.contains(Flags::Seconds) { 9 } else { 6 };
            positions.push((TimeComponent::AmPm, start_pos, 2));
        }
        
        positions
    }

    fn increment_component(&mut self, component: TimeComponent, amount: i32) {
        let (min_val, max_val) = self.get_component_range(component);
        
        match component {
            TimeComponent::Hour => {
                let new_val = ((self.hour as i32 + amount - min_val as i32).rem_euclid((max_val - min_val + 1) as i32) + min_val as i32) as u8;
                self.hour = new_val;
            },
            TimeComponent::Minute => {
                let new_val = ((self.minute as i32 + amount).rem_euclid(60)) as u8;
                self.minute = new_val;
            },
            TimeComponent::Second => {
                let new_val = ((self.second as i32 + amount).rem_euclid(60)) as u8;
                self.second = new_val;
            },
            TimeComponent::AmPm => {
                self.is_pm = !self.is_pm;
            },
        }
        
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::TimePicker(EventData {
                evtype: TimePickerEventsType::OnTimeChanged,
            }),
        });
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
                            self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::TimePicker(EventData {
                evtype: TimePickerEventsType::OnTimeChanged,
            }),
        });
                        }
                    } else {
                        if digit_val <= 2 {
                            // Could be 20-23
                            self.hour = digit_val;
                        } else {
                            // Single digit hour (3-9)
                            self.hour = digit_val;
                            self.input_buffer.clear();
                            self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::TimePicker(EventData {
                evtype: TimePickerEventsType::OnTimeChanged,
            }),
        });
                        }
                    }
                } else if self.input_buffer.len() == 2 {
                    // Second digit
                    let first_digit = self.input_buffer.chars().nth(0).unwrap().to_digit(10).unwrap() as u8;
                    let new_hour = first_digit * 10 + digit_val;
                    
                    if new_hour >= min_val && new_hour <= max_val {
                        self.hour = new_hour;
                        self.input_buffer.clear();
                        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::TimePicker(EventData {
                evtype: TimePickerEventsType::OnTimeChanged,
            }),
        });
                    }
                }
            },
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
                        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::TimePicker(EventData {
                evtype: TimePickerEventsType::OnTimeChanged,
            }),
        });
                    }
                } else if self.input_buffer.len() == 2 {
                    let first_digit = self.input_buffer.chars().nth(0).unwrap().to_digit(10).unwrap() as u8;
                    let new_val = first_digit * 10 + digit_val;
                    
                    if new_val <= 59 {
                        *current_val = new_val;
                        self.input_buffer.clear();
                        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::TimePicker(EventData {
                evtype: TimePickerEventsType::OnTimeChanged,
            }),
        });
                    }
                }
            },
            TimeComponent::AmPm => {
                // Toggle AM/PM on any digit
                self.is_pm = !self.is_pm;
                self.input_buffer.clear();
                self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::TimePicker(EventData {
                evtype: TimePickerEventsType::OnTimeChanged,
            }),
        });
            },
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
            },
            TimeComponent::Second => {
                if self.flags.contains(Flags::AMPM) {
                    TimeComponent::AmPm
                } else {
                    TimeComponent::Hour
                }
            },
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
            },
            TimeComponent::Minute => TimeComponent::Hour,
            TimeComponent::Second => TimeComponent::Minute,
            TimeComponent::AmPm => {
                if self.flags.contains(Flags::Seconds) {
                    TimeComponent::Second
                } else {
                    TimeComponent::Minute
                }
            },
        };
        self.input_buffer.clear();
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
        
        let time_str = self.format_time();
        let show_cursor = self.has_focus();
        let positions = self.get_component_positions();
        
        // Draw the time string
        for (i, ch) in time_str.chars().enumerate() {
            let mut char_attr = attr;
            
            // Highlight the selected component
            if show_cursor {
                for (component, start_pos, length) in &positions {
                    if *component == self.selected_component && i >= *start_pos && i < start_pos + length {
                        char_attr = theme.editor.pressed_or_selectd;
                        break;
                    }
                }
            }
            
            surface.write_char(i as i32 + 1, 0, Character::with_attributes(ch, char_attr));
        }
        
        // Set cursor position on the selected component
        if show_cursor {
            for (component, start_pos, _) in &positions {
                if *component == self.selected_component {
                    surface.set_cursor(*start_pos as i32 + 1, 0);
                    break;
                }
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
            },
            KeyCode::Down => {
                self.increment_component(self.selected_component, -1);
                EventProcessStatus::Processed
            },
            KeyCode::PageUp => {
                let increment = match self.selected_component {
                    TimeComponent::Hour => 6,
                    TimeComponent::Minute | TimeComponent::Second => 10,
                    TimeComponent::AmPm => 1,
                };
                self.increment_component(self.selected_component, increment);
                EventProcessStatus::Processed
            },
            KeyCode::PageDown => {
                let increment = match self.selected_component {
                    TimeComponent::Hour => 6,
                    TimeComponent::Minute | TimeComponent::Second => 10,
                    TimeComponent::AmPm => 1,
                };
                self.increment_component(self.selected_component, -increment);
                EventProcessStatus::Processed
            },
            KeyCode::Left => {
                self.move_to_prev_component();
                EventProcessStatus::Processed
            },
            KeyCode::Right | KeyCode::Tab => {
                self.move_to_next_component();
                EventProcessStatus::Processed
            },
            _ => {
                if _character.is_ascii_digit() {
                    self.process_digit_input(_character);
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            },
        }
    }
}

impl OnMouseEvent for TimePicker {
    fn on_mouse_event(&mut self, mouse_event: &MouseEvent) -> EventProcessStatus {
        if !self.is_enabled() {
            return EventProcessStatus::Ignored;
        }

        match mouse_event {
            MouseEvent::Pressed(data) if data.button == MouseButton::Left => {
                let positions = self.get_component_positions();
                let click_x = data.x as usize - 1; // Adjust for border
                
                // Find which component was clicked
                for (component, start_pos, length) in positions {
                    if click_x >= start_pos && click_x < start_pos + length {
                        self.selected_component = component;
                        self.input_buffer.clear();
                        return EventProcessStatus::Processed;
                    }
                }
            },
            _ => {}
        }
        
        EventProcessStatus::Ignored
    }
}

impl OnResize for TimePicker {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {}
}

impl OnFocus for TimePicker {
    fn on_focus(&mut self) {
        self.input_buffer.clear();
    }
}
