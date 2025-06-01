# Terminals

AppCUI supports various terminals (but each one comes with advantages and drawbacks).

## OS Support
| OS      | Windows Console | NCurses | Termios |
| ------- | --------------- | ------- | ------- |
| Windows | Yes             | ?       | -       |
| Linux   | -               | Yes     | Yes     |
| Mac/OSX | -               | Yes     | Yes     |
| Web     | Yes             | Yes     | -       |


## Display

| Display       | Windows Console     | NCurses             | Termios             |
| ------------- | ------------------- | ------------------- | ------------------- |
| Colors        | 16 (fore),16 (back) | 16 (fore),16 (back) | 16 (fore),16 (back) |
| Bold          | -                   | Yes                 | -                   |
| Underline     | Yes                 | Yes                 | -                   |
| Character Set | Ascii,WTF-16        | Ascii,UTF-8         | -                   |
| Cursor        | Yes                 | Yes                 | -                   |


## Keyboard

| Keys               | Windows Console | NCurses | Termios |
| ------------------ | --------------- | ------- | ------- |
| Alt+Key            | Yes             | Wip     | -       |
| Shift+Key          | Yes             | Yes     | -       |
| Ctrl+Key           | Yes             | Yes     | -       |
| Alt+Shift+Key      | Yes             | -       | -       |
| Ctrl+Shift+Key     | Yes             | -       | -       |
| Ctrl+Alt+Key       | Yes             | -       | -       |
| Ctrl+Alt+Shift+Key | Yes             | -       | -       |
| Alt pressed        | Yes             | -       | -       |
| Shift pressed      | Yes             | -       | -       |
| Ctrl pressed       | Yes             | -       | -       |

## Mouse

| Mouse events | Windows Console | NCurses | Termios |
| ------------ | --------------- | ------- | ------- |
| Click        | Yes             | Yes     | -       |
| Move & Drag  | Yes             | Yes     | -       |
| Wheel        | Yes             | Yes     | -       |


## System events

| Events         | Windows Console | NCurses | Termios |
| -------------- | --------------- | ------- | ------- |
| Console Resize | Yes             | Yes     | -       |
| Console closed | Yes             | -       | -       |

## Other capabilities

| Capabilities  | Windows Console | NCurses       | Termios |
| ------------- | --------------- | ------------- | ------- |
| Set dimension | Yes             | Terminal size | -       |
| Set title     | Yes             | -             | -       |

## Clipboard
AppCUI provides clipboard support for copying and pasting text. The clipboard functionality is available on the following terminals:

| Terminal        | Clipboard Support | API Used        |
| --------------- | ----------------- | --------------- |
| Windows Console | Yes               | Windows API     |
| NCurses         | Yes               | copypasta crate |
| Termios         | -                 | -               |

## Defaults

By default, when using initializing a terminal, the folowing will be used:

| OS      | Default terminal |
| ------- | ---------------- |
| Windows | Windows Console  |
| Linux   | NCurses          |
| Mac/OSX | Termios          |
