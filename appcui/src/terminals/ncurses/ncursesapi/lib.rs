#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::{CStr, CString};
use std::ptr;
use super::constants::*;
use super::externs::*;
use super::structs::{MEVENT, LcCategory, WchResult};


pub type WINDOW = *mut i8;


pub fn ncurses_initscr() -> WINDOW {
    unsafe { initscr() }
}

pub fn ncurses_endwin() -> i32 {
    unsafe { endwin() }
}

pub fn ncurses_printw(s: &str) {
    unsafe {
        let c_str = CString::new(s).unwrap();
        printw(c_str.as_ptr());
    }
}

pub fn ncurses_refresh() -> i32 {
    unsafe { refresh() }
}

pub fn ncurses_wrefresh(w: WINDOW) -> i32{ 
    unsafe { wrefresh(w) } 
}

pub fn ncurses_getmouse(event: *mut MEVENT) -> i32
{ unsafe { getmouse(event) } }

pub fn ncurses_getch() -> i32 {
    unsafe { getch() }
}

pub fn ncurses_nodelay(win: WINDOW, bf: bool) -> i32 {
    unsafe { nodelay(win, bf as c_bool) }
}

pub fn ncurses_halfdelay(tens: i32) -> i32 {
    unsafe { halfdelay(tens) }
}

pub fn ncurses_keypad(win: WINDOW, bf: bool) -> i32 {
    unsafe { keypad(win, bf as c_bool) }
}

pub fn ncurses_cbreak() -> i32 {
    unsafe { cbreak() }
}

pub fn ncurses_noecho() -> i32 {
    unsafe { noecho() }
}

pub fn ncurses_nonl() -> i32 {
    unsafe { nonl() }
}

pub fn ncurses_raw() -> i32 {
    unsafe { raw() }
}

pub fn ncurses_meta(win: WINDOW, bf: bool) -> i32 {
    unsafe { meta(win, bf as c_bool) }
}

pub fn ncurses_mousemask(newmask: mmask_t, oldmask: Option<&mut mmask_t>) -> mmask_t
{
    match oldmask {
	None => { unsafe { mousemask(newmask, ptr::null_mut()) } },
	Some(old) => { unsafe { mousemask(newmask, old) } },
    }
}

pub fn ncurses_mouseinterval(interval: i32) -> i32 {
    unsafe { mouseinterval(interval) }
}

pub fn ncurses_wmove(w: WINDOW, y: i32, x: i32) -> i32
{ unsafe { wmove(w, y, x) } }

pub fn ncurses_set_escdelay(ms: i32) -> i32 {
    unsafe { set_escdelay(ms) }
}
pub fn ncurses_wclear(w: WINDOW) -> i32 {
    unsafe { wclear(w) }
}

pub fn ncurses_mvaddch(y: i32, x: i32, ch: chtype) -> i32 {
    unsafe { mvaddch(y, x, ch) }
}

pub fn ncurses_getmaxyx(w: WINDOW,  y: &mut i32, x: &mut i32) {
    unsafe {
        *y = getmaxy(w);
        *x = getmaxx(w);
    }
}

pub fn ncurses_wget_wch(w:WINDOW) -> Option<WchResult> {
    unsafe {
        let mut x = 0;
        match wget_wch(w,&mut x) {
            OK => {
                Some(WchResult::Char(x))
            }
            KEY_CODE_YES => {
                Some(WchResult::KeyCode(x as i32))
            }
            _ => {
                None
            }
        }
    }
}

pub fn ncurses_get_wch() -> Option<WchResult> {
    unsafe {
        let mut x = 0;
        match get_wch(&mut x) {
            OK => {
                Some(WchResult::Char(x))
            }
            KEY_CODE_YES => {
                Some(WchResult::KeyCode(x as i32))
            }
            _ => {
                None
            }
        }
    }
}

trait ToCStr {
    fn to_c_str(&self) -> Result<CString, std::ffi::NulError>;
}

impl <'a>ToCStr for &'a str {
    fn to_c_str(&self) -> Result<CString, std::ffi::NulError> {
        CString::new(*self)
    }
}

pub fn setlocale(lc: LcCategory, locale: &str) -> Result<String, std::ffi::NulError>
{
  unsafe {
    let c_str = locale.to_c_str()?;
    let buf = c_str.as_ptr();
    let ret = libc::setlocale(lc as libc::c_int, buf);
    if ret == ptr::null_mut() {
        Ok(String::new())
    } else {
        // The clone is necessary, as the returned pointer
        // can change at any time
        Ok(CStr::from_ptr(ret).to_string_lossy().into_owned())
    }
  }
}

pub fn ncurses_start_color() -> i32
{ unsafe { start_color() } }

pub fn ncurses_use_default_colors() -> i32
{ unsafe { use_default_colors() } }

pub fn ncurses_init_pair(pair: i16, f: i16, b: i16) -> i32
{ unsafe { init_pair(pair, f, b) } }

pub fn ncurses_wattron(w: WINDOW, attr: NCURSES_ATTR_T) -> i32
{ unsafe { wattron(w, attr) } }


pub fn ncurses_wattroff(w: WINDOW, attr: NCURSES_ATTR_T) -> i32
{ unsafe { wattroff(w, attr) } }

pub fn ncurses_COLOR_PAIR(n: i16) -> attr_t {
    unsafe {
        COLOR_PAIR(n as i32) as attr_t
    }
}

pub fn ncurses_stdscr() -> WINDOW {
    unsafe {
        stdscr()
    }
}

// pub fn ncurses_ACS_ULCORNER() -> chtype {
//     unsafe { impl_ACS_ULCORNER() }
// }
// pub fn ncurses_ACS_LLCORNER() -> chtype {
//     unsafe { impl_ACS_LLCORNER() }
// }
// pub fn ncurses_ACS_URCORNER() -> chtype {
//     unsafe { impl_ACS_URCORNER() }
// }
// pub fn ncurses_ACS_LRCORNER() -> chtype {
//     unsafe { impl_ACS_LRCORNER() }
// }
// pub fn ncurses_ACS_LTEE() -> chtype {
//     unsafe { impl_ACS_LTEE() }
// }
// pub fn ncurses_ACS_RTEE() -> chtype {
//     unsafe { impl_ACS_RTEE() }
// }
// pub fn ncurses_ACS_BTEE() -> chtype {
//     unsafe { impl_ACS_BTEE() }
// }
// pub fn ncurses_ACS_TTEE() -> chtype {
//     unsafe { impl_ACS_TTEE() }
// }
// pub fn ncurses_ACS_HLINE() -> chtype {
//     unsafe { impl_ACS_HLINE() }
// }
// pub fn ncurses_ACS_VLINE() -> chtype {
//     unsafe { impl_ACS_VLINE() }
// }
// pub fn ncurses_ACS_PLUS() -> chtype {
//     unsafe { impl_ACS_PLUS() }
// }
// pub fn ncurses_ACS_S1() -> chtype {
//     unsafe { impl_ACS_S1() }
// }
// pub fn ncurses_ACS_S9() -> chtype {
//     unsafe { impl_ACS_S9() }
// }
// pub fn ncurses_ACS_DIAMOND() -> chtype {
//     unsafe { impl_ACS_DIAMOND() }
// }
// pub fn ncurses_ACS_CKBOARD() -> chtype {
//     unsafe { impl_ACS_CKBOARD() }
// }
// pub fn ncurses_ACS_DEGREE() -> chtype {
//     unsafe { impl_ACS_DEGREE() }
// }
// pub fn ncurses_ACS_PLMINUS() -> chtype {
//     unsafe { impl_ACS_PLMINUS() }
// }
// pub fn ncurses_ACS_BULLET() -> chtype {
//     unsafe { impl_ACS_BULLET() }
// }
// pub fn ncurses_ACS_LARROW() -> chtype {
//     unsafe { impl_ACS_LARROW() }
// }
// pub fn ncurses_ACS_RARROW() -> chtype {
//     unsafe { impl_ACS_RARROW() }
// }
// pub fn ncurses_ACS_DARROW() -> chtype {
//     unsafe { impl_ACS_DARROW() }
// }
// pub fn ncurses_ACS_UARROW() -> chtype {
//     unsafe { impl_ACS_UARROW() }
// }
// pub fn ncurses_ACS_BOARD() -> chtype {
//     unsafe { impl_ACS_BOARD() }
// }
// pub fn ncurses_ACS_LANTERN() -> chtype {
//     unsafe { impl_ACS_LANTERN() }
// }
// pub fn ncurses_ACS_BLOCK() -> chtype {
//     unsafe { impl_ACS_BLOCK() }
// }
// pub fn ncurses_ACS_S3() -> chtype {
//     unsafe { impl_ACS_S3() }
// }
// pub fn ncurses_ACS_S7() -> chtype {
//     unsafe { impl_ACS_S7() }
// }
// pub fn ncurses_ACS_LEQUAL() -> chtype {
//     unsafe { impl_ACS_LEQUAL() }
// }
// pub fn ncurses_ACS_GEQUAL() -> chtype {
//     unsafe { impl_ACS_GEQUAL() }
// }
// pub fn ncurses_ACS_PI() -> chtype {
//     unsafe { impl_ACS_PI() }
// }
// pub fn ncurses_ACS_NEQUAL() -> chtype {
//     unsafe { impl_ACS_NEQUAL() }
// }
// pub fn ncurses_ACS_STERLING() -> chtype {
//     unsafe { impl_ACS_STERLING() }
// }
// pub fn ncurses_ACS_BSSB() -> chtype {
//     unsafe { impl_ACS_BSSB() }
// }
// pub fn ncurses_ACS_SSBB() -> chtype {
//     unsafe { impl_ACS_SSBB() }
// }
// pub fn ncurses_ACS_BBSS() -> chtype {
//     unsafe { impl_ACS_BBSS() }
// }
// pub fn ncurses_ACS_SBBS() -> chtype {
//     unsafe { impl_ACS_SBBS() }
// }
// pub fn ncurses_ACS_SBSS() -> chtype {
//     unsafe { impl_ACS_SBSS() }
// }
// pub fn ncurses_ACS_SSSB() -> chtype {
//     unsafe { impl_ACS_SSSB() }
// }
// pub fn ncurses_ACS_SSBS() -> chtype {
//     unsafe { impl_ACS_SSBS() }
// }
// pub fn ncurses_ACS_BSSS() -> chtype {
//     unsafe { impl_ACS_BSSS() }
// }
// pub fn ncurses_ACS_BSBS() -> chtype {
//     unsafe { impl_ACS_BSBS() }
// }
// pub fn ncurses_ACS_SBSB() -> chtype {
//     unsafe { impl_ACS_SBSB() }
// }
// pub fn ncurses_ACS_SSSS() -> chtype {
//     unsafe { impl_ACS_SSSS() }
// }
