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

static LINE_TYPE_CHARS: [LineTypeChars; 6] = [
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
        horizontal: '\u{2502}',
        vertical: '\u{2500}',
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
        horizontal: '\u{2551}',
        vertical: '\u{2550}',
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
        horizontal: '\u{2503}',
        vertical: '\u{2501}',
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
];

/*
struct AppCUI::Graphics::LineTypeChars line_types_chars[] = {
    //{ 0x250C, 0x2500, 0x2510, 0x2502, 0x2518, 0x2500, 0x2514, 0x2502 }, /* Single Lines */
    //{ 0x2554, 0x2550, 0x2557, 0x2551, 0x255D, 0x2550, 0x255A, 0x2551 }, /* Double Lines */
    //{ 0x250F, 0x2501, 0x2513, 0x2503, 0x251B, 0x2501, 0x2517, 0x2503 }, /* Single Thick lines */
    //{ 0x2584, 0x2584, 0x2584, 0x2588, 0x2580, 0x2580, 0x2580, 0x2588 }, /* Border */
    //{ '+', '-', '+', '|', '+', '-', '+', '|' },                         /* Ascii */
    //{ '/', '-', '\\', '|', '/', '-', '\\', '|' },                       /* Ascii Round */
    { 0x256D, 0x2500, 0x256E, 0x2502, 0x256F, 0x2500, 0x2570, 0x2502 }, /* Single Round */
};
*/

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum LineType {
    Single = 0,
    Double,
    SingleThick,
    Border,
    Ascii,
    AsciiRound,
    SingleRound,
}

impl LineType {
    pub(super) fn get_chars(&self) -> &'static LineTypeChars {
        return &LINE_TYPE_CHARS[(*self as u8) as usize];
    }
}
