#[derive(Copy, Clone, Debug, Default)]
pub enum CharVariant {
    #[default]
    None,
    Uppercase,
    Lowercase,
    MathBold,
    MathItalic,
    MathBoldItalic,
    Script,
    ScriptBold,
    Fraktur,
    FrakturBold,
    DoubleStruck,
    SansSerif,
    SansSerifBold,
    SansSerifItalic,
    SansSerifBoldItalic,
    Monospace,
    Circled,
    CircledNegative,
    Squared,
    SquaredNegative,
    Superscript,
    Subscript,
    SmallCaps,
    Custom(fn(char) -> char),
}

impl CharVariant {
    pub(super) fn conversion_callback(&self) -> Option<fn(char) -> char> {
        match self {
            CharVariant::None => None,
            CharVariant::Custom(callback) => Some(*callback),
            CharVariant::Uppercase => Some(convert_upper_case),
            CharVariant::Lowercase => Some(convert_lower_case),
            CharVariant::MathBold => Some(convert_math_bold),
            CharVariant::MathItalic => Some(convert_math_italic),
            CharVariant::MathBoldItalic => Some(convert_math_bold_italic),
            CharVariant::Script => Some(convert_script),
            CharVariant::ScriptBold => Some(convert_bold_script),
            CharVariant::Fraktur => Some(convert_fraktur),
            CharVariant::FrakturBold => Some(convert_bold_fraktur),
            CharVariant::DoubleStruck => Some(convert_double_struck),
            CharVariant::SansSerif => Some(convert_sans_serif),
            CharVariant::SansSerifBold => Some(convert_sans_serif_bold),
            CharVariant::SansSerifItalic => Some(convert_sans_serif_italic),
            CharVariant::SansSerifBoldItalic => Some(convert_sans_serif_bold_italic),
            CharVariant::Monospace => Some(convert_monospace),
            CharVariant::Circled => Some(convert_circled),
            CharVariant::CircledNegative => Some(convert_circled_negative),
            CharVariant::Squared => Some(convert_squared),
            CharVariant::SquaredNegative => Some(convert_squared_negative),
            CharVariant::Superscript => Some(convert_superscript),
            CharVariant::Subscript => Some(convert_subscript),
            CharVariant::SmallCaps => Some(convert_small_caps),
        }
    }
    #[inline(always)]
    pub fn map(&self, ch: char) -> char {
        if let Some(cbk) = self.conversion_callback() {
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

#[inline]
fn convert_circled(c: char) -> char {
    match c {
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x24B6 - 'A' as u32) },
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x24D0 - 'a' as u32) },
        '1'..='9' => unsafe { char::from_u32_unchecked((c as u32) + 0x2460 - '1' as u32) },
        // Circled zero is U+24EA, outside the ①–⑨ run.
        '0' => '\u{24EA}',
        _ => c,
    }
}

#[inline]
fn convert_circled_negative(c: char) -> char {
    match c {
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1F150 - 'A' as u32) },
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1F150 - 'a' as u32) },
        '1'..='9' => unsafe { char::from_u32_unchecked((c as u32) + 0x2776 - '1' as u32) },
        // Negative circled zero is U+24FF, outside the ❶–❾ run.
        '0' => '\u{24FF}',
        _ => c,
    }
}

#[inline]
fn convert_squared(c: char) -> char {
    match c {
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1F130 - 'A' as u32) },
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1F130 - 'a' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_squared_negative(c: char) -> char {
    match c {
        'A'..='Z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1F170 - 'A' as u32) },
        'a'..='z' => unsafe { char::from_u32_unchecked((c as u32) + 0x1F170 - 'a' as u32) },
        _ => c,
    }
}

#[inline]
fn convert_superscript(c: char) -> char {
    match c {
        // There are no superscript forms for C, F, Q, S, X, Y and Z (other than the poorly supported
        // U+A7F2..U+A7F4), so those letters fall back to their lower case superscript form.
        'A' => 'ᴬ',
        'B' => 'ᴮ',
        'C' => 'ᶜ',
        'D' => 'ᴰ',
        'E' => 'ᴱ',
        'F' => 'ᶠ',
        'G' => 'ᴳ',
        'H' => 'ᴴ',
        'I' => 'ᴵ',
        'J' => 'ᴶ',
        'K' => 'ᴷ',
        'L' => 'ᴸ',
        'M' => 'ᴹ',
        'N' => 'ᴺ',
        'O' => 'ᴼ',
        'P' => 'ᴾ',
        'R' => 'ᴿ',
        'S' => 'ˢ',
        'T' => 'ᵀ',
        'U' => 'ᵁ',
        'V' => 'ⱽ',
        'W' => 'ᵂ',
        'X' => 'ˣ',
        'Y' => 'ʸ',
        'Z' => 'ᶻ',
        'a' => 'ᵃ',
        'b' => 'ᵇ',
        'c' => 'ᶜ',
        'd' => 'ᵈ',
        'e' => 'ᵉ',
        'f' => 'ᶠ',
        'g' => 'ᵍ',
        'h' => 'ʰ',
        'i' => 'ⁱ',
        'j' => 'ʲ',
        'k' => 'ᵏ',
        'l' => 'ˡ',
        'm' => 'ᵐ',
        'n' => 'ⁿ',
        'o' => 'ᵒ',
        'p' => 'ᵖ',
        // no superscript q
        'r' => 'ʳ',
        's' => 'ˢ',
        't' => 'ᵗ',
        'u' => 'ᵘ',
        'v' => 'ᵛ',
        'w' => 'ʷ',
        'x' => 'ˣ',
        'y' => 'ʸ',
        'z' => 'ᶻ',
        // ¹, ² and ³ live in Latin-1, outside the ⁰, ⁴–⁹ run.
        '0' => '⁰',
        '1' => '¹',
        '2' => '²',
        '3' => '³',
        '4'..='9' => unsafe { char::from_u32_unchecked((c as u32) + 0x2074 - '4' as u32) },
        '+' => '⁺',
        '-' => '⁻',
        '=' => '⁼',
        '(' => '⁽',
        ')' => '⁾',
        _ => c,
    }
}

#[inline]
fn convert_subscript(c: char) -> char {
    match c {
        // There are no upper case subscript letters and only some lower case ones exist,
        // so upper case is converted to lower case first and the missing letters are left unchanged.
        'A' | 'a' => 'ₐ',
        'E' | 'e' => 'ₑ',
        'H' | 'h' => 'ₕ',
        'I' | 'i' => 'ᵢ',
        'J' | 'j' => 'ⱼ',
        'K' | 'k' => 'ₖ',
        'L' | 'l' => 'ₗ',
        'M' | 'm' => 'ₘ',
        'N' | 'n' => 'ₙ',
        'O' | 'o' => 'ₒ',
        'P' | 'p' => 'ₚ',
        'R' | 'r' => 'ᵣ',
        'S' | 's' => 'ₛ',
        'T' | 't' => 'ₜ',
        'U' | 'u' => 'ᵤ',
        'V' | 'v' => 'ᵥ',
        'X' | 'x' => 'ₓ',
        '0'..='9' => unsafe { char::from_u32_unchecked((c as u32) + 0x2080 - '0' as u32) },
        '+' => '₊',
        '-' => '₋',
        '=' => '₌',
        '(' => '₍',
        ')' => '₎',
        _ => c,
    }
}

#[inline]
fn convert_small_caps(c: char) -> char {
    match c {
        'A'|'a' => 'ᴀ',
        'B'|'b' => 'ʙ',
        'C'|'c' => 'ᴄ',
        'D'|'d' => 'ᴅ',
        'E'|'e' => 'ᴇ',
        'F'|'f' => 'ꜰ',
        'G'|'g' => 'ɢ',
        'H'|'h' => 'ʜ',
        'I'|'i' => 'ɪ',
        'J'|'j' => 'ᴊ',
        'K'|'k' => 'ᴋ',
        'L'|'l' => 'ʟ',
        'M'|'m' => 'ᴍ',
        'N'|'n' => 'ɴ',
        'O'|'o' => 'ᴏ',
        'P'|'p' => 'ᴘ',
        'Q'|'q' => 'ꞯ',
        'R'|'r' => 'ʀ',
        'S'|'s' => 'ꜱ',
        't' => 'ᴛ',
        'U'|'u' => 'ᴜ',
        'V'|'v' => 'ᴠ',
        'W'|'w' => 'ᴡ',
        // no small capital x
        'Y'|'y' => 'ʏ',
        'Z'|'z' => 'ᴢ',
        _ => c,
    }
}

