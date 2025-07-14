use appcui::prelude::*;

#[derive(Copy,Clone,PartialEq,Eq)]
pub(crate) enum Shape {
    Square,
    Rectangle,
    Triangle,
    Circle
}

impl Shape {
    pub(crate) fn image(&self) -> Image {
        let s= match self {
            Shape::Square => "|........|,|.RRRRRR.|,|.RRRRRR.|,|.RRRRRR.|,|.RRRRRR.|,|.RRRRRR.|,|.RRRRRR.|,|........|",
            Shape::Rectangle => "|........|,|........|,|.GGGGGG.|,|.GGGGGG.|,|.GGGGGG.|,|.GGGGGG.|,|........|,|........|",
            Shape::Triangle => "|........|,|...BB...|,|...BB...|,|..BBBB..|,|..BBBB..|,|.BBBBBB.|,|.BBBBBB.|,|........|",
            Shape::Circle => "|..wwww..|,|.wwwwww.|,|wwwwwwww|,|wwwwwwww|,|wwwwwwww|,|wwwwwwww|,|.wwwwww.|,|..wwww..|",
        };
        Image::from_str(s).unwrap()
    }
}

impl EnumSelector for Shape {
    const COUNT: u32 = 4;

    fn from_index(index: u32) -> Option<Self> where Self: Sized {
        match index {
            0 => Some(Shape::Square),
            1 => Some(Shape::Rectangle),
            2 => Some(Shape::Triangle),
            3 => Some(Shape::Circle),
            _ => None
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Shape::Square => "Square",
            Shape::Rectangle => "Rectangle",
            Shape::Triangle => "Triangle",
            Shape::Circle => "Circle",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Shape::Square => "a red square",
            Shape::Rectangle => "a green rectangle",
            Shape::Triangle => "a blue triangle",
            Shape::Circle => "a white circle",
        }        
    }
}