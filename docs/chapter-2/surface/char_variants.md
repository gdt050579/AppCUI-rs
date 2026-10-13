# Char variants

A char variant is a transformation that is applied to every character of a text **at the moment it is written** on a surface. It allows you to display a text in a different style (bold, italic, script, circled, superscript, etc.) without having to change the text itself. The styled forms are regular Unicode characters (most of them from the *Mathematical Alphanumeric Symbols*, *Enclosed Alphanumerics* and *Phonetic Extensions* blocks), so they work in any terminal whose font contains those glyphs.

Char variants are described by the `CharVariant` enum:

```rust
pub enum CharVariant {
    None,                    // default - no conversion
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
```

## Usage

### With `write_text(...)`

The most common way to use a char variant is through the `TextFormatBuilder` (see [Text](text.md#write-a-formatted-text)). The `char_variant(...)` method sets the variant that will be applied to every character written by `write_text(...)`:

```rust
use appcui::prelude::*;

let mut surface = Surface::new(40, 5);
let format = TextFormatBuilder::new()
    .position(1, 1)
    .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkBlue))
    .align(TextAlignment::Left)
    .char_variant(CharVariant::MathBold)
    .build();
surface.write_text("Hello World 2026", &format);
// the surface now contains: 𝐇𝐞𝐥𝐥𝐨 𝐖𝐨𝐫𝐥𝐝 𝟐𝟎𝟐𝟔
```

The conversion is done **one character at a time** and always produces exactly one character, so:
* the number of characters (and therefore alignment, wrapping and hotkey positions) is the same as for the original text;
* the hotkey character (if any) is converted as well, but it keeps the hotkey attribute;
* the original string is never modified - only what is drawn on the surface changes.

The default value is `CharVariant::None`, meaning that the text is written as it is.

### With `map(...)`

A `CharVariant` can also be used directly, to convert a single character through the `map(...)` method:

```rust
pub fn map(&self, ch: char) -> char
```

This is useful if you want to build a converted string yourself (for example to use it with `write_string(...)` or to store it):

```rust
use appcui::prelude::*;

let v = CharVariant::Circled;
assert_eq!(v.map('A'), 'Ⓐ');
assert_eq!(v.map('7'), '⑦');
assert_eq!(v.map('!'), '!'); // characters without a variant are left unchanged

let text: String = "Hello".chars().map(|c| CharVariant::DoubleStruck.map(c)).collect();
assert_eq!(text, "ℍ𝕖𝕝𝕝𝕠");
```

### Custom variants

If none of the predefined variants fits your needs, `CharVariant::Custom` accepts any function with the signature `fn(char) -> char`:

```rust
use appcui::prelude::*;

fn leet(c: char) -> char {
    match c {
        'o' | 'O' => '0',
        'l' | 'L' => '1',
        'e' | 'E' => '3',
        _ => c,
    }
}

let format = TextFormatBuilder::new()
    .position(0, 0)
    .char_variant(CharVariant::Custom(leet))
    .build();
surface.write_text("Hello World", &format); // writes: H3110 W0r1d
```

## Translation rules

The following table describes how each variant translates characters. Any character that is **not** listed in the *Translation* column (punctuation, spaces, accented letters, other Unicode characters, etc.) is left **unchanged**.

| Variant               | Translation                                                                                                                                                        | Notes                                                                                                                                                                      |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `None`                | -                                                                                                                                                                  | No conversion (default).                                                                                                                                                   |
| `Uppercase`           | any letter → its upper case form                                                                                                                                   | Uses Rust's Unicode case mapping (works for non-ASCII letters as well, e.g. `ă` → `Ă`). Only the first character of the mapping is kept.                                   |
| `Lowercase`           | any letter → its lower case form                                                                                                                                   | Same as `Uppercase`, but for lower case.                                                                                                                                   |
| `MathBold`            | `A-Z` → 𝐀-𝐙, `a-z` → 𝐚-𝐳, `0-9` → 𝟎-𝟗                                                                                                                              |                                                                                                                                                                            |
| `MathItalic`          | `A-Z` → 𝐴-𝑍, `a-z` → 𝑎-𝑧                                                                                                                                           | `h` → `ℎ` (Planck constant), because the regular italic `h` slot is not assigned in Unicode. Digits are not converted.                                                     |
| `MathBoldItalic`      | `A-Z` → 𝑨-𝒁, `a-z` → 𝒂-𝒛, `0-9` → 𝟎-𝟗                                                                                                                              | There are no bold-italic digits, so digits use the **bold** form.                                                                                                          |
| `Script`              | `A-Z` → 𝒜-𝒵, `a-z` → 𝒶-𝓏                                                                                                                                           | `B E F H I L M R` → `ℬ ℰ ℱ ℋ ℐ ℒ ℳ ℛ` and `e g o` → `ℯ ℊ ℴ` (taken from the *Letterlike Symbols* block). Digits are not converted.                                         |
| `ScriptBold`          | `A-Z` → 𝓐-𝓩, `a-z` → 𝓪-𝔃, `0-9` → 𝟎-𝟗                                                                                                                              | Digits use the **bold** form.                                                                                                                                              |
| `Fraktur`             | `A-Z` → 𝔄-ℨ, `a-z` → 𝔞-𝔷                                                                                                                                           | `C H I R Z` → `ℭ ℌ ℑ ℜ ℨ` (taken from the *Letterlike Symbols* block). Digits are not converted.                                                                           |
| `FrakturBold`         | `A-Z` → 𝕬-𝖅, `a-z` → 𝖆-𝖟, `0-9` → 𝟎-𝟗                                                                                                                              | Digits use the **bold** form.                                                                                                                                              |
| `DoubleStruck`        | `A-Z` → 𝔸-ℤ, `a-z` → 𝕒-𝕫, `0-9` → 𝟘-𝟡                                                                                                                              | `C H N P Q R Z` → `ℂ ℍ ℕ ℙ ℚ ℝ ℤ` (taken from the *Letterlike Symbols* block).                                                                                             |
| `SansSerif`           | `A-Z` → 𝖠-𝖹, `a-z` → 𝖺-𝗓, `0-9` → 𝟢-𝟫                                                                                                                              |                                                                                                                                                                            |
| `SansSerifBold`       | `A-Z` → 𝗔-𝗭, `a-z` → 𝗮-𝘇, `0-9` → 𝟬-𝟵                                                                                                                              |                                                                                                                                                                            |
| `SansSerifItalic`     | `A-Z` → 𝘈-𝘡, `a-z` → 𝘢-𝘻, `0-9` → 𝟢-𝟫                                                                                                                              | There are no sans-serif italic digits, so digits use the **sans-serif** form.                                                                                              |
| `SansSerifBoldItalic` | `A-Z` → 𝘼-𝙕, `a-z` → 𝙖-𝙯, `0-9` → 𝟬-𝟵                                                                                                                              | Digits use the **sans-serif bold** form.                                                                                                                                   |
| `Monospace`           | `A-Z` → 𝙰-𝚉, `a-z` → 𝚊-𝚣, `0-9` → 𝟶-𝟿                                                                                                                              |                                                                                                                                                                            |
| `Circled`             | `A-Z` → Ⓐ-Ⓩ, `a-z` → ⓐ-ⓩ, `1-9` → ①-⑨, `0` → ⓪                                                                                                                     |                                                                                                                                                                            |
| `CircledNegative`     | `A-Z` → 🅐-🅩, `a-z` → 🅐-🅩, `1-9` → ❶-❾, `0` → ⓿                                                                                                                     | Only upper case letters exist, so lower case letters are converted to their upper case form first.                                                                         |
| `Squared`             | `A-Z` → 🄰-🅉, `a-z` → 🄰-🅉                                                                                                                                           | Only upper case letters exist (lower case is converted to upper case first). Digits are not converted.                                                                     |
| `SquaredNegative`     | `A-Z` → 🅰-🆉, `a-z` → 🅰-🆉                                                                                                                                           | Only upper case letters exist (lower case is converted to upper case first). Digits are not converted. Some fonts render 🅰 🅱 🅾 🅿 as emoji (two cells wide).                |
| `Superscript`         | `A-Z` → ᴬ ᴮ ᶜ ᴰ ᴱ ᶠ ᴳ ᴴ ᴵ ᴶ ᴷ ᴸ ᴹ ᴺ ᴼ ᴾ Q ᴿ ˢ ᵀ ᵁ ⱽ ᵂ ˣ ʸ ᶻ<br>`a-z` → ᵃ ᵇ ᶜ ᵈ ᵉ ᶠ ᵍ ʰ ⁱ ʲ ᵏ ˡ ᵐ ⁿ ᵒ ᵖ q ʳ ˢ ᵗ ᵘ ᵛ ʷ ˣ ʸ ᶻ<br>`0-9` → ⁰-⁹, `+ - = ( )` → ⁺ ⁻ ⁼ ⁽ ⁾ | `C F S X Y Z` have no superscript capital, so the lower case superscript form is used. `Q` and `q` have no superscript form and are left unchanged.                        |
| `Subscript`           | `a e h i j k l m n o p r s t u v x` (and their upper case forms) → ₐ ₑ ₕ ᵢ ⱼ ₖ ₗ ₘ ₙ ₒ ₚ ᵣ ₛ ₜ ᵤ ᵥ ₓ<br>`0-9` → ₀-₉, `+ - = ( )` → ₊ ₋ ₌ ₍ ₎                       | Unicode has no upper case subscript letters and only some lower case ones, so upper case is mapped to the lower case subscript and `b c d f g q w y z` are left unchanged. |
| `SmallCaps`           | `a-z` (and their upper case forms) → ᴀ ʙ ᴄ ᴅ ᴇ ꜰ ɢ ʜ ɪ ᴊ ᴋ ʟ ᴍ ɴ ᴏ ᴘ ꞯ ʀ ꜱ ᴛ ᴜ ᴠ ᴡ x ʏ ᴢ                                                                           | There is no small capital `x`, so `x`/`X` are left unchanged. Currently `T` (upper case) is not converted - only `t` is.                                                   |
| `Custom(f)`           | every character `c` → `f(c)`                                                                                                                                       | The function is called for every character that is written.                                                                                                                |

## Examples

The following table shows how the text `Hello World 2026` is displayed with each variant:

| Variant               | Result           |
| --------------------- | ---------------- |
| `None`                | Hello World 2026 |
| `Uppercase`           | HELLO WORLD 2026 |
| `Lowercase`           | hello world 2026 |
| `MathBold`            | 𝐇𝐞𝐥𝐥𝐨 𝐖𝐨𝐫𝐥𝐝 𝟐𝟎𝟐𝟔 |
| `MathItalic`          | 𝐻𝑒𝑙𝑙𝑜 𝑊𝑜𝑟𝑙𝑑 2026 |
| `MathBoldItalic`      | 𝑯𝒆𝒍𝒍𝒐 𝑾𝒐𝒓𝒍𝒅 𝟐𝟎𝟐𝟔 |
| `Script`              | ℋℯ𝓁𝓁ℴ 𝒲ℴ𝓇𝓁𝒹 2026 |
| `ScriptBold`          | 𝓗𝓮𝓵𝓵𝓸 𝓦𝓸𝓻𝓵𝓭 𝟐𝟎𝟐𝟔 |
| `Fraktur`             | ℌ𝔢𝔩𝔩𝔬 𝔚𝔬𝔯𝔩𝔡 2026 |
| `FrakturBold`         | 𝕳𝖊𝖑𝖑𝖔 𝖂𝖔𝖗𝖑𝖉 𝟐𝟎𝟐𝟔 |
| `DoubleStruck`        | ℍ𝕖𝕝𝕝𝕠 𝕎𝕠𝕣𝕝𝕕 𝟚𝟘𝟚𝟞 |
| `SansSerif`           | 𝖧𝖾𝗅𝗅𝗈 𝖶𝗈𝗋𝗅𝖽 𝟤𝟢𝟤𝟨 |
| `SansSerifBold`       | 𝗛𝗲𝗹𝗹𝗼 𝗪𝗼𝗿𝗹𝗱 𝟮𝟬𝟮𝟲 |
| `SansSerifItalic`     | 𝘏𝘦𝘭𝘭𝘰 𝘞𝘰𝘳𝘭𝘥 𝟤𝟢𝟤𝟨 |
| `SansSerifBoldItalic` | 𝙃𝙚𝙡𝙡𝙤 𝙒𝙤𝙧𝙡𝙙 𝟮𝟬𝟮𝟲 |
| `Monospace`           | 𝙷𝚎𝚕𝚕𝚘 𝚆𝚘𝚛𝚕𝚍 𝟸𝟶𝟸𝟼 |
| `Circled`             | Ⓗⓔⓛⓛⓞ Ⓦⓞⓡⓛⓓ ②⓪②⑥ |
| `CircledNegative`     | 🅗🅔🅛🅛🅞 🅦🅞🅡🅛🅓 ❷⓿❷❻ |
| `Squared`             | 🄷🄴🄻🄻🄾 🅆🄾🅁🄻🄳 2026 |
| `SquaredNegative`     | 🅷🅴🅻🅻🅾 🆆🅾🆁🅻🅳 2026 |
| `Superscript`         | ᴴᵉˡˡᵒ ᵂᵒʳˡᵈ ²⁰²⁶ |
| `Subscript`           | ₕₑₗₗₒ Wₒᵣₗd ₂₀₂₆ |
| `SmallCaps`           | ʜᴇʟʟᴏ ᴡᴏʀʟᴅ 2026 |

### Writing the same text with multiple variants

The following example writes `Hello World 2026` on several lines of a surface, each one using a different variant:

```rust
use appcui::prelude::*;

let variants = [
    ("None", CharVariant::None),
    ("MathBold", CharVariant::MathBold),
    ("Script", CharVariant::Script),
    ("Fraktur", CharVariant::Fraktur),
    ("DoubleStruck", CharVariant::DoubleStruck),
    ("Circled", CharVariant::Circled),
    ("Superscript", CharVariant::Superscript),
    ("SmallCaps", CharVariant::SmallCaps),
];
let mut surface = Surface::new(40, variants.len() as u32);
for (index, (name, variant)) in variants.iter().enumerate() {
    // the name of the variant (written as it is)
    surface.write_string(0, index as i32, name, CharAttribute::with_fore_color(Color::Gray), false);
    // the text, converted with the current variant
    let format = TextFormatBuilder::new()
        .position(16, index as i32)
        .attribute(CharAttribute::with_color(Color::Yellow, Color::DarkBlue))
        .char_variant(*variant)
        .build();
    surface.write_text("Hello World 2026", &format);
}
```

The result will look like this:

```text
None            Hello World 2026
MathBold        𝐇𝐞𝐥𝐥𝐨 𝐖𝐨𝐫𝐥𝐝 𝟐𝟎𝟐𝟔
Script          ℋℯ𝓁𝓁ℴ 𝒲ℴ𝓇𝓁𝒹 2026
Fraktur         ℌ𝔢𝔩𝔩𝔬 𝔚𝔬𝔯𝔩𝔡 2026
DoubleStruck    ℍ𝕖𝕝𝕝𝕠 𝕎𝕠𝕣𝕝𝕕 𝟚𝟘𝟚𝟞
Circled         Ⓗⓔⓛⓛⓞ Ⓦⓞⓡⓛⓓ ②⓪②⑥
Superscript     ᴴᵉˡˡᵒ ᵂᵒʳˡᵈ ²⁰²⁶
SmallCaps       ʜᴇʟʟᴏ ᴡᴏʀʟᴅ 2026
```

## Remarks

* Char variants are purely visual: the converted characters are different Unicode code points, so text written this way should not be used for copy/paste, search or accessibility purposes.
* Font support differs between terminals. The *Mathematical Alphanumeric Symbols* (`MathBold`, `Script`, `Fraktur`, `DoubleStruck`, `SansSerif*`, `Monospace`) are widely supported, while some of the *Enclosed Alphanumeric Supplement* characters (`CircledNegative`, `Squared`, `SquaredNegative`) may be missing or rendered as wide emoji.
* Because the translation is done per character, variants that do not have a form for a specific character (for example `Subscript` for `b`, or `Squared` for digits) simply leave that character as it is.
