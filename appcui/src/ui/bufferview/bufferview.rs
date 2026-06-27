use super::format::*;
use super::initialization_flags::{BufferAccess, Flags};
use super::output_buffer::OutputBuffer;
use super::{Interval, IntervalSet, Segment};
use crate::prelude::*;
use flat_string::FlatString;

const MAX_ADDRESS_WIDTH: u32 = 24;
const MAX_LABEL_WIDTH: u32 = 64;

/// Identifies one of the two resizable columns (address or label) by its vertical separator line.
#[derive(Copy, Clone, PartialEq, Eq)]
enum Separator {
    Address,
    Label,
}

#[CustomControl(overwrite = [OnPaint, OnKeyPressed, OnMouseEvent, OnResize], internal = true)]
pub struct BufferView<T>
where
    T: BufferAccess,
{
    flags: Flags,
    buffer: T,
    start_view: u64,
    pos: u64,
    repr: Representation,
    temp_buffer: Vec<u8>,
    buf_surface: Surface,
    addr_width: u32,
    label_width: u32,
    AddrName: FlatString<14>,
    LabelName: FlatString<14>,
    selected_separator: Option<Separator>,
    hovered_separator: Option<Separator>,
    mouse_capture: bool,
    intervals: IntervalSet,
    current_segment: Segment,
    current_segment_attr: CharAttribute,
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
            AddrName: FlatString::from_str("Address"),
            LabelName: FlatString::from_str("Label"),
            selected_separator: None,
            hovered_separator: None,
            mouse_capture: false,
            intervals: IntervalSet::new(),
            current_segment: Segment::default(),
            current_segment_attr: CharAttribute::default(),
        }
    }
    #[inline(always)]
    pub fn set_offset_format(&mut self, format: OffsetFormat) {
        self.repr.offset_format = format;
    }
    #[inline(always)]
    pub fn offset_format(&self) -> OffsetFormat {
        self.repr.offset_format
    }
    #[inline(always)]
    pub fn data_representation_format(&self) -> DataRepresentationFormat {
        self.repr.format
    }
    pub fn set_data_representation_format(&mut self, format: DataRepresentationFormat) {
        self.repr.format = format;
        self.recompute_sizes(self.size());
    }
    #[inline(always)]
    pub fn format(&self) -> DataRepresentationFormat {
        self.repr.format
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
    pub fn set_address_name(&mut self, name: &str) {
        self.AddrName.set(name);
    }
    pub fn set_label_name(&mut self, name: &str) {
        self.LabelName.set(name);
    }
    pub fn label(&self, pos: u64) -> Option<&str> {
        let segment = self.intervals.pos_to_segment(pos, self.buffer.len());
        if segment.exists() {
            self.intervals.get(segment.index).map(|interval| interval.name.as_str())
        } else {
            None
        }
    }
    pub fn set_intervals(&mut self, intervals: &[Interval]) {
        self.intervals.set(intervals);
        self.paint_buffer();
    }
    fn write_column_title(surface: &mut Surface, attr: CharAttribute, title: &FlatString<14>, len: u32, x: i32) {
        let chars_count = title.chars_count() as u32;
        if chars_count <= len {
            surface.write_string(x + ((len - chars_count) / 2) as i32, 0, title.as_str(), attr, false);
        } else {
            let format = TextFormatBuilder::new()
                .attribute(attr)
                .wrap_type(WrapType::SingleLineWrap(len as u16))
                .chars_count(chars_count as u16)
                .align(TextAlignment::Left)
                .position(x, 0)
                .build();
            surface.write_text(title.as_str(), &format);
        }
    }
    fn write_label(surface: &mut Surface, attr: CharAttribute, label: &str, len: u32, x: i32, y: i32) {
        if len == 0 {
            return;
        }
        let format = TextFormatBuilder::new()
            .attribute(attr)
            .wrap_type(WrapType::SingleLineWrap(len as u16))
            .position(x, y)
            .build();
        surface.write_text(label, &format);
    }
    fn write_offset(surface: &mut Surface, attr: CharAttribute, addr: u64, len: u32, y: i32, repr: OffsetFormat) {
        if len == 0 {
            return;
        }
        let mut buf: [u8; 24] = [0; 24];
        let mut pos = 23;
        let mut addr = addr;
        match repr {
            OffsetFormat::Hex => loop {
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
            },
            OffsetFormat::Dec => loop {
                let digit = (addr % 10) as u8;
                buf[pos] = digit + b'0';
                addr /= 10;
                pos -= 1;
                if addr == 0 {
                    break;
                }
            },
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
                let fill_char = match repr {
                    OffsetFormat::Hex => '0',
                    OffsetFormat::Dec => ' ',
                };
                surface.fill_horizontal_line_with_size(0, y, dif as u32, Character::with_attributes(fill_char, attr));
            }
            surface.write_ascii(dif as i32, y, &buf[pos..24], attr, false);
        }
    }
    fn write_chars(&mut self, pos: u64) {
        let mut x = 0;
        let mut y = 0;
        let w = self.repr.columns_count as i32;
        let h = self.repr.rows_count as i32;
        let mut pos = pos;
        while (pos < self.buffer.len()) && (y < h) {
            let b = self.buffer.byte(pos).unwrap_or(0);
            if !self.current_segment.contains(pos) {
                self.update_current_segment(pos);
            }
            let ch = if b < 0x20 || b >= 0x7F { '?' } else { b as char };
            self.buf_surface
                .write_char(x, y, Character::with_attributes(ch, self.current_segment_attr));
            x += 1;
            if x >= w {
                x = 0;
                y += 1;
            }
            pos += 1;
        }
    }
    fn write_line(&mut self, pos: u64, x: i32, y: i32) {
        let mut x = x + 1;
        let mut output = OutputBuffer::new();
        let mut bytes = [0; 8];
        let bytes_count = self.repr.format.bytes_count() as usize;
        let to_read = bytes_count * self.repr.columns_count as usize;
        self.temp_buffer.clear();
        self.buffer.copy(pos as u64, to_read as u64, &mut self.temp_buffer);
        let mut x_char = x + ((self.repr.format.display_chars() + 1) * self.repr.columns_count as u32 + 3) as i32;
        let mut pos = pos;
        if bytes_count == 1 {
            let min_len = (self.temp_buffer.len() as u64).min(to_read as u64) as usize;
            for i in 0..min_len {
                let val = self.temp_buffer[i];
                if !self.current_segment.contains(pos) {
                    self.update_current_segment(pos);
                }
                bytes[0] = val;
                self.repr.format.write(bytes, &mut output);
                self.buf_surface.write_ascii(x, y, output.as_slice(), self.current_segment_attr, false);
                let ch = if val < 0x20 || val >= 0x7F { '?' } else { val as char };
                self.buf_surface
                    .write_char(x_char, y, Character::with_attributes(ch, self.current_segment_attr));
                x += (output.len() as i32) + 1;
                x_char += 1;
                pos += 1;
            }
        } else {
        }
    }
    fn update_current_segment(&mut self, pos: u64) {
        self.current_segment = self.intervals.pos_to_segment(pos, self.buffer.len());
        if self.current_segment.exists() {
            self.current_segment_attr = self
                .intervals
                .get(self.current_segment.index)
                .map(|interval| interval.attr)
                .unwrap_or(self.theme().text.inactive);
        } else {
            self.current_segment_attr = self.theme().text.normal;
        }
    }
    fn paint_buffer(&mut self) {
        let attr = self.theme().text.normal;
        let mut start = self.start_view;
        let height = self.size().height as i32;
        self.buf_surface.reset(Character::with_attributes(' ', attr));
        self.update_current_segment(start);
        if self.repr.format.is_char() {
            self.write_chars(start);
        } else {
            for y in 0..height {
                self.write_line(start, 0, y as i32);
                start += self.repr.columns_count as u64 * self.repr.format.bytes_count() as u64;
            }
        }
    }
    fn paint_header(&self, surface: &mut Surface, theme: &Theme) {
        if self.flags.contains(Flags::HideHeader) {
            return;
        }
        let attr = theme.header.text.normal;
        surface.fill_horizontal_line_with_size(0, 0, self.size().width, Character::with_attributes(' ', attr));

        let mut x = 0;
        if self.addr_width > 0 {
            Self::write_column_title(surface, attr, &self.AddrName, self.addr_width as u32, 0);
            x += (1 + self.addr_width) as i32;
        }
        if self.label_width > 0 {
            Self::write_column_title(surface, attr, &self.LabelName, self.label_width as u32, x);
            x += (1 + self.label_width) as i32;
        }
        if !self.repr.format.is_char() {
            const HEX: &[u8; 16] = b"0123456789ABCDEF";
            let display_chars = self.repr.format.display_chars() as usize;
            let mut buf = [b'0'; 3];
            x += 1;
            for c in 0..self.repr.columns_count as usize {
                let output = match self.repr.offset_format {
                    OffsetFormat::Hex => {
                        buf[0] = HEX[(c >> 4) & 0x0F];
                        buf[1] = HEX[c & 0x0F];
                        &buf[..2]
                    }
                    OffsetFormat::Dec => {
                        if c < 10 {
                            buf[0] = 32;
                            buf[1] = c as u8 + b'0';
                            &buf[..2]
                        } else {
                            buf[0] = (c.min(99) as u8 / 10) + b'0';
                            buf[1] = (c as u8 % 10) + b'0';
                            &buf[..2]
                        }
                    }
                };
                surface.write_ascii(x, 0, output, attr, false);
                x += (display_chars + 1) as i32;
            }
            x += 3;
        }
        surface.write_string(x, 0, "Characters", attr, false);
    }
    fn paint_border(&self, surface: &mut Surface, theme: &Theme, top: i32) {
        if !self.flags.contains_one(Flags::ShowAddress | Flags::ShowLabels) {
            return;
        }
        let mut start = self.start_view;
        let mut y = top;
        for _ in 0..self.repr.rows_count {
            let mut x = 0;
            if self.addr_width > 0 {
                Self::write_offset(surface, theme.text.inactive, start, self.addr_width as u32, y, self.repr.offset_format);
                x += (1 + self.addr_width) as i32;
            }
            if self.label_width > 0 {
                if let Some(label) = self.label(start) {
                    Self::write_label(surface, theme.text.normal, label, self.label_width as u32, x, y);
                } else {
                    surface.fill_horizontal_line_with_size(x, y, self.label_width as u32, Character::with_attributes('-', theme.text.inactive));
                }
            }
            start += self.repr.columns_count as u64 * self.repr.format.bytes_count() as u64;
            if start >= self.buffer.len() {
                break;
            }
            y += 1;
        }
        let mut x = 0;
        let bottom = self.size().height as i32;
        if self.addr_width > 0 {
            surface.draw_vertical_line(
                self.addr_width as i32,
                0,
                bottom,
                LineType::Single,
                self.separator_attr(theme, Separator::Address),
            );
            x += (1 + self.addr_width) as i32;
        }
        if self.label_width > 0 {
            surface.draw_vertical_line(
                x + self.label_width as i32,
                0,
                bottom,
                LineType::Single,
                self.separator_attr(theme, Separator::Label),
            );
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
    fn separator_at(&self, x: i32) -> Option<Separator> {
        if self.addr_width > 0 && x == self.addr_width as i32 {
            return Some(Separator::Address);
        }
        if self.label_width > 0 {
            let base = if self.addr_width > 0 { self.addr_width as i32 + 1 } else { 0 };
            if x == base + self.label_width as i32 {
                return Some(Separator::Label);
            }
        }
        None
    }
    #[inline(always)]
    fn separator_exists(&self, separator: Separator) -> bool {
        match separator {
            Separator::Address => self.addr_width > 0,
            Separator::Label => self.label_width > 0,
        }
    }
    fn first_separator(&self) -> Option<Separator> {
        if self.addr_width > 0 {
            Some(Separator::Address)
        } else if self.label_width > 0 {
            Some(Separator::Label)
        } else {
            None
        }
    }
    fn set_separator_width(&mut self, separator: Separator, width: u32) -> bool {
        let changed = match separator {
            Separator::Address if self.addr_width > 0 => {
                let new_width = width.clamp(1, MAX_ADDRESS_WIDTH);
                if new_width != self.addr_width {
                    self.addr_width = new_width;
                    true
                } else {
                    false
                }
            }
            Separator::Label if self.label_width > 0 => {
                let new_width = width.clamp(1, MAX_LABEL_WIDTH);
                if new_width != self.label_width {
                    self.label_width = new_width;
                    true
                } else {
                    false
                }
            }
            _ => false,
        };
        if changed {
            self.recompute_sizes(self.size());
        }
        changed
    }
    fn resize_separator_to(&mut self, separator: Separator, x: i32) -> bool {
        let width = match separator {
            Separator::Address => x.max(1) as u32,
            Separator::Label => {
                let base = if self.addr_width > 0 { self.addr_width as i32 + 1 } else { 0 };
                (x - base).max(1) as u32
            }
        };
        self.set_separator_width(separator, width)
    }
    fn separator_attr(&self, theme: &Theme, separator: Separator) -> CharAttribute {
        if self.selected_separator == Some(separator) {
            theme.lines.pressed_or_selected
        } else if self.hovered_separator == Some(separator) {
            theme.lines.hovered
        } else {
            theme.lines.normal
        }
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
    fn goto_position(&mut self, new_pos: u64, select: bool) -> bool {
        let new_pos = new_pos.min(self.buffer.len().saturating_sub(1));
        let old_start_view = self.start_view;
        let cols = self.repr.columns_count as u64;
        let rows = self.repr.rows_count as u64;
        let visible_count = cols * rows;
        let column = (new_pos as isize - self.start_view as isize).rem_euclid(cols as isize) as u64;
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
    fn move_view_with(&mut self, delta: i32) {
        let unit = self.repr.format.bytes_count() as u64;
        let mut new_pos = if delta > 0 {
            self.start_view.saturating_add(delta as u64 * unit)
        } else {
            self.start_view.saturating_sub((-delta) as u64 * unit)
        };
        if new_pos >= self.buffer.len() {
            new_pos = self.buffer.len().saturating_sub(unit);
        }
        if self.start_view != new_pos {
            self.start_view = new_pos;
            self.paint_buffer();
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
            let column = dif % self.repr.columns_count as u64;
            let row = (dif / self.repr.columns_count as u64) as i32 + top;
            let ch = Character::with_attributes(0, theme.list_current_item.focus);
            if self.repr.format.is_char() {
                let x = (border_width + column as u32) as i32;
                surface.write_char(x, row, ch);
            } else {
                let len = self.repr.format.display_chars() as u32;
                let x = (border_width + column as u32 * (len + 1)) as i32;
                surface.fill_horizontal_line_with_size(x, row, len + 2, ch);
            }
        }
    }
}

impl<T: BufferAccess> OnKeyPressed for BufferView<T> {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        // column resize mode (address / label columns)
        if let Some(separator) = self.selected_separator {
            match key.value() {
                key!("Left") => {
                    let width = match separator {
                        Separator::Address => self.addr_width,
                        Separator::Label => self.label_width,
                    };
                    self.set_separator_width(separator, width.saturating_sub(1));
                    return EventProcessStatus::Processed;
                }
                key!("Right") => {
                    let width = match separator {
                        Separator::Address => self.addr_width,
                        Separator::Label => self.label_width,
                    };
                    self.set_separator_width(separator, width.saturating_add(1));
                    return EventProcessStatus::Processed;
                }
                key!("Tab") | key!("Ctrl+Left") | key!("Ctrl+Right") | key!("Ctrl+Alt+Left") | key!("Ctrl+Alt+Right") => {
                    let other = match separator {
                        Separator::Address => Separator::Label,
                        Separator::Label => Separator::Address,
                    };
                    if self.separator_exists(other) {
                        self.selected_separator = Some(other);
                    }
                    return EventProcessStatus::Processed;
                }
                key!("Escape") | key!("Enter") => {
                    self.selected_separator = None;
                    return EventProcessStatus::Processed;
                }
                _ => {
                    self.selected_separator = None;
                    return EventProcessStatus::Processed;
                }
            }
        }
        let select = key.modifier.contains(KeyModifier::Shift);
        match key.value() {
            key!("Ctrl+Alt+Left") | key!("Ctrl+Alt+Right") => {
                if let Some(separator) = self.first_separator() {
                    self.selected_separator = Some(separator);
                    return EventProcessStatus::Processed;
                }
                return EventProcessStatus::Ignored;
            }
            key!("Left") | key!("Shift+Left") => {
                self.goto_position(self.pos.saturating_sub(1), select);
                return EventProcessStatus::Processed;
            }
            key!("Right") | key!("Shift+Right") => {
                self.goto_position(self.pos.saturating_add(1), select);
                return EventProcessStatus::Processed;
            }
            key!("Up") | key!("Shift+Up") => {
                self.goto_position(self.pos.saturating_sub(self.repr.columns_count as u64), select);
                return EventProcessStatus::Processed;
            }
            key!("Down") | key!("Shift+Down") => {
                self.goto_position(self.pos.saturating_add(self.repr.columns_count as u64), select);
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
                    self.pos.saturating_sub(self.repr.columns_count as u64 * (self.repr.rows_count as u64)),
                    select,
                );
                return EventProcessStatus::Processed;
            }
            key!("PageDown") | key!("Shift+PageDown") => {
                self.goto_position(
                    self.pos.saturating_add(self.repr.columns_count as u64 * (self.repr.rows_count as u64)),
                    select,
                );
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Left") => {
                self.move_view_with(-1);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Right") => {
                self.move_view_with(1);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Up") => {
                self.move_view_with(-(self.repr.columns_count as i32));
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Down") => {
                self.move_view_with(self.repr.columns_count as i32);
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        return EventProcessStatus::Ignored;
    }
}

impl<T: BufferAccess> OnMouseEvent for BufferView<T> {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Leave => {
                if self.hovered_separator.is_some() {
                    self.hovered_separator = None;
                    return EventProcessStatus::Processed;
                }
                EventProcessStatus::Ignored
            }
            MouseEvent::Over(p) => {
                let hovered = self.separator_at(p.x);
                if hovered != self.hovered_separator {
                    self.hovered_separator = hovered;
                    return EventProcessStatus::Processed;
                }
                EventProcessStatus::Ignored
            }
            MouseEvent::Pressed(ev) => {
                if let Some(separator) = self.separator_at(ev.x) {
                    self.selected_separator = Some(separator);
                    self.mouse_capture = true;
                    return EventProcessStatus::Processed;
                }
                EventProcessStatus::Ignored
            }
            MouseEvent::Drag(ev) => {
                if let (true, Some(separator)) = (self.mouse_capture, self.selected_separator) {
                    self.resize_separator_to(separator, ev.x);
                    return EventProcessStatus::Processed;
                }
                EventProcessStatus::Ignored
            }
            MouseEvent::Released(ev) => {
                if let (true, Some(separator)) = (self.mouse_capture, self.selected_separator) {
                    self.resize_separator_to(separator, ev.x);
                    self.selected_separator = None;
                    self.mouse_capture = false;
                    return EventProcessStatus::Processed;
                }
                EventProcessStatus::Ignored
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl<T: BufferAccess> OnResize for BufferView<T> {
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.recompute_sizes(new_size);
    }
}
