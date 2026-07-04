use super::events::EventData;
use super::format::*;
use super::output_buffer::OutputBuffer;
use super::*;
use super::{Codepage, Interval, IntervalSet, Segment, Selection};
use crate::prelude::*;
use flat_string::FlatString;

const MAX_ADDRESS_WIDTH: u32 = 24;
const MAX_INTERVAL_NAME_WIDTH: u32 = 64;
#[cfg(target_endian = "little")]
const NATIVE_ENDIAN: Endian = Endian::Little;
#[cfg(target_endian = "big")]
const NATIVE_ENDIAN: Endian = Endian::Big;

/// Identifies one of the two resizable columns (address or interval name) by its vertical separator line.
#[derive(Copy, Clone, PartialEq, Eq)]
enum Separator {
    Address,
    IntervalName,
}

/// Identifies which panel the cursor is currently active on (the data/hex panel or the characters panel).
#[derive(Copy, Clone, PartialEq, Eq)]
enum ActivePanel {
    DataRepresentation,
    Char,
}

struct CharWriterPos {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    pos: u64,
    len: u64,
}

impl CharWriterPos {
    fn new(w: i32, h: i32, pos: u64, len: u64) -> Self {
        Self { x: 0, y: 0, w, h, pos, len }
    }
    #[inline(always)]
    fn move_cursor(&mut self, delta_pos: u64) -> bool {
        self.pos += delta_pos;
        if self.pos >= self.len {
            return false;
        }
        self.x += 1;
        if self.x >= self.w {
            self.x = 0;
            self.y += 1;
            if self.y >= self.h {
                return false;
            }
        }
        true
    }
    #[inline(always)]
    fn is_valid(&self) -> bool {
        self.pos < self.len && self.y < self.h
    }
}

#[CustomControl(overwrite = [OnPaint, OnKeyPressed, OnMouseEvent, OnResize], internal = true)]
pub struct BufferView<T>
where
    T: BufferAccess + 'static,
{
    flags: Flags,
    buffer: Buffer<T>,
    start_view: u64,
    pos: u64,
    repr: Representation,
    buf_surface: Surface,
    addr_width: u32,
    interval_name_width: u32,
    AddrName: FlatString<14>,
    interval_name_title: FlatString<14>,
    selected_separator: Option<Separator>,
    hovered_separator: Option<Separator>,
    hovered_panel: Option<ActivePanel>,
    mouse_capture: bool,
    selecting: bool,
    intervals: IntervalSet,
    selection: Selection,
    current_segment: Segment,
    current_segment_attr: CharAttribute,
    cp: Codepage,
    comp: ListScrollBars,
    h_offset: u32,
    active_panel: ActivePanel,
    search_bytes: Vec<u8>,
    edit_text: FlatString<32>,
}

impl<T: BufferAccess + 'static> BufferView<T> {
    pub fn new(buffer: T, layout: Layout, flags: Flags) -> Self {
        let mut status_flags = StatusFlags::Enabled | StatusFlags::Visible | StatusFlags::AcceptInput;
        if flags.contains(Flags::ScrollBars) {
            status_flags |= StatusFlags::IncreaseBottomMarginOnFocus;
            status_flags |= StatusFlags::IncreaseRightMarginOnFocus;
        }
        if flags.contains(Flags::SearchBar) {
            status_flags |= StatusFlags::IncreaseBottomMarginOnFocus;
        }
        Self {
            base: ControlBase::with_status_flags(layout, status_flags),
            flags,
            buffer: Buffer::new(buffer),
            start_view: 0,
            pos: 0,
            repr: Representation::new(),
            buf_surface: Surface::new(1, 1),
            addr_width: 6,
            interval_name_width: 6,
            AddrName: FlatString::from_str("Address"),
            interval_name_title: FlatString::from_str("Name"),
            selected_separator: None,
            hovered_separator: None,
            hovered_panel: None,
            mouse_capture: false,
            selecting: false,
            intervals: IntervalSet::new(),
            selection: Selection::NONE,
            current_segment: Segment::default(),
            current_segment_attr: CharAttribute::default(),
            cp: Codepage::CP437,
            comp: ListScrollBars::new(flags.contains(Flags::ScrollBars), flags.contains(Flags::SearchBar)),
            h_offset: 0,
            active_panel: ActivePanel::DataRepresentation,
            search_bytes: Vec::new(),
            edit_text: FlatString::new(),
        }
    }
    pub fn set_codepage(&mut self, cp: Codepage) {
        self.cp = cp;
        self.paint_buffer();
    }
    #[inline(always)]
    pub fn codepage(&self) -> &Codepage {
        &self.cp
    }
    #[inline(always)]
    pub fn set_offset_format(&mut self, format: OffsetFormat) {
        self.repr.offset_format = format;
        self.paint_buffer();
    }
    #[inline(always)]
    pub fn offset_format(&self) -> OffsetFormat {
        self.repr.offset_format
    }
    #[inline(always)]
    pub fn set_endian(&mut self, endian: Endian) {
        self.repr.endian = endian;
        self.paint_buffer();
    }
    #[inline(always)]
    pub fn endian(&self) -> Endian {
        self.repr.endian
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
        self.addr_width = width.clamp(1, MAX_ADDRESS_WIDTH);
        if self.flags.contains(Flags::ShowAddress) {
            self.recompute_sizes(self.size());
        }
    }
    pub fn set_address_visible(&mut self, visible: bool) {
        if visible == self.flags.contains(Flags::ShowAddress) {
            return;
        }
        if visible {
            self.flags.set(Flags::ShowAddress);
        } else {
            self.flags.remove(Flags::ShowAddress);
        }
        self.recompute_sizes(self.size());
    }
    pub fn set_address_name(&mut self, name: &str) {
        self.AddrName.set(name);
    }
    #[inline(always)]
    pub fn is_address_visible(&self) -> bool {
        self.flags.contains(Flags::ShowAddress)
    }
    pub fn set_interval_name_width(&mut self, width: u32) {
        self.interval_name_width = width.clamp(1, MAX_INTERVAL_NAME_WIDTH);
        if self.flags.contains(Flags::ShowIntervalNames) {
            self.recompute_sizes(self.size());
        }
    }
    pub fn set_interval_name_title(&mut self, title: &str) {
        self.interval_name_title.set(title);
    }
    pub fn set_interval_names_visible(&mut self, visible: bool) {
        if visible == self.flags.contains(Flags::ShowIntervalNames) {
            return;
        }
        if visible {
            self.flags.set(Flags::ShowIntervalNames);
        } else {
            self.flags.remove(Flags::ShowIntervalNames);
        }
        self.recompute_sizes(self.size());
    }
    #[inline(always)]
    pub fn is_interval_names_visible(&self) -> bool {
        self.flags.contains(Flags::ShowIntervalNames)
    }
    pub fn interval_name_at(&self, pos: u64) -> Option<&str> {
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
    pub fn set_ascii_strings_visible(&mut self, visible: bool) {
        if visible == self.flags.contains(Flags::ShowAsciiStrings) {
            return;
        }
        if visible {
            self.flags.set(Flags::ShowAsciiStrings);
        } else {
            self.flags.remove(Flags::ShowAsciiStrings);
        }
        self.paint_buffer();
    }
    #[inline(always)]
    pub fn is_ascii_strings_visible(&self) -> bool {
        self.flags.contains(Flags::ShowAsciiStrings)
    }
    pub fn set_unicode_strings_visible(&mut self, visible: bool) {
        if visible == self.flags.contains(Flags::ShowUnicodeStrings) {
            return;
        }
        if visible {
            self.flags.set(Flags::ShowUnicodeStrings);
        } else {
            self.flags.remove(Flags::ShowUnicodeStrings);
        }
        self.paint_buffer();
    }
    #[inline(always)]
    pub fn is_unicode_strings_visible(&self) -> bool {
        self.flags.contains(Flags::ShowUnicodeStrings)
    }
    pub fn set_decode_utf8(&mut self, decode_utf8: bool) {
        if decode_utf8 == self.flags.contains(Flags::DecodeUTF8Characters) {
            return;
        }
        if decode_utf8 {
            self.flags.set(Flags::DecodeUTF8Characters);
        } else {
            self.flags.remove(Flags::DecodeUTF8Characters);
        }
        self.paint_buffer();
    }
    /// Returns the current selection as a half-open byte range `[start, end)`, or `None` if nothing is selected.
    #[inline(always)]
    pub fn selection(&self) -> Option<(u64, u64)> {
        self.selection.range()
    }
    #[inline(always)]
    pub fn has_selection(&self) -> bool {
        !self.selection.is_empty()
    }
    pub fn clear_selection(&mut self) {
        if self.selection.is_empty() {
            return;
        }
        self.selection.clear();
    }
    #[inline(always)]
    pub fn current_pos(&self) -> u64 {
        self.pos
    }
    #[inline(always)]
    pub fn set_current_pos(&mut self, pos: u64) {
        self.goto_position(pos, false, false);
    }
    pub fn delete(&mut self, pos: u64, count: u64) -> bool {
        if self.flags.contains(Flags::ReadOnly) {
            return false;
        }
        self.buffer.delete(pos, count)
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
    fn write_interval_name(surface: &mut Surface, attr: CharAttribute, name: &str, len: u32, x: i32, y: i32) {
        if len == 0 {
            return;
        }
        let format = TextFormatBuilder::new()
            .attribute(attr)
            .wrap_type(WrapType::SingleLineWrap(len as u16))
            .position(x, y)
            .build();
        surface.write_text(name, &format);
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
    #[inline(always)]
    fn is_ascii_char(b: u8) -> bool {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' => true,
            b'\t' | b'\n' | b'\r' | b' ' => true,
            b'<' | b'>' | b'=' | b'+' | b'-' | b'*' | b'/' | b'|' | b'\\' | b'\"' | b'.' | b',' | b';' | b':' | b'!' | b'?' | b'(' | b')' | b'['
            | b']' => true,
            _ => false,
        }
    }
    fn decode_ascii(&mut self, pos: u64) -> Option<u32> {
        let mut len = 0;
        let mut pos = pos;
        let size = self.size();
        let end = pos.saturating_add(size.width as u64 * size.height as u64).min(self.buffer.len());
        while pos < end {
            if let Some(b) = self.buffer.get(pos) {
                if Self::is_ascii_char(b) {
                    len += 1;
                    pos += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        if len > 3 {
            Some(len)
        } else {
            None
        }
    }
    fn decode_unicode(&mut self, pos: u64) -> Option<u32> {
        let mut len = 0;
        let mut pos = pos;
        let size = self.size();
        let end = pos.saturating_add(size.width as u64 * size.height as u64).min(self.buffer.len());
        let mut bytes = [0; 2];
        while pos < end {
            if self.buffer.read_bytes(pos, &mut bytes) {
                if bytes[1] != 0 || !Self::is_ascii_char(bytes[0]) {
                    break;
                }
                pos += 2;
                len += 1;
            } else {
                break;
            }
        }
        if len > 3 {
            Some(len)
        } else {
            None
        }
    }
    fn decode_utf8(&mut self, pos: u64, b: u8) -> Option<(char, u8)> {
        if b & 0b1110_0000 == 0b1100_0000 {
            let mut bytes = [0; 2];
            if !self.buffer.read_bytes(pos, &mut bytes) {
                return None;
            }
            let s = std::str::from_utf8(&bytes).ok()?;
            let ch = s.chars().next()?;
            Some((ch, 2))
        } else if b & 0b1111_0000 == 0b1110_0000 {
            let mut bytes = [0; 3];
            if !self.buffer.read_bytes(pos, &mut bytes) {
                return None;
            }
            let s = std::str::from_utf8(&bytes).ok()?;
            let ch = s.chars().next()?;
            Some((ch, 3))
        } else if b & 0b1111_1000 == 0b1111_0000 {
            let mut bytes = [0; 4];
            if !self.buffer.read_bytes(pos, &mut bytes) {
                return None;
            }
            let s = std::str::from_utf8(&bytes).ok()?;
            let ch = s.chars().next()?;
            Some((ch, 4))
        } else {
            None
        }
    }
    fn write_chars(&mut self, position: u64) {
        let mut cwp = CharWriterPos::new(self.repr.columns_count as i32, self.repr.rows_count as i32, position, self.buffer.len());
        let decode_utf8 = self.flags.contains(Flags::DecodeUTF8Characters);
        let show_ascii = self.flags.contains(Flags::ShowAsciiStrings);
        let show_unicode = self.flags.contains(Flags::ShowUnicodeStrings);
        let enph_attr_1 = self.theme().text.enphasized_1;
        let enph_attr_2 = self.theme().text.enphasized_2;
        let enph_attr_3 = self.theme().text.enphasized_3;
        while cwp.is_valid() {
            if let Some(b) = self.buffer.get(cwp.pos) {
                if !self.current_segment.contains(cwp.pos) {
                    self.update_current_segment(cwp.pos);
                }
                if decode_utf8 && b >= 0x80 {
                    if let Some((ch, len)) = self.decode_utf8(cwp.pos, b) {
                        for i in 0..len {
                            let c = if i == 0 { ch } else { ' ' };
                            self.buf_surface.write_char(cwp.x, cwp.y, Character::with_attributes(c, enph_attr_1));
                            if !cwp.move_cursor(1) {
                                break;
                            }
                        }
                        continue;
                    }
                }
                if show_ascii && Self::is_ascii_char(b) {
                    if let Some(len) = self.decode_ascii(cwp.pos) {
                        for _ in 0..len {
                            let c = self.buffer.get(cwp.pos).unwrap_or(b' ');
                            self.buf_surface.write_char(cwp.x, cwp.y, Character::with_attributes(c, enph_attr_2));
                            if !cwp.move_cursor(1) {
                                break;
                            }
                        }
                        continue;
                    }
                }
                if show_unicode && Self::is_ascii_char(b) {
                    if let Some(len) = self.decode_unicode(cwp.pos) {
                        for _ in 0..len {
                            let c = self.buffer.get(cwp.pos).unwrap_or(b' ');
                            self.buf_surface.write_char(cwp.x, cwp.y, Character::with_attributes(c, enph_attr_3));
                            if !cwp.move_cursor(2) {
                                break;
                            }
                        }
                        if cwp.is_valid() {
                            let c = Character::with_attributes(' ', enph_attr_3);
                            for _ in 0..len {
                                self.buf_surface.write_char(cwp.x, cwp.y, c);
                                if !cwp.move_cursor(0) {
                                    break;
                                }
                            }
                        }
                        continue;
                    }
                }
                let ch = self.cp.get(b);
                self.buf_surface
                    .write_char(cwp.x, cwp.y, Character::with_attributes(ch, self.current_segment_attr));
                cwp.move_cursor(1);
            } else {
                // error - unable to reach character / byte
                self.buf_surface
                    .write_char(cwp.x, cwp.y, Character::with_attributes('?', self.theme().text.error));
                cwp.move_cursor(1);
            }
        }
    }
    fn write_panels(&mut self, position: u64) {
        let mut cwp = CharWriterPos::new(self.repr.columns_count as i32, self.repr.rows_count as i32, position, self.buffer.len());
        let mut output = OutputBuffer::new();
        let mut bytes = [0; 8];
        let mut chars = [0 as char; 8];
        let errch = Character::with_attributes('?', self.theme().text.error);
        let bytes_count = self.repr.format.bytes_count().min(8) as usize;
        let display_chars_width = (self.repr.format.display_chars() + 1) as i32;
        let inactive_attr = self.theme().text.inactive;
        let start_x_char = 1 + ((self.repr.format.display_chars() + 1) * self.repr.columns_count as u32 + 3) as i32;
        let inactive_data_panel = if self.flags.contains(Flags::NoPanelDimming) {
            false
        } else {
            self.active_panel == ActivePanel::Char
        };
        let inactive_char_panel = if self.flags.contains(Flags::NoPanelDimming) {
            false
        } else {
            self.active_panel == ActivePanel::DataRepresentation
        };

        while cwp.is_valid() {
            let mut i = 0;
            let read_ok = loop {
                if let Some(b) = self.buffer.get(cwp.pos + i as u64) {
                    bytes[i] = b;
                    i += 1;
                    if i == bytes_count {
                        break true;
                    }
                } else {
                    break false;
                }
            };
            if read_ok {
                if !self.current_segment.contains(cwp.pos) {
                    self.update_current_segment(cwp.pos);
                }
                // mai intai copii ca sa nu ma impacteze inversarea de la BE
                for i in 0..bytes_count {
                    chars[i] = self.cp.get(bytes[i]);
                }
                if self.repr.endian != NATIVE_ENDIAN && bytes_count > 1 {
                    bytes[0..bytes_count].reverse();
                }
                self.repr.format.write(bytes, &mut output);
                let attr = if inactive_data_panel { inactive_attr } else { self.current_segment_attr };
                let x = cwp.x * display_chars_width + 1;
                self.buf_surface.write_ascii(x, cwp.y, output.as_slice(), attr, false);
                let attr = if inactive_char_panel { inactive_attr } else { self.current_segment_attr };
                let x = start_x_char + cwp.x;
                for i in 0..bytes_count {
                    self.buf_surface
                        .write_char(x + i as i32, cwp.y, Character::with_attributes(chars[i], attr));
                }
            } else {
                // error - unable to reach byte
                let x = cwp.x * display_chars_width + 1;
                self.buf_surface
                    .fill_horizontal_line_with_size(x, cwp.y, self.repr.format.display_chars(), errch);
                let x = start_x_char + cwp.x;
                self.buf_surface.fill_horizontal_line_with_size(x, cwp.y, bytes_count as u32, errch);
            }
            cwp.move_cursor(bytes_count as u64);
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
        self.buf_surface.reset(Character::with_attributes(' ', attr));
        self.update_current_segment(self.start_view);
        if self.repr.format.is_char() {
            self.write_chars(self.start_view);
        } else {
            self.write_panels(self.start_view);
        }
    }

    pub(super) fn hex_format(index: u32, _display_chars: u32, output: &mut [u8; 4]) -> u8 {
        const HEX: &[u8; 16] = b"0123456789ABCDEF";
        output[0] = HEX[((index >> 4) & 0x0F) as usize];
        output[1] = HEX[(index & 0x0F) as usize];
        2
    }
    pub(super) fn dec_format(index: u32, display_chars: u32, output: &mut [u8; 4]) -> u8 {
        if index < 10 {
            output[0] = b'+';
            output[1] = index as u8 + b'0';
            2
        } else if index < 100 {
            if display_chars > 2 {
                output[0] = b'+';
                output[1] = ((index / 10) as u8) + b'0';
                output[2] = (index as u8 % 10) + b'0';
                3
            } else {
                output[0] = ((index / 10) as u8) + b'0';
                output[1] = (index as u8 % 10) + b'0';
                2
            }
        } else if index < 1000 {
            let d1 = (index / 100) as u8;
            let rest = index % 100;
            let d2 = (rest / 10) as u8;
            let d3 = (rest % 10) as u8;
            match display_chars {
                2 => {
                    output[0] = d2 + b'0';
                    output[1] = d3 + b'0';
                    2
                }
                3 => {
                    output[0] = d1 + b'0';
                    output[1] = d2 + b'0';
                    output[2] = d3 + b'0';
                    3
                }
                _ => {
                    output[0] = b'+';
                    output[1] = d1 + b'0';
                    output[2] = d2 + b'0';
                    output[3] = d3 + b'0';
                    4
                }
            }
        } else {
            let d1 = (index / 1000) as u8;
            let rest = index % 1000;
            let d2 = (rest / 100) as u8;
            let rest = rest % 100;
            let d3 = (rest / 10) as u8;
            let d4 = (rest % 10) as u8;
            match display_chars {
                2 => {
                    output[0] = d3 + b'0';
                    output[1] = d4 + b'0';
                    2
                }
                3 => {
                    output[0] = d2 + b'0';
                    output[1] = d3 + b'0';
                    output[2] = d4 + b'0';
                    3
                }
                _ => {
                    output[0] = d1 + b'0';
                    output[1] = d2 + b'0';
                    output[2] = d3 + b'0';
                    output[3] = d4 + b'0';
                    4
                }
            }
        }
    }
    fn paint_header(&self, surface: &mut Surface, theme: &Theme) {
        if self.flags.contains(Flags::HideHeader) {
            return;
        }
        let has_focus = self.has_focus();
        let attr = match () {
            _ if !self.is_enabled() => theme.header.text.inactive,
            _ if has_focus => theme.header.text.focused,
            _ => theme.header.text.normal,
        };
        surface.fill_horizontal_line_with_size(0, 0, self.size().width, Character::with_attributes(' ', attr));

        let border_width = self.border_width() as i32;
        let mut border_x = 0;
        if self.flags.contains(Flags::ShowAddress) {
            Self::write_column_title(surface, attr, &self.AddrName, self.addr_width as u32, border_x);
            border_x += (1 + self.addr_width) as i32;
        }
        if self.flags.contains(Flags::ShowIntervalNames) {
            Self::write_column_title(surface, attr, &self.interval_name_title, self.interval_name_width as u32, border_x);
        }
        let h_offset = self.h_offset as i32;
        let right = self.size().width as i32 - 1;
        // the data/character header columns scroll horizontally with the buffer content
        surface.set_relative_clip(border_width, 0, right, 0);
        let data_x = border_width - h_offset;
        if !self.repr.format.is_char() {
            let hex_start = data_x;
            let display_chars = self.repr.format.display_chars() as u32;
            let mut buf = [b'0'; 4];
            let mut x = data_x + 1;
            for id in 0..self.repr.columns_count as u32 {
                let c = id * self.repr.format.bytes_count() as u32;
                let output_len = match self.repr.offset_format {
                    OffsetFormat::Hex => Self::hex_format(c, display_chars, &mut buf),
                    OffsetFormat::Dec => Self::dec_format(c, display_chars, &mut buf),
                };
                surface.write_ascii(x, 0, &buf[..output_len as usize], attr, false);
                x += (display_chars + 1) as i32;
            }
            if has_focus {
                if let Some(panel_attr) = self.header_panel_attr(theme, ActivePanel::DataRepresentation) {
                    surface.fill_horizontal_line(hex_start, 0, x, Character::with_attributes(0, panel_attr));
                }
            }
            x += 3;
            let char_start = x;
            surface.write_string(x, 0, "Characters", attr, false);
            if has_focus {
                if let Some(panel_attr) = self.header_panel_attr(theme, ActivePanel::Char) {
                    surface.fill_horizontal_line(char_start - 1, 0, right, Character::with_attributes(0, panel_attr));
                }
            }
        } else {
            surface.write_string(data_x, 0, "Characters", attr, false);
        }
        surface.reset_clip();
    }
    /// Returns the highlight attribute for a header panel: the selected attribute when it is the
    /// active panel, the hovered attribute when the mouse is over it, or `None` when neither applies.
    fn header_panel_attr(&self, theme: &Theme, panel: ActivePanel) -> Option<CharAttribute> {
        if self.active_panel == panel {
            Some(theme.header.text.pressed_or_selected)
        } else if self.hovered_panel == Some(panel) {
            Some(theme.header.text.hovered)
        } else {
            None
        }
    }
    /// Maps a header (row 0) coordinate to the panel underneath it, or `None` when the position is
    /// outside the two switchable panels (border columns, character-only mode, or header hidden).
    fn header_panel_at(&self, x: i32, y: i32) -> Option<ActivePanel> {
        if y != 0 || self.flags.contains(Flags::HideHeader) || self.repr.format.is_char() {
            return None;
        }
        let border_width = self.border_width() as i32;
        if x < border_width || x >= self.size().width as i32 {
            return None;
        }
        // translate screen x into the same coordinates used by `buf_surface`
        let surf_x = x - border_width + self.h_offset as i32;
        if surf_x < 0 {
            return None;
        }
        let display_chars = self.repr.format.display_chars() as i32;
        let char_start = self.repr.columns_count as i32 * (display_chars + 1) + 3;
        if surf_x >= char_start {
            Some(ActivePanel::Char)
        } else {
            Some(ActivePanel::DataRepresentation)
        }
    }
    fn paint_cursor(&self, surface: &mut Surface, theme: &Theme, border_width: i32, top: i32) {
        if self.pos >= self.start_view {
            let h_offset = self.h_offset as i32;
            let unit = self.repr.format.bytes_count();
            let dif = (self.pos - self.start_view) / unit as u64;
            let column = (dif % self.repr.columns_count as u64) as i32;
            let row = (dif / self.repr.columns_count as u64) as i32 + top;
            let active_attr = theme.list_current_item.focus;
            let attr_just_text = CharAttribute::new(Color::Transparent, Color::Transparent, CharFlags::None);
            let can_edit = self.can_edit();
            // the panel that does not hold the cursor is marked with a dimmed attribute
            let inactive_attr = theme.list_current_item.over_inactive;
            if self.repr.format.is_char() {
                let x = border_width + column - h_offset;
                surface.write_char(x, row, Character::with_attributes(0, active_attr));
                if can_edit {
                    surface.set_cursor(x + self.edit_text.len() as i32, row);
                    if !self.edit_text.is_empty() {
                        surface.write_string(x, row, self.edit_text.as_str(), attr_just_text, false);
                    }
                }
            } else {
                let len = self.repr.format.display_chars() as i32;
                let byte_in_element = ((self.pos - self.start_view) % unit as u64) as i32;
                let hex_x = border_width + column * (len + 1) - h_offset;
                let char_block_x = border_width + self.repr.columns_count as i32 * (len + 1) + column * unit as i32 + 4 - h_offset;
                match self.active_panel {
                    ActivePanel::DataRepresentation => {
                        // active hex element + dimmed mirror over all of its bytes in the character part
                        surface.fill_horizontal_line_with_size(hex_x, row, (len + 2) as u32, Character::with_attributes(0, active_attr));
                        surface.fill_horizontal_line_with_size(char_block_x, row, unit as u32, Character::with_attributes(0, inactive_attr));
                        if can_edit {
                            surface.set_cursor(hex_x + 1 + self.edit_text.len() as i32, row);
                            if !self.edit_text.is_empty() {
                                surface.write_string(hex_x + 1, row, self.edit_text.as_str(), attr_just_text, false);
                            }
                        }
                    }
                    ActivePanel::Char => {
                        // active single byte in the character part + dimmed mirror over its hex element
                        surface.fill_horizontal_line_with_size(hex_x, row, (len + 2) as u32, Character::with_attributes(0, inactive_attr));
                        surface.write_char(char_block_x + byte_in_element, row, Character::with_attributes(0, active_attr));
                        if can_edit {
                            surface.set_cursor(char_block_x + byte_in_element + self.edit_text.len() as i32, row);
                            if !self.edit_text.is_empty() {
                                surface.write_string(char_block_x + byte_in_element, row, self.edit_text.as_str(), attr_just_text, false);
                            }
                        }
                    }
                }
            }
        }
    }
    fn paint_selection(&self, surface: &mut Surface, theme: &Theme, border_width: i32, top: i32) {
        if let Some((sel_start, sel_end)) = self.selection.range() {
            let h_offset = self.h_offset as i32;
            let cols = self.repr.columns_count as u64;
            let rows = self.repr.rows_count as u64;
            let bytes_count = self.repr.format.bytes_count() as u64;
            let view_end = self.start_view.saturating_add(cols * rows * bytes_count);
            let from = sel_start.max(self.start_view);
            let to = sel_end.min(view_end).min(self.buffer.len());
            let sel_ch = if self.comp.is_in_edit_mode() {
                Character::with_attributes(0, theme.list_current_item.over_selection)
            } else {
                Character::with_attributes(0, theme.list_current_item.selected)
            };
            let len = self.repr.format.display_chars() as i32;
            let char_base = self.repr.columns_count as i32 * (len + 1) + 4;
            let mut pos = from;
            while pos < to {
                let dif = pos - self.start_view;
                let element = dif / bytes_count;
                let column = (element % cols) as i32;
                let row = (element / cols) as i32 + top;
                let byte_in_element = (dif % bytes_count) as i32;
                if self.repr.format.is_char() {
                    let x = border_width + column - h_offset;
                    surface.write_char(x, row, sel_ch);
                } else {
                    // highlight the whole hex cell once per element, and the exact byte in the char part
                    if byte_in_element == 0 {
                        let x = border_width + column * (len + 1) - h_offset;
                        surface.fill_horizontal_line_with_size(x, row, (len + 1) as u32, sel_ch);
                    }
                    let xc = border_width + char_base + column * bytes_count as i32 + byte_in_element - h_offset;
                    surface.write_char(xc, row, sel_ch);
                }
                pos += 1;
            }
        }
    }
    fn paint_border(&self, surface: &mut Surface, theme: &Theme, top: i32) {
        if !self.flags.contains_one(Flags::ShowAddress | Flags::ShowIntervalNames) {
            return;
        }
        let mut start = self.start_view;
        let mut y = top;
        let has_focus = self.has_focus();
        let is_in_edit_mode = self.comp.is_in_edit_mode();
        for _ in 0..self.repr.rows_count {
            let mut x = 0;
            if self.flags.contains(Flags::ShowAddress) {
                Self::write_offset(surface, theme.text.inactive, start, self.addr_width as u32, y, self.repr.offset_format);
                x += (1 + self.addr_width) as i32;
            }
            if self.flags.contains(Flags::ShowIntervalNames) {
                let segment = self.intervals.pos_to_segment(start, self.buffer.len());
                if segment.exists() {
                    if let Some(int) = self.intervals.get(segment.index) {
                        let attr = match () {
                            _ if !self.is_enabled() => theme.text.inactive,
                            _ if is_in_edit_mode => theme.text.inactive,
                            _ if has_focus => int.attr,
                            _ => theme.text.normal,
                        };
                        Self::write_interval_name(surface, attr, int.name.as_str(), self.interval_name_width as u32, x, y);
                    } else {
                        surface.fill_horizontal_line_with_size(
                            x,
                            y,
                            self.interval_name_width as u32,
                            Character::with_attributes('-', theme.text.inactive),
                        );
                    }
                } else {
                    surface.fill_horizontal_line_with_size(
                        x,
                        y,
                        self.interval_name_width as u32,
                        Character::with_attributes('-', theme.text.inactive),
                    );
                }
            }
            start += self.repr.columns_count as u64 * self.repr.format.bytes_count() as u64;
            if start >= self.buffer.len() {
                break;
            }
            y += 1;
        }
        let mut x = 0;
        let bottom = (self.size().height as i32).saturating_sub(if self.has_focus() { 1 } else { 0 });
        if self.flags.contains(Flags::ShowAddress) {
            surface.draw_vertical_line(
                self.addr_width as i32,
                0,
                bottom,
                if self.flags.contains(Flags::ShowIntervalNames) {
                    LineType::Single
                } else {
                    LineType::Double
                },
                self.separator_attr(theme, Separator::Address),
            );
            x += (1 + self.addr_width) as i32;
        }
        if self.flags.contains(Flags::ShowIntervalNames) {
            surface.draw_vertical_line(
                x + self.interval_name_width as i32,
                0,
                bottom,
                LineType::Double,
                self.separator_attr(theme, Separator::IntervalName),
            );
        }
    }
    fn border_width(&self) -> u32 {
        let mut border_width = 0;
        if self.flags.contains(Flags::ShowAddress) {
            border_width += (1 + self.addr_width) as u32;
        }
        if self.flags.contains(Flags::ShowIntervalNames) {
            border_width += (1 + self.interval_name_width) as u32;
        }
        border_width
    }
    fn separator_at(&self, x: i32) -> Option<Separator> {
        if self.flags.contains(Flags::ShowAddress) && x == self.addr_width as i32 {
            return Some(Separator::Address);
        }
        if self.flags.contains(Flags::ShowIntervalNames) {
            let base = if self.flags.contains(Flags::ShowAddress) {
                self.addr_width as i32 + 1
            } else {
                0
            };
            if x == base + self.interval_name_width as i32 {
                return Some(Separator::IntervalName);
            }
        }
        None
    }
    #[inline(always)]
    fn separator_exists(&self, separator: Separator) -> bool {
        match separator {
            Separator::Address => self.flags.contains(Flags::ShowAddress),
            Separator::IntervalName => self.flags.contains(Flags::ShowIntervalNames),
        }
    }
    fn first_separator(&self) -> Option<Separator> {
        if self.flags.contains(Flags::ShowAddress) {
            Some(Separator::Address)
        } else if self.flags.contains(Flags::ShowIntervalNames) {
            Some(Separator::IntervalName)
        } else {
            None
        }
    }
    fn set_separator_width(&mut self, separator: Separator, width: u32) -> bool {
        let changed = match separator {
            Separator::Address if self.flags.contains(Flags::ShowAddress) => {
                let new_width = width.clamp(1, MAX_ADDRESS_WIDTH);
                if new_width != self.addr_width {
                    self.addr_width = new_width;
                    true
                } else {
                    false
                }
            }
            Separator::IntervalName if self.flags.contains(Flags::ShowIntervalNames) => {
                let new_width = width.clamp(1, MAX_INTERVAL_NAME_WIDTH);
                if new_width != self.interval_name_width {
                    self.interval_name_width = new_width;
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
            Separator::IntervalName => {
                let base = if self.flags.contains(Flags::ShowAddress) {
                    self.addr_width as i32 + 1
                } else {
                    0
                };
                (x - base).max(1) as u32
            }
        };
        self.set_separator_width(separator, width)
    }
    fn separator_attr(&self, theme: &Theme, separator: Separator) -> CharAttribute {
        if !self.is_enabled() {
            theme.lines.inactive
        } else if self.selected_separator == Some(separator) {
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
        let bytes_count = self.repr.format.bytes_count() as u32;
        let column_width = self.repr.format.display_chars() + 1 + bytes_count;
        let nr_columns = match self.repr.columns {
            ColumnsCount::Fixed(count) => count as u32,
            ColumnsCount::Auto => {
                let space_left = screen_size.width.saturating_sub(self.border_width());
                let columns = if self.repr.format.is_char() {
                    space_left
                } else {
                    space_left.saturating_sub(4) / column_width
                };
                columns.max(1) as u32
            }
        };
        self.repr.columns_count = nr_columns.clamp(1, 255);
        self.repr.rows_count = h;
        let w = if self.repr.format.is_char() {
            self.repr.columns_count as u32
        } else {
            column_width * (self.repr.columns_count as u32) + 4
        };
        self.buf_surface.resize(Size::new(w, h));
        if !self.goto_position(self.pos, false, false) {
            self.paint_buffer();
        }
    }
    fn goto_position(&mut self, new_pos: u64, select: bool, emit_event: bool) -> bool {
        let new_pos = new_pos.min(self.buffer.len().saturating_sub(1));
        let old_pos = self.pos;
        let old_selection = self.selection.range();
        let old_start_view = self.start_view;
        let rows = self.repr.rows_count as u64;
        let bytes_per_line = self.bytes_per_line();
        let visible_count = bytes_per_line * rows;
        let line_offset = (new_pos as i64 - self.start_view as i64).rem_euclid(bytes_per_line as i64) as u64;
        if new_pos < self.start_view {
            self.start_view = new_pos.saturating_sub(line_offset);
        } else if new_pos >= self.start_view + visible_count {
            let row_start = new_pos - line_offset;
            self.start_view = row_start.saturating_sub(rows.saturating_sub(1) * bytes_per_line);
        }
        if select {
            self.selection.update(self.pos, new_pos);
        } else if new_pos != self.pos {
            self.selection.clear();
        }
        self.pos = new_pos;
        self.ensure_visible();
        self.update_scrollbars();
        if emit_event {
            if old_pos != new_pos {
                self.emit_current_pos_changed_event();
            }
            if old_selection != self.selection.range() {
                self.emit_selection_update_event();
            }
        }
        if old_start_view != self.start_view {
            self.paint_buffer();
            true
        } else {
            false
        }
    }
    #[inline(always)]
    fn buffer_visible_width(&self) -> u32 {
        self.size().width.saturating_sub(self.border_width())
    }
    fn ensure_visible(&mut self) {
        let visible_width = self.buffer_visible_width() as i32;
        if visible_width <= 0 {
            return;
        }
        let surface_width = self.buf_surface.size().width as i32;
        let cols = (self.repr.columns_count as u64).max(1);
        let bytes_count = (self.repr.format.bytes_count() as u64).max(1);
        let dif = self.pos.saturating_sub(self.start_view);
        let column = ((dif / bytes_count) % cols) as i32;
        let byte_in_element = (dif % bytes_count) as i32;
        let (cell_x, cell_w) = if self.repr.format.is_char() {
            (column, 1)
        } else {
            let len = self.repr.format.display_chars() as i32;
            match self.active_panel {
                ActivePanel::DataRepresentation => (column * (len + 1), len + 2),
                ActivePanel::Char => {
                    let char_base = cols as i32 * (len + 1) + 4;
                    (char_base + column * bytes_count as i32 + byte_in_element, 1)
                }
            }
        };
        let mut off = self.h_offset as i32;
        if cell_x < off {
            off = cell_x;
        } else if cell_x + cell_w > off + visible_width {
            off = cell_x + cell_w - visible_width;
        }
        let max_off = (surface_width - visible_width).max(0);
        self.h_offset = off.clamp(0, max_off) as u32;
    }
    #[inline(always)]
    fn bytes_per_line(&self) -> u64 {
        (self.repr.columns_count as u64).max(1) * (self.repr.format.bytes_count() as u64).max(1)
    }
    fn update_scrollbars(&mut self) {
        let bytes_per_line = self.bytes_per_line();
        let total_lines = self.buffer.len().saturating_add(bytes_per_line - 1) / bytes_per_line;
        let visible_lines = self.repr.rows_count;
        let surface_width = self.buf_surface.size().width as u64;
        let visible_width = self.buffer_visible_width();
        let h_offset = self.h_offset as u64;
        let start_line = self.start_view / bytes_per_line;
        let visible_space = Size::new(visible_width, visible_lines);
        self.comp.resize(surface_width, total_lines, &self.base, visible_space);
        self.comp.set_indexes(h_offset, start_line);
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        self.h_offset = self.comp.horizontal_index() as u32;
        let new_start = self.comp.vertical_index().saturating_mul(self.bytes_per_line());
        if new_start != self.start_view {
            self.start_view = new_start;
            self.paint_buffer();
        }
        self.update_scrollbars();
    }
    #[inline(always)]
    fn matches_bytes(&mut self, pos: u64) -> bool {
        let bytes = &self.search_bytes;
        if bytes.is_empty() {
            return true;
        }
        if pos.saturating_add(bytes.len() as u64) > self.buffer.len() {
            return false;
        }
        for (index, &byte) in bytes.iter().enumerate() {
            if self.buffer.get(pos + index as u64) != Some(byte) {
                return false;
            }
        }
        true
    }
    fn search_bytes_from_offset(&mut self, offset: u64) -> Option<u64> {
        let bytes = &self.search_bytes;
        if bytes.is_empty() {
            return None;
        }
        let buffer_len = self.buffer.len() as u64;
        let bytes_len = bytes.len() as u64;
        if bytes_len > buffer_len {
            return None;
        }
        let start_offset = offset % buffer_len;
        for i in 0..buffer_len - bytes_len + 1 {
            let pos = (i + start_offset) % buffer_len;
            if self.matches_bytes(pos) {
                return Some(pos);
            }
        }
        None
    }
    fn emit_current_pos_changed_event(&self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::BufferView(EventData {
                event_type: super::events::BufferViewEventTypes::CurrentPosChanged,
                type_id: std::any::TypeId::of::<T>(),
            }),
        });
    }
    fn emit_selection_update_event(&self) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::BufferView(EventData {
                event_type: super::events::BufferViewEventTypes::SelectionChanged,
                type_id: std::any::TypeId::of::<T>(),
            }),
        });
    }
    fn search(&mut self, next: bool) {
        if self.buffer.len() == 0 {
            return;
        }
        let start_search_pos = if next {
            self.pos.saturating_add(1) % self.buffer.len()
        } else {
            self.pos
        };
        let text = self.comp.search_text();
        if super::search_parser::parse(text, &mut self.search_bytes).is_ok() {
            if let Some(pos) = self.search_bytes_from_offset(start_search_pos) {
                let end = pos + self.search_bytes.len() as u64 - 1;
                if !self.goto_position(pos, false, true) {
                    self.paint_buffer();
                }
                self.selection.clear();
                self.selection.update(pos, end);
                self.emit_selection_update_event();
                return;
            }
        }
        if !self.selection.is_empty() {
            self.selection.clear();
            self.emit_selection_update_event();
        }
    }
    fn process_selector_key(&mut self, key: Key) -> EventProcessStatus {
        let separator = match self.selected_separator {
            Some(separator) => separator,
            None => return EventProcessStatus::Ignored,
        };
        match key.value() {
            key!("Left") => {
                let width = match separator {
                    Separator::Address => self.addr_width,
                    Separator::IntervalName => self.interval_name_width,
                };
                self.set_separator_width(separator, width.saturating_sub(1));
            }
            key!("Right") => {
                let width = match separator {
                    Separator::Address => self.addr_width,
                    Separator::IntervalName => self.interval_name_width,
                };
                self.set_separator_width(separator, width.saturating_add(1));
            }
            key!("Tab") | key!("Ctrl+Left") | key!("Ctrl+Right") | key!("Ctrl+Alt+Left") | key!("Ctrl+Alt+Right") => {
                let other = match separator {
                    Separator::Address => Separator::IntervalName,
                    Separator::IntervalName => Separator::Address,
                };
                if self.separator_exists(other) {
                    self.selected_separator = Some(other);
                }
            }
            _ => {
                self.selected_separator = None;
            }
        }
        EventProcessStatus::Processed
    }
    fn set_active_panel(&mut self, panel: ActivePanel) {
        if self.active_panel == panel || self.repr.format.is_char() {
            return;
        }
        if panel == ActivePanel::DataRepresentation {
            // moving onto the hex panel: snap the position to the start of its element
            // so that hex navigation (stepping by `bytes_count`) stays aligned
            let bytes_count = self.repr.format.bytes_count() as u64;
            if bytes_count > 1 {
                self.pos -= self.pos % bytes_count;
            }
        }
        self.active_panel = panel;
        self.paint_buffer();
        self.ensure_visible();
    }
    fn toggle_panel(&mut self) {
        let other = match self.active_panel {
            ActivePanel::DataRepresentation => ActivePanel::Char,
            ActivePanel::Char => ActivePanel::DataRepresentation,
        };
        self.set_active_panel(other);
    }
    fn process_search_key(&mut self, key: Key, character: char) -> EventProcessStatus {
        if self.comp.process_key_pressed(key, character) {
            self.search(false);
            return EventProcessStatus::Processed;
        }
        if self.comp.is_in_edit_mode() {
            match key.value() {
                key!("Enter") => {
                    self.search(true);
                    return EventProcessStatus::Processed;
                }
                _ => {}
            }
        }
        EventProcessStatus::Ignored
    }
    #[inline(always)]
    fn can_edit(&self) -> bool {
        self.buffer.can_edit() && !self.flags.contains(Flags::ReadOnly)
    }
    fn update_buffer_content(&mut self) {
        let (bytes, len) = if !self.repr.format.is_char() {
            match self.active_panel {
                ActivePanel::DataRepresentation => self.repr.format.convert_to_bytes(&self.edit_text),
                ActivePanel::Char => DataRepresentationFormat::Char.convert_to_bytes(&self.edit_text),
            }
        } else {
            self.repr.format.convert_to_bytes(&self.edit_text)
        };
        if len > 0 {
            let _ = self.buffer.write_bytes(self.pos, &bytes[..len as usize]);
            // daca nu s-a updatat si paint-ul - redesenam
            if !self.goto_position(self.pos.saturating_add(len as u64), false, true) {
                self.paint_buffer();
            }
        }
        self.edit_text.clear();
    }
    fn process_edit_key(&mut self, key: Key, character: char) -> EventProcessStatus {
        if !self.can_edit() {
            return EventProcessStatus::Ignored;
        }
        match key.value() {
            key!("Backspace") => {
                if self.edit_text.len() > 0 {
                    self.edit_text.pop();
                    return EventProcessStatus::Processed;
                } else {
                    return EventProcessStatus::Ignored;
                }
            }
            key!("Enter") | key!("Ctrl+Enter") | key!("Shift+Enter") | key!("Tab") | key!("Left") => {
                if self.edit_text.len() > 0 {
                    self.update_buffer_content();
                    return EventProcessStatus::Processed;
                }
                return EventProcessStatus::Ignored;
            }
            _ => {}
        }
        if character >= ' ' {
            self.edit_text.push(character);
            let res = if !self.repr.format.is_char() {
                match self.active_panel {
                    ActivePanel::DataRepresentation => self.repr.format.validate(&self.edit_text),
                    ActivePanel::Char => DataRepresentationFormat::Char.validate(&self.edit_text),
                }
            } else {
                self.repr.format.validate(&self.edit_text)
            };
            match res {
                ValidateResult::Valid => {} // do nothing
                ValidateResult::FormatError => {
                    self.edit_text.pop();
                }
                ValidateResult::Update => self.update_buffer_content(),
            };
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
    fn process_navigation_key(&mut self, key: Key) -> EventProcessStatus {
        let select = key.modifier.contains(KeyModifier::Shift);
        let bytes_count = self.repr.format.bytes_count() as u64;
        let h_unit = if self.active_panel == ActivePanel::Char { 1 } else { bytes_count };
        let row = self.repr.columns_count as u64 * bytes_count;
        match key.value() {
            key!("Ctrl+Tab") => {
                self.toggle_panel();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Alt+Left") | key!("Ctrl+Alt+Right") => {
                if let Some(separator) = self.first_separator() {
                    self.selected_separator = Some(separator);
                    return EventProcessStatus::Processed;
                }
                return EventProcessStatus::Ignored;
            }
            key!("Left") | key!("Shift+Left") => {
                self.goto_position(self.pos.saturating_sub(h_unit), select, true);
                return EventProcessStatus::Processed;
            }
            key!("Right") | key!("Shift+Right") => {
                self.goto_position(self.pos.saturating_add(h_unit), select, true);
                return EventProcessStatus::Processed;
            }
            key!("Up") | key!("Shift+Up") => {
                self.goto_position(self.pos.saturating_sub(row), select, true);
                return EventProcessStatus::Processed;
            }
            key!("Down") | key!("Shift+Down") => {
                self.goto_position(self.pos.saturating_add(row), select, true);
                return EventProcessStatus::Processed;
            }
            key!("Home") | key!("Shift+Home") => {
                self.goto_position(0, select, true);
                return EventProcessStatus::Processed;
            }
            key!("End") | key!("Shift+End") => {
                self.goto_position(self.buffer.len(), select, true);
                return EventProcessStatus::Processed;
            }
            key!("PageUp") | key!("Shift+PageUp") => {
                self.goto_position(self.pos.saturating_sub(row * self.repr.rows_count as u64), select, true);
                return EventProcessStatus::Processed;
            }
            key!("PageDown") | key!("Shift+PageDown") => {
                self.goto_position(self.pos.saturating_add(row * self.repr.rows_count as u64), select, true);
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
                self.move_view_with(-(row as i32));
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Down") => {
                self.move_view_with(row as i32);
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        EventProcessStatus::Ignored
    }
    fn move_view_with(&mut self, delta: i32) {
        //by design - to allow one to align 2 / 4 /8 bytes
        //bytes_count() will not be used to adjust the position
        let mut new_pos = if delta > 0 {
            self.start_view.saturating_add(delta as u64)
        } else {
            self.start_view.saturating_sub((-delta) as u64)
        };
        if new_pos >= self.buffer.len() {
            new_pos = self.buffer.len().saturating_sub(1);
        }
        if self.start_view != new_pos {
            self.start_view = new_pos;
            self.update_scrollbars();
            self.paint_buffer();
        }
    }
    fn scroll_horizontal(&mut self, delta: i32) -> bool {
        let surface_width = self.buf_surface.size().width as i32;
        let visible_width = self.buffer_visible_width() as i32;
        let max_off = (surface_width - visible_width).max(0);
        let new_off = (self.h_offset as i32 + delta).clamp(0, max_off) as u32;
        if new_off != self.h_offset {
            self.h_offset = new_off;
            self.update_scrollbars();
            true
        } else {
            false
        }
    }
    fn mouse_to_pos(&self, x: i32, y: i32) -> Option<(u64, ActivePanel)> {
        let top = if self.flags.contains(Flags::HideHeader) { 0 } else { 1 };
        if y < top {
            return None;
        }
        let surf_y = (y - top) as i64;
        if surf_y >= self.repr.rows_count as i64 {
            return None;
        }
        let border_width = self.border_width() as i32;
        if x < border_width {
            return None;
        }
        let surf_x = (x - border_width) as i64 + self.h_offset as i64;
        let cols = self.repr.columns_count as i64;
        let bytes_count = self.repr.format.bytes_count() as i64;
        if self.repr.format.is_char() {
            if !(0..cols).contains(&surf_x) {
                return None;
            }
            let pos = self.start_view + (surf_y as u64) * (cols as u64) + surf_x as u64;
            return if pos >= self.buffer.len() {
                None
            } else {
                Some((pos, ActivePanel::Char))
            };
        }
        let cell = self.repr.format.display_chars() as i64 + 1;
        let char_start = cols * cell + 4;
        // byte offset inside the row + which panel was clicked
        let (byte_in_row, panel) = if surf_x >= char_start {
            // character display part: each byte is one contiguous char, so the click is byte-precise
            let off = surf_x - char_start;
            if off >= cols * bytes_count {
                return None;
            }
            (off, ActivePanel::Char)
        } else if surf_x >= 0 {
            // hex part: snap to the start of the element under the cursor
            let c = surf_x / cell;
            if c >= cols {
                return None;
            }
            (c * bytes_count, ActivePanel::DataRepresentation)
        } else {
            return None;
        };
        let pos = self.start_view + (surf_y as u64) * (cols as u64) * bytes_count as u64 + byte_in_row as u64;
        if pos >= self.buffer.len() {
            None
        } else {
            Some((pos, panel))
        }
    }
}

impl<T: BufferAccess + 'static> OnPaint for BufferView<T> {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let top = if self.flags.contains(Flags::HideHeader) { 0 } else { 1 };
        let has_focus = self.has_focus();
        let border_width = self.border_width() as i32;
        let h_offset = self.h_offset as i32;
        let size = self.size();

        self.paint_header(surface, theme);
        self.paint_border(surface, theme, top);

        surface.set_relative_clip(border_width, top, size.width as i32 - 1, size.height as i32 - 1);
        let attr_overwrite = match () {
            _ if !self.is_enabled() => Some(theme.text.inactive),
            _ if self.comp.is_in_edit_mode() => Some(theme.text.inactive),
            _ if !has_focus => Some(theme.text.normal),
            _ => None,
        };
        if let Some(attr) = attr_overwrite {
            surface.draw_surface_with_transform(border_width - h_offset, top, &self.buf_surface, |ch| {
                Character::with_attributes(ch.code, attr)
            });
        } else {
            surface.draw_surface(border_width - h_offset, top, &self.buf_surface);
        }
        if has_focus {
            self.paint_selection(surface, theme, border_width, top);
            self.paint_cursor(surface, theme, border_width, top);
            surface.reset_clip();
            self.comp.paint(surface, theme, &self.base);
        }
    }
}

impl<T: BufferAccess + 'static> OnKeyPressed for BufferView<T> {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        if self.process_selector_key(key) == EventProcessStatus::Processed {
            return EventProcessStatus::Processed;
        }
        if self.comp.is_in_edit_mode() {
            // daca sunt in edit mode - procesez doar search key
            if self.process_search_key(key, character) == EventProcessStatus::Processed {
                return EventProcessStatus::Processed;
            }
        } else {
            // daca nu sunt in edit mode, mai intai edit_key si daca nu search key
            if self.process_edit_key(key, character) == EventProcessStatus::Processed {
                return EventProcessStatus::Processed;
            }
            if self.process_search_key(key, character) == EventProcessStatus::Processed {
                return EventProcessStatus::Processed;
            }
        }
        if self.process_navigation_key(key) == EventProcessStatus::Processed {
            self.comp.exit_edit_mode();
            return EventProcessStatus::Processed;
        }

        EventProcessStatus::Ignored
    }
}

impl<T: BufferAccess + 'static> OnMouseEvent for BufferView<T> {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.comp.process_mouse_event(event) {
            self.update_scroll_pos_from_scrollbars();
            return EventProcessStatus::Processed;
        }
        let result = match event {
            MouseEvent::Leave => {
                let mut changed = false;
                if self.hovered_separator.is_some() {
                    self.hovered_separator = None;
                    changed = true;
                }
                if self.hovered_panel.is_some() {
                    self.hovered_panel = None;
                    changed = true;
                }
                if changed {
                    return EventProcessStatus::Processed;
                }
                EventProcessStatus::Ignored
            }
            MouseEvent::Over(p) => {
                let hovered = self.separator_at(p.x);
                let hovered_panel = self.header_panel_at(p.x, p.y);
                let mut changed = false;
                if hovered != self.hovered_separator {
                    self.hovered_separator = hovered;
                    changed = true;
                }
                if hovered_panel != self.hovered_panel {
                    self.hovered_panel = hovered_panel;
                    changed = true;
                }
                if changed {
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
                if ev.button == MouseButton::Left {
                    if let Some(panel) = self.header_panel_at(ev.x, ev.y) {
                        self.set_active_panel(panel);
                        return EventProcessStatus::Processed;
                    }
                    if let Some((pos, panel)) = self.mouse_to_pos(ev.x, ev.y) {
                        if panel != self.active_panel {
                            self.active_panel = panel;
                            self.paint_buffer();
                        }
                        self.goto_position(pos, false, true);
                        self.selecting = true;
                        self.mouse_capture = true;
                        return EventProcessStatus::Processed;
                    }
                }
                EventProcessStatus::Ignored
            }
            MouseEvent::Drag(ev) => {
                if let (true, Some(separator)) = (self.mouse_capture, self.selected_separator) {
                    self.resize_separator_to(separator, ev.x);
                    return EventProcessStatus::Processed;
                }
                if self.selecting {
                    if let Some((pos, _)) = self.mouse_to_pos(ev.x, ev.y) {
                        self.goto_position(pos, true, true);
                    }
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
                if self.selecting {
                    self.selecting = false;
                    self.mouse_capture = false;
                    return EventProcessStatus::Processed;
                }
                EventProcessStatus::Ignored
            }
            MouseEvent::Wheel(direction) => {
                let line_bytes_count = self.repr.columns_count as i32 * self.repr.format.bytes_count() as i32;
                let processed = match direction {
                    MouseWheelDirection::Up => {
                        self.move_view_with(-line_bytes_count);
                        true
                    }
                    MouseWheelDirection::Down => {
                        self.move_view_with(line_bytes_count);
                        true
                    }
                    MouseWheelDirection::Left => self.scroll_horizontal(-1),
                    MouseWheelDirection::Right => self.scroll_horizontal(1),
                };
                if processed {
                    return EventProcessStatus::Processed;
                }
                EventProcessStatus::Ignored
            }
            _ => EventProcessStatus::Ignored,
        };
        if result == EventProcessStatus::Ignored && self.comp.should_repaint() {
            return EventProcessStatus::Processed;
        }
        result
    }
}

impl<T: BufferAccess + 'static> OnResize for BufferView<T> {
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.recompute_sizes(new_size);
        self.update_scrollbars();
    }
}
