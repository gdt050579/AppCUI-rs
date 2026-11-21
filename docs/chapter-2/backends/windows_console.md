# Windows Console

This backend relies on the following Windows APIs for various console-related tasks:

| API                               | Task(s)                                                            |
| --------------------------------- | ------------------------------------------------------------------ |
| `GetStdHandle(...)`               | To gain access to **stdin** and **stdout**                         |
| `GetConsoleScreenBufferInfo(...)` | To get information about console size and position                 |
| `GetConsoleMode(...)`             | To get information about the current mode of the console           |
| `WriteConsoleOutputW(...)`        | To write a buffer of characters directly into the console          |
| `ReadConsoleInputW(...)`          | To read input events (keys, mouse, resizing, console closing)      |
| `SetConsoleTitleW(...)`           | To set the title (caption) of the console                          |
| `SetConsoleScreenBufferSize(...)` | To resize the console to a specific **width** and **height**       |
| `SetConsoleCursorInfo(...)`       | To move the caret (cursor) into a specific position of the console |
| `SetConsoleScreenBufferInfoEx(...)` | To replace the console's 16-color table, and to restore it on exit |

For clipboard based operations, it relies on the following APIs:
* OpenClipboard
* EmptyClipboard
* CloseClipboard
* SetClipboardData
* GetClipboardData
* IsClipboardFormatAvailable

**Remarks**: For this type of backend to work, there is no need for a 3rd party crate (everything is done via FFI and direct API calls).

## Palette

Windows Console always paints with 16 colors. Each character stores a foreground index and a background index (`0`–`15`). Those indexes select slots in the console color table. The general rules for the two builder methods are in [Palette](../application.md#palette).

`.palette(...)` decides whether that table is replaced when the application starts. The call is `SetConsoleScreenBufferInfoEx`. When the application ends, the previous table is written back.

| Palette | What Windows Console does |
| ------- | ------------------------- |
| `Palette::Terminal` | Leaves the console color table unchanged. This is the default. |
| `Palette::BuiltIn` | Replaces the 16 slots with the standard AppCUI colors. |
| `Palette::Custom` | Writes the RGB values from the `ColorPalette` into the 16 slots. |

`.palette_as_rgb(...)` does not change how this backend draws. It never sends a 24-bit color for a character. `.palette_as_rgb(false)` is the setting that matches Windows Console. `.palette_as_rgb(true)` is still accepted when the palette is `BuiltIn` or `Custom`, and the characters are still drawn through the 16-color table. `.palette_as_rgb(true)` with `Palette::Terminal` is rejected before the backend starts, as described in [Palette](../application.md#palette).

## Limitations

- 24-bit RGB colors cannot be displayed. Every character is one of the 16 palette indexes. Enabling the `TRUE_COLORS` feature does not change this: a `Color::RGB` value is reduced to one of those indexes. `.palette_as_rgb(true)` does not turn on true color for this backend either.

- Windows uses **WTF-16** (which does not encode the full range of Unicode characters). While Unicode surrogates are supported, depending on the version of Windows, some characters (usually with a code higher than **0xFFFF**) might not be displayed accurately or may cause the line they appear on to shift left.


