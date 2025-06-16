# Windows Console

This backend replies on the following Windows API for various console related tasks:

| API                               | Task(s)                                                            |
| --------------------------------- | ------------------------------------------------------------------ |
| `GetStdHandle(...)`               | To gain access to **stdin** and **stdout**                         |
| `GetConsoleScreenBufferInfo(...)` | To get information about console size and position                 |
| `GetConsoleMode(...)`             | To get information about the current mode of the console           |
| `WriteConsoleOutputW(...)`        | To write a buffer of characters directly into the console          |
| `ReadConsoleInputW(...)`          | To read input events (keys, mouse, resizing, console closing)      |
| `SetConsoleTitleW(...)`           | To set the title (caption) of the console                          |
| `SetConsoleScreenBufferSize(...)` | To resize the console to a specific **width** and **heighr**       |
| `SetConsoleCursorInfo(...)`       | To move the caret (cursor) into a specific position of the console |

For clipboard based operations, it relies on the following APIs:
* OpenClipboard
* EmptyClipboard
* CloseClipboard
* SetClipboardData
* GetClipboardData
* IsClipboardFormatAvailable

**Remarks**: For this type of backend to work, there is no need for a 3rd party crate (everything is done via FFI and direct API calls).

## Limitations

Windows uses WTF-16 (that does not encode the full range of unicode characters). While unicode surrogates are supported, depending on the version of windows some characters (usually with a code higher than **0xFFFF**) might not be disply accurtely or my move the line they are down into to the left.


