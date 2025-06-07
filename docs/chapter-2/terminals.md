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

| Display       | Windows Console     | NCurses             | Termios             | Web Terminal        |
| ------------- | ------------------- | ------------------- | ------------------- |-------------------- |
| Colors        | 16 (fore),16 (back) | 16 (fore),16 (back) | 16 (fore),16 (back) | 16 (fore),16 (back) |
| Bold          | -                   | Yes                 | -                   | -                   |
| Underline     | Yes                 | Yes                 | -                   | Yes                 |
| Character Set | Ascii,WTF-16        | Ascii,UTF-8         | -                   | Ascii,WTF-16        |
| Cursor        | Yes                 | Yes                 | -                   | Yes                 |


## Keyboard

| Keys               | Windows Console | NCurses | Termios | Web Terminal |
| ------------------ | --------------- | ------- | ------- | ------------ |
| Alt+Key            | Yes             | Wip     | -       | Yes          |
| Shift+Key          | Yes             | Yes     | -       | Yes          |
| Ctrl+Key           | Yes             | Yes     | -       | Yes          |
| Alt+Shift+Key      | Yes             | -       | -       | -            |
| Ctrl+Shift+Key     | Yes             | -       | -       | -            |
| Ctrl+Alt+Key       | Yes             | -       | -       | -            |
| Ctrl+Alt+Shift+Key | Yes             | -       | -       | -            |
| Alt pressed        | Yes             | -       | -       | -            |
| Shift pressed      | Yes             | -       | -       | -            |
| Ctrl pressed       | Yes             | -       | -       | -            |

## Mouse

| Mouse events | Windows Console | NCurses | Termios | Web Terminal |
| ------------ | --------------- | ------- | ------- | ------------ |
| Click        | Yes             | Yes     | -       | Yes          |
| Move & Drag  | Yes             | Yes     | -       | Yes          |
| Wheel        | Yes             | Yes     | -       | Yes          |


## System events

| Events         | Windows Console | NCurses | Termios | Web Terminal |
| -------------- | --------------- | ------- | ------- | ------------ |
| Console Resize | Yes             | Yes     | -       | Yes          |
| Console closed | Yes             | -       | -       | Yes          |

## Other capabilities

| Capabilities  | Windows Console | NCurses       | Termios | Web Terminal |
| ------------- | --------------- | ------------- | ------- | ------------ |
| Set dimension | Yes             | Terminal size | -       | Yes          |
| Set title     | Yes             | -             | -       | Yes          |

## Clipboard
AppCUI provides clipboard support for copying and pasting text. The clipboard functionality is available on the following terminals:

| Terminal        | Clipboard Support | API Used        |
| --------------- | ----------------- | --------------- |
| Windows Console | Yes               | Windows API     |
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
