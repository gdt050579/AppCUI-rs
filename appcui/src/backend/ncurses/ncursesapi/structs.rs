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

use super::constants::*;
use libc::{c_int, c_short};

#[derive(Debug, Clone, Copy)]
pub(crate) enum CURSOR_VISIBILITY {
    CURSOR_INVISIBLE = 0,
    CURSOR_VISIBLE,
    CURSOR_VERY_VISIBLE,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) struct MEVENT {
    pub(crate) id: c_short,
    pub(crate) x: c_int,
    pub(crate) y: c_int,
    pub(crate) z: c_int,
    pub(crate) bstate: mmask_t,
}

#[derive(Debug)]
pub(crate) enum WchResult {
    KeyCode(i32),
    Char(winttype),
}

#[derive(Debug)]
#[repr(i32)]
pub(crate) enum LcCategory {
    all = LC_ALL,
    collate = LC_COLLATE,
    ctype = LC_CTYPE,
    monetary = LC_MONETARY,
    numeric = LC_NUMERIC,
    time = LC_TIME,
    messages = LC_MESSAGES,
}
