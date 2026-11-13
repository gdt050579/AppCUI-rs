use crate::{
    graphics::{CharAttribute, Character, Color, Rect, Size, Surface},
    prelude::Number,
};

pub(crate) struct PaintParams {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) size: Size,
    pub(crate) percentage: u8,
    pub(crate) default_attr: CellAttribute,
    pub(crate) default_fill: CellFill,
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum CellAttribute {
    Fixed(CharAttribute),
    Range(CharAttribute, CharAttribute),
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum CellFill {
    Shades,
    Custom(char),
}
pub(crate) struct Cell<T: Number> {
    pub(crate) value: T,
    label: String,
    attr: Option<CellAttribute>,
    fill: Option<CellFill>,
}
impl<T: Number> Cell<T> {
    const SHADES: &[char] = &[' ', '░', '▒', '▓', '█'];
    // where percentage is a value between 0 and 100
    pub(super) fn paint(&self, surface: &mut Surface, params: &PaintParams) {
        let attr = self.compute_attr(params);
        let fill = self.compute_fill(params);
        let ch = Character::with_attributes(fill, attr);
        let r = Rect::with_size(params.x, params.y, params.size.width as u16, params.size.height as u16);
        surface.fill_rect(r, ch);
    }
    fn compute_fill(&self, params: &PaintParams) -> char {
        let fill = self.fill.unwrap_or(params.default_fill);
        match fill {
            CellFill::Shades => Self::percentage_to_char(Self::SHADES, params.percentage),
            CellFill::Custom(ch) => ch,
        }
    }
    fn compute_attr(&self, params: &PaintParams) -> CharAttribute {
        let attr = self.attr.as_ref().unwrap_or(&params.default_attr);
        match attr {
            CellAttribute::Fixed(attr) => *attr,
            CellAttribute::Range(min, max) => Self::attribute_from_range(*min, *max, params.percentage),
        }
    }
    fn attribute_from_range(start: CharAttribute, end: CharAttribute, percentage: u8) -> CharAttribute {
        let fore = Self::color_from_range(start.foreground, end.foreground, percentage);
        let back = Self::color_from_range(start.background, end.background, percentage);
        if percentage <= 50 {
            CharAttribute::new(fore, back, start.flags)
        } else {
            CharAttribute::new(fore, back, end.flags)
        }
    }
    fn blend(from: u8, to: u8, percentage: u32) -> u8 {
        let (from, to) = (from as u32, to as u32);
        if to >= from {
            (from + (to - from) * percentage / 100) as u8
        } else {
            (from - (from - to) * percentage / 100) as u8
        }
    }
    fn color_from_range(start: Color, end: Color, percentage: u8) -> Color {
        let percentage = percentage.min(100) as u32;
        match (start.to_rgb(), end.to_rgb()) {
            (Some(start_rgb), Some(end_rgb)) => {
                let r = Self::blend(start_rgb.0, end_rgb.0, percentage);
                let g = Self::blend(start_rgb.1, end_rgb.1, percentage);
                let b = Self::blend(start_rgb.2, end_rgb.2, percentage);
                Color::from_rgb(r, g, b)
            }
            _ => {
                if percentage <= 50 {
                    start
                } else {
                    end
                }
            }
        }
    }
    fn percentage_to_char(list: &[char], proc: u8) -> char {
        let len = list.len();
        let csz = (100_usize / len).max(1);
        let index = (proc as usize / csz).min(len - 1);
        list[index]
    }
}

/// A cell with this value and the heat map's default fill and attribute.
impl<T: Number> From<T> for Cell<T> {
    fn from(value: T) -> Self {
        Self {
            value,
            label: String::new(),
            attr: None,
            fill: None,
        }
    }
}
