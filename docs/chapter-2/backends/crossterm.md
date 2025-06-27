# CrossTerm

This backend relies on the `crossterm` crate to provide a terminal abstraction layer. It supports various terminal features such as mouse input, keyboard input, and screen manipulation.

For clipboard operations, it uses the `copypaste` crate's built-in clipboard functionality for UNIX-like systems, and windows API for Windows systems. 

## Limitations

Some flickering issues may be seen when using this backend (in particular for old terminals)