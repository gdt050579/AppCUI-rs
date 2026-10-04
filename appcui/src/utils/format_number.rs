//! Describes how an integer or floating-point value is rendered as text.
//!
//! [`FormatNumber`] selects the numeric base, digit grouping, minimum width,
//! decimal places, prefix, and suffix. The `numericformat!` macro builds the
//! same value from a parameter string.

use core::panic;
use std::ops::{Add, Div, DivAssign, Mul, Rem, Sub};

// order
// suffix
// number (from right to left + groups)
// representation_digis
// prefix
// fill
/// Rendering rules for an integer or floating-point value.
///
/// A value is written from right to left as fill padding, an optional minus
/// sign, the prefix, the digits, the fractional part, and the suffix. For
/// `-1234.5` with a `$` prefix, two decimals, and a ` USD` suffix, the text
/// is `-$1234.50 USD`. Digit groups are separated from the right. Hexadecimal
/// digits above 9 are written as `A` through `F`.
///
/// Builders consume `self` and can be chained in a constant.
///
/// # Example
/// ```rust
/// use appcui::prelude::*;
///
/// const PRICE: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(2).prefix("$");
/// ```
pub struct FormatNumber {
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
    /// Writes `value` as a decimal number into `buf`, starting from the right.
    ///
    /// Returns the slice that holds the digits. When `value` has more digits than
    /// `buf`, the extra high-order digits are dropped. An empty buffer is returned
    /// unchanged.
    ///
    /// This method ignores any format stored in [`FormatNumber`].
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// let mut buf = [0u8; 8];
    /// assert_eq!(FormatNumber::write_to_buffer(1234, &mut buf), b"1234");
    /// ```
    pub fn write_to_buffer(value: u64, buf: &mut [u8]) -> &[u8] {
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
    /// Creates a format that writes digits in `base` with no grouping, padding, decimals, prefix, or suffix.
    ///
    /// # Panics
    ///
    /// Panics when `base` is not 2, 8, 10, or 16.
    #[inline(always)]
    pub const fn new(base: u8) -> Self {
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
    /// Groups digits from the right in blocks of `size`, separated by `separator`.
    ///
    /// `size` is `3` or `4`. Pass `0` for both arguments to disable grouping.
    /// `separator` is a printable ASCII byte, such as `b','`.
    ///
    /// # Panics
    ///
    /// Panics when `size` is not 0, 3, or 4. Panics when grouping is disabled and
    /// `separator` is not 0, or when grouping is enabled and `separator` is not a
    /// printable ASCII byte.
    #[inline(always)]
    pub const fn group(mut self, size: u8, separator: u8) -> Self {
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
    /// Pads the result on the left with `fill_char` until it is `size` bytes wide.
    ///
    /// Text that is already at least `size` bytes is left unchanged. Pass `0` for
    /// both arguments to disable padding. `fill_char` is a printable ASCII byte,
    /// such as `b' '`.
    ///
    /// # Panics
    ///
    /// Panics when padding is disabled and `fill_char` is not 0, or when padding
    /// is enabled and `fill_char` is not a printable ASCII byte.
    #[inline(always)]
    pub const fn fill(mut self, size: u8, fill_char: u8) -> Self {
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
    /// Writes at least `value` digits, padding with `0` on the left.
    ///
    /// Grouping applies to those zeros as well. The maximum depends on the base:
    /// 128 for base 2, 43 for base 8, 39 for base 10, and 32 for base 16.
    ///
    /// # Panics
    ///
    /// Panics when `value` is 0 or greater than the maximum for this format's base.
    #[inline(always)]
    pub const fn representation_digits(mut self, value: u8) -> Self {
        if value == 0 {
            panic!("Invalid number of representation digits for FormatNumber (expected a number greater than 0)");
        }
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
            16
                if value > 32 => {
                    panic!("Invalid number of representation digits for FormatNumber (maximum number of digits is 32 for base 16)");
                }
            _ => {}
        }
        self.representation_digits = value;
        self
    }

    /// Appends `suffix` after the digits.
    #[inline(always)]
    pub const fn suffix(mut self, suffix: &'static str) -> Self {
        self.suffix = suffix;
        self
    }
    /// Inserts `prefix` before the digits and after the minus sign.
    #[inline(always)]
    pub const fn prefix(mut self, prefix: &'static str) -> Self {
        self.prefix = prefix;
        self
    }
    /// Writes `value` digits after a decimal point.
    ///
    /// `0` omits the fractional part. The maximum is 8.
    ///
    /// # Panics
    ///
    /// Panics when `value` is greater than 8.
    pub const fn decimals(mut self, value: u8) -> Self {
        if value > 8 {
            panic!("Invalid number of decimals for FormatNumber (maximum number of decimals is 8)");
        }
        self.number_of_decimals = value;
        self
    }

    //////////////////////////////////////////////////////////////////////////////////////
    fn write_str(&self, value: &str, offset: usize, buffer: &mut [u8]) -> Option<usize> {
        if offset > buffer.len() {
            return None;
        }
        if value.is_empty() {
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
    fn write_integer_number<T: FormatableInteger>(&self, mut value: T, offset: usize, buffer: &mut [u8]) -> Option<usize> {
        if (offset > buffer.len()) || (offset == 0) {
            return None;
        }
        let mut pos = offset - 1;
        let mut digits = 0u8;

        let base: T = T::from(self.base);
        loop {
            let v = T::digit(value, base);
            value /= base;
            if v < 10 {
                buffer[pos] = v + 48u8;
            } else {
                buffer[pos] = v + 55u8;
            }
            digits += 1;
            if value == 0.into() {
                break;
            }
            if pos == 0 {
                return None;
            }
            pos -= 1;
            if self.group_size > 0 && digits.is_multiple_of(self.group_size) {
                buffer[pos] = self.separator_char;
                if pos == 0 {
                    return None;
                }
                pos -= 1;
            }
        }
        if digits < self.representation_digits {
            if pos == 0 {
                return None;
            }
            pos -= 1;
            if self.group_size > 0 && digits.is_multiple_of(self.group_size) {
                buffer[pos] = self.separator_char;
                if pos == 0 {
                    return None;
                }
                pos -= 1;
            }
            loop {
                buffer[pos] = b'0';
                digits += 1;
                if digits == self.representation_digits {
                    break;
                }
                if pos == 0 {
                    return None;
                }
                pos -= 1;

                if self.group_size > 0 && digits.is_multiple_of(self.group_size) {
                    buffer[pos] = self.separator_char;
                    if pos == 0 {
                        return None;
                    }
                    pos -= 1;
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
        for item in buffer.iter_mut().take(offset).skip(start_pos) {
            *item = self.fill_char;
        }
        Some(start_pos)
    }
    fn write_decimals(&self, value: u64, offset: usize, buffer: &mut [u8]) -> Option<usize> {
        if offset > buffer.len() {
            return None;
        }
        if self.number_of_decimals == 0 {
            return Some(offset);
        }
        if offset == 0 {
            return None;
        }
        if self.number_of_decimals as usize + 1 > offset {
            return None;
        }
        let mut buf: [u8; 32] = [0; 32];
        let mut count_digits = 0;
        let mut value = value;
        loop {
            buf[count_digits] = (value % 10) as u8 + 48;
            value /= 10;
            count_digits += 1;
            if value == 0 {
                break;
            }
        }
        let max_digits = self.number_of_decimals.min(count_digits as u8);
        let mut pos = offset - 1;
        let mut cnt = 0;
        while cnt < max_digits {
            buffer[pos] = buf[cnt as usize];
            pos -= 1;
            cnt += 1;
        }
        while cnt < self.number_of_decimals {
            buffer[pos] = b'0';
            pos -= 1;
            cnt += 1;
        }
        buffer[pos] = b'.';
        Some(pos)
    }

    pub(crate) fn write_float<'a>(&self, value: f64, output_buffer: &'a mut [u8]) -> Option<&'a str> {
        let len = output_buffer.len();
        if len == 0 {
            return None;
        }
        let negative = value.is_sign_negative();
        let int_part = value.trunc().abs() as u64;
        let decimans = if self.number_of_decimals == 0 {
            0
        } else {
            let factor = match self.number_of_decimals {
                1 => 10f64,
                2 => 100f64,
                3 => 1000f64,
                4 => 10000f64,
                5 => 100000f64,
                6 => 1000000f64,
                7 => 10000000f64,
                _ => 100000000f64,
            };
            (value.fract().abs() * factor) as u64
        };
        let pos = self.write_str(self.suffix, len, output_buffer)?;
        let pos = self.write_decimals(decimans, pos, output_buffer)?;
        let pos = self.write_integer_number(int_part, pos, output_buffer)?;
        let mut pos = self.write_str(self.prefix, pos, output_buffer)?;
        if negative {
            if pos == 0 {
                return None;
            }
            pos -= 1;
            output_buffer[pos] = b'-';
        }
        let pos = self.write_fill_char(pos, output_buffer)?;
        Some(unsafe { std::str::from_utf8_unchecked(&output_buffer[pos..]) })
    }

    pub(crate) fn write_number<'a, T: FormatableInteger>(&self, value: T, output_buffer: &'a mut [u8]) -> Option<&'a str> {
        let len = output_buffer.len();
        if len == 0 {
            return None;
        }
        let negative = value.is_negative();
        let value = value.abs_value();
        let pos = self.write_str(self.suffix, len, output_buffer)?;
        let pos = self.write_integer_number(value, pos, output_buffer)?;
        let mut pos = self.write_str(self.prefix, pos, output_buffer)?;
        if negative {
            if pos == 0 {
                return None;
            }
            pos -= 1;
            output_buffer[pos] = b'-';
        }
        let pos = self.write_fill_char(pos, output_buffer)?;
        Some(unsafe { std::str::from_utf8_unchecked(&output_buffer[pos..]) })
    }
    pub(crate) fn write_fraction<'a, T: FormatableInteger>(&self, value: T, devider: T, output_buffer: &'a mut [u8]) -> Option<&'a str> {
        let len = output_buffer.len();
        if (len == 0) || (devider == 0.into()) {
            return None;
        }
        let int_part: T = value / devider;
        let decimans = if self.number_of_decimals == 0 {
            0
        } else {
            let reminder: u64 = (value - (int_part * devider)).into_u64();
            let devider = devider.into_u64();
            let factor = match self.number_of_decimals {
                1 => 10u64,
                2 => 100u64,
                3 => 1000u64,
                4 => 10000u64,
                5 => 100000u64,
                6 => 1000000u64,
                7 => 10000000u64,
                _ => 100000000u64,
            };
            (reminder * factor) / devider
        };
        let negative = value.is_negative() != devider.is_negative();
        let value = int_part.abs_value();
        let pos = self.write_str(self.suffix, len, output_buffer)?;
        let pos = self.write_decimals(decimans, pos, output_buffer)?;
        let pos = self.write_integer_number(value, pos, output_buffer)?;
        let mut pos = self.write_str(self.prefix, pos, output_buffer)?;
        if negative {
            if pos == 0 {
                return None;
            }
            pos -= 1;
            output_buffer[pos] = b'-';
        }
        let pos = self.write_fill_char(pos, output_buffer)?;
        Some(unsafe { std::str::from_utf8_unchecked(&output_buffer[pos..]) })
    }
}

pub(crate) trait FormatableInteger:
    Copy + Add + PartialOrd + Ord + PartialEq + Eq + DivAssign + Rem + Div<Output = Self> + Mul<Output = Self> + Sub<Output = Self> + From<u8>
{
    fn abs_value(self) -> Self;
    fn is_negative(self) -> bool;
    fn digit(value: Self, divider: Self) -> u8;
    fn into_u64(self) -> u64;
}

macro_rules! IMPL_FOR_UNSIGNED {
    ($t: ty) => {
        impl FormatableInteger for $t {
            #[inline(always)]
            fn abs_value(self) -> Self {
                self
            }
            #[inline(always)]
            fn is_negative(self) -> bool {
                false
            }
            #[inline(always)]
            fn digit(value: Self, divider: Self) -> u8 {
                (value % divider) as u8
            }
            #[inline(always)]
            fn into_u64(self) -> u64 {
                self as u64
            }
        }
    };
}

macro_rules! IMPL_FOR_SIGNED {
    ($t: ty) => {
        impl FormatableInteger for $t {
            #[inline(always)]
            fn abs_value(self) -> Self {
                if self < 0 {
                    -self
                } else {
                    self
                }
            }
            #[inline(always)]
            fn is_negative(self) -> bool {
                self < 0
            }
            #[inline(always)]
            fn digit(value: Self, divider: Self) -> u8 {
                (value % divider) as u8
            }
            #[inline(always)]
            fn into_u64(self) -> u64 {
                if self < 0 {
                    -self as u64
                } else {
                    self as u64
                }
            }
        }
    };
}

IMPL_FOR_UNSIGNED!(u8);
IMPL_FOR_UNSIGNED!(u16);
IMPL_FOR_UNSIGNED!(u32);
IMPL_FOR_UNSIGNED!(u64);
IMPL_FOR_UNSIGNED!(u128);
IMPL_FOR_SIGNED!(i16);
IMPL_FOR_SIGNED!(i32);
IMPL_FOR_SIGNED!(i64);
IMPL_FOR_SIGNED!(i128);
