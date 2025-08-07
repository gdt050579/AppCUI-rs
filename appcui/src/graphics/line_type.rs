use appcui_proc_macro::EnumSelector;
use crate::ui::selector::EnumSelector;

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

static LINE_TYPE_CHARS: [LineTypeChars; 7] = [
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
];

/// LineType is an enum that represents the type of line to be drawn (single, double, thick, etc)
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumSelector)]
pub enum LineType {
    #[VariantInfo(name = "Single Lines", description = "Single lines with corners and vertical/horizontal lines")]
    Single,
    #[VariantInfo(name = "Double Lines", description = "Double lines with corners and vertical/horizontal lines")]
    Double,
    #[VariantInfo(name = "Single Thick Lines", description = "Single thick lines with corners and vertical/horizontal lines")]
    SingleThick,
    #[VariantInfo(name = "Border", description = "A border style with thick lines")]
    Border,
    #[VariantInfo(name = "Ascii", description = "ASCII characters for lines")]
    Ascii,
    #[VariantInfo(name = "Ascii Round", description = "ASCII characters with rounded corners")]
    AsciiRound,
    #[VariantInfo(name = "Single Round", description = "Single lines with rounded corners")]
    SingleRound,
}

impl LineType {
    pub(super) fn charset(&self) -> &'static LineTypeChars {
        &LINE_TYPE_CHARS[(*self as u8) as usize]
    }
}
