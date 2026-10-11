#[derive(Copy, Clone, Debug, Default)]
pub enum CharVariant {
    #[default]
    None,
    UpperCase,
    LowerCase,
    MathBold,
    MathItalic,
    MathBoldItalic,
    Script,
    BoldScript,
    Fraktur,
    BoldFraktur,
    DoubleStruck,
    SansSerif,
    SansSerifBold,
    SansSerifItalic,
    SansSerifBoldItalic,
    Monospace,
    Custom(fn(char) -> char),
}

impl CharVariant {
    pub(super) fn conversion_calback(&self) -> Option<fn(char) -> char> {
        match self {
            CharVariant::None => None,
            CharVariant::Custom(callback) => Some(*callback),
            CharVariant::UpperCase => Some(convert_upper_case),
            CharVariant::LowerCase => Some(convert_lower_case),
            CharVariant::MathBold => Some(convert_math_bold),
            CharVariant::MathItalic => Some(convert_math_italic),
            CharVariant::MathBoldItalic => Some(convert_math_bold_italic),
            CharVariant::Script => Some(convert_script),
            CharVariant::BoldScript => Some(convert_bold_script),
            CharVariant::Fraktur => Some(convert_fraktur),
            CharVariant::BoldFraktur => Some(convert_bold_fraktur),
            CharVariant::DoubleStruck => Some(convert_double_struck),
            CharVariant::SansSerif => Some(convert_sans_serif),
            CharVariant::SansSerifBold => Some(convert_sans_serif_bold),
            CharVariant::SansSerifItalic => Some(convert_sans_serif_italic),
            CharVariant::SansSerifBoldItalic => Some(convert_sans_serif_bold_italic),
            CharVariant::Monospace => Some(convert_monospace),
        }
    }
    #[inline(always)]
    pub fn map(&self, ch: char) -> char {
        if let Some(cbk) = self.conversion_calback() {
            cbk(ch)
        } else {
            ch
        }
    }
}

// conversion callbacks
#[inline]
fn convert_upper_case(c: char) -> char {
    c.to_uppercase().next().unwrap_or(c)
}

#[inline]
fn convert_lower_case(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

#[inline]
fn convert_math_bold(c: char) -> char {
    match c {
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D400 - 'A' as u32) },
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D41A - 'a' as u32) },
        '0'..='9' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D7CE - '0' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_math_italic(c: char) -> char {
    match c {
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D434 - 'A' as u32) },
        'h' => '\u{210E}',
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D44E - 'a' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_math_bold_italic(c: char) -> char {
    match c {
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D468 - 'A' as u32) },
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D482 - 'a' as u32) },
        '0'..='9' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D7CE - '0' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_script(c: char) -> char {
    match c {
        'A' => '𝒜',
        'B' => 'ℬ',
        'C' => '𝒞',
        'D' => '𝒟',
        'E' => 'ℰ',
        'F' => 'ℱ',
        'G' => '𝒢',
        'H' => 'ℋ',
        'I' => 'ℐ',
        'J' => '𝒥',
        'K' => '𝒦',
        'L' => 'ℒ',
        'M' => 'ℳ',
        'N' => '𝒩',
        'O' => '𝒪',
        'P' => '𝒫',
        'Q' => '𝒬',
        'R' => 'ℛ',
        'S' => '𝒮',
        'T' => '𝒯',
        'U' => '𝒰',
        'V' => '𝒱',
        'W' => '𝒲',
        'X' => '𝒳',
        'Y' => '𝒴',
        'Z' => '𝒵',
        // U+1D4BA, U+1D4BC and U+1D4C4 are unassigned; those letters live in Letterlike Symbols.
        'e' => 'ℯ',
        'g' => 'ℊ',
        'o' => 'ℴ',
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D4B6 - 'a' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_bold_script(c: char) -> char {
    match c {
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D4D0 - 'A' as u32) },
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D4EA - 'a' as u32) },
        '0'..='9' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D7CE - '0' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_fraktur(c: char) -> char {
    match c {
        // U+1D506, U+1D50B, U+1D50C, U+1D515 and U+1D51D are unassigned; those letters live in Letterlike Symbols.
        'C' => 'ℭ',
        'H' => 'ℌ',
        'I' => 'ℑ',
        'R' => 'ℜ',
        'Z' => 'ℨ',
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D504 - 'A' as u32) },
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D51E - 'a' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_bold_fraktur(c: char) -> char {
    match c {
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D56C - 'A' as u32) },
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D586 - 'a' as u32) },
        '0'..='9' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D7CE - '0' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_double_struck(c: char) -> char {
    match c {
        'C' => 'ℂ',
        'H' => 'ℍ',
        'N' => 'ℕ',
        'P' => 'ℙ',
        'Q' => 'ℚ',
        'R' => 'ℝ',
        'Z' => 'ℤ',
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D538 - 'A' as u32) },
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D552 - 'a' as u32) },
        '0'..='9' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D7D8 - '0' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_sans_serif(c: char) -> char {
    match c {
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D5A0 - 'A' as u32) },
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D5BA - 'a' as u32) },
        '0'..='9' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D7E2 - '0' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_sans_serif_bold(c: char) -> char {
    match c {
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D5D4 - 'A' as u32) },
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D5EE - 'a' as u32) },
        '0'..='9' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D7EC - '0' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_sans_serif_italic(c: char) -> char {
    match c {
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D608 - 'A' as u32) },
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D622 - 'a' as u32) },
        '0'..='9' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D7E2 - '0' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_sans_serif_bold_italic(c: char) -> char {
    match c {
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D63C - 'A' as u32) },
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D656 - 'a' as u32) },
        '0'..='9' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D7EC - '0' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_monospace(c: char) -> char {
    match c {
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D670 - 'A' as u32) },
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D68A - 'a' as u32) },
        '0'..='9' => unsafe { char::from_u32_unchecked((c as u32) + 0x1D7F6 - '0' as u32) },
        _ => c,
    }
}