use core::str;
use std::u16;
use chrono::prelude::*;
use chrono::TimeDelta;

use super::Flags;
use crate::prelude::*;

trait MillisecondsSupport {
    fn milliseconds(&self) -> u32;
}

static MIDNIGHT: Option<NaiveTime> = NaiveTime::from_hms_opt(0, 0, 0);
impl MillisecondsSupport for NaiveTime {
    fn milliseconds(&self) -> u32 {
        // Get the number of milliseconds since midnight
        let duration = self.signed_duration_since(MIDNIGHT.unwrap());
        
        // Convert the duration to milliseconds
        (duration.num_milliseconds() % 1000).try_into().unwrap() 
    }
}

#[derive(Copy, PartialEq, Clone)]
enum Components {
    H,
    M,
    S,
    Ms,
    AMPM,
}

#[CustomControl(overwrite=[OnPaint,OnKeyPressed,OnMouseEvent],internal=true)]
pub struct TimePicker {
    flags: Flags,
    time: NaiveTime,
    list_of_comp: Vec<Components>,

    cursor: u8,
    hover: u8,
    inscope: Option<Components>,
}

impl TimePicker {
    #[inline]
    fn get_min_width(&self) -> u16 {
        match () {
            _ if self.flags.contains(Flags::Miliseconds | Flags::AMPMFormat) => 15,
            _ if self.flags.contains(Flags::Seconds | Flags::AMPMFormat) => 11,
            _ if self.flags.contains(Flags::Miliseconds) => 12,
            _ if self.flags.contains(Flags::Seconds) => 8,
            _ if self.flags.contains(Flags::AMPMFormat) => 8,
            _ => 5,
        }
    }

    fn get_comp_list(flags: &Flags) -> Vec<Components> {
        match () {
            _ if flags.contains(Flags::Miliseconds | Flags::AMPMFormat) => {
                vec![Components::H, Components::M, Components::S, Components::Ms, Components::AMPM]
            }
            _ if flags.contains(Flags::Seconds | Flags::AMPMFormat) => vec![Components::H, Components::M, Components::S, Components::AMPM],
            _ if flags.contains(Flags::Miliseconds) => vec![Components::H, Components::M, Components::S, Components::Ms],
            _ if flags.contains(Flags::Seconds) => vec![Components::H, Components::M, Components::S],
            _ if flags.contains(Flags::AMPMFormat) => vec![Components::H, Components::M, Components::AMPM],
            _ => vec![Components::H, Components::M],
        }
    }

    fn set_current_time(&mut self) {
        let local: DateTime<Utc> = Utc::now(); 
        self.time = local.naive_utc().time();
    }

    pub fn new(time: NaiveTime, layout: Layout, flags: Flags) -> Self {
        let comp_list = TimePicker::get_comp_list(&flags);

        let mut control = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            time,
            list_of_comp: comp_list,
            cursor: 0,
            hover: 0,
            inscope: None,
        };
        let min_width = control.get_min_width();
        control.set_size_bounds(min_width, 1, u16::MAX, 1);
        control
    }

    #[inline(always)]
    pub fn time(&self) -> NaiveTime {
        self.time
    }

    pub fn add_miliseconds(&mut self, ms: u32) {
        let res = self.time.overflowing_add_signed(TimeDelta::try_milliseconds(ms as i64).unwrap());
        self.time = res.0;
    }
    
    pub fn add_seconds(&mut self, s:u32) {
        let res = self.time.overflowing_add_signed(TimeDelta::try_seconds(s as i64).unwrap());
        self.time = res.0;
    }
    
    pub fn add_minutes(&mut self, m:u32) {
        let res = self.time.overflowing_add_signed(TimeDelta::try_minutes(m as i64).unwrap());
        self.time = res.0;
    }
    
    pub fn add_hours(&mut self, h:u32) {
        let res = self.time.overflowing_add_signed(TimeDelta::try_hours(h as i64).unwrap());
        self.time = res.0;
    }
    
    pub fn sub_miliseconds(&mut self, ms: u32) {
        let res = self.time.overflowing_sub_signed(TimeDelta::try_milliseconds(ms as i64).unwrap());
        self.time = res.0;
    }
    
    pub fn sub_seconds(&mut self, s:u32) {
        let res = self.time.overflowing_sub_signed(TimeDelta::try_seconds(s as i64).unwrap());
        self.time = res.0;
    }
    
    pub fn sub_minutes(&mut self, m:u32) {
        let res = self.time.overflowing_sub_signed(TimeDelta::try_minutes(m as i64).unwrap());
        self.time = res.0;
    }
    
    pub fn sub_hours(&mut self, h:u32) {
        let res = self.time.overflowing_sub_signed(TimeDelta::try_hours(h as i64).unwrap());
        self.time = res.0;
    }

    pub fn set_miliseconds(&mut self, ms: u32) {
        let res = self.time.overflowing_sub_signed(TimeDelta::try_milliseconds(self.time.milliseconds() as i64).unwrap());
        let res = res.0.overflowing_add_signed(TimeDelta::try_milliseconds(ms as i64).unwrap());
        self.time = res.0;
    }
    
    pub fn set_seconds(&mut self, s:u32) {
        let res = self.time.overflowing_sub_signed(TimeDelta::try_seconds(self.time.second() as i64).unwrap());
        let res = res.0.overflowing_add_signed(TimeDelta::try_seconds(s as i64).unwrap());
        self.time = res.0;
    }
    
    pub fn set_minutes(&mut self, m:u32) {
        let res = self.time.overflowing_sub_signed(TimeDelta::try_minutes(self.time.minute() as i64).unwrap());
        let res = res.0.overflowing_add_signed(TimeDelta::try_minutes(m as i64).unwrap());
        self.time = res.0;
    }
    
    pub fn set_hours(&mut self, h:u32) {
        let res = self.time.overflowing_sub_signed(TimeDelta::try_hours(self.time.hour() as i64).unwrap());
        let res = res.0.overflowing_add_signed(TimeDelta::try_hours(h as i64).unwrap());
        self.time = res.0;
    }
}

impl OnPaint for TimePicker {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let basetheme = match () {
            _ if !self.is_enabled() => theme.editor.inactive,
            _ if self.has_focus() => theme.editor.focused,
            _ if self.is_mouse_over() => theme.editor.hovered,
            _ => theme.editor.normal,
        };
        surface.clear(Character::with_attributes(' ', basetheme));

        struct PainterInput {
            write_prefix: bool,
            prefix: &'static str,
            datasize: u8,
            data: u16,
        };

        let mut paint_offset: i32 = 0;
        let mut component_offset: u8 = 1;
        let mut paint_component = |input: &PainterInput| {
            let zatheme = match self.cursor {
                _ if !self.is_enabled() => theme.editor.inactive,
                _ if self.cursor == component_offset => theme.editor.pressed_or_selectd,
                _ if self.hover == component_offset => theme.editor.hovered,
                _ if self.has_focus() => theme.editor.focused,
                _ => theme.editor.normal,
            };

            if input.write_prefix{
                surface.write_string(paint_offset as i32, 0, input.prefix, zatheme, false);
                paint_offset += 1;
            }

            let mut buff: [u8;3] = [0;3];
            let number = match input.datasize {
                3 => {
                    buff[0] = (input.data / 100 + 48) as u8;
                    let data = input.data % 100;
                    buff[1] = (data / 10 + 48) as u8;
                    buff[2] = (data % 10 + 48) as u8;
                    unsafe {str::from_utf8_unchecked(&buff)}
                },
                2 => {
                    buff[0] = (input.data / 10 + 48) as u8;
                    buff[1] = (input.data % 10 + 48) as u8;
                    unsafe {str::from_utf8_unchecked(&buff[..2])}
                },
                _ => {
                    return;
                }
            };

            surface.write_string(paint_offset as i32, 0, number, zatheme, false);
            paint_offset += input.datasize as i32;
            component_offset += 1;
        };

        //hours
        let mut hours = self.time.hour();
        if self.flags.contains(Flags::AMPMFormat) {
            hours = hours % 12;
        }
        let tpl = PainterInput {
            write_prefix: false,
            prefix: "",
            datasize: 2,
            data: hours as u16,
        };
        paint_component(&tpl);
        
        //minutes
        let tpl = PainterInput {
            write_prefix: true,
            prefix: ":",
            datasize: 2,
            data: self.time.minute() as u16,
        };
        paint_component(&tpl);

        //seconds
        if self.flags.contains_one(Flags::Seconds | Flags::Miliseconds) {
            let tpl = PainterInput {
                write_prefix: true,
                prefix: ":",
                datasize: 2,
                data: self.time.second() as u16,
            };
            paint_component(&tpl);
        }

        // ms
        if self.flags.contains(Flags::Miliseconds) {
            let tpl = PainterInput {
                write_prefix: true,
                prefix: ".",
                datasize: 3,
                data: self.time.milliseconds() as u16,
            };
            paint_component(&tpl);
        }

        // am/pm
        if self.flags.contains(Flags::AMPMFormat) {
            let mut pref = " AM"; 
            if self.time.hour() >= 12 {
                pref = " PM";
            }

            let tpl = PainterInput {
                write_prefix: true,
                prefix: pref,
                datasize: 0,
                data: 0,
            };
            paint_component(&tpl);
        }
    }
}

impl OnKeyPressed for TimePicker {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        if self.inscope.is_none() {
            return EventProcessStatus::Ignored;
        }

        struct KeyPressTarget {
            add_func: fn(&mut TimePicker, u32),
            sub_func: fn(&mut TimePicker, u32),
            set_func: fn(&mut TimePicker, u32),
            get_func: fn(&NaiveTime) -> u32,
            increment: u32,
            jump: u32,
            maxvalue: u32
        };

        let ampm_wrapper = |t: &mut TimePicker, _a: u32| {t.add_hours(12)};
        let target: KeyPressTarget = match self.inscope.unwrap() {
           Components::H => KeyPressTarget {add_func: TimePicker::add_hours, sub_func: TimePicker::sub_hours, set_func: TimePicker::set_hours, get_func: NaiveTime::hour, increment: 1, jump: 1, maxvalue: 23},
           Components::S => KeyPressTarget {add_func: TimePicker::add_seconds, sub_func: TimePicker::sub_seconds, set_func: TimePicker::set_seconds, get_func: NaiveTime::second, increment: 1, jump: 10, maxvalue: 59},
           Components::M => KeyPressTarget {add_func: TimePicker::add_minutes, sub_func: TimePicker::sub_minutes, set_func: TimePicker::set_minutes, get_func: NaiveTime::minute, increment: 1, jump: 10, maxvalue: 59},
           Components::Ms => KeyPressTarget {add_func: TimePicker::add_miliseconds, sub_func: TimePicker::sub_miliseconds, set_func: TimePicker::set_miliseconds, get_func: NaiveTime::milliseconds, increment: 1, jump: 100, maxvalue: 999},
           Components::AMPM => KeyPressTarget {add_func: ampm_wrapper, sub_func: ampm_wrapper, get_func: NaiveTime::hour, set_func: TimePicker::set_hours, increment: 12, jump: 12, maxvalue: 23},
        };
        
        match key.value() {
            key!("Up") => {
                (target.add_func)(self, target.increment);
            }
            key!("Down") => {
                (target.sub_func)(self, target.increment);
            }
            key!("Left") => {
                self.cursor -= 1;
                if self.cursor < 1 {
                    self.cursor = (self.list_of_comp.len()) as u8;
                }
                self.hover = self.cursor;
                self.inscope = Some(self.list_of_comp[(self.cursor - 1) as usize]);
            }
            key!("Right") => {
                self.cursor += 1;
                if self.cursor > self.list_of_comp.len() as u8 {
                    self.cursor = 1;
                }
                self.hover = self.cursor;
                self.inscope = Some(self.list_of_comp[(self.cursor - 1) as usize]);
            }
            key!("PageDown") => {
                (target.sub_func)(self, target.jump);
            }
            key!("PageUp") => {
                (target.add_func)(self, target.jump);
            }
            key!("Home") => {
                (target.sub_func)(self, (target.get_func)(&self.time));
            }
            key!("End") => {
                (target.add_func)(self, target.maxvalue - (target.get_func)(&self.time));
            }
            key!("0")
            | key!("1")
            | key!("2")
            | key!("3")
            | key!("4")
            | key!("5")
            | key!("6")
            | key!("7")
            | key!("8")
            | key!("9") => {
                if self.inscope.unwrap() != Components::AMPM {
                    let nr = key.code as u32 - KeyCode::N0 as u32;
                    let mut val = (target.get_func)(&self.time);
                    val = (val * 10) % target.maxvalue + nr; 
                    (target.set_func)(self, val);
                }
            },
            key!("Space") => self.set_current_time(),
            _ => return EventProcessStatus::Ignored,
        };
        EventProcessStatus::Processed
    }
}

impl OnMouseEvent for TimePicker {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Over(data) => {
                match data.x {
                    0..=1 => self.hover = 1,
                    2..=4 => self.hover = 2,
                    5..=7 => self.hover = 3,
                    8..=11 => self.hover = 4,
                    12..=15 => self.hover = 5,
                    _ => self.hover = 0,
                }
                EventProcessStatus::Processed
            }
            MouseEvent::Pressed(data) => {
                match data.x {
                    0..=1 => self.cursor = 1,
                    2..=4 => self.cursor = 2,
                    5..=7 => self.cursor = 3,
                    8..=11 => self.cursor = 4,
                    12..=15 => self.cursor = 5,
                    _ => self.cursor = 0,
                };
                if self.list_of_comp.len() >= self.cursor as usize {
                    self.inscope = Some(self.list_of_comp[(self.cursor - 1) as usize]);
                }

                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
