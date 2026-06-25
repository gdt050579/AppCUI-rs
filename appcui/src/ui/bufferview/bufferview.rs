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
    start_view: usize,
    pos: usize,
    repr: Representation,
    temp_buffer: Vec<u8>,
    buf_surface: Surface,
    addr_width: u32,
    label_width: u32,
}

impl<T: BufferAccess> BufferView<T> {
    pub fn new(buffer: T, layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            flags,
            buffer,
            start_view: 0,
            pos: 0,
            repr: Representation::new(),
            temp_buffer: Vec::new(),
            buf_surface: Surface::new(1, 1),
            addr_width: 0,
            label_width: 0,
        }
    }
    pub fn set_columns_count(&mut self, count: ColumnsCount) {
        self.repr.columns = count;
        self.recompute_sizes(self.size());
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
        let mut start = self.start_view;
        let height = self.size().height as i32;
        for y in 0..height {
            self.write_line(col, start, 0, y as i32);
            start += self.repr.columns_count as usize * self.repr.format.bytes_count() as usize;
        }
    }
    fn border_width(&self) -> u32 {
        let mut border_width = 0;
        if self.addr_width > 0 {
            border_width += (1 + self.addr_width) as u32;
        }
        if self.label_width > 0 {
            border_width += (1 + self.label_width) as u32;
        }
        border_width
    }
    fn recompute_sizes(&mut self, screen_size: Size) {
        let h = screen_size.height.saturating_sub(1).max(1);
        let nr_columns = match self.repr.columns {
            ColumnsCount::Fixed(count) => count as u32,
            ColumnsCount::Auto => {
                let space_left = screen_size.width.saturating_sub(self.border_width());
                let columns = if self.repr.format.is_char() {
                    space_left
                } else {
                    (space_left.saturating_sub(4)) / (self.repr.format.display_chars() + 2)
                };
                columns.max(1) as u32
            }
        };
        self.repr.columns_count = nr_columns.clamp(1, 255);
        self.repr.rows_count = h;
        let w = if self.repr.format.is_char() {
            self.repr.columns_count as u32
        } else {
            (self.repr.format.display_chars() + 2) * (self.repr.columns_count as u32) + 4
        };
        self.buf_surface.resize(Size::new(w, h));
        if !self.goto_position(self.pos, false) {
            self.paint_buffer();
        }
    }
    fn goto_position(&mut self, new_pos: usize, select: bool) -> bool {
        let new_pos = new_pos.min(self.buffer.len().saturating_sub(1));
        let old_start_view = self.start_view;
        if new_pos < self.start_view {
            self.start_view = new_pos;
        }
        let visible_count = self.repr.columns_count as usize * self.repr.rows_count as usize;
        if new_pos >= self.start_view + visible_count {
            self.start_view = new_pos + 1 - visible_count;
        }
        self.pos = new_pos;
        if old_start_view != self.start_view {
            self.paint_buffer();
            true
        } else {
            false
        }
    }
}

impl<T: BufferAccess> OnPaint for BufferView<T> {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.draw_surface(0, 0, &self.buf_surface);
        // convert self.pos to column and row, knowing that the view starts form self.start_view
        if self.pos >= self.start_view {
            let dif = self.pos - self.start_view;
            let column = dif % self.repr.columns_count as usize;
            let row = (dif / self.repr.columns_count as usize) as i32;
            let len = self.repr.format.display_chars() as u32;
            let x = (self.border_width() + column as u32 * (len + 1)) as i32;
            surface.fill_horizontal_line_with_size(x, row, len + 2, Character::with_attributes(0, theme.editor.pressed_or_selected));
        }
    }
}

impl<T: BufferAccess> OnKeyPressed for BufferView<T> {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        let select = key.modifier.contains(KeyModifier::Shift);
        match key.value() {
            key!("Left") | key!("Shift+Left") => {
                self.goto_position(self.pos.saturating_sub(1), select);
                return EventProcessStatus::Processed;
            }
            key!("Right") | key!("Shift+Right") => {
                self.goto_position(self.pos.saturating_add(1), select);
                return EventProcessStatus::Processed;
            }
            key!("Up") | key!("Shift+Up") => {
                self.goto_position(self.pos.saturating_sub(self.repr.columns_count as usize), select);
                return EventProcessStatus::Processed;
            }
            key!("Down") | key!("Shift+Down") => {
                self.goto_position(self.pos.saturating_add(self.repr.columns_count as usize), select);
                return EventProcessStatus::Processed;
            }
            key!("Home") | key!("Shift+Home") => {
                self.goto_position(0, select);
                return EventProcessStatus::Processed;
            }
            key!("End") | key!("Shift+End") => {
                self.goto_position(self.buffer.len(), select);
                return EventProcessStatus::Processed;
            }
            key!("PageUp") | key!("Shift+PageUp") => {
                self.goto_position(
                    self.pos
                        .saturating_sub(self.repr.columns_count as usize * (self.repr.rows_count as usize)),
                    select,
                );
                return EventProcessStatus::Processed;
            }
            key!("PageDown") | key!("Shift+PageDown") => {
                self.goto_position(
                    self.pos
                        .saturating_add(self.repr.columns_count as usize * (self.repr.rows_count as usize)),
                    select,
                );
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        return EventProcessStatus::Ignored;
    }
}

impl<T: BufferAccess> OnMouseEvent for BufferView<T> {
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        // TODO: implement mouse handling
        EventProcessStatus::Ignored
    }
}

impl<T: BufferAccess> OnResize for BufferView<T> {
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.recompute_sizes(new_size);
    }
}
