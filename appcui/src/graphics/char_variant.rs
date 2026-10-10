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
    Sans,
    SansBold,
    SansItalic,
    SansBoldItalic,
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
            CharVariant::Script => todo!(),
            CharVariant::BoldScript => todo!(),
            CharVariant::Fraktur => todo!(),
            CharVariant::BoldFraktur => todo!(),
            CharVariant::DoubleStruck => todo!(),
            CharVariant::Sans => todo!(),
            CharVariant::SansBold => todo!(),
            CharVariant::SansItalic => todo!(),
            CharVariant::SansBoldItalic => todo!(),
            CharVariant::Monospace => todo!(),
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
        'A'..='Z' => unsafe {char::from_u32_unchecked((c as u32) + 0x1D400 - 'A' as u32) },
        'a'..='z' => unsafe {char::from_u32_unchecked((c as u32) + 0x1D41A - 'a' as u32) },
        '0'..='9' => unsafe {char::from_u32_unchecked((c as u32) + 0x1D7CE - '0' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_math_italic(c: char) -> char {
    match c {
        'A'..='Z' => unsafe {char::from_u32_unchecked((c as u32) + 0x1D434 - 'A' as u32) },
        'h' => '\u{210E}',
        'a'..='z' => unsafe {char::from_u32_unchecked((c as u32) + 0x1D44E - 'a' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_math_bold_italic(c: char) -> char {
    match c {
        'A'..='Z' => unsafe {char::from_u32_unchecked((c as u32) + 0x1D468 - 'A' as u32) },
        'a'..='z' => unsafe {char::from_u32_unchecked((c as u32) + 0x1D482 - 'a' as u32) },
        '0'..='9' => unsafe {char::from_u32_unchecked((c as u32) + 0x1D7CE - '0' as u32) },
        _ => c,
    }
}
