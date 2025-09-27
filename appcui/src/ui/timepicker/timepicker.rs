use super::{events::EventData, Flags};
use crate::input::KeyCode;
use crate::prelude::*;
use crate::ui::common::{ControlEvent, ControlEventData};
use chrono::{NaiveTime, Timelike};

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
    flags: Flags,
    selected_component: TimeComponent,
    editable_digit_is_first: bool,
    hovered_component: Option<TimeComponent>,
}

impl TimePicker {
    /// Creates a new TimePicker with the given time string (e.g. "12:34:56") and layout
    /// 
    /// # Arguments
    /// 
    /// * `time_str` - The time string to parse
    /// * `layout` - The layout to use
    /// * `flags` - The flags to use
    /// 
    /// # Example
    /// ```rust,no_run
    /// use appcui::prelude::*;
    /// 
    /// let timepicker = TimePicker::new("12:34:56", 
    ///                                 layout!("x:1,y:1,w:10"), 
    ///                                 timepicker::Flags::Seconds);
    /// ```
    pub fn new(time_str: &str, layout: Layout, flags: Flags) -> Self {
        let time = time_str.parse::<NaiveTime>().unwrap();
        Self::with_time(time, layout, flags)
    }

    /// Creates a new TimePicker with the given time and layout
    /// 
    /// # Arguments
    /// 
    /// * `time` - The time to use
    /// * `layout` - The layout to use
    /// * `flags` - The flags to use
    /// 
    /// # Example
    /// ```rust,no_run
    /// use appcui::prelude::*;
    /// use chrono::NaiveTime;
    /// 
    /// let timepicker = TimePicker::with_time(
    ///     NaiveTime::from_hms_opt(12, 34, 56).unwrap(), 
    ///     layout!("x:1,y:1,w:10"), 
    ///     timepicker::Flags::Seconds);
    /// ```
    pub fn with_time(time: NaiveTime, layout: Layout, flags: Flags) -> Self {
        let hour = time.hour().min(23) as u8;
        let minute = time.minute().min(59) as u8;
        let second = time.second().min(59) as u8;

        let mut obj = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            hour,
            minute,
            second,
            flags,
            selected_component: TimeComponent::Hour,
            editable_digit_is_first: true,
            hovered_component: None,
        };

        let min_width = 7 + if flags.contains(Flags::AMPM) { 3 } else { 0 } + if flags.contains(Flags::Seconds) { 3 } else { 0 };
        obj.set_size_bounds(min_width, 1, u16::MAX, 1);
        obj
    }

    fn raise_time_changed_event(&mut self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::TimePicker(EventData {
                time: self.time(),
            }),
        });
    }

    /// Gets the current time
    #[inline(always)]
    pub fn time(&self) -> NaiveTime {
        NaiveTime::from_hms_opt(self.hour as u32, self.minute as u32, self.second as u32).unwrap()
    }

    /// Sets the time
    pub fn set_time(&mut self, time: NaiveTime) {
        self.hour = time.hour().min(23) as u8;
        self.minute = time.minute().min(59) as u8;
        self.second = time.second().min(59) as u8;
    }

    fn mouse_pos_to_component(&self, x: i32, y: i32) -> Option<TimeComponent> {
        if y != 0 {
            return None;
        }
        if !self.is_enabled() {
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
    fn update_value(value: u8, max: u8, increment: bool) -> u8 {
        let value = value % max;
        if increment {
            (value + 1) % max
        } else {
            (value + max - 1) % max
        }
    }
    fn update_value_withdigit(value: u8, max: u8, digit: u8, first_digit: bool) -> Option<u8> {
        if first_digit {
            if digit > max / 10 {
                None
            } else {
                Some((digit * 10 + (value % 10)).min(max))
            }
        } else {
            let new_value = digit + ((value / 10) * 10);
            if new_value > max {
                None
            } else {
                Some(new_value)
            }
        }
    }

    fn update_digit(&mut self, digit: u8) {
        let (value, max) = match self.selected_component {
            TimeComponent::Hour => (self.hour, 23),
            TimeComponent::Minute => (self.minute, 59),
            TimeComponent::Second => (self.second, 59),
            TimeComponent::AmPm => {
                return;
            }
        };
        if let Some(v) = Self::update_value_withdigit(value, max, digit, self.editable_digit_is_first) {
            match self.selected_component {
                TimeComponent::Hour => self.hour = v,
                TimeComponent::Minute => self.minute = v,
                TimeComponent::Second => self.second = v,
                TimeComponent::AmPm => {}
            }
            self.editable_digit_is_first = !self.editable_digit_is_first;
            if self.editable_digit_is_first {
                self.move_to_next_component();
            }
            self.raise_time_changed_event();
        }
    }
    fn increment_decrement_selected_component(&mut self, increment: bool) {
        match self.selected_component {
            TimeComponent::Hour => {
                self.hour = Self::update_value(self.hour, 24, increment);
            }
            TimeComponent::Minute => {
                self.minute = Self::update_value(self.minute, 60, increment);
            }
            TimeComponent::Second => {
                self.second = Self::update_value(self.second, 60, increment);
            }
            TimeComponent::AmPm => {
                // referse the hour to reflect AM or PM
                if self.hour >= 12 {
                    self.hour -= 12;
                } else {
                    self.hour += 12;
                }
                self.raise_time_changed_event();
            }
        }
        self.raise_time_changed_event();
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
        self.editable_digit_is_first = true;
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
        self.editable_digit_is_first = true;
    }
    #[inline(always)]
    fn paint_number(&self, surface: &mut Surface, theme: &Theme, x: i32, comp: TimeComponent, attr: CharAttribute, has_focus: bool) {
        let num = match comp {
            TimeComponent::Hour => {
                if self.flags.contains(Flags::AMPM) {
                    match self.hour % 12 {
                        0 => 12,
                        n => n,
                    }
                } else {
                    self.hour
                }
            }
            TimeComponent::Minute => self.minute,
            TimeComponent::Second => self.second,
            TimeComponent::AmPm => 0,
        };
        let buf: [u8; 2] = [48 + num / 10, 48 + num % 10];
        let (attr, show_cursor) = if has_focus {
            if comp == self.selected_component {
                (theme.editor.pressed_or_selectd, true)
            } else if Some(comp) == self.hovered_component {
                (theme.editor.hovered, false)
            } else {
                (attr, false)
            }
        } else if Some(comp) == self.hovered_component {
            (theme.editor.hovered, false)
        } else {
            (attr, false)
        };
        surface.write_ascii(x, 0, &buf, attr, false);
        if show_cursor {
            surface.set_cursor(x + if self.editable_digit_is_first { 0 } else { 1 }, 0);
        }
    }
}

impl OnPaint for TimePicker {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let attr = match () {
            _ if !self.is_enabled() => theme.editor.inactive,
            _ if self.has_focus() => theme.editor.focused,
            _ => theme.editor.normal,
        };
        surface.clear(Character::with_attributes(' ', attr));

        let has_focus = self.has_focus();
        let sep = Character::with_attributes(':', theme.editor.inactive);

        let mut x = 1;

        // hour
        self.paint_number(surface, theme, x, TimeComponent::Hour, attr, has_focus);
        surface.write_char(x + 2, 0, sep);
        x += 3;

        // minute
        self.paint_number(surface, theme, x, TimeComponent::Minute, attr, has_focus);
        x += 2;

        // second
        if self.flags.contains(Flags::Seconds) {
            surface.write_char(x, 0, sep);
            x += 1;
            self.paint_number(surface, theme, x, TimeComponent::Second, attr, has_focus);
            x += 2;
        }
        if self.flags.contains(Flags::AMPM) {
            x += 1;
            let attr = if has_focus {
                if self.selected_component == TimeComponent::AmPm {
                    theme.editor.pressed_or_selectd
                } else if Some(TimeComponent::AmPm) == self.hovered_component {
                    theme.editor.hovered
                } else {
                    attr
                }
            } else if Some(TimeComponent::AmPm) == self.hovered_component {
                theme.editor.hovered
            } else {
                attr
            };
            if self.hour >= 12 {
                surface.write_ascii(x, 0, b"PM", attr, false);
            } else {
                surface.write_ascii(x, 0, b"AM", attr, false);
            }
        }
    }
}

impl OnKeyPressed for TimePicker {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        match key.code {
            KeyCode::Up => {
                self.increment_decrement_selected_component(true);
                EventProcessStatus::Processed
            }
            KeyCode::Down => {
                self.increment_decrement_selected_component(false);
                EventProcessStatus::Processed
            }
            KeyCode::Left => {
                self.move_to_prev_component();
                EventProcessStatus::Processed
            }
            KeyCode::Right => {
                self.move_to_next_component();
                EventProcessStatus::Processed
            }
            KeyCode::Backspace => {
                self.editable_digit_is_first = true;
                EventProcessStatus::Processed
            }
            _ => {
                if character.is_ascii_digit() {
                    self.update_digit((character as u8) - b'0');
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
                EventProcessStatus::Processed
            }
            MouseEvent::Enter | MouseEvent::Leave => {
                self.hovered_component = None;
                EventProcessStatus::Processed
            }
            MouseEvent::Over(point) => {
                let new_pos = self.mouse_pos_to_component(point.x, point.y);
                if new_pos != self.hovered_component {
                    self.hovered_component = new_pos;
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            MouseEvent::Wheel(dir) => {
                match dir {
                    MouseWheelDirection::Left => self.move_to_prev_component(),
                    MouseWheelDirection::Right => self.move_to_next_component(),
                    MouseWheelDirection::Up => self.increment_decrement_selected_component(true),
                    MouseWheelDirection::Down => self.increment_decrement_selected_component(false),
                };
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnFocus for TimePicker {
    fn on_focus(&mut self) {
        self.selected_component = TimeComponent::Hour;
        self.editable_digit_is_first = true;
    }
}
