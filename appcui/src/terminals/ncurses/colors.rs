use std::collections::HashMap;

use super::super::super::graphics::Color;
use super::ncursesapi;
#[derive(Clone, Copy, Debug)]
pub enum NCursesColor {
    Black = 0,
    Red = 1,
    Green = 2,
    Olive = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    Silver = 7,
    Gray = 8,
    LightRed = 9,
    LightGreen = 10,
    Yellow = 11,
    LightBlue = 12,
    LightMagenta = 13,
    LightCyan = 14,
    White = 15,
    Transparent = 16,
}
// impl NCursesColor {
//     pub fn from_value(value: i32) -> Option<NCursesColor> {
//         match value {
//             0 => Some(NCursesColor::Black),
//             1 => Some(NCursesColor::Red),
//             2 => Some(NCursesColor::Green),
//             3 => Some(NCursesColor::Olive),
//             4 => Some(NCursesColor::Blue),
//             5 => Some(NCursesColor::Magenta),
//             6 => Some(NCursesColor::Cyan),
//             7 => Some(NCursesColor::Silver),
//             8 => Some(NCursesColor::Gray),
//             9 => Some(NCursesColor::LightRed),
//             10 => Some(NCursesColor::LightGreen),
//             11 => Some(NCursesColor::Yellow),
//             12 => Some(NCursesColor::LightBlue),
//             13 => Some(NCursesColor::LightMagenta),
//             14 => Some(NCursesColor::LightCyan),
//             15 => Some(NCursesColor::White),
//             16 => Some(NCursesColor::Transparent),
//             _ => None,
//         }
//     }
// }

pub struct ColorManager {
    nr_colors: i16,
    color_mapping: HashMap<i16, bool>
}

#[cfg(target_family = "unix")]
impl ColorManager {
    const NR_COLORS: i16 = 16;
    const PAIR_MAPPING: [i16; 256] = [
        1,   2,    3,   4,   5,   6,   7,   8,   9,   10,  11,  12,  13,  14,  15,  16,  17,  18,  19,  20,  21,  22,
        23,  24,   25,  26,  27,  28,  29,  30,  31,  32,  33,  34,  35,  36,  37,  38,  39,  40,  41,  42,  43,  44,
        45,  46,   47,  48,  49,  50,  51,  52,  53,  54,  55,  56,  57,  58,  59,  60,  61,  62,  63,  64,  65,  66,
        67,  68,   69,  70,  71,  72,  73,  74,  75,  76,  77,  78,  79,  80,  81,  82,  83,  84,  85,  86,  87,  88,
        89,  90,   91,  92,  93,  94,  95,  96,  97,  98,  99,  100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
        111, 112,  113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132,
        133, 134,  135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
        155, 156,  157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176,
        177, 178,  179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198,
        199, 200,  201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
        221, 222,  223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 0,   241,
        242, 243,  244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255];

    pub(crate) fn new() -> ColorManager {
        let mut result = ColorManager{nr_colors : 0, color_mapping: HashMap::new()};
        ncursesapi::lib::ncurses_start_color();
        ncursesapi::lib::ncurses_use_default_colors();
        result.nr_colors = ncursesapi::constants::NR_COLORS;
        result
    }

    pub fn translate_to_ncurses_color(color: &Color) -> NCursesColor {
        match color{
            Color::Black => NCursesColor::Black,
            Color::DarkRed => NCursesColor::Red,
            Color::DarkGreen => NCursesColor::Green,
            Color::Olive => NCursesColor::Olive,
            Color::DarkBlue => NCursesColor::Blue,
            Color::Magenta => NCursesColor::Magenta,
            Color::Teal => NCursesColor::Cyan,
            Color::Silver => NCursesColor::Silver,
            Color::Gray => NCursesColor::Gray,
            Color::Red => NCursesColor::LightRed,
            Color::Green => NCursesColor::LightGreen,
            Color::Yellow => NCursesColor::Yellow,
            Color::Blue => NCursesColor::LightBlue,
            Color::Pink => NCursesColor::LightMagenta,
            Color::Aqua => NCursesColor::LightCyan,
            Color::White => NCursesColor::White,
            Color::Transparent => NCursesColor::Transparent,
        }
    }

    // pub fn from_int(value: i32) -> NCursesColor {
    //     NCursesColor::from_value(value).unwrap_or(NCursesColor::Transparent)
    // }

    pub fn set_color_pair(&mut self, foreground: &Color, background: &Color) {
        
        let foreground_color = ColorManager::translate_to_ncurses_color(foreground);
        let background_color = ColorManager::translate_to_ncurses_color(background);
        let pair_index = foreground_color as i16 * ColorManager::NR_COLORS + background_color as i16;
        let pair_index = ColorManager::PAIR_MAPPING[pair_index as usize];
        
        if !self.color_mapping.contains_key(&pair_index) {
            ncursesapi::lib::ncurses_init_pair(pair_index, foreground_color as i16, background_color as i16);
        }
        self.color_mapping.insert(pair_index, true);

        ncursesapi::lib::ncurses_wattron(ncursesapi::lib::ncurses_stdscr(), ncursesapi::lib::ncurses_COLOR_PAIR(pair_index));
    }

    pub fn unset_color_pair(&mut self, foreground: &Color, background: &Color) {
        
        let foreground_color = ColorManager::translate_to_ncurses_color(foreground);
        let background_color = ColorManager::translate_to_ncurses_color(background);
        let pair_index = foreground_color as i16 * ColorManager::NR_COLORS + background_color as i16;

        let pair_index = ColorManager::PAIR_MAPPING[pair_index as usize];
        ncursesapi::lib::ncurses_wattroff(ncursesapi::lib::ncurses_stdscr(), ncursesapi::lib::ncurses_COLOR_PAIR(pair_index));
    }
}
