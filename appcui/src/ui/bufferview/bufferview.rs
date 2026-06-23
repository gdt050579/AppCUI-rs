use super::format::*;
use super::initialization_flags::{BufferAccess, Flags};
use super::output_buffer::OutputBuffer;
use crate::prelude::*;

#[CustomControl(overwrite = [OnPaint, OnKeyPressed, OnMouseEvent, OnResize], internal = true)]
pub struct BufferView<T>
where
    T: BufferAccess,
{
    flags: Flags,
    buffer: T,
    pos: usize,
    repr: Representation,
    temp_buffer: Vec<u8>,
    buf_surface: Surface,
}

impl<T: BufferAccess> BufferView<T> {
    pub fn new(buffer: T, layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            buffer,
            pos: 0,
            repr: Representation::new(),
            temp_buffer: Vec::new(),
            buf_surface: Surface::new(1, 1),
        }
    }
    fn write_offset(surface: &mut Surface, attr: CharAttribute, addr: usize, len: u32, y: i32, hex: bool) {
        if len == 0 {
            return;
        }
        let mut buf: [u8; 24] = [0; 24];
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
                1 => pos = 23,
                2 => {
                    buf[22] = b'.';
                    pos = 22;
                }
                3 => {
                    buf[21] = buf[pos];
                    buf[22] = b'.';
                    pos = 21;
                }
                4 => {
                    buf[20] = buf[pos];
                    buf[21] = b'.';
                    pos = 21;
                }
                5..24 => {
                    // 4 and more
                    buf[24 - len as usize] = buf[pos];
                    buf[25 - len as usize] = b'.';
                    buf[26 - len as usize] = b'.';
                    pos = 24 - len as usize;
                }
                _ => return,
            }
        }
        surface.write_ascii(0, y, &buf[pos..25], attr, false);
    }
    fn write_line(&mut self, attr: CharAttribute, pos: usize, x: i32, y: i32) {
        let mut x = x + 1;
        let mut output = OutputBuffer::new();
        let mut bytes = [0; 8];
        let bytes_count = self.repr.format.bytes_count() as usize;
        let to_read = bytes_count * self.repr.columns_count as usize;
        self.temp_buffer.clear();
        self.buffer.copy(pos, to_read, &mut self.temp_buffer);
        let mut x_char = x + ((self.repr.format.display_chars() + 1) * self.repr.columns_count as u32 + 3) as i32;

        if bytes_count == 1 {
            let min_len = self.temp_buffer.len().min(to_read);
            let slice = &self.temp_buffer[..min_len];
            for val in slice {
                bytes[0] = *val;
                self.repr.format.write(bytes, &mut output);
                self.buf_surface.write_ascii(x, y, output.as_slice(), attr, false);
                let ch = if *val < 0x20 || *val >= 0x7F { '?' } else { (*val) as char };
                self.buf_surface.write_char(x_char, y, Character::with_attributes(ch, attr));
                x += (output.len() as i32) + 1;
                x_char += 1;
            }
        } else {
        }
    }
    fn paint_buffer(&mut self) {
        let col = self.theme().editor.normal;
        let mut start = self.pos;
        let height = self.size().height as i32;
        for y in 0..height {
            self.write_line(col, start, 0, y as i32);
            start += self.repr.columns_count as usize * self.repr.format.bytes_count() as usize;
        }
    }
}

impl<T: BufferAccess> OnPaint for BufferView<T> {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.draw_surface(0, 0, &self.buf_surface);
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

impl<T: BufferAccess> OnResize for BufferView<T> {
    fn on_resize(&mut self, _old_size: Size, new_size: Size) {
        self.buf_surface.resize(new_size);
        self.paint_buffer();
    }
}
