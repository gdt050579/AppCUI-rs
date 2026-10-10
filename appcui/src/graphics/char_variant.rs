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
            CharVariant::MathBold => todo!(),
            CharVariant::MathItalic => todo!(),
            CharVariant::MathBoldItalic => todo!(),
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
