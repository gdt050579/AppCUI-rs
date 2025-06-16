/*
    Copyright (c) 2016 Jesse 'Jeaye' Wilkerson

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

    Except as contained in this notice, the name(s) of the above copyright
    holders shall not be used in advertising or otherwise to promote the
    sale, use or other dealings in this Software without prior written
    authorization.
*/

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]


use libc::{c_char, c_int, c_short};

#[allow(clippy::upper_case_acronyms)]
pub(crate) type WINDOW = *mut i8;
use super::constants::*;
use super::structs::MEVENT;

mod wrapped {
    use libc::{ c_char, c_int };

    use super::WINDOW;
    use super::chtype;
    
    extern "C"
    {
        pub(crate) static curscr: WINDOW;
        pub(crate) static newscr: WINDOW;
        pub(crate) static stdscr: WINDOW;
        pub(crate) static ttytype: *mut c_char;
        pub(crate) static COLORS: c_int;
        pub(crate) static COLOR_PAIRS: c_int;
        pub(crate) static COLS: c_int;
        pub(crate) static ESCDELAY: c_int;
        pub(crate) static LINES: c_int;
        pub(crate) static TABSIZE: c_int;

        /* Line graphics */
        pub(crate) static mut acs_map: [chtype; 0];
    }
}

macro_rules! wrap_extern {
    ($name:ident: $t:ty) => {
        pub(crate) fn $name() -> $t {
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
pub(crate) fn acs_map() -> *const chtype {
    &raw const wrapped::acs_map as *const chtype
}


#[cfg_attr(target_os = "linux", link(name = "ncursesw"))]
#[cfg_attr(target_os = "macos", link(name = "ncurses"))]  // on macos `ncurses` already includes wide chars support
#[cfg(target_family = "unix")]
extern "C"{
    pub(crate) fn initscr() -> WINDOW;
    pub(crate) fn endwin() -> c_int;
    pub(crate) fn refresh() -> c_int;
    pub(crate) fn wrefresh(w:WINDOW) -> c_int;
    pub(crate) fn wresize(win: WINDOW, lines: c_int, columns: c_int) -> c_int;
    pub(crate) fn getch() -> c_int;
    pub(crate) fn nodelay(win: WINDOW, bf: c_bool) -> c_int;
    pub(crate) fn halfdelay(tens: c_int) -> c_int;
    pub(crate) fn keypad(win: WINDOW, bf: c_bool) -> c_int;
    pub(crate) fn cbreak() -> c_int;
    pub(crate) fn noecho() -> c_int;
    pub(crate) fn nonl() -> c_int;
    pub(crate) fn raw() -> c_int;
    pub(crate) fn meta(win: WINDOW, bf: c_bool) -> c_int;
    pub(crate) fn mousemask(_:mmask_t,_:*mut mmask_t) -> mmask_t;
    pub(crate) fn mouseinterval(interval: c_int) -> c_int;
    pub(crate) fn getmouse(_:*mut MEVENT) -> c_int;
    pub(crate) fn wmove(_:WINDOW,_:c_int,_:c_int) -> c_int;
    pub(crate) fn set_escdelay(ms: c_int) -> c_int;
    pub(crate) fn wclear(w: WINDOW) -> c_int;
    pub(crate) fn mvaddch(y: c_int, x: c_int, ch: chtype) -> c_int;
    pub(crate) fn mvaddwstr(y: c_int, x: c_int, str: *const i8) -> c_int;
    pub(crate) fn getmaxy(w: WINDOW) -> c_int;
    pub(crate) fn getmaxx(w: WINDOW) -> c_int;
    pub(crate) fn wget_wch(w: WINDOW, _:*mut winttype) -> c_int;
    pub(crate) fn get_wch(_:*mut winttype) -> c_int;
    
    pub(crate) fn start_color() -> c_int;
    pub(crate) fn use_default_colors() -> c_int;
    pub(crate) fn init_pair(_:c_short,_:c_short,_:c_short) -> c_int;
    pub(crate) fn wattron(_:WINDOW, _:NCURSES_ATTR_T) -> c_int;
    pub(crate) fn wattroff(_:WINDOW, _:NCURSES_ATTR_T) -> c_int;
    pub(crate) fn COLOR_PAIR(_:c_int) -> c_int;

    
    pub(crate) fn mvaddstr(_:c_int, _:c_int, _:*const c_char) -> c_int;
    pub(crate) fn addstr(_:*const c_char) -> c_int;
    pub(crate) fn curs_set(_:c_int) -> c_int;

    pub(crate) fn wcwidth(c: i32) -> c_int;
    
}