#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use libc::{c_short, c_int};
use super::constants::*;

#[derive(Debug, Clone, Copy)]
pub enum CURSOR_VISIBILITY
{
  CURSOR_INVISIBLE = 0,
  CURSOR_VISIBLE,
  CURSOR_VERY_VISIBLE
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MEVENT { pub id: c_short, pub x: c_int, pub y: c_int, pub z: c_int, pub bstate: mmask_t}

#[derive(Debug)]
pub enum WchResult {
    KeyCode(i32),
    Char(winttype),
}

#[derive(Debug)]
#[repr(i32)]
pub enum LcCategory {
    all = LC_ALL,
    collate = LC_COLLATE,
    ctype = LC_CTYPE,
    monetary = LC_MONETARY,
    numeric = LC_NUMERIC,
    time = LC_TIME,
    messages = LC_MESSAGES,
}