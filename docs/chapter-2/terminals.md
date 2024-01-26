# Terminals

AppCUI supports various terminals (but each one comes with advantages and drawbacks).

## OS Support
| OS      | Windows Console    | NCurses |
|---------|--------------------|---------|
| Windows | Yes                | ?       |
| Linux   | -                  | Yes     |
| Mac/OSX | -                  | ?       |
| Web     | -                  | -       |


## Display

| Display       | Windows Console    | NCurses             |
|---------------|--------------------|---------------------|
| Colors        | 16 (fore),16 (back)| 16 (fore),16 (back) | 
| Bold          | Yes                | -                   |
| Underline     | Yes                | -                   |
| Character Set | Ascii,WTF-16       | -                   |
| Cursor        | Yes                | -                   |


## Keyboard

| Keys               | Windows Console    | NCurses             |
|--------------------|--------------------|---------------------|
| Alt+Key            | Yes                | -                   | 
| Shift+Key          | Yes                | -                   |
| Ctrl+Key           | Yes                | -                   |
| Alt+Shift+Key      | Yes                | -                   |
| Ctrl+Shift+Key     | Yes                | -                   |
| Ctrl+Alt+Key       | Yes                | -                   |
| Ctrl+Alt+Shift+Key | Yes                | -                   |
| Alt pressed        | Yes                | -                   |
| Shift pressed      | Yes                | -                   |
| Ctrl pressed       | Yes                | -                   |

## Mouse

| Mouse events       | Windows Console    | NCurses             |
|--------------------|--------------------|---------------------|
| Click              | Yes                | -                   | 
| Move & Drag        | Yes                | -                   |
| Wheel              | Yes                | -                   |


## System events

| Keys               | Windows Console    | NCurses             |
|--------------------|--------------------|---------------------|
| Console Resize     | Yes                | -                   | 
| Console closed     | Yes                | -                   |

## Other capabilities

| Keys               | Windows Console    | NCurses             |
|--------------------|--------------------|---------------------|
| Set dimension      | Yes                | -                   | 
| Set title          | Yes                | -                   |

## Defaults

By default, when using initializing a terminal, the folowing will be used:

|OS       | Default terminal |
|---------|------------------|
| Windows | Windows Console  |
| Linux   | N/A              |
| Mac/OSX | N/A              |
| Web     | N/A              |
