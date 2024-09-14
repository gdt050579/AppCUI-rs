use crate::prelude::*;
use crate::utils::FormatDateTime;
use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};

#[derive(Copy,Clone,Eq,PartialEq)]
pub enum DateTimeFormat {
    Full,
    Normal,
    Short,
}
pub enum RenderMethod<'a> {
    Text(&'a str),
    Ascii(&'a str),
    DateTime(NaiveDateTime, DateTimeFormat),
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
    fn paint_datetime(
        dt: &NaiveDateTime,
        format: DateTimeFormat,
        surface: &mut Surface,
        theme: &Theme,
        alignment: TextAlignament,
        width: u16,
        attr: Option<CharAttribute>,
    ) {
        let mut inner_buffer: [u8; 40] = [0; 40];
        let txt = match format {
            DateTimeFormat::Full => FormatDateTime::full(dt, &mut inner_buffer),
            DateTimeFormat::Normal => FormatDateTime::normal(dt, &mut inner_buffer),
            DateTimeFormat::Short => FormatDateTime::short(dt, &mut inner_buffer),
        };
        if let Some(txt) = txt {
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
            RenderMethod::Ascii(txt) => {
                RenderMethod::paint_ascii(txt, surface, theme, alignment, width, attr);
                true
            }
            RenderMethod::DateTime(dt, format) => {
                RenderMethod::paint_datetime(dt, *format, surface, theme, alignment, width, attr);
                true
            }
            RenderMethod::Custom => false,
        }
    }
}
