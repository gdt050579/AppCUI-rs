    use crate::{graphics::CharAttribute, ui::common::Number};

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
                _ => 1
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Bar<T: Number + 'static> {
        value: T,
        attr: Option<CharAttribute>,
        label: String,
        thickness: u8,
        spacing: u8,
        draw_mode: BarDrawMode,
    }

    pub(crate) struct BarWithLayout<T: Number + 'static> {
        bar: Bar<T>,
        x: i32,
        y: i32,
        h: u32,
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

    impl<T> From<T> for Bar<T> where T: Number + 'static {
        fn from(value: T) -> Self {
            BarBuilder::new(value).build()
        }
    }
