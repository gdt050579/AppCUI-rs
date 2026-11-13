use crate::{
    graphics::{CharAttribute, Character, Rect, Size, Surface}, prelude::Number,
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
    fn attribute_from_range(min: CharAttribute, max: CharAttribute, percentage: u8) -> CharAttribute {
        todo!()
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
