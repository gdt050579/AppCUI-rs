use super::format::*;
use super::initialization_flags::{BufferAccess, Flags};
use super::output_buffer::OutputBuffer;
use crate::prelude::*;

const MAX_ADDRESS_WIDTH: u32 = 24;
const MAX_LABEL_WIDTH: u32 = 128;

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
            addr_width: if flags.contains(Flags::ShowAddress) { 6 } else { 0 },
            label_width: if flags.contains(Flags::ShowLabels) { 6 } else { 0 },
        }
    }
    pub fn set_columns_count(&mut self, count: ColumnsCount) {
        self.repr.columns = count;
        self.recompute_sizes(self.size());
    }
    pub fn set_address_width(&mut self, width: u32) {
        if self.flags.contains(Flags::ShowAddress) {
            self.addr_width = width.clamp(1, MAX_ADDRESS_WIDTH);
        } else {
            self.addr_width = 0;
        }
        self.recompute_sizes(self.size());
    }
    pub fn set_label_width(&mut self, width: u32) {
        if self.flags.contains(Flags::ShowLabels) {
            self.label_width = width.clamp(1, MAX_LABEL_WIDTH);
        } else {
            self.label_width = 0;
        }
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
        let addr_len = (24 - pos) as u32;
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
            surface.write_ascii(0, y, &buf[pos..24], attr, false);
        } else {
            let dif = len - addr_len;
            if dif > 0 {
                surface.fill_horizontal_line_with_size(0, y, dif as u32, Character::with_attributes(if hex { '0' } else { ' ' }, attr));
            }
            surface.write_ascii(dif as i32, y, &buf[pos..24], attr, false);
        }
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
    fn paint_header(&self, surface: &mut Surface, theme: &Theme) {
        if self.flags.contains(Flags::HideHeader) {
            return;
        }
        const HEX: &[u8; 16] = b"0123456789ABCDEF";
        let attr = theme.header.text.normal;
        let width = self.size().width;
        // fill the whole header row so it reads as a single band
        surface.fill_horizontal_line_with_size(0, 0, width, Character::with_attributes(' ', attr));

        let display_chars = self.repr.format.display_chars() as usize;
        let mut digits = [b'0'; 16];
        let mut x = self.border_width() as i32 + 1;
        for c in 0..self.repr.columns_count as usize {
            // column index, in hex, zero-padded to the width of a value cell
            let mut v = c;
            for i in (0..display_chars).rev() {
                digits[i] = HEX[v & 0x0F];
                v >>= 4;
            }
            // same horizontal layout as `write_line`: a leading space, then `display_chars + 1` per cell
            surface.write_ascii(x, 0, &digits[..display_chars], attr, false);
            x += (display_chars + 1) as i32;
        }
    }
    fn paint_border(&self, surface: &mut Surface, theme: &Theme, top: i32) {
        if !self.flags.contains_one(Flags::ShowAddress | Flags::ShowLabels) {
            return;
        }
        let attr = theme.border.normal;
        let mut start = self.start_view;
        let mut y = top;
        for _ in 0..self.repr.rows_count {
            let mut x = 0;
            if self.addr_width > 0 {
                Self::write_offset(surface, attr, start, self.addr_width as u32, y, true);
                x += (1 + self.addr_width) as i32;
            }
            if self.label_width > 0 {
                //self.write_offset(surface, theme.header.text.normal, start, self.label_width, top, false);
            }
            start += self.repr.columns_count as usize * self.repr.format.bytes_count() as usize;
            if start >= self.buffer.len() {
                break;
            }
            y += 1;
        }
        let mut x = 0;
        let bottom = self.size().height as i32;
        if self.addr_width > 0 {
            surface.draw_vertical_line(self.addr_width as i32, 0, bottom, LineType::Single, theme.lines.normal);
            x += (1 + self.addr_width) as i32;
        }
        if self.label_width > 0 {
            surface.draw_vertical_line(x + self.label_width as i32, 0, bottom, LineType::Single, theme.lines.normal);
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
        let h = if self.flags.contains(Flags::HideHeader) {
            screen_size.height.max(1)
        } else {
            screen_size.height.saturating_sub(1).max(1)
        };
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
        let cols = self.repr.columns_count as usize;
        let rows = self.repr.rows_count as usize;
        let visible_count = cols * rows;
        let column = (new_pos as isize - self.start_view as isize).rem_euclid(cols as isize) as usize;
        if new_pos < self.start_view {
            self.start_view = new_pos.saturating_sub(column);
        } else if new_pos >= self.start_view + visible_count {
            let row_start = new_pos - column;
            self.start_view = row_start.saturating_sub(rows.saturating_sub(1) * cols);
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
        let top = if self.flags.contains(Flags::HideHeader) { 0 } else { 1 };
        self.paint_header(surface, theme);
        self.paint_border(surface, theme, top);
        let border_width = self.border_width();
        surface.draw_surface(border_width as i32, top, &self.buf_surface);
        // convert self.pos to column and row, knowing that the view starts form self.start_view
        if self.pos >= self.start_view {
            let dif = self.pos - self.start_view;
            let column = dif % self.repr.columns_count as usize;
            let row = (dif / self.repr.columns_count as usize) as i32 + top;
            let len = self.repr.format.display_chars() as u32;
            let x = (border_width + column as u32 * (len + 1)) as i32;
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
