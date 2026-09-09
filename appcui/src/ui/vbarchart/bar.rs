use crate::{graphics::{CharAttribute, Character, Rect, Surface}, ui::common::Number};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BarDrawMode {
    Normal,     // spaces
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
    pub(crate) attr: Option<CharAttribute>,
    pub(crate) label: String,
    pub(crate) thickness: u8,
    pub(crate) spacing: u8,
    pub(crate) draw_mode: BarDrawMode,
}

impl<T: Number + 'static> Bar<T> {
    pub(super) fn paint(&self, surface: &mut Surface, attr: CharAttribute, rect: Rect) {
        match self.draw_mode {
            BarDrawMode::Normal => surface.fill_rect(rect, Character::with_attributes(' ', attr)),
            BarDrawMode::Rectangle => todo!(),
            BarDrawMode::Char(ch) => surface.fill_rect(rect, Character::with_attributes(ch, attr)),
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
                thickness: 1,
                spacing: 1,
                draw_mode: BarDrawMode::Normal,
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
        self.bar.thickness = thickness;
        self
    }
    pub fn spacing(mut self, spacing: u8) -> Self {
        self.bar.spacing = spacing;
        self
    }
    pub fn draw_mode(mut self, draw_mode: BarDrawMode) -> Self {
        self.bar.draw_mode = draw_mode;
        self
    }
    pub fn build(mut self) -> Bar<T> {
        self.bar.thickness = self.bar.thickness.max(self.bar.draw_mode.min_thickness());
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
