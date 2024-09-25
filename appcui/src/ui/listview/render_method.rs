use super::{BoolFormat, DateTimeFormat, NumericFormat};
use crate::prelude::*;
use crate::utils::FormatDateTime;
use chrono::NaiveDateTime;
use listview::SizeFormat;

pub enum RenderMethod<'a> {
    Text(&'a str),
    Ascii(&'a str),
    DateTime(NaiveDateTime, DateTimeFormat),
    Int64(i64, NumericFormat),
    UInt64(u64, NumericFormat),
    Bool(bool, BoolFormat),
    Size(u64, SizeFormat),
    /*
    Date(NaiveDate,format),
    Time(NaiveTime,format),
    Float(f64,...),
    Percentage(f64,zecimals),
    Progress(f64),
    Currency(f64,currency),

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
            | RenderMethod::Int64(_, _)
            | RenderMethod::UInt64(_, _)
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
                let txt = match format {
                    DateTimeFormat::Full => FormatDateTime::full(dt, output),
                    DateTimeFormat::Normal => FormatDateTime::normal(dt, output),
                    DateTimeFormat::Short => FormatDateTime::short(dt, output),
                };
                txt
            }
            RenderMethod::Int64(value, format) => format.formatter().write_number(*value as i64, output),
            RenderMethod::UInt64(value, format) => format.formatter().write_number(*value as u64, output),
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
            RenderMethod::Int64(value, format) => {
                let mut output: [u8; 64] = [0; 64];
                format
                    .formatter()
                    .write_number(*value as i64, &mut output)
                    .map(|p| p.len() as u32)
                    .unwrap_or(0)
            }
            RenderMethod::UInt64(value, format) => {
                let mut output: [u8; 64] = [0; 64];
                format
                    .formatter()
                    .write_number(*value as u64, &mut output)
                    .map(|p| p.len() as u32)
                    .unwrap_or(0)
            }
            RenderMethod::Size(value, format) => {
                let mut output: [u8; 64] = [0; 64];
                format.write(*value, &mut output).map(|p| p.len() as u32).unwrap_or(0)
            }
            RenderMethod::Bool(value, format) => format.text(*value).chars().count() as u32,
            RenderMethod::Custom => 0,
        }
    }
}
