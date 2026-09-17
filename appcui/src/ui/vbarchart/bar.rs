use crate::{
    graphics::{CharAttribute, Character, Rect, SpecialChar, Surface},
    ui::common::Number,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BarDrawMode {
    #[default]
    Normal, // spaces
    Rectangle,  // implies thickness = minimum 2
    Char(char), // fill with a specific character
    SingleLine,
    DoubleLine,
}

impl BarDrawMode {
    fn min_thickness(&self) -> u8 {
        match self {
            BarDrawMode::Rectangle => 2,
            _ => 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Bar<T: Number + 'static> {
    pub(crate) value: T,
    pub(crate) label: String,
    pub(crate) attr: Option<CharAttribute>,
    pub(crate) thickness: Option<u8>,
    pub(crate) spacing: Option<u8>,
    pub(crate) draw_mode: Option<BarDrawMode>,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct BarDefaults {
    pub(crate) attr: CharAttribute,
    pub(crate) thickness: u8,
    pub(crate) spacing: u8,
    pub(crate) draw_mode: BarDrawMode,
}
#[derive(Copy, Clone, Default)]
pub(crate) struct BarLayout {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) length: i16,
    pub(crate) digits: u8,
}

impl<T: Number + 'static> Bar<T> {
    #[inline(always)]
    fn paint_vertical_rect(&self, surface: &mut Surface, c: Character, layout: &BarLayout, defaults: &BarDefaults) {
        let thickness = self.actual_thickness(&defaults) as u32;
        if layout.length == 0 {
            let ch = Character::new('_', c.foreground, c.background, c.flags);
            surface.fill_horizontal_line_with_size(layout.x, layout.y, thickness, ch);
        } else {
            let abs_h = layout.length.unsigned_abs();
            let top = if layout.length > 0 { layout.y + 1 - layout.length as i32 } else { layout.y + 1 };
            let r = Rect::with_size(layout.x, top, thickness as u16, abs_h);
            surface.fill_rect(r, c);
        }
    }
    #[inline(always)]
    pub(super) fn actual_thickness(&self, defaults: &BarDefaults) -> u8 {
        let mode = self.draw_mode.unwrap_or(defaults.draw_mode);
        self.thickness.unwrap_or(defaults.thickness).max(mode.min_thickness())
    }
    pub(super) fn paint_vertical(&self, surface: &mut Surface, layout: &BarLayout, defaults: &BarDefaults) {
        let mode = self.draw_mode.unwrap_or(defaults.draw_mode);
        let attr = self.attr.unwrap_or(defaults.attr);
        match mode {
            BarDrawMode::Normal => self.paint_vertical_rect(surface, Character::with_attributes(SpecialChar::Block100, attr), layout, defaults),
            BarDrawMode::Rectangle => todo!(),
            BarDrawMode::Char(ch) => self.paint_vertical_rect(surface, Character::with_attributes(ch, attr), layout, defaults),
            BarDrawMode::SingleLine => todo!(),
            BarDrawMode::DoubleLine => todo!(),
        }
    }
}

pub struct BarBuilder<T: Number + 'static> {
    bar: Bar<T>,
}
impl<T: Number + 'static> BarBuilder<T> {
    pub fn new(value: T) -> Self {
        Self {
            bar: Bar {
                value,
                attr: None,
                label: String::new(),
                thickness: None,
                spacing: None,
                draw_mode: None,
            },
        }
    }
    pub fn attr(mut self, attr: CharAttribute) -> Self {
        self.bar.attr = Some(attr);
        self
    }
    pub fn label(mut self, label: &str) -> Self {
        self.bar.label = label.to_string();
        self
    }
    pub fn thickness(mut self, thickness: u8) -> Self {
        self.bar.thickness = Some(thickness);
        self
    }
    pub fn spacing(mut self, spacing: u8) -> Self {
        self.bar.spacing = Some(spacing);
        self
    }
    pub fn draw_mode(mut self, draw_mode: BarDrawMode) -> Self {
        self.bar.draw_mode = Some(draw_mode);
        self
    }
    pub fn build(self) -> Bar<T> {
        self.bar
    }
}

impl<T> From<T> for Bar<T>
where
    T: Number + 'static,
{
    fn from(value: T) -> Self {
        BarBuilder::new(value).build()
    }
}
impl<T> From<&T> for Bar<T>
where
    T: Number + 'static,
{
    fn from(value: &T) -> Self {
        (*value).into()
    }
}
