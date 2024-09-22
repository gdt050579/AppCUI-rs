use std::ops::{Add, DivAssign, Rem};

// order
// suffix
// number (from right to left + groups)
// representation_digis
// prefix
// fill
pub(crate) struct FormatNumber {
    base: u8, // 2, 8, 10, 16
    group_size: u8,
    separator_char: u8,
    width: u8,
    fill_char: u8,
    representation_digits: u8,
    number_of_decimals: u8,
    prefix: &'static str,
    suffix: &'static str,
}

impl FormatNumber {
    pub(crate) fn write_to_buffer(value: u64, buf: &mut [u8]) -> &[u8] {
        let len = buf.len();
        if len == 0 {
            return buf;
        }
        let mut pos = len - 1;
        let mut value = value;
        loop {
            buf[pos] = (value % 10 + 48) as u8;
            value /= 10;
            if (value == 0) || (pos == 0) {
                break;
            }
            pos -= 1;
        }
        &buf[pos..]
    }
    pub(crate) fn number_of_digits(value: u64) -> u8 {
        // calculates the number of digits for a number using a intervals (0-9, 10-99, 100-999, 1000-9999, etc)
        match value {
            0..=9 => 1,
            10..=99 => 2,
            100..=999 => 3,
            1000..=9999 => 4,
            10000..=99999 => 5,
            _ => {
                let mut result = 1;
                let mut value = value / 10;
                while value > 0 {
                    result += 1;
                    value /= 10;
                }
                result
            }
        }
    }
    #[inline(always)]
    pub(crate) const fn new(base: u8) -> Self {
        match base {
            2 | 8 | 10 | 16 => (),
            _ => panic!("Invalid base value for FormatNumber (expected 2, 8, 10 or 16)"),
        }

        Self {
            base,
            group_size: 0,
            separator_char: 0,
            width: 0,
            fill_char: 0,
            number_of_decimals: 0,
            representation_digits: 0,
            prefix: "",
            suffix: "",
        }
    }
    #[inline(always)]
    pub(crate) const fn group(mut self, size: u8, separator: u8) -> Self {
        match size {
            0 | 3 | 4 => (),
            _ => panic!("Invalid group size for FormatNumber (expected 0, 3 or 4)"),
        }
        if size == 0 {
            match separator {
                0 => (),
                _ => panic!("Invalid separator char for FormatNumber (expected 0) if group size is 0"),
            }
        } else {
            match separator {
                32..=126 => (),
                _ => panic!("Invalid separator char for FormatNumber (expected a printable ASCII character) if group size si bigger than 0"),
            }
        }
        self.group_size = size;
        self.separator_char = separator;
        self
    }
    #[inline(always)]
    pub(crate) const fn fill(mut self, size: u8, fill_char: u8) -> Self {
        match size {
            0 => match fill_char {
                0 => (),
                _ => panic!("Invalid fill char for FormatNumber (expected 0) if width is 0"),
            },
            _ => match fill_char {
                32..=126 => (),
                _ => panic!("Invalid fill char for FormatNumber (expected a printable ASCII character) if width is bigger than 0"),
            },
        }
        self.width = size;
        self.fill_char = fill_char;
        self
    }
    #[inline(always)]
    pub(crate) const fn representation_digits(mut self, value: u8) -> Self {
        match self.base {
            2 => {
                if value > 128 {
                    panic!("Invalid number of representation digits for FormatNumber (maximum number of digits is 128 for base 2)");
                }
            }
            8 => {
                if value > 43 {
                    panic!("Invalid number of representation digits for FormatNumber (maximum number of digits is 43 for base 8)");
                }
            }
            10 => {
                if value > 39 {
                    panic!("Invalid number of representation digits for FormatNumber (maximum number of digits is 39 for base 10)");
                }
            }
            16 => {
                if value > 32 {
                    panic!("Invalid number of representation digits for FormatNumber (maximum number of digits is 32 for base 16)");
                }
            }
            _ => {}
        }
        self.representation_digits = value;
        self
    }

    #[inline(always)]
    pub(crate) const fn suffix(mut self, suffix: &'static str) -> Self {
        self.suffix = suffix;
        self
    }
    #[inline(always)]
    pub(crate) const fn prefix(mut self, prefix: &'static str) -> Self {
        self.prefix = prefix;
        self
    }
    pub(crate) const fn decimals(mut self, value: u8) -> Self {
        self.number_of_decimals = value;
        self
    }

    //////////////////////////////////////////////////////////////////////////////////////
    fn write_str(&self, value: &str, offset: usize, buffer: &mut [u8]) -> Option<usize> {
        if offset > buffer.len() {
            return None;
        }
        if value.len() == 0 {
            return Some(offset);
        }
        if value.len() > offset {
            return None;
        }
        // bitwise copy value into buffer[offset - value.len()]
        let pos = offset - value.len();
        buffer[pos..offset].copy_from_slice(value.as_bytes());
        Some(pos)
    }
    fn write_number<T: Copy + Add + PartialOrd + Ord + PartialEq + Eq + DivAssign + IntoU8 + Rem<Output = T> + From<u8>>(
        &self,
        mut value: T,
        offset: usize,
        buffer: &mut [u8],
    ) -> Option<usize> {
        if (offset > buffer.len()) || (offset == 0) {
            return None;
        }
        let mut pos = offset - 1;
        let mut digits = 0u8;

        let base: T = T::from(self.base);
        loop {
            let v: u8 = (value % base).into_u8();
            value /= base;
            if v < 10.into() {
                buffer[pos] = v + 48u8;
            } else {
                buffer[pos] = v + 55u8;
            }
            digits += 1;
            if value == 0.into() {
                break;
            }
            pos -= 1;
            if pos == 0 {
                return None;
            }
            if self.group_size > 0 {
                if digits % self.group_size == 0 {
                    buffer[pos] = self.separator_char;
                    pos -= 1;
                    if pos == 0 {
                        return None;
                    }
                }
            }
        }
        if digits < self.representation_digits {
            if pos == 0 {
                return None;
            }
            pos -= 1;
            if self.group_size > 0 {
                if digits % self.group_size == 0 {
                    buffer[pos] = self.separator_char;
                    pos -= 1;
                    if pos == 0 {
                        return None;
                    }
                }
            }
            loop {
                buffer[pos] = b'0';
                digits += 1;
                if digits == self.representation_digits {
                    break;
                }
                pos -= 1;
                if pos == 0 {
                    return None;
                }
                if self.group_size > 0 {
                    if digits % self.group_size == 0 {
                        buffer[pos] = self.separator_char;
                        pos -= 1;
                        if pos == 0 {
                            return None;
                        }
                    }
                }
            }
        }
        Some(pos)
    }
    fn write_fill_char(&self, offset: usize, buffer: &mut [u8]) -> Option<usize> {
        if offset >= buffer.len() {
            return None;
        }
        let w = self.width as usize;
        if self.width == 0 {
            return Some(offset);
        }
        if w > buffer.len() {
            return None;
        }
        let start_pos = buffer.len() - w;
        if start_pos >= offset {
            return Some(offset);
        }
        for i in start_pos..offset {
            buffer[i] = self.fill_char;
        }
        Some(start_pos)
    }

    #[inline(always)]
    fn add_buffer_to_string(&self, buffer: &[u8], prefix: &'static str, writer: &mut String) {
        let index = buffer.len();
        if self.group_size > 0 {
            if self.width > 0 {
                let fill_char = self.fill_char as char;
                let width = self.width as usize;
                let prefix_len = prefix.len();
                let buffer_len = index + (index - 1) / self.group_size as usize;
                let total_len = prefix_len + buffer_len;
                if width > total_len {
                    let fill_len = width - total_len;
                    for _ in 0..fill_len {
                        writer.push(fill_char);
                    }
                }
            }
            if !prefix.is_empty() {
                writer.push_str(prefix);
            }
            let mut group_index = self.group_size - (index % self.group_size as usize) as u8;
            if group_index == self.group_size {
                group_index = 0;
            }
            for i in (0..index).rev() {
                writer.push(buffer[i] as char);
                group_index += 1;
                if group_index == self.group_size {
                    if i > 0 {
                        writer.push(self.separator_char as char);
                    }
                    group_index = 0;
                }
            }
        } else {
            if self.width > 0 {
                let fill_char = self.fill_char as char;
                let width = self.width as usize;
                let prefix_len = prefix.len();
                let buffer_len = index;
                let total_len = prefix_len + buffer_len;
                if width > total_len {
                    let fill_len = width - total_len;
                    for _ in 0..fill_len {
                        writer.push(fill_char);
                    }
                }
            }
            if !prefix.is_empty() {
                writer.push_str(prefix);
            }
            for i in (0..index).rev() {
                writer.push(buffer[i] as char);
            }
        }
    }
    #[inline(always)]
    fn write_unsigned_dec(&self, value: u128, prefix: &'static str, writer: &mut String) {
        let mut buffer = [0u8; 40];
        let mut index = 0;
        let mut value = value;
        loop {
            let digit = (value % 10) as u8;
            value /= 10;
            buffer[index] = digit + 48;
            index += 1;
            if value == 0 {
                break;
            }
        }
        if self.representation_digits as usize > index {
            let mut fill = self.representation_digits as usize - index;
            while fill > 0 {
                buffer[index] = 48;
                index += 1;
                fill -= 1;
            }
        }
        self.add_buffer_to_string(&buffer[0..index], prefix, writer);
    }
    #[inline(always)]
    fn write_unsigned_hex(&self, value: u128, prefix: &'static str, writer: &mut String) {
        let mut buffer = [0u8; 40];
        let mut index = 0;
        let mut value = value;
        loop {
            let digit = (value % 16) as u8;
            value /= 16;
            buffer[index] = if digit < 10 { digit + 48 } else { digit + 55 };
            index += 1;
            if value == 0 {
                break;
            }
        }
        if self.representation_digits as usize > index {
            let mut fill = self.representation_digits as usize - index;
            while fill > 0 {
                buffer[index] = 48;
                index += 1;
                fill -= 1;
            }
        }
        self.add_buffer_to_string(&buffer[0..index], prefix, writer);
    }
    #[inline(always)]
    fn write_unsigned_bin(&self, value: u128, prefix: &'static str, writer: &mut String) {
        let mut buffer = [0u8; 132];
        let mut index = 0;
        let mut value = value;
        loop {
            let digit = (value % 2) as u8;
            value /= 2;
            buffer[index] = digit + 48;
            index += 1;
            if value == 0 {
                break;
            }
        }
        if self.representation_digits as usize > index {
            let mut fill = self.representation_digits as usize - index;
            while fill > 0 {
                buffer[index] = 48;
                index += 1;
                fill -= 1;
            }
        }
        self.add_buffer_to_string(&buffer[0..index], prefix, writer);
    }
    pub(crate) fn write_unsigned(&self, value: u128, writer: &mut String) {
        // match self.base {
        //     2 => self.write_unsigned_bin(value, "0b", writer),
        //     10 => self.write_unsigned_dec(value, "", writer),
        //     16 => self.write_unsigned_hex(value, "0x", writer),
        //     _ => {}
        // }
        let mut output: [u8;256] = [0;256];
        if let Some(str_rep) = self.write_u128(value, &mut output) {
            writer.push_str(str_rep);
        }
    }
    pub(crate) fn write_signed(&self, value: i128, writer: &mut String) {
        if value >= 0 {
            match self.base {
                2 => self.write_unsigned_bin(value as u128, "0b", writer),
                10 => self.write_unsigned_dec(value as u128, "", writer),
                16 => self.write_unsigned_hex(value as u128, "0x", writer),
                _ => {}
            }
        } else {
            match self.base {
                2 => self.write_unsigned_bin((-value) as u128, "-0b", writer),
                10 => self.write_unsigned_dec((-value) as u128, "-", writer),
                16 => self.write_unsigned_hex((-value) as u128, "-0x", writer),
                _ => {}
            }
        }
    }
    pub(crate) fn write_float(&self, value: f64, write: &mut String) {
        let mut buffer = [0u8; 40];
        let mut index = 0;
        let mut value = value;
        if value < 0.0 {
            value = -value;
            write.push('-');
        }
        let mut int_part = value.trunc() as i64;
        let mut fract_part = value.fract();
        loop {
            let digit = (int_part % 10) as u8;
            int_part /= 10;
            buffer[index] = digit + 48;
            index += 1;
            if int_part == 0 {
                break;
            }
        }
        if self.representation_digits as usize > index {
            let mut fill = self.representation_digits as usize - index;
            while fill > 0 {
                buffer[index] = 48;
                index += 1;
                fill -= 1;
            }
        }
        self.add_buffer_to_string(&buffer[0..index], "", write);
        if self.number_of_decimals > 0 {
            write.push('.');
            for _ in 0..self.number_of_decimals {
                fract_part *= 10.0;
                let digit = fract_part.trunc() as u8;
                fract_part = fract_part.fract();
                write.push((digit + 48) as char);
            }
        }
    }
    pub(crate) fn write_int64<'a>(&self, value: i64, buffer: &'a mut [u8]) -> Option<&'a str> {
        let len = buffer.len();
        if len == 0 {
            return None;
        }
        let negative;
        let value = if value < 0 {
            negative = true;
            -value as u64
        } else {
            negative = false;
            value as u64
        };
        let pos = self.write_str(self.suffix, len, buffer)?;
        let pos = self.write_number(value as u64, pos, buffer)?;
        let mut pos = self.write_str(&self.prefix, pos, buffer)?;
        if negative {
            if pos == 0 {
                return None;
            }
            pos -= 1;
            buffer[pos] = b'-';
        }
        let pos = self.write_fill_char(pos, buffer)?;
        Some(unsafe { std::str::from_utf8_unchecked(&buffer[pos..]) })
    }

    pub(crate) fn write_u64<'a>(&self, value: u64, buffer: &'a mut [u8]) -> Option<&'a str> {
        let len = buffer.len();
        if len == 0 {
            return None;
        }
        let pos = self.write_str(self.suffix, len, buffer)?;
        let pos = self.write_number(value, pos, buffer)?;
        let pos = self.write_str(&self.prefix, pos, buffer)?;
        let pos = self.write_fill_char(pos, buffer)?;
        Some(unsafe { std::str::from_utf8_unchecked(&buffer[pos..]) })
    }
    pub(crate) fn write_u128<'a>(&self, value: u128, buffer: &'a mut [u8]) -> Option<&'a str> {
        let len = buffer.len();
        if len == 0 {
            return None;
        }
        let pos = self.write_str(self.suffix, len, buffer)?;
        let pos = self.write_number(value, pos, buffer)?;
        let pos = self.write_str(&self.prefix, pos, buffer)?;
        let pos = self.write_fill_char(pos, buffer)?;
        Some(unsafe { std::str::from_utf8_unchecked(&buffer[pos..]) })
    }
}


trait IntoU8 {
    fn into_u8(self) -> u8;
}
impl IntoU8 for u64 {
    fn into_u8(self) -> u8 {
        self as u8
    }
}
impl IntoU8 for u128 {
    fn into_u8(self) -> u8 {
        self as u8
    }
}