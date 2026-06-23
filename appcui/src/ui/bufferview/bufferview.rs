use super::initialization_flags::{BufferAccess, Flags};
use crate::prelude::*;

#[CustomControl(overwrite = [OnPaint, OnKeyPressed, OnMouseEvent], internal = true)]
pub struct BufferView<T> where T: BufferAccess {
    flags: Flags,
    buffer: T,
    pos: usize,
}

impl<T: BufferAccess> BufferView<T> {
    pub fn new(buffer: T, layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            buffer,
            pos: 0,
        }
    }
    fn write_offset(surface: &mut Surface, attr: CharAttribute, addr: usize, len: u32, y: i32, hex: bool) {
        if len == 0 {
            return;
        }
        let mut buf: [u8;24] = [0;24];
        let mut pos = 23;
        let mut addr = addr;
        if hex {
            // hex
            loop {
                let digit = (addr % 16) as u8;
                if digit < 10 {
                    buf[pos] = digit + b'0';
                } else {
                    buf[pos] = digit - 10 + b'A';
                }
                addr /= 16;
                pos -= 1;
                if addr == 0 {
                    break;
                }
            }
        } else {
            // decimal
            loop {
                let digit = (addr % 10) as u8;
                buf[pos] = digit + b'0';
                addr /= 10;
                pos -= 1;
                if addr == 0 {
                    break;
                }
            }
        }
        pos += 1;
        let addr_len = (25 - pos) as u32;
        if addr_len > len {
            match len {
                1 =>  pos = 23,
                2 => { buf[22] = b'.'; pos = 22; }
                3 => { buf[21] = buf[pos]; buf[22] = b'.'; pos = 21; }
                4..24 => {
                    // 4 and more
                    buf[24-len as usize] = buf[pos];
                    buf[25-len as usize] = b'.';
                    pos = 24-len as usize;
                }
                _ => return,
            }
        }
        surface.write_ascii(0, y, &buf[pos..25], attr, false);
    }
}

impl<T: BufferAccess> OnPaint for BufferView<T> {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {
        // TODO: implement painting of the buffer view
    }
}

impl<T: BufferAccess> OnKeyPressed for BufferView<T> {
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        // TODO: implement keyboard handling
        EventProcessStatus::Ignored
    }
}

impl<T: BufferAccess> OnMouseEvent for BufferView<T> {
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        // TODO: implement mouse handling
        EventProcessStatus::Ignored
    }
}
