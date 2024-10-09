use crate::prelude::*;
use crate::utils::{FormatDate, FormatDateTime, FormatTime};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use listview::formats::percentage_format::PercentageFormat;
use listview::{
    AreaFormat, BoolFormat, DateFormat, DateTimeFormat, FloatFormat, NumericFormat, SizeFormat, Status, StatusFormat, TemperatureFormat, TimeFormat,
};

pub enum RenderMethod<'a> {
    Text(&'a str),
    Ascii(&'a str),
    DateTime(NaiveDateTime, DateTimeFormat),
    Time(NaiveTime, TimeFormat),
    Date(NaiveDate, DateFormat),
    Int64(i64, NumericFormat),
    UInt64(u64, NumericFormat),
    Bool(bool, BoolFormat),
    Size(u64, SizeFormat),
    Percentage(f64, PercentageFormat),
    Float(f64, FloatFormat),
    Status(Status, StatusFormat),
    Temperature(f64, TemperatureFormat),
    Area(u64, AreaFormat),
    /*
    Currency(f64,currency),
    Metrics(f64,metrics), // km, m, cm, mm, inch, foot, yard, mile
    Speed(f64,speed), // km/h, m/s, mph, knot
    Weight(f64,weight), // kg, g, mg, t, lb, oz
    Volume(f64,volume), // l, ml, cm3, m3, gal, pt, qt, fl oz
    */
    Custom,
}
impl<'a> RenderMethod<'a> {
    #[inline(always)]
    fn paint_text(txt: &str, surface: &mut Surface, theme: &Theme, alignment: TextAlignament, width: u16, attr: Option<CharAttribute>) {
        let format = TextFormat {
            x: match alignment {
                TextAlignament::Left => 0,
                TextAlignament::Center => (width as i32) / 2,
                TextAlignament::Right => (width as i32) - 1,
            },
            y: 0,
            width: Some(width),
            char_attr: attr.unwrap_or(theme.text.focused),
            hotkey_attr: None,
            hotkey_pos: None,
            chars_count: None,
            align: alignment,
            text_wrap: TextWrap::None,
            multi_line: false,
        };
        surface.write_text(txt, &format);
    }
    #[inline(always)]
    fn paint_ascii(txt: &str, surface: &mut Surface, theme: &Theme, alignment: TextAlignament, width: u16, attr: Option<CharAttribute>) {
        let format = TextFormat {
            x: match alignment {
                TextAlignament::Left => 0,
                TextAlignament::Center => (width as i32) / 2,
                TextAlignament::Right => (width as i32) - 1,
            },
            y: 0,
            width: Some(width),
            char_attr: attr.unwrap_or(theme.text.focused),
            hotkey_attr: None,
            hotkey_pos: None,
            chars_count: Some(txt.len() as u16),
            align: alignment,
            text_wrap: TextWrap::None,
            multi_line: false,
        };
        surface.write_text(txt, &format);
    }

    #[inline(always)]
    fn paint_status(
        status: Status,
        format: StatusFormat,
        surface: &mut Surface,
        theme: &Theme,
        alignment: TextAlignament,
        width: u16,
        attr: Option<CharAttribute>,
    ) {
        if let Status::Running(value) = status {
            let mut output: [u8; 32] = [0; 32];
            let txt = status.to_str(&mut output);
            if (width >= 10) && (txt.len() >= 4) {
                // [xxx]<space>xxx% => 7 chars
                let attr = attr.unwrap_or(theme.text.focused);
                surface.write_char(0, 0, Character::with_attributes('[', attr));
                surface.write_char(width as i32 - 5, 0, Character::with_attributes(' ', attr));
                surface.write_char(width as i32 - 6, 0, Character::with_attributes(']', attr));
                surface.write_string((width as i32) - 4, 0, &txt[(txt.len() - 4)..], attr, false);
                let sz = (((width - 7) as f64 * Status::proc(value) / 100.0) as u32).min((width as u32) - 7);
                match format {
                    StatusFormat::Hashtag => {
                        surface.fill_horizontal_line_with_size(1, 0, width as u32 - 7, Character::with_attributes('-', attr));
                        surface.fill_horizontal_line_with_size(1, 0, sz, Character::with_attributes('#', attr));
                    }
                    StatusFormat::Graphical => {
                        surface.fill_horizontal_line_with_size(1, 0, width as u32 - 7, Character::with_attributes(SpecialChar::Block25, attr));
                        surface.fill_horizontal_line_with_size(1, 0, sz, Character::with_attributes(SpecialChar::Block100, attr));
                    }
                    StatusFormat::Arrow => {
                        surface.fill_horizontal_line_with_size(1, 0, width as u32 - 7, Character::with_attributes(' ', attr));
                        surface.fill_horizontal_line_with_size(1, 0, sz, Character::with_attributes('=', attr));
                        surface.write_char(1 + (sz.saturating_sub(1) as i32), 0, Character::with_attributes('>', attr));
                    }
                }
            } else {
                RenderMethod::paint_ascii(txt, surface, theme, alignment, width, attr);
            }
        } else {
            let mut output: [u8; 32] = [0; 32];
            let txt = status.to_str(&mut output);
            RenderMethod::paint_ascii(txt, surface, theme, alignment, width, attr);
        }
    }

    #[inline(always)]
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, alignment: TextAlignament, width: u16, attr: Option<CharAttribute>) -> bool {
        match self {
            RenderMethod::Text(txt) => {
                RenderMethod::paint_text(txt, surface, theme, alignment, width, attr);
                true
            }
            RenderMethod::Bool(_, _) | RenderMethod::Temperature(_, _) | RenderMethod::Area(_, _) => {
                let mut output: [u8; 32] = [0; 32];
                if let Some(str_rep) = self.string_representation(&mut output) {
                    RenderMethod::paint_text(str_rep, surface, theme, alignment, width, attr);
                    true
                } else {
                    false
                }
            }
            RenderMethod::Ascii(_)
            | RenderMethod::DateTime(_, _)
            | RenderMethod::Time(_, _)
            | RenderMethod::Date(_, _)
            | RenderMethod::Int64(_, _)
            | RenderMethod::UInt64(_, _)
            | RenderMethod::Float(_, _)
            | RenderMethod::Percentage(_, _)
            | RenderMethod::Size(_, _) => {
                let mut output: [u8; 256] = [0; 256];
                if let Some(str_rep) = self.string_representation(&mut output) {
                    RenderMethod::paint_ascii(str_rep, surface, theme, alignment, width, attr);
                    true
                } else {
                    false
                }
            }
            RenderMethod::Status(status, format) => {
                RenderMethod::paint_status(*status, *format, surface, theme, alignment, width, attr);
                true
            }
            RenderMethod::Custom => false,
        }
    }
    pub(super) fn string_representation(&self, output: &'a mut [u8]) -> Option<&'a str> {
        match self {
            RenderMethod::Text(txt) => Some(txt),
            RenderMethod::Ascii(txt) => Some(txt),
            RenderMethod::DateTime(dt, format) => match format {
                DateTimeFormat::Full => FormatDateTime::full(dt, output),
                DateTimeFormat::Normal => FormatDateTime::normal(dt, output),
                DateTimeFormat::Short => FormatDateTime::short(dt, output),
            },
            RenderMethod::Time(dt, format) => match format {
                TimeFormat::Short => FormatTime::short(dt, output),
                TimeFormat::AMPM => FormatTime::am_pm(dt, output),
                TimeFormat::Normal => FormatTime::normal(dt, output),
            },
            RenderMethod::Date(dt, format) => match format {
                DateFormat::Full => FormatDate::full(dt, output),
                DateFormat::YearMonthDay => FormatDate::ymd(dt, output),
                DateFormat::DayMonthYear => FormatDate::dmy(dt, output),
            },
            RenderMethod::Int64(value, format) => format.formatter().write_number(*value, output),
            RenderMethod::UInt64(value, format) => format.formatter().write_number(*value, output),
            RenderMethod::Float(value, format) => format.formatter().write_float(*value, output),
            RenderMethod::Percentage(value, format) => format.formatter().write_float(*value * 100.0, output),
            RenderMethod::Temperature(value, format) => format.formatter().write_float(*value, output),
            RenderMethod::Bool(value, format) => Some(format.text(*value)),
            RenderMethod::Size(value, format) => format.write(*value, output),
            RenderMethod::Area(value, format) => format.write(*value, output),
            RenderMethod::Status(status, _) => Some(status.to_str(output)),
            RenderMethod::Custom => None,
        }
    }
    pub(super) fn min_width(&self) -> u32 {
        match self {
            RenderMethod::Text(txt) => txt.chars().count() as u32,
            RenderMethod::Ascii(txt) => txt.len() as u32,
            RenderMethod::DateTime(_, _) => 19,
            RenderMethod::Time(_, format) => match format {
                TimeFormat::Short => 5,
                TimeFormat::AMPM => 8,
                TimeFormat::Normal => 8,
            },
            RenderMethod::Date(_, format) => match format {
                DateFormat::Full => 16,
                DateFormat::YearMonthDay => 10,
                DateFormat::DayMonthYear => 10,
            },
            RenderMethod::Int64(value, format) => {
                let mut output: [u8; 64] = [0; 64];
                format.formatter().write_number(*value, &mut output).map(|p| p.len() as u32).unwrap_or(0)
            }
            RenderMethod::UInt64(value, format) => {
                let mut output: [u8; 64] = [0; 64];
                format.formatter().write_number(*value, &mut output).map(|p| p.len() as u32).unwrap_or(0)
            }
            RenderMethod::Float(value, format) => {
                let mut output: [u8; 64] = [0; 64];
                format.formatter().write_float(*value, &mut output).map(|p| p.len() as u32).unwrap_or(0)
            }
            RenderMethod::Temperature(value, format) => {
                let mut output: [u8; 64] = [0; 64];
                format.formatter().write_float(*value, &mut output).map(|p| p.len() as u32).unwrap_or(0)
            }
            RenderMethod::Size(value, format) => {
                let mut output: [u8; 64] = [0; 64];
                format.write(*value, &mut output).map(|p| p.len() as u32).unwrap_or(0)
            }
            RenderMethod::Area(value, format) => {
                let mut output: [u8; 64] = [0; 64];
                format.write(*value, &mut output).map(|p| p.len() as u32).unwrap_or(0)
            }
            RenderMethod::Percentage(value, format) => {
                let mut output: [u8; 64] = [0; 64];
                format
                    .formatter()
                    .write_float(*value * 100.0, &mut output)
                    .map(|p| p.len() as u32)
                    .unwrap_or(0)
            }
            RenderMethod::Bool(value, format) => format.text(*value).chars().count() as u32,
            RenderMethod::Status(status, _) => {
                let mut output: [u8; 32] = [0; 32];
                status.to_str(&mut output).len() as u32
            }
            RenderMethod::Custom => 0,
        }
    }
}
