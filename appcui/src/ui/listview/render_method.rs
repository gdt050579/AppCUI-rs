use crate::prelude::*;
use crate::utils::FormatDateTime;
use crate::utils::FormatNumber;
use chrono::NaiveDateTime;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DateTimeFormat {
    Full,
    Normal,
    Short,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum NumericFormat {
    Normal,
    Separator,
}

const DECIMAL_FORMAT: FormatNumber = FormatNumber::new(10);
const DECIMAL_FORMAT_SEPARATIR: FormatNumber = FormatNumber::new(10).group(3, b',');

pub enum RenderMethod<'a> {
    Text(&'a str),
    Ascii(&'a str),
    DateTime(NaiveDateTime, DateTimeFormat),
    Int(i64, NumericFormat),
    /*
    Date(NaiveDate,format),
    Time(NaiveTime,format),
    Bool(bool,format)
    Int(i64,...),
    Float(f64,...),
    UInt(u64,...),
    Percentage(f64,zecimals),
    Size(u64,format),
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
            RenderMethod::Ascii(_) | RenderMethod::DateTime(_, _) | RenderMethod::Int(_, _) => {
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
            RenderMethod::Int(value, format) => {
                let txt = match format {
                    NumericFormat::Normal => DECIMAL_FORMAT.write_number(*value as i128, output),
                    NumericFormat::Separator => DECIMAL_FORMAT_SEPARATIR.write_number(*value as i128, output),
                };
                txt
            }
            RenderMethod::Custom => None,
        }
    }
    pub(super) fn min_width(&self) -> u32 {
        match self {
            RenderMethod::Text(txt) => txt.chars().count() as u32,
            RenderMethod::Ascii(txt) => txt.len() as u32,
            RenderMethod::DateTime(_, _) => 19,
            RenderMethod::Int(value, format) => {
                let mut output: [u8; 64] = [0; 64];
                let txt = match format {
                    NumericFormat::Normal => DECIMAL_FORMAT.write_number(*value as i128, &mut output),
                    NumericFormat::Separator => DECIMAL_FORMAT_SEPARATIR.write_number(*value as i128, &mut output),
                };
                if let Some(txt) = txt {
                    txt.len() as u32
                } else {
                    0
                }
            }
            RenderMethod::Custom => 0,
        }
    }
}
