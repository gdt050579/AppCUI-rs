use crate::prelude::*;
use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};

static SHORT_MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

pub enum RenderMethod<'a> {
    Text(&'a str),
    DateTime(NaiveDateTime),
    Custom,
}
impl RenderMethod<'_> {
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
    fn paint_datetime(dt: &NaiveDateTime, surface: &mut Surface, theme: &Theme, alignment: TextAlignament, width: u16, attr: Option<CharAttribute>) {
        let mut inner_buffer: [u8; 20] = [0; 20];
        // add year
        let year = dt.year();
        inner_buffer[0] = (year / 1000) as u8 + 48;
        inner_buffer[1] = ((year % 1000) / 100) as u8 + 48;
        inner_buffer[2] = ((year % 100) / 10) as u8 + 48;
        inner_buffer[3] = (year % 10) as u8 + 48;
        inner_buffer[4] = b'-';
        let month = SHORT_MONTHS[dt.month().saturating_sub(1) as usize].as_bytes();
        inner_buffer[5] = month[0];
        inner_buffer[6] = month[1];
        inner_buffer[7] = month[2];
        inner_buffer[8] = b'-';
        let day = dt.day();
        inner_buffer[9] = ((day / 10) as u8) + 48;  
        inner_buffer[10] = ((day % 10) as u8) + 48;
        inner_buffer[11] = b' ';
        let hour = dt.hour();
        if hour < 10 {
            inner_buffer[12] = b' ';
        } else {
            inner_buffer[12] = ((hour / 10) as u8) + 48;
        }
        inner_buffer[13] = ((hour % 10) as u8) + 48;
        inner_buffer[14] = b':';
        let minute = dt.minute();
        inner_buffer[15] = ((minute / 10) as u8) + 48;
        inner_buffer[16] = ((minute % 10) as u8) + 48;
        inner_buffer[17] = b':';
        let second = dt.second();
        inner_buffer[18] = ((second / 10) as u8) + 48;
        inner_buffer[19] = ((second % 10) as u8) + 48;
        let txt = unsafe { core::str::from_utf8_unchecked(&inner_buffer) };

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
            chars_count: Some(20),
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
            RenderMethod::DateTime(dt) => {
                RenderMethod::paint_datetime(dt, surface, theme, alignment, width, attr);
                true
            }
            RenderMethod::Custom => false,
        }
    }
}
