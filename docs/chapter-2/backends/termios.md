# Termios

## Summary
Termios is a low-level library for terminal manipulation on UNIX systems. AppCUI uses it for terminal input and output on Linux and macOS, and it is the default backend on both.

## Implementation Details
When a `TermiosTerminal` instance is initialized, the terminal is configured to operate in *raw mode*. This mode allows the application to capture individual key press events directly, bypassing line buffering and other default terminal behaviors. To facilitate advanced input handling, a specific byte sequence is sent to `stdout` to enable mouse events, allowing AppCUI to handle mouse interactions.

To adapt to dynamic terminal conditions, a signal handler is set up to monitor window resize events (`SIGWINCH`). This ensures that the terminal layout is updated appropriately when the terminal window's dimensions change.

At the lowest level, the implementation involves reading one or more bytes directly from `stdin`. This is performed using a blocking read operation on a separate thread to avoid interfering with the application's main execution flow. These bytes are then interpreted into a `SystemEvent`, which is dispatched to the runtime manager for further processing.

## Palette

Termios paints named colors with ANSI sequences. The general rules for `.palette(...)` and `.palette_as_rgb(...)` are in [Palette](../application.md#palette).

| Palette | What Termios does |
| ------- | ----------------- |
| `Palette::Terminal` | Leaves the terminal colors unchanged. This is the default. |
| `Palette::BuiltIn` | Uses the standard AppCUI colors. |
| `Palette::Custom` | Uses the RGB values from the `ColorPalette`. |

`.palette_as_rgb(...)` chooses how those values reach the terminal:

* `false` (the default). Characters use the 16 color indexes. `BuiltIn` and `Custom` are written into those indexes with an OSC 4 sequence (`]4;<index>;rgb:RR/GG/BB`).
* `true`. Each character is painted with a 24-bit RGB sequence (`38;2` / `48;2`) taken from the palette. The terminal's 16 color slots stay as they were. This requires `Palette::BuiltIn` or `Palette::Custom`.

When the application ends, Termios sends OSC 104 (`]104`) if it changed the 16 color slots (`BuiltIn` or `Custom` together with `.palette_as_rgb(false)`). That sequence asks the terminal to restore its previous colors. With `.palette_as_rgb(true)` the slots stay unchanged, so Termios skips the reset.

## Limitations
- **Screen flickering**: Screen updates may cause flickering because the screen content is not flushed all at once. We have not yet found a solution for this issue.
- **Palette reset**: OSC 104 is only a request. Putting the previous colors back depends on the terminal. Some terminals ignore the sequence and keep the colors installed at startup. WSL is one example: after the application exits, the console can stay on the palette AppCUI set.
- **Key Combination Limitations**: Certain key combinations on macOS cannot be uniquely identified due to limitations in the terminal's input byte encoding. For example:
    - `Enter`, `Command + Enter`, `Option + Enter`, and `Control + Enter` all produce the same byte sequence
    - `Control + H` and `Control + Backspace` produce conflicting byte sequences

## TODO
The following features are still missing from the Termios implementation:
- **Key mapping**: Arrows, Home, End, Page Up, Page Down, Insert, Delete, Tab, and F1–F12 are mapped, including Shift, Alt, and Ctrl on those sequences. Letters and digits are mapped, including Alt from an ESC prefix and the common macOS Option characters on a US layout. Still open:
    - `Ctrl+Shift` and `Ctrl+Alt+Shift` on a letter. The terminal sends the same byte as `Ctrl` and `Ctrl+Alt`. Termios does not enable a keyboard protocol that would separate them.
    - Sequences outside that set are dropped. `Ctrl+6` (`0x1E`) and SS3 arrows (`ESC O A` through `ESC O D`) are in that group. F1–F4 sent as SS3 are mapped.
    - macOS Option combinations outside the US-layout table are dropped.
- **Mouse wheel**: Wheel reports are recognized and discarded, so scrolling is not delivered as a mouse event.
