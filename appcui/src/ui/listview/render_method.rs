use crate::prelude::*;
use crate::utils::format_datetime::FormatDuration;
use crate::utils::{FormatDate, FormatDateTime, FormatRatings, FormatTime};
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use listview::formats::percentage_format::PercentageFormat;
use listview::{
    AreaFormat, BoolFormat, CurrencyFormat, DateFormat, DateTimeFormat, DistanceFormat, DurationFormat, FloatFormat, NumericFormat, RatingFormat,
    SizeFormat, Status, StatusFormat, TemperatureFormat, TimeFormat,
};

const MAX_RATING_STARS: u8 = 10;

pub(crate) struct RenderData<'a> {
    pub(crate) theme: &'a Theme,
    pub(crate) alignment: TextAlignament,
    pub(crate) width: u16,
    pub(crate) attr: Option<CharAttribute>,
}

pub enum RenderMethod<'a> {
    Text(&'a str),
    Ascii(&'a str),
    DateTime(NaiveDateTime, DateTimeFormat),
    Time(NaiveTime, TimeFormat),
    Date(NaiveDate, DateFormat),
    Duration(Duration, DurationFormat),
    Int64(i64, NumericFormat),
    UInt64(u64, NumericFormat),
    Bool(bool, BoolFormat),
    Size(u64, SizeFormat),
    Percentage(f64, PercentageFormat),
    Float(f64, FloatFormat),
    Status(Status, StatusFormat),
    Temperature(f64, TemperatureFormat),
    Area(u64, AreaFormat),
    Rating(u32, RatingFormat),
    Currency(f64, CurrencyFormat),
    Distance(u64, DistanceFormat),
    /*
    Speed(f64,speed), // km/h, m/s, mph, knot
    Weight(f64,weight), // kg, g, mg, t, lb, oz
    Volume(f64,volume), // l, ml, cm3, m3, gal, pt, qt, fl oz
    */
    Custom,
}
impl<'a> RenderMethod<'a> {
    #[inline(always)]
    fn paint_text(txt: &str, surface: &mut Surface, rd: &RenderData) {
        let format = TextFormat {
            x: match rd.alignment {
                TextAlignament::Left => 0,
                TextAlignament::Center => (rd.width as i32) / 2,
                TextAlignament::Right => (rd.width as i32) - 1,
            },
            y: 0,
            width: Some(rd.width),
            char_attr: rd.attr.unwrap_or(rd.theme.text.focused),
            hotkey_attr: None,
            hotkey_pos: None,
            chars_count: None,
            align: rd.alignment,
            text_wrap: TextWrap::None,
            multi_line: false,
        };
        surface.write_text(txt, &format);
    }
    #[inline(always)]
    fn paint_ascii(txt: &str, surface: &mut Surface, rd: &RenderData) {
        let format = TextFormat {
            x: match rd.alignment {
                TextAlignament::Left => 0,
                TextAlignament::Center => (rd.width as i32) / 2,
                TextAlignament::Right => (rd.width as i32) - 1,
            },
            y: 0,
            width: Some(rd.width),
            char_attr: rd.attr.unwrap_or(rd.theme.text.focused),
            hotkey_attr: None,
            hotkey_pos: None,
            chars_count: Some(txt.len() as u16),
            align: rd.alignment,
            text_wrap: TextWrap::None,
            multi_line: false,
        };
        surface.write_text(txt, &format);
    }

    #[inline(always)]
    fn paint_currency(value: f64, format: &CurrencyFormat, surface: &mut Surface, rd: &RenderData) {
        let (currency_name, len) = format.name();
        let mut output: [u8; 48] = [0; 48];
        let txt = CurrencyFormat::NUMERIC_FORMAT.write_float(value, &mut output).unwrap_or("?");
        let attr = rd.attr.unwrap_or(rd.theme.text.focused);
        surface.write_string(0, 0, currency_name, attr, false);
        let pos = ((rd.width as i32) - (txt.len() as i32)).max(len as i32 + 1);
        surface.write_string(pos, 0, txt, attr, false);
    }

    #[inline(always)]
    fn paint_status(status: Status, format: StatusFormat, surface: &mut Surface, rd: &RenderData) {
        if let Status::Running(value) = status {
            let mut output: [u8; 32] = [0; 32];
            let txt = status.string_representation(&mut output);
            let width = rd.width as i32;
            if (width >= 10) && (txt.len() >= 4) {
                // [xxx]<space>xxx% => 7 chars
                let attr = rd.attr.unwrap_or(rd.theme.text.focused);
                surface.write_char(0, 0, Character::with_attributes('[', attr));
                surface.write_char(width - 5, 0, Character::with_attributes(' ', attr));
                surface.write_char(width - 6, 0, Character::with_attributes(']', attr));
                surface.write_string(width - 4, 0, &txt[(txt.len() - 4)..], attr, false);
                let draw_sz = (width as u32) - 7;
                let sz = (((draw_sz as f64) * Status::proc(value) / 100.0) as u32).min(draw_sz);
                match format {
                    StatusFormat::Hashtag => {
                        surface.fill_horizontal_line_with_size(1, 0, draw_sz, Character::with_attributes('-', attr));
                        surface.fill_horizontal_line_with_size(1, 0, sz, Character::with_attributes('#', attr));
                    }
                    StatusFormat::Graphical => {
                        surface.fill_horizontal_line_with_size(1, 0, draw_sz, Character::with_attributes(SpecialChar::Block25, attr));
                        surface.fill_horizontal_line_with_size(1, 0, sz, Character::with_attributes(SpecialChar::Block100, attr));
                    }
                    StatusFormat::Arrow => {
                        surface.fill_horizontal_line_with_size(1, 0, draw_sz, Character::with_attributes(' ', attr));
                        surface.fill_horizontal_line_with_size(1, 0, sz, Character::with_attributes('=', attr));
                        surface.write_char(1 + (sz.saturating_sub(1) as i32), 0, Character::with_attributes('>', attr));
                    }
                }
            } else {
                RenderMethod::paint_ascii(txt, surface, rd);
            }
        } else {
            let mut output: [u8; 32] = [0; 32];
            let txt = status.string_representation(&mut output);
            RenderMethod::paint_ascii(txt, surface, rd);
        }
    }

    #[inline(always)]
    pub(super) fn paint(&self, surface: &mut Surface, rd: &RenderData) -> bool {
        match self {
            RenderMethod::Text(txt) => {
                RenderMethod::paint_text(txt, surface, rd);
                true
            }
            RenderMethod::Currency(value, format) => {
                RenderMethod::paint_currency(*value, format, surface, rd);
                true
            }
            RenderMethod::Bool(_, _) | RenderMethod::Temperature(_, _) | RenderMethod::Area(_, _) | RenderMethod::Rating(_, _) => {
                let mut output: [u8; 32] = [0; 32];
                if let Some(str_rep) = self.string_representation(&mut output) {
                    RenderMethod::paint_text(str_rep, surface, rd);
                    true
                } else {
                    false
                }
            }
            RenderMethod::Ascii(_)
            | RenderMethod::DateTime(_, _)
            | RenderMethod::Time(_, _)
            | RenderMethod::Date(_, _)
            | RenderMethod::Duration(_, _)
            | RenderMethod::Int64(_, _)
            | RenderMethod::UInt64(_, _)
            | RenderMethod::Float(_, _)
            | RenderMethod::Percentage(_, _)
            | RenderMethod::Distance(_, _)
            | RenderMethod::Size(_, _) => {
                let mut output: [u8; 256] = [0; 256];
                if let Some(str_rep) = self.string_representation(&mut output) {
                    RenderMethod::paint_ascii(str_rep, surface, rd);
                    true
                } else {
                    false
                }
            }
            RenderMethod::Status(status, format) => {
                RenderMethod::paint_status(*status, *format, surface, rd);
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
            RenderMethod::Duration(duration, format) => match format {
                DurationFormat::Auto => FormatDuration::auto_hms(duration, output),
                DurationFormat::Seconds => DurationFormat::seconds(duration, output),
                DurationFormat::Details => FormatDuration::details(duration, output),
            },
            RenderMethod::Rating(value, format) => match format {
                RatingFormat::Numerical(max_value) => FormatRatings::raport(*value, *max_value, output),
                RatingFormat::Stars(count) => FormatRatings::two_chars('☆', '★', *value, *count, (*count as u8).min(MAX_RATING_STARS), output),
                RatingFormat::Circles(count) => {
                    FormatRatings::two_chars('\u{25CB}', '\u{25CF}', *value, *count, (*count as u8).min(MAX_RATING_STARS), output)
                }
                RatingFormat::Asterix(count) => FormatRatings::two_chars(' ', '*', *value, *count, (*count as u8).min(MAX_RATING_STARS), output),
            },
            RenderMethod::Int64(value, format) => format.formatter().write_number(*value, output),
            RenderMethod::UInt64(value, format) => format.formatter().write_number(*value, output),
            RenderMethod::Float(value, format) => format.formatter().write_float(*value, output),
            RenderMethod::Percentage(value, format) => format.formatter().write_float(*value * 100.0, output),
            RenderMethod::Temperature(value, format) => format.formatter().write_float(*value, output),
            RenderMethod::Bool(value, format) => Some(format.text(*value)),
            RenderMethod::Size(value, format) => format.write(*value, output),
            RenderMethod::Area(value, format) => format.write(*value, output),
            RenderMethod::Distance(value, format) => format.write(*value, output),
            RenderMethod::Status(status, _) => Some(status.string_representation(output)),
            RenderMethod::Currency(value, format) => format.formatter().write_float(*value, output),
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
            RenderMethod::Duration(_, _) => {
                let mut output: [u8; 64] = [0; 64];
                self.string_representation(&mut output).map(|p| p.len() as u32).unwrap_or(0)
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
            RenderMethod::Distance(value, format) => {
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
            RenderMethod::Currency(value, format) => {
                let mut output: [u8; 64] = [0; 64];
                format.formatter().write_float(*value, &mut output).map(|p| p.len() as u32).unwrap_or(0)
            }
            RenderMethod::Bool(value, format) => format.text(*value).chars().count() as u32,
            RenderMethod::Status(status, _) => {
                let mut output: [u8; 32] = [0; 32];
                status.string_representation(&mut output).len() as u32
            }
            RenderMethod::Rating(value, format) => match format {
                RatingFormat::Numerical(max_value) => {
                    let mut output: [u8; 32] = [0; 32];
                    FormatRatings::raport(*value, *max_value, &mut output)
                        .map(|p| p.len() as u32)
                        .unwrap_or(0)
                }
                RatingFormat::Stars(count) | RatingFormat::Circles(count) | RatingFormat::Asterix(count) => (*count).min(MAX_RATING_STARS as u32),
            },
            RenderMethod::Custom => 0,
        }
    }
}
