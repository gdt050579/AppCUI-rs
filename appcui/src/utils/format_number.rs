pub(crate) struct FormatNumber {
    base: u8, // 2, 8, 10, 16
    group_size: u8,
    separator_char: u8,
    width: u8,
    fill_char: u8,
    number_of_decimals: u8,
}

impl FormatNumber {
    pub(crate) const fn new(base: u8, group_size: u8, separator_char: u8, width: u8, fill_char: u8, number_of_decimals: u8) -> Self {
        match base {
            2 | 8 | 10 | 16 => (),
            _ => panic!("Invalid base value for FormatNumber (expected 2, 8, 10 or 16)"),
        }
        match group_size {
            0 | 3 | 4 => (),
            _ => panic!("Invalid group size for FormatNumber (expected 0, 3 or 4)"),
        }
        if group_size == 0 {
            match separator_char {
                0 => (),
                _ => panic!("Invalid separator char for FormatNumber (expected 0) if group size is 0"),
            }
        } else {
            match separator_char {
                32..=126 => (),
                _ => panic!("Invalid separator char for FormatNumber (expected a printable ASCII character) if group size si bigger than 0"),
            }
        }
        if width == 0 {
            match fill_char {
                0 => (),
                _ => panic!("Invalid width for FormatNumber (expected 0 if width is 0)"),
            }
        } else {
            match fill_char {
                32..=126 => (),
                _ => panic!("Invalid fill char for FormatNumber (expected a printable ASCII character) if width is bigger than 0"),
            }
        }
        Self {
            base,
            group_size,
            separator_char,
            width,
            fill_char,
            number_of_decimals,
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
            let mut group_index = 0;
            let mut group_count = 0;
            for i in (0..index).rev() {
                writer.push(buffer[i] as char);
                group_index += 1;
                group_count += 1;
                if group_index == self.group_size {
                    writer.push(self.separator_char as char);
                    group_index = 0;
                }
            }
            if group_count > 0 {
                writer.pop();
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
    pub(crate) fn write_unsigned(&self, value: u128, writer: &mut String) {
        match self.base {
            10 => self.write_unsigned_dec(value, "", writer),
            _ => {}
        }
    }
    pub(crate) fn write_signed(&self, value: i128, writer: &mut String) {
        if value >= 0 {
            match self.base {
                10 => self.write_unsigned_dec(value as u128, "", writer),
                _ => {}
            }
        } else {
            match self.base {
                10 => self.write_unsigned_dec((-value) as u128, "-", writer),
                _ => {}
            }
        }
    }
}
