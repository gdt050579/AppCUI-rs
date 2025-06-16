# Backends

AppCUI supports various backends (but each one comes with advantages and drawbacks).
A backend is the terminal that takes the information (characters) from the virtual screen of AppCUI and displays them.

<img src="img/backend.png" width=400/>

Each backend supported by AppCUI has the following properties:
* **Output rendering** - each character from the AppCUI surface is display on the screen
* **Input reading** - the backend is capable of identifying keyboard and mouse events and convert them to internal AppCUI events
* **Clipboard support** - the backend interacts with the OS and provides functionality for Copy / Cut / Paste based on OS-es API



## OS Support

| OS      | Windows Console | Windows VT | NCurses | Termios | Web Terminal |
| ------- | --------------- | ---------- | ------- | ------- | ------------ |
| Windows | Yes             | Yes        | -       | -       | -            |
| Linux   | -               | -          | Yes     | Yes     | -            |
| Mac/OSX | -               | -          | Yes     | Yes     | -            |
| Web     | -               | -          | -       | -       | Yes          |


## Display

| Display       | Windows Console     | Windows VT  | NCurses             | Termios             | Web Terminal        |
| ------------- | ------------------- | ----------- | ------------------- | ------------------- | ------------------- |
| Colors        | 16 (fore),16 (back) | True colors | 16 (fore),16 (back) | 16 (fore),16 (back) | 16 (fore),16 (back) |
| Bold          | -                   | -           | Yes                 | -                   | -                   |
| Underline     | Yes                 | -           | Yes                 | -                   | Yes                 |
| Italic        | -                   | -           | -                   | -                   | -                   |
| Character Set | Ascii,WTF-16        | Ascii,UTF-8 | Ascii,UTF-8         | Ascii,UTF-8         | Ascii,UTF-8         |
| Cursor        | Yes                 | Yes         | Yes                 | -                   | Yes                 |


## Keyboard

| Keys               | Windows Console | Windows VT | NCurses | Termios | Web Terminal |
| ------------------ | --------------- | ---------- | ------- | ------- | ------------ |
| Alt+Key            | Yes             | Yes        | Wip     | -       | Yes          |
| Shift+Key          | Yes             | Yes        | Yes     | -       | Yes          |
| Ctrl+Key           | Yes             | Yes        | Yes     | -       | Yes          |
| Alt+Shift+Key      | Yes             | Yes        | -       | -       | -            |
| Ctrl+Shift+Key     | Yes             | Yes        | -       | -       | -            |
| Ctrl+Alt+Key       | Yes             | Yes        | -       | -       | -            |
| Ctrl+Alt+Shift+Key | Yes             | Yes        | -       | -       | -            |
| Alt pressed        | Yes             | Yes        | -       | -       | -            |
| Shift pressed      | Yes             | Yes        | -       | -       | -            |
| Ctrl pressed       | Yes             | Yes        | -       | -       | -            |

## Mouse

| Mouse events | Windows Console | Windows VT | NCurses | Termios | Web Terminal |
| ------------ | --------------- | ---------- | ------- | ------- | ------------ |
| Click        | Yes             | Yes        | Yes     | Yes     | Yes          |
| Move & Drag  | Yes             | Yes        | Yes     | Yes     | Yes          |
| Wheel        | Yes             | Yes        | Yes     | -       | Yes          |


## System events

| Events         | Windows Console | Windows VT | NCurses | Termios | Web Terminal |
| -------------- | --------------- | ---------- | ------- | ------- | ------------ |
| Console Resize | Yes             | Yes        | Yes     | -       | Yes          |
| Console closed | Yes             | Yes        | -       | -       | Yes          |

## Other capabilities

| Capabilities  | Windows Console | Windows VT | NCurses | Termios | Web Terminal |
| ------------- | --------------- | ---------- | ------- | ------- | ------------ |
| Set dimension | Yes             | Yes        | -       | -       | Yes          |
| Set title     | Yes             | Yes        | -       | -       | Yes          |

## Clipboard

AppCUI provides clipboard support for copying and pasting text. The clipboard functionality is available on the following terminals:

| Terminal        | Clipboard Support | API Used        |
| --------------- | ----------------- | --------------- |
| Windows Console | Yes               | Windows API     |
| Windows VT      | Yes               | Windows API     |
| NCurses         | Yes               | copypasta crate |
| Termios         | -                 | -               |
| Web Terminal    | Yes               | Browser API     |

## Defaults

By default, when using initializing a terminal, the folowing will be used:

| OS      | Default terminal |
| ------- | ---------------- |
| Windows | Windows Console  |
| Linux   | NCurses          |
| Mac/OSX | Termios          |
