# Termios

## Summary
Termios is a low-level library for terminal manipulation on UNIX systems. It is utilized in AppCUI to handle terminal input and output on macOS.

## Implementation Details
When a `TermiosTerminal` instance is initialized, the terminal is configured to operate in *raw mode*. This mode allows the application to capture individual key press events directly, bypassing line buffering and other default terminal behaviors. To facilitate advanced input handling, a specific byte sequence is sent to `stdout` to enable mouse events, allowing AppCUI to handle mouse interactions.

To adapt to dynamic terminal conditions, a signal handler is set up to monitor window resize events (`SIGWINCH`). This ensures that the terminal layout is updated appropriately when the terminal window's dimensions change.

At the lowest level, the implementation involves reading one or more bytes directly from `stdin`. This is performed using a blocking read operation on a separate thread to avoid interfering with the application's main execution flow. These bytes are then interpreted into a `SystemEvent`, which is dispatched to the runtime manager for further processing.

## Limitations
- **Screen flickering**: Screen updates may cause flickering because the screen content is not flushed all at once. We have not yet found a solution for this issue.
- **Key Combination Limitations**: Certain key combinations on macOS cannot be uniquely identified due to limitations in the terminal's input byte encoding. For example:
    - `Enter`, `Command + Enter`, `Option + Enter`, and `Control + Enter` all produce the same byte sequence
    - `Control + H` and `Control + Backspace` produce conflicting byte sequences

## TODO
The following features are missing from the Termios Terminal implementation:
- **Text Styling**: Support for `CharFlags` (**bold**, *italic*, underlined characters)
- **Key Mapping**: Some keys and key combinations are either unmapped or incorrectly mapped
- **Cursor Visibility**: Hiding the cursor is not supported
