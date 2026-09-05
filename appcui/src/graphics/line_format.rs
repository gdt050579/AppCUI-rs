use super::CharAttribute;
use crate::ui::selector::EnumSelector;
use appcui_proc_macro::EnumSelector;

pub(super) struct LineTypeChars {
    pub(super) corner_top_left: char,
    pub(super) horizontal_on_top: char,
    pub(super) corner_top_right: char,
    pub(super) vertical_on_right: char,
    pub(super) corner_bottom_right: char,
    pub(super) horizontal_on_bottom: char,
    pub(super) corner_bottom_left: char,
    pub(super) vertical_on_left: char,
    pub(super) horizontal: char,
    pub(super) vertical: char,
}

static LINE_TYPE_CHARS: [LineTypeChars; 8] = [
    /* Single Lines */
    LineTypeChars {
        corner_top_left: '\u{250C}',
        horizontal_on_top: '\u{2500}',
        corner_top_right: '\u{2510}',
        vertical_on_right: '\u{2502}',
        corner_bottom_right: '\u{2518}',
        horizontal_on_bottom: '\u{2500}',
        corner_bottom_left: '\u{2514}',
        vertical_on_left: '\u{2502}',
        horizontal: '\u{2500}',
        vertical: '\u{2502}',
    },
    /* Double Lines */
    LineTypeChars {
        corner_top_left: '\u{2554}',
        horizontal_on_top: '\u{2550}',
        corner_top_right: '\u{2557}',
        vertical_on_right: '\u{2551}',
        corner_bottom_right: '\u{255D}',
        horizontal_on_bottom: '\u{2550}',
        corner_bottom_left: '\u{255A}',
        vertical_on_left: '\u{2551}',
        horizontal: '\u{2550}',
        vertical: '\u{2551}',
    },
    /* Single Thick lines */
    LineTypeChars {
        corner_top_left: '\u{250F}',
        horizontal_on_top: '\u{2501}',
        corner_top_right: '\u{2513}',
        vertical_on_right: '\u{2503}',
        corner_bottom_right: '\u{251B}',
        horizontal_on_bottom: '\u{2501}',
        corner_bottom_left: '\u{2517}',
        vertical_on_left: '\u{2503}',
        horizontal: '\u{2501}',
        vertical: '\u{2503}',
    },
    /* Border */
    LineTypeChars {
        corner_top_left: '\u{2584}',
        horizontal_on_top: '\u{2584}',
        corner_top_right: '\u{2584}',
        vertical_on_right: '\u{2588}',
        corner_bottom_right: '\u{2580}',
        horizontal_on_bottom: '\u{2580}',
        corner_bottom_left: '\u{2580}',
        vertical_on_left: '\u{2588}',
        horizontal: '\u{2588}',
        vertical: '\u{2588}',
    },
    /* Ascii */
    LineTypeChars {
        corner_top_left: '+',
        horizontal_on_top: '-',
        corner_top_right: '+',
        vertical_on_right: '|',
        corner_bottom_right: '+',
        horizontal_on_bottom: '-',
        corner_bottom_left: '+',
        vertical_on_left: '|',
        horizontal: '-',
        vertical: '|',
    },
    /* Ascii Round*/
    LineTypeChars {
        corner_top_left: '/',
        horizontal_on_top: '-',
        corner_top_right: '\\',
        vertical_on_right: '|',
        corner_bottom_right: '/',
        horizontal_on_bottom: '-',
        corner_bottom_left: '\\',
        vertical_on_left: '|',
        horizontal: '-',
        vertical: '|',
    },
    /* SingleRound */
    LineTypeChars {
        corner_top_left: '\u{256D}',
        horizontal_on_top: '\u{2500}',
        corner_top_right: '\u{256E}',
        vertical_on_right: '\u{2502}',
        corner_bottom_right: '\u{256F}',
        horizontal_on_bottom: '\u{2500}',
        corner_bottom_left: '\u{2570}',
        vertical_on_left: '\u{2502}',
        horizontal: '\u{2500}',
        vertical: '\u{2502}',
    },
    /* Braille double line */
    LineTypeChars {
        corner_top_left: '\u{28F6}',
        corner_top_right: '\u{28F6}',
        corner_bottom_right: '\u{283F}',
        corner_bottom_left: '\u{283F}',

        vertical_on_right: '\u{28FF}',
        vertical_on_left: '\u{28FF}',
        vertical: '\u{28FF}',
        horizontal_on_bottom: '\u{2836}',
        horizontal_on_top: '\u{2836}',
        horizontal: '\u{2836}',
    },
];

/// LineType is an enum that represents the type of line to be drawn (single, double, thick, etc)
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumSelector)]
pub enum LineType {
    #[VariantInfo(name = "Single Lines", description = "Single lines with corners and vertical/horizontal lines")]
    Single,
    #[VariantInfo(name = "Double Lines", description = "Double lines with corners and vertical/horizontal lines")]
    Double,
    #[VariantInfo(
        name = "Single Thick Lines",
        description = "Single thick lines with corners and vertical/horizontal lines"
    )]
    SingleThick,
    #[VariantInfo(name = "Border", description = "A border style with thick lines")]
    Border,
    #[VariantInfo(name = "Ascii", description = "ASCII characters for lines")]
    Ascii,
    #[VariantInfo(name = "Ascii Round", description = "ASCII characters with rounded corners")]
    AsciiRound,
    #[VariantInfo(name = "Single Round", description = "Single lines with rounded corners")]
    SingleRound,
    #[VariantInfo(name = "Braille", description = "Double line drawn with braille characters")]
    Braille,
}

impl LineType {
    pub(super) fn charset(&self) -> &'static LineTypeChars {
        &LINE_TYPE_CHARS[(*self as u8) as usize]
    }
}

#[derive(Copy, Clone)]
pub(super) struct LineCapChars {
    pub(super) up: char,
    pub(super) down: char,
    pub(super) left: char,
    pub(super) right: char,
}

static LINE_CAP_CHARS_ARROWS: LineCapChars = LineCapChars {
    up: '\u{25B2}',
    down: '\u{25BC}',
    left: '\u{25C0}',
    right: '\u{25B6}',
};
static LINE_CAP_CHARS_TRIANGLES: LineCapChars = LineCapChars {
    up: '\u{25B2}',
    down: '\u{25BC}',
    left: '\u{25C0}',
    right: '\u{25B6}',
};

#[derive(Copy, Clone)]
pub enum LineCap {
    Arrow, // direction inferred from the terminal segment at draw time
    Triangle,
    Char(char),
}
impl LineCap {
    pub(super) fn charset(&self) -> Option<&'static LineCapChars> {
        match self {
            LineCap::Arrow => Some(&LINE_CAP_CHARS_ARROWS),
            LineCap::Triangle => Some(&LINE_CAP_CHARS_TRIANGLES),
            LineCap::Char(_) => None
        }
    }
}

#[derive(Copy, Clone)]
pub struct PolyLineFormat {
    pub(crate) line_type: LineType,
    pub(crate) attr: CharAttribute,

    pub(crate) start_cap: Option<LineCap>,
    pub(crate) start_attr: Option<CharAttribute>, // None => inherit `attr`
    pub(crate) end_cap: Option<LineCap>,
    pub(crate) end_attr: Option<CharAttribute>, // None => inherit `attr`

    pub(crate) joint: Option<char>,               // None => auto-resolve corner glyph from directions
    pub(crate) joint_attr: Option<CharAttribute>, // None => inherit `attr`
}

pub struct PolyLineFormatBuilder {
    format: PolyLineFormat,
}

impl PolyLineFormatBuilder {
    pub fn new(line_type: LineType, attr: CharAttribute) -> Self {
        Self {
            format: PolyLineFormat {
                line_type: line_type,
                attr: attr,
                start_cap: None,
                start_attr: None,
                end_cap: None,
                end_attr: None,
                joint: None,
                joint_attr: None,
            },
        }
    }
}
impl PolyLineFormatBuilder {
    pub fn line_type(mut self, line_type: LineType) -> Self {
        self.format.line_type = line_type;
        self
    }
    pub fn attr(mut self, attr: CharAttribute) -> Self {
        self.format.attr = attr;
        self
    }
    pub fn start_cap(mut self, start_cap: LineCap) -> Self {
        self.format.start_cap = Some(start_cap);
        self
    }
    pub fn start_attr(mut self, start_attr: CharAttribute) -> Self {
        self.format.start_attr = Some(start_attr);
        self
    }
    pub fn end_cap(mut self, end_cap: LineCap) -> Self {
        self.format.end_cap = Some(end_cap);
        self
    }
    pub fn end_attr(mut self, end_attr: CharAttribute) -> Self {
        self.format.end_attr = Some(end_attr);
        self
    }
    pub fn joint(mut self, joint: char) -> Self {
        self.format.joint = Some(joint);
        self
    }
    pub fn joint_attr(mut self, joint_attr: CharAttribute) -> Self {
        self.format.joint_attr = Some(joint_attr);
        self
    }
    pub fn build(self) -> PolyLineFormat {
        self.format
    }
}
