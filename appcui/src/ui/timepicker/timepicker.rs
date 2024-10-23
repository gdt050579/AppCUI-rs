use std::u16;
use chrono::prelude::*;

use super::Flags;
use crate::prelude::*;

#[derive(Copy, PartialEq, Clone)]
enum Components {
    H,
    M,
    S,
    Ms,
    AMPM,
    None,
}

#[CustomControl(overwrite=[OnPaint,OnKeyPressed,OnMouseEvent],internal=true)]
pub struct TimePicker {
    h: i32,
    m: i32,
    s: i32,
    ms: i32,
    flags: Flags,
    list_of_comp: Vec<Components>,

    cursor: u8,
    hover: u8,
    inscope: Components,
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

    pub fn new(layout: Layout, flags: Flags) -> Self {
        let comp_list = TimePicker::get_comp_list(&flags);

        let mut control = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            h: 11,
            m: 0,
            s: 0,
            ms: 0,
            flags,
            list_of_comp: comp_list,
            cursor: 0,
            hover: 0,
            inscope: Components::None,
        };
        let min_width = control.get_min_width();
        control.set_size_bounds(min_width, 1, u16::MAX, 1);
        control
    }

    pub fn set_current_time(&mut self) {
        let local: DateTime<Utc> = Utc::now(); 
        let ms = local.timestamp_millis();

        self.ms = (ms % 1000) as i32;
        self.s = (ms / 1000 % 60) as i32;
        self.m = ( (ms / 1000 / 60) % 60) as i32;
        self.h = ( (ms / 1000 / 60 / 60 ) % 24) as i32;
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

        struct PainterInput<'Life> {
            textsize: u8,
            suffix: &'Life str,
            datasize: u8,
            data: String,
            currentcomp: Components,
        };

        let mut paint_offset: i32 = 0;
        let mut component_offset: u8 = 1;
        let mut paint_component = |input: PainterInput| -> _ {
            let number = match input.datasize {
                3 => format!("{:0>3}", input.data),
                _ => format!("{:0>2}", input.data),
            };
            let zatheme = match self.cursor {
                _ if !self.is_enabled() => theme.editor.inactive,
                _ if self.cursor == component_offset => theme.editor.pressed_or_selectd,
                _ if self.hover == component_offset => theme.editor.hovered,
                _ if self.has_focus() => theme.editor.focused,
                _ => theme.editor.normal,
            };

            surface.write_string(paint_offset as i32, 0, number.as_str(), zatheme, false);
            surface.write_string(paint_offset + input.datasize as i32, 0, input.suffix, basetheme, false);
            paint_offset += input.textsize as i32;
            component_offset += 1;
        };

        //hours
        let mut hours = self.h;
        if self.flags.contains(Flags::AMPMFormat) {
            hours = hours % 12;
        }
        let tpl = PainterInput {
            textsize: 3,
            suffix: ":",
            datasize: 2,
            data: hours.to_string(),
            currentcomp: Components::H,
        };
        paint_component(tpl);

        //minutes
        let tpl = match () {
            _ if self.flags.contains_one(Flags::Miliseconds | Flags::Seconds) => PainterInput {
                textsize: 3,
                suffix: ":",
                datasize: 2,
                data: self.m.to_string(),
                currentcomp: Components::M,
            },
            _ if self.flags.contains_one(Flags::AMPMFormat) => PainterInput {
                textsize: 3,
                suffix: " ",
                datasize: 2,
                data: self.m.to_string(),
                currentcomp: Components::M,
            },
            _ => PainterInput {
                textsize: 2,
                suffix: "",
                datasize: 2,
                data: self.m.to_string(),
                currentcomp: Components::M,
            },
        };
        paint_component(tpl);

        //seconds
        if self.flags.contains_one(Flags::Miliseconds | Flags::Seconds) {
            let tpl = match () {
                _ if self.flags.contains(Flags::Miliseconds) => PainterInput {
                    textsize: 3,
                    suffix: ".",
                    datasize: 2,
                    data: self.s.to_string(),
                    currentcomp: Components::S,
                },
                _ if self.flags.contains(Flags::AMPMFormat) => PainterInput {
                    textsize: 3,
                    suffix: " ",
                    datasize: 2,
                    data: self.s.to_string(),
                    currentcomp: Components::S,
                },
                _ => PainterInput {
                    textsize: 2,
                    suffix: "",
                    datasize: 2,
                    data: self.s.to_string(),
                    currentcomp: Components::S,
                },
            };
            paint_component(tpl);
        }

        // ms
        if self.flags.contains(Flags::Miliseconds) {
            let tpl = match () {
                _ if self.flags.contains(Flags::AMPMFormat) => PainterInput {
                    textsize: 4,
                    suffix: " ",
                    datasize: 3,
                    data: self.ms.to_string(),
                    currentcomp: Components::Ms,
                },
                _ => PainterInput {
                    textsize: 3,
                    suffix: "",
                    datasize: 3,
                    data: self.ms.to_string(),
                    currentcomp: Components::Ms,
                },
            };
            paint_component(tpl);
        }

        // am/pm
        if self.flags.contains(Flags::AMPMFormat) {
            let txt = match () {
                _ if self.h < 12 => "AM".to_string(),
                _ => "PM".to_string(),
            };
            let tpl = PainterInput {
                textsize: 3,
                suffix: "",
                datasize: 2,
                data: txt,
                currentcomp: Components::AMPM,
            };
            paint_component(tpl);
        }
    }
}

impl OnKeyPressed for TimePicker {
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        let mut target: (&mut i32, i32, i32, i32) = match self.inscope {
            Components::H => (&mut self.h, 1, 24, 1),
            Components::S => (&mut self.s, 1, 60, 10),
            Components::M => (&mut self.m, 1, 60, 10),
            Components::Ms => (&mut self.ms, 1, 1000, 100),
            Components::AMPM => (&mut self.h, 12, 24, 12),
            Components::None => (&mut self.ms, 0, 1000, 0),
        };

        if self.inscope == Components::H && self.flags.contains(Flags::AMPMFormat) {
            target.2 = 12;
        }

        match _key.code {
            KeyCode::Up => {
                *target.0 += target.1;
                *target.0 = *target.0 % target.2;
            }
            KeyCode::Down => {
                *target.0 += target.1 * -1;
                if *target.0 < 0 {
                    *target.0 = target.2 + (*target.0 % target.2);
                }
            }
            KeyCode::Left => {
                self.cursor -= 1;
                if self.cursor < 1 {
                    self.cursor = (self.list_of_comp.len()) as u8;
                }
                self.hover = self.cursor;
                self.inscope = self.list_of_comp[(self.cursor - 1) as usize];
            }
            KeyCode::Right => {
                self.cursor += 1;
                if self.cursor > self.list_of_comp.len() as u8 {
                    self.cursor = 1;
                }
                self.hover = self.cursor;
                self.inscope = self.list_of_comp[(self.cursor - 1) as usize];
            }
            KeyCode::PageDown => {
                *target.0 += target.3 * -1;
                if *target.0 < 0 {
                    *target.0 = target.2 + (*target.0 % target.2);
                }
            }
            KeyCode::PageUp => {
                *target.0 += target.3;
                *target.0 = *target.0 % target.2;
            }
            KeyCode::Home => {
                *target.0 = 0;
            }
            KeyCode::End => {
                *target.0 = target.2 + (-1 % target.2);
            }
            KeyCode::N0
            | KeyCode::N1
            | KeyCode::N2
            | KeyCode::N3
            | KeyCode::N4
            | KeyCode::N5
            | KeyCode::N6
            | KeyCode::N7
            | KeyCode::N8
            | KeyCode::N9 => {
                let nr = _key.code as u32 - KeyCode::N0 as u32;
                *target.0 = *target.0 * 10 + nr as i32;
                if *target.0 < 0 {
                    *target.0 = target.2 + (*target.0 % target.2);
                }
                else {
                    *target.0 = *target.0 % target.2;
                }
            },
            KeyCode::Space => self.set_current_time(),
            _ => return EventProcessStatus::Ignored,
        };
        EventProcessStatus::Processed
    }
}

impl OnMouseEvent for TimePicker {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => EventProcessStatus::Processed,
            MouseEvent::Leave => EventProcessStatus::Processed,
            MouseEvent::Over(data) => {
                println!("Hovering also");
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
                    self.inscope = self.list_of_comp[(self.cursor - 1) as usize];
                }

                EventProcessStatus::Processed
            }
            MouseEvent::Released(_) => EventProcessStatus::Ignored,
            MouseEvent::DoubleClick(_) => EventProcessStatus::Ignored,
            MouseEvent::Drag(_) => EventProcessStatus::Ignored,
            MouseEvent::Wheel(_) => EventProcessStatus::Ignored,
        }
    }
}
