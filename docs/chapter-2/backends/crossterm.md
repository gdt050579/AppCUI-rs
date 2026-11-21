# CrossTerm

This backend relies on the `crossterm` crate to provide a terminal abstraction layer. It supports various terminal features such as mouse input, keyboard input, and screen manipulation.

For clipboard operations, it uses the `copypasta` crate's built-in clipboard functionality for UNIX-like systems, and the Windows API on Windows.

## Palette

CrossTerm paints named colors with ANSI sequences, sent through the `crossterm` crate. The general rules for `.palette(...)` and `.palette_as_rgb(...)` are in [Palette](../application.md#palette).

| Palette             | What CrossTerm does                                        |
| ------------------- | ---------------------------------------------------------- |
| `Palette::Terminal` | Leaves the terminal colors unchanged. This is the default. |
| `Palette::BuiltIn`  | Uses the standard AppCUI colors.                           |
| `Palette::Custom`   | Uses the RGB values from the `ColorPalette`.               |

`.palette_as_rgb(...)` chooses how those values reach the terminal:

* `false` (the default). Characters use the 16 color indexes. `BuiltIn` and `Custom` are written into those indexes with an OSC 4 sequence (`]4;<index>;rgb:RR/GG/BB`).
* `true`. Each character is painted with a 24-bit RGB sequence (`38;2` / `48;2`) taken from the palette. The terminal's 16 color slots stay as they were. This requires `Palette::BuiltIn` or `Palette::Custom`.

When the application ends, CrossTerm sends OSC 104 (`]104`) if it changed the 16 color slots (`BuiltIn` or `Custom` together with `.palette_as_rgb(false)`). That sequence asks the terminal to restore its previous colors. With `.palette_as_rgb(true)` the slots stay unchanged, so CrossTerm skips the reset. OSC 104 is only a request: some terminals ignore it and keep the colors installed at startup. WSL is one example.

## Limitations

- Some flickering issues may be seen when using this backend (in particular for old terminals). This is due to the way `crossterm` handles terminal output and input. If you experience flickering, consider using a different backend or adjusting your terminal settings.

- **Palette reset**: restoring the 16 colors depends on the terminal. CrossTerm sends OSC 104 when the application ends, and some terminals, including WSL, leave the new colors in place. See [Palette](#palette).

## Usage

This backend is not the default on any operating system. Linux and macOS use Termios, and Windows uses the Windows Console. To compile it, enable the `CROSSTERM` feature in your `Cargo.toml` file:

```toml
[dependencies]
appcui = { version = "*", features = ["CROSSTERM"] }
```

Then, you can create your application using the `.backend(...)` method, specifying the `Type::CrossTerm` backend:

```rust
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .backend(appcui::backend::Type::CrossTerm)
        .window(|| window!("'Demo',a:c,w:30,h:9"))
        .run()
}
```