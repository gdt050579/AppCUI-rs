#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use libc::{c_char, c_int, c_short};

pub type WINDOW = *mut i8;
use super::constants::*;
use super::structs::MEVENT;

mod wrapped {
    use libc::{ c_char, c_int };

    use super::WINDOW;
    use super::chtype;
    
    extern "C"
    {
        pub static curscr: WINDOW;
        pub static newscr: WINDOW;
        pub static stdscr: WINDOW;
        pub static ttytype: *mut c_char;
        pub static COLORS: c_int;
        pub static COLOR_PAIRS: c_int;
        pub static COLS: c_int;
        pub static ESCDELAY: c_int;
        pub static LINES: c_int;
        pub static TABSIZE: c_int;

        /* Line graphics */
        pub static mut acs_map: [chtype; 0];
    }
}

macro_rules! wrap_extern {
    ($name:ident: $t:ty) => {
        pub fn $name() -> $t {
            unsafe { wrapped::$name }
        }
    }
}


wrap_extern!(curscr: WINDOW);
wrap_extern!(newscr: WINDOW);
wrap_extern!(stdscr: WINDOW);
wrap_extern!(ttytype: *mut c_char);
wrap_extern!(COLORS: c_int);
wrap_extern!(COLOR_PAIRS: c_int);
wrap_extern!(COLS: c_int);
wrap_extern!(ESCDELAY: c_int);
wrap_extern!(LINES: c_int);
wrap_extern!(TABSIZE: c_int);
pub fn acs_map() -> *const chtype {
    unsafe {
        &wrapped::acs_map as *const chtype
    }
}
// #[cfg(target_family = "windows")]
// #[link(name = "pdcurses")]
// extern {
//     fn initscr() -> WINDOW;
//     fn endwin() -> i32;
//     fn printw(s: *const i8);
//     fn refresh() -> i32;
//     fn getch() -> i32;
//     fn nodelay(win: WINDOW, bf: c_int) -> i32;
//     fn halfdelay(tens: c_int) -> i32;
//     fn keypad(win: WINDOW, bf: c_bool) -> c_int;
//     fn cbreak() -> i32;
//     fn noecho() -> i32;
//     fn nonl() -> i32;
//     fn raw() -> i32;
//     fn meta(win: WINDOW, bf: c_int) -> i32;
//     fn mousemask(newmask: c_uint, oldmask: *mut c_uint) -> c_uint;
//     fn mouseinterval(interval: c_int) -> c_int;
//     fn set_escdelay(ms: c_int) -> i32;
// }

#[cfg(target_family = "unix")]
#[link(name = "ncursesw")]
extern "C"{
    pub fn initscr() -> WINDOW;
    pub fn endwin() -> c_int;
    pub fn printw(s: *const i8);
    pub fn refresh() -> c_int;
    pub fn wrefresh(w:WINDOW) -> c_int;
    pub fn getch() -> c_int;
    pub fn nodelay(win: WINDOW, bf: c_bool) -> c_int;
    pub fn halfdelay(tens: c_int) -> c_int;
    pub fn keypad(win: WINDOW, bf: c_bool) -> c_int;
    pub fn cbreak() -> c_int;
    pub fn noecho() -> c_int;
    pub fn nonl() -> c_int;
    pub fn raw() -> c_int;
    pub fn meta(win: WINDOW, bf: c_bool) -> c_int;
    pub fn mousemask(_:mmask_t,_:*mut mmask_t) -> mmask_t;
    pub fn mouseinterval(interval: c_int) -> c_int;
    pub fn getmouse(_:*mut MEVENT) -> c_int;
    pub fn wmove(_:WINDOW,_:c_int,_:c_int) -> c_int;
    pub fn set_escdelay(ms: c_int) -> c_int;
    pub fn wclear(w: WINDOW) -> c_int;
    pub fn mvaddch(y: c_int, x: c_int, ch: chtype) -> c_int;
    pub fn getmaxy(w: WINDOW) -> c_int;
    pub fn getmaxx(w: WINDOW) -> c_int;
    pub fn wget_wch(w: WINDOW, _:*mut winttype) -> c_int;
    pub fn get_wch(_:*mut winttype) -> c_int;
    
    pub fn start_color() -> c_int;
    pub fn use_default_colors() -> c_int;
    pub fn init_pair(_:c_short,_:c_short,_:c_short) -> c_int;
    pub fn wattron(_:WINDOW, _:NCURSES_ATTR_T) -> c_int;
    pub fn wattroff(_:WINDOW, _:NCURSES_ATTR_T) -> c_int;
    pub fn COLOR_PAIR(_:c_int) -> c_int;

    
    // pub fn impl_ACS_ULCORNER() -> chtype;
    // pub fn impl_ACS_LLCORNER() -> chtype;
    // pub fn impl_ACS_URCORNER() -> chtype;
    // pub fn impl_ACS_LRCORNER() -> chtype;
    // pub fn impl_ACS_LTEE() -> chtype;
    // pub fn impl_ACS_RTEE() -> chtype;
    // pub fn impl_ACS_BTEE() -> chtype;
    // pub fn impl_ACS_TTEE() -> chtype;
    // pub fn impl_ACS_HLINE() -> chtype;
    // pub fn impl_ACS_VLINE() -> chtype;
    // pub fn impl_ACS_PLUS() -> chtype;
    // pub fn impl_ACS_S1() -> chtype;
    // pub fn impl_ACS_S9() -> chtype;
    // pub fn impl_ACS_DIAMOND() -> chtype;
    // pub fn impl_ACS_CKBOARD() -> chtype;
    // pub fn impl_ACS_DEGREE() -> chtype;
    // pub fn impl_ACS_PLMINUS() -> chtype;
    // pub fn impl_ACS_BULLET() -> chtype;
    // pub fn impl_ACS_LARROW() -> chtype;
    // pub fn impl_ACS_RARROW() -> chtype;
    // pub fn impl_ACS_DARROW() -> chtype;
    // pub fn impl_ACS_UARROW() -> chtype;
    // pub fn impl_ACS_BOARD() -> chtype;
    // pub fn impl_ACS_LANTERN() -> chtype;
    // pub fn impl_ACS_BLOCK() -> chtype;
    // pub fn impl_ACS_S3() -> chtype;
    // pub fn impl_ACS_S7() -> chtype;
    // pub fn impl_ACS_LEQUAL() -> chtype;
    // pub fn impl_ACS_GEQUAL() -> chtype;
    // pub fn impl_ACS_PI() -> chtype;
    // pub fn impl_ACS_NEQUAL() -> chtype;
    // pub fn impl_ACS_STERLING() -> chtype;
    // pub fn impl_ACS_BSSB() -> chtype;
    // pub fn impl_ACS_SSBB() -> chtype;
    // pub fn impl_ACS_BBSS() -> chtype;
    // pub fn impl_ACS_SBBS() -> chtype;
    // pub fn impl_ACS_SBSS() -> chtype;
    // pub fn impl_ACS_SSSB() -> chtype;
    // pub fn impl_ACS_SSBS() -> chtype;
    // pub fn impl_ACS_BSSS() -> chtype;
    // pub fn impl_ACS_BSBS() -> chtype;
    // pub fn impl_ACS_SBSB() -> chtype;
    // pub fn impl_ACS_SSSS() -> chtype;
}