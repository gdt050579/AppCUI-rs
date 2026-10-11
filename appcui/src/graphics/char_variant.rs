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
    Parenthesized,
    Superscript,
    Subscript,
    SmallCaps,
    Fullwidth,
    Braille,
    Custom(fn(char) -> char),
}
/*
Variant	Mapping	Notes
Braille	a–z → the cells from the table above; digits → a–j cells	Irregular, needs a 26-entry lookup table. With char → char you can’t emit the ⠼ number sign or a capital sign, so digits and case are lossy.
FullWidth	'!'..='~' → c + 0xFEE0, space → U+3000	One offset covers all printable ASCII, but each character takes two terminal cells.
Circled	A–Z → U+24B6, a–z → U+24D0, 1–9 → U+2460, 0 → U+24EA	Zero is out of sequence.
Squared	A–Z → U+1F130	Uppercase only, so fold lowercase first.
NegativeCircled	A–Z → U+1F150	Uppercase only.
NegativeSquared	A–Z → U+1F170	Uppercase only; 🅰 🅱 🅾 🅿 render as emoji in many fonts.
Parenthesized	a–z → U+249C, 1–9 → U+2474	No zero.
Superscript / Subscript	Lookup table	Digits are complete (⁰–⁹, ₀–₉), but letters are partial: no superscript q, and only about half the subscript letters exist.
SmallCaps	a–z → ᴀʙᴄᴅᴇꜰɢʜɪᴊᴋʟᴍɴᴏᴘꞯʀꜱᴛᴜᴠᴡxʏᴢ	Lookup table; x has no small-cap form.
SegmentedDigits	0–9 → U+1FBF0	Seven-segment digits from Symbols for Legacy Computing; font support is still patchy.

*/

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
            CharVariant::CircledNegative => todo!(),
            CharVariant::Squared => todo!(),
            CharVariant::SquaredNegative => todo!(),
            CharVariant::Parenthesized => todo!(),
            CharVariant::Superscript => todo!(),
            CharVariant::Subscript => todo!(),
            CharVariant::SmallCaps => todo!(),
            CharVariant::Fullwidth => todo!(),
            CharVariant::Braille => todo!(),
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
