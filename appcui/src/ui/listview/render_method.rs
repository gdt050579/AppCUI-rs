use crate::prelude::*;
use crate::utils::{FormatDateTime, FormatTime, FormatDate};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use listview::formats::percentage_format::PercentageFormat;
use listview::{BoolFormat, DateTimeFormat, NumericFormat, SizeFormat, TimeFormat, DateFormat, FloatFormat};

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
    /*
    Progress(f64),
    Currency(f64,currency),
    Metrics(f64,metrics), // km, m, cm, mm, inch, foot, yard, mile
    Temperature(f64,temperature), // Celsius, Fahrenheit, Kelvin
    Speed(f64,speed), // km/h, m/s, mph, knot
    Weight(f64,weight), // kg, g, mg, t, lb, oz
    Volume(f64,volume), // l, ml, cm3, m3, gal, pt, qt, fl oz
    Area(f64,area), // m2, cm2, km2, ha, a, ft2, in2, yd2, mi2    
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
    pub(super) fn paint(&self, surface: &mut Surface, theme: &Theme, alignment: TextAlignament, width: u16, attr: Option<CharAttribute>) -> bool {
        match self {
            RenderMethod::Text(txt) => {
                RenderMethod::paint_text(txt, surface, theme, alignment, width, attr);
                true
            }
            RenderMethod::Bool(_, _) => {
                let mut output: [u8; 16] = [0; 16];
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
            RenderMethod::Custom => false,
        }
    }
    pub(super) fn string_representation(&self, output: &'a mut [u8]) -> Option<&'a str> {
        match self {
            RenderMethod::Text(txt) => Some(txt),
            RenderMethod::Ascii(txt) => Some(txt),
            RenderMethod::DateTime(dt, format) => {
                match format {
                    DateTimeFormat::Full => FormatDateTime::full(dt, output),
                    DateTimeFormat::Normal => FormatDateTime::normal(dt, output),
                    DateTimeFormat::Short => FormatDateTime::short(dt, output),
                }
            }
            RenderMethod::Time(dt, format) => {
                match format {
                    TimeFormat::Short => FormatTime::short(dt, output),
                    TimeFormat::AMPM => FormatTime::am_pm(dt, output),
                    TimeFormat::Normal => FormatTime::normal(dt, output),                
                }
            }
            RenderMethod::Date(dt, format) => {
                match format {
                    DateFormat::Full => FormatDate::full(dt, output),
                    DateFormat::YearMonthDay => FormatDate::ymd(dt, output),
                    DateFormat::DayMonthYear => FormatDate::dmy(dt, output),                    
                }
            }            
            RenderMethod::Int64(value, format) => format.formatter().write_number(*value, output),
            RenderMethod::UInt64(value, format) => format.formatter().write_number(*value, output),
            RenderMethod::Float(value, format) => format.formatter().write_float(*value, output),
            RenderMethod::Percentage(value, format) => format.formatter().write_float(*value * 100.0, output),
            RenderMethod::Bool(value, format) => Some(format.text(*value)),
            RenderMethod::Size(value, format) => format.write(*value, output),
            RenderMethod::Custom => None,
        }
    }
    pub(super) fn min_width(&self) -> u32 {
        match self {
            RenderMethod::Text(txt) => txt.chars().count() as u32,
            RenderMethod::Ascii(txt) => txt.len() as u32,
            RenderMethod::DateTime(_, _) => 19,
            RenderMethod::Time(_, format) => {
                match format {
                    TimeFormat::Short => 5,
                    TimeFormat::AMPM => 8,
                    TimeFormat::Normal => 8,                 
                }
            }
            RenderMethod::Date(_, format) => {
                match format {
                    DateFormat::Full => 16,
                    DateFormat::YearMonthDay => 10,
                    DateFormat::DayMonthYear => 10,                    
                }                
            }            
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
            RenderMethod::Size(value, format) => {
                let mut output: [u8; 64] = [0; 64];
                format.write(*value, &mut output).map(|p| p.len() as u32).unwrap_or(0)
            }
            RenderMethod::Percentage(value, format) => {
                let mut output: [u8; 64] = [0; 64];
                format.formatter().write_float(*value * 100.0, &mut output).map(|p| p.len() as u32).unwrap_or(0)
            }
            RenderMethod::Bool(value, format) => format.text(*value).chars().count() as u32,
            RenderMethod::Custom => 0,
        }
    }
}
