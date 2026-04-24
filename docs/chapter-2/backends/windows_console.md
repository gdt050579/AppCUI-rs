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

For clipboard based operations, it relies on the following APIs:
* OpenClipboard
* EmptyClipboard
* CloseClipboard
* SetClipboardData
* GetClipboardData
* IsClipboardFormatAvailable

**Remarks**: For this type of backend to work, there is no need for a 3rd party crate (everything is done via FFI and direct API calls).

## Limitations

Windows uses WTF-16 (which does not encode the full range of Unicode characters). While Unicode surrogates are supported, depending on the version of Windows, some characters (usually with a code higher than **0xFFFF**) might not be displayed accurately or may cause the line they appear on to shift left.


