# Windows VT (Virtual Terminal)

This backend is based on both Windows API and VT100 escape sequences.

For clipboard based operations, it relies on the following APIs:
* OpenClipboard
* EmptyClipboard
* CloseClipboard
* SetClipboardData
* GetClipboardData
* IsClipboardFormatAvailable

Input (mouse / keyboard / console resize) is handled by the following APIs:

| API                               | Task(s)                                                            |
| --------------------------------- | ------------------------------------------------------------------ |
| `GetStdHandle(...)`               | To gain access to **stdin** and **stdout**                         |
| `GetConsoleScreenBufferInfo(...)` | To get information about console size and position                 |
| `GetConsoleMode(...)`             | To get information about the current mode of the console           |
| `ReadConsoleInputW(...)`          | To read input events (keys, mouse, resizing, console closing)      |
| `SetConsoleTitleW(...)`           | To set the title (caption) of the console                          |
| `SetConsoleScreenBufferSize(...)` | To resize the console to a specific **width** and **heighr**       |

The output is done via VT100 escape sequences (please refer to [Wikipedia](https://en.wikipedia.org/wiki/ANSI_escape_code) for more information). This backend supports true colors (24 bits per pixel) and wide characters (2 bytes per character) but it depends on the Windows version to support them.

## Limitations:

Because of the way VT100 escape sequences work, the backend is much slower than a regular Windows Console backend (that renders the output directly into the console). If speed is a priority, it is recommended to use the Windows Console backend instead.

Keep in mind that the speed limitation can be mitigated by using a 3rd party terminal (that use the GPU to render the output)such as:
* [RIO](https://rioterm.com/)
* [Alacritty](https://alacritty.org/)

## Usage

Windows VT is not the default backend on Windows. To use it, you need to specify the `WindowsVT` backend type when creating the application:

```rust
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let app = App::with_terminal(appcui::backend::Type::WindowsVT).build()?;
    // build your application here
    Ok(())
}
```

Further more, if you also want to use `true colors` you will need to enable the `TRUE_COLORS` feature when building the application:

```toml
[dependencies]
appcui = { version = "*", features = ["TRUE_COLORS"] }
```






