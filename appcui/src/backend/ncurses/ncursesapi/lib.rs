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

use std::ffi::{CStr, CString};
use std::{mem, ptr};
use super::constants::*;
use super::externs::*;
use super::structs::{LcCategory, WchResult, CURSOR_VISIBILITY, MEVENT};

#[allow(clippy::upper_case_acronyms)]
pub(crate) type WINDOW = *mut i8;

pub(crate) fn ncurses_initscr() -> WINDOW {
    unsafe { initscr() }
}

pub(crate) fn ncurses_endwin() -> i32 {
    unsafe { endwin() }
}

pub(crate) fn ncurses_refresh() -> i32 {
    unsafe { refresh() }
}

pub(crate) fn ncurses_wrefresh(w: WINDOW) -> i32{ 
    unsafe { wrefresh(w) } 
}

pub(crate) fn ncurses_wresize(w: WINDOW, height: i32, width: i32) -> i32 {
    unsafe { wresize(w, height, width) }
}

pub(crate) fn ncurses_getmouse(event: *mut MEVENT) -> i32
{ unsafe { getmouse(event) } }

pub(crate) fn ncurses_getch() -> i32 {
    unsafe { getch() }
}

pub(crate) fn ncurses_nodelay(win: WINDOW, bf: bool) -> i32 {
    unsafe { nodelay(win, bf as c_bool) }
}

pub(crate) fn ncurses_halfdelay(tens: i32) -> i32 {
    unsafe { halfdelay(tens) }
}

pub(crate) fn ncurses_keypad(win: WINDOW, bf: bool) -> i32 {
    unsafe { keypad(win, bf as c_bool) }
}

pub(crate) fn ncurses_cbreak() -> i32 {
    unsafe { cbreak() }
}

pub(crate) fn ncurses_noecho() -> i32 {
    unsafe { noecho() }
}

pub(crate) fn ncurses_nonl() -> i32 {
    unsafe { nonl() }
}

pub(crate) fn ncurses_raw() -> i32 {
    unsafe { raw() }
}

pub(crate) fn ncurses_meta(win: WINDOW, bf: bool) -> i32 {
    unsafe { meta(win, bf as c_bool) }
}

pub(crate) fn ncurses_mousemask(newmask: mmask_t, oldmask: Option<&mut mmask_t>) -> mmask_t
{
    match oldmask {
	None => { unsafe { mousemask(newmask, ptr::null_mut()) } },
	Some(old) => { unsafe { mousemask(newmask, old) } },
    }
}

pub(crate) fn ncurses_mouseinterval(interval: i32) -> i32 {
    unsafe { mouseinterval(interval) }
}

pub(crate) fn ncurses_wmove(w: WINDOW, y: i32, x: i32) -> i32
{ unsafe { wmove(w, y, x) } }

pub(crate) fn ncurses_set_escdelay(ms: i32) -> i32 {
    unsafe { set_escdelay(ms) }
}
pub(crate) fn ncurses_wclear(w: WINDOW) -> i32 {
    unsafe { wclear(w) }
}

pub(crate) fn ncurses_mvaddch(y: i32, x: i32, ch: chtype) -> i32 {
    unsafe { mvaddch(y, x, ch) }
}

pub(crate) fn ncurses_wcwidth(c: char) -> i32 {
    unsafe { wcwidth((c as u32) as i32) }
}

pub(crate) fn ncurses_getmaxyx(w: WINDOW,  y: &mut i32, x: &mut i32) {
    unsafe {
        *y = getmaxy(w);
        *x = getmaxx(w);
    }
}

pub(crate) fn ncurses_wget_wch(w:WINDOW) -> Option<WchResult> {
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

pub(crate) fn ncurses_get_wch() -> Option<WchResult> {
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

impl ToCStr for &str {
    fn to_c_str(&self) -> Result<CString, std::ffi::NulError> {
        CString::new(*self)
    }
}

pub(crate) fn setlocale(lc: LcCategory, locale: &str) -> Result<String, std::ffi::NulError>
{
  unsafe {
    let c_str = locale.to_c_str()?;
    let buf = c_str.as_ptr();
    let ret = libc::setlocale(lc as libc::c_int, buf);
    if ret.is_null() {
        Ok(String::new())
    } else {
        // The clone is necessary, as the returned pointer
        // can change at any time
        Ok(CStr::from_ptr(ret).to_string_lossy().into_owned())
    }
  }
}

pub(crate) fn ncurses_start_color() -> i32
{ unsafe { start_color() } }

pub(crate) fn ncurses_use_default_colors() -> i32
{ unsafe { use_default_colors() } }

pub(crate) fn ncurses_init_pair(pair: i16, f: i16, b: i16) -> i32
{ unsafe { init_pair(pair, f, b) } }

pub(crate) fn ncurses_wattron(w: WINDOW, attr: NCURSES_ATTR_T) -> i32
{ unsafe { wattron(w, attr) } }


pub(crate) fn ncurses_wattroff(w: WINDOW, attr: NCURSES_ATTR_T) -> i32
{ unsafe { wattroff(w, attr) } }

pub(crate) fn ncurses_COLOR_PAIR(n: i16) -> attr_t {
    unsafe {
        COLOR_PAIR(n as i32) as attr_t
    }
}

pub(crate) fn ncurses_stdscr() -> WINDOW {        
    stdscr()

}

pub(crate) fn ncurses_addstr(s: &str) -> Result<i32, std::ffi::NulError>
{ unsafe { Ok(addstr(s.to_c_str()?.as_ptr())) } }

pub(crate) fn ncurses_mvaddstr(y: i32, x: i32, s: &str) -> Result<i32, std::ffi::NulError>
{
  if ncurses_wmove(stdscr(),y, x) == ERR
  { return Ok(ERR); }
  ncurses_addstr(s)
}

pub(crate) fn ncurses_curs_set(visibility: CURSOR_VISIBILITY) -> Option<CURSOR_VISIBILITY>
{
  unsafe
  {
    match curs_set(visibility as i32)
    {
      ERR => None,
      ret => Some(mem::transmute::<i8, CURSOR_VISIBILITY>(ret as i8)),
    }
  }
}
