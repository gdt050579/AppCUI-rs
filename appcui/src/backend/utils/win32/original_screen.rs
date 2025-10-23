use super::api;
use super::constants;
use super::structs::*;
use crate::graphics::{Point, Size};

#[derive(Clone)]
pub(super) struct OriginalScreen {
    stdout: HANDLE,
    size: Size,
    pos: Point,
    data: Vec<CHAR_INFO>,
}

impl OriginalScreen {
    pub(super) fn new(stdout: HANDLE, size: Size, x: i32, y: i32) -> Option<Self> {
        let sz = size.width as usize * size.height as usize;
        let mut v: Vec<CHAR_INFO> = Vec::with_capacity(sz);
        v.fill(CHAR_INFO { code: 0, attr: 0 });
        let mut sr = SMALL_RECT {
            left: x as i16,
            top: y as i16,
            right: ((x + size.width as i32) - 1) as i16,
            bottom: ((y + size.height as i32) - 1) as i16,
        };
        unsafe {
            if api::ReadConsoleOutputW(
                stdout,
                v.as_mut_ptr(),
                COORD {
                    x: size.width as i16,
                    y: size.height as i16,
                },
                COORD { x: 0, y: 0 },
                &mut sr,
            ) == constants::FALSE
            {
                return None;
            }
        }
        Some(Self {
            stdout,
            size,
            pos: Point::new(x, y),
            data: v,
        })
    }
    pub(super) fn restore(self) {
        let sr = SMALL_RECT {
            left: self.pos.x as i16,
            top: self.pos.y as i16,
            right: ((self.pos.x + self.size.width as i32) - 1) as i16,
            bottom: ((self.pos.y + self.size.height as i32) - 1) as i16,
        };
        unsafe {
            api::WriteConsoleOutputW(
                self.stdout,
                self.data.as_ptr(),
                COORD {
                    x: self.size.width as i16,
                    y: self.size.height as i16,
                },
                COORD { x: 0, y: 0 },
                &sr,
            );
        }
    }
}
