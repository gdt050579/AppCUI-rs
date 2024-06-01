pub(crate) struct FormatNumber {
    base: u8, // 2, 8, 10, 16
    group_size: u8,
    separator_char: u8,
    width: u8,
    fill_char: u8,
    number_of_decimals: u8,
}

impl FormatNumber {
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
        }
    }
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
    pub(crate) const fn decimals(mut self, value: u8) -> Self {
        self.number_of_decimals = value;
        self
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
        self.add_buffer_to_string(&buffer[0..index], prefix, writer);
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
