# Ncurses Terminal

## Installing ncurses Library

To use the ncurses terminal library in your Rust project, you need to install the ncurses library on your system. Here's how you can install it on different platforms:

### Ubuntu/Debian
sudo apt-get install libncurses5-dev libncursesw5-dev

### macOS
brew install ncurses

<!-- To DO: port ncurses for windows
### Windows
1. Download the precompiled ncurses library from the [PDCurses website](https://pdcurses.org/).
2. Extract the downloaded archive.
3. Copy the extracted files to a directory in your system's PATH. -->

## Limitations of ncurses
While ncurses is a powerful terminal library, it does have some limitations to be aware of:
- ncurses is primarily designed for ASCII characters and may not handle wide characters properly.
- Wide characters, such as Unicode characters, may not be displayed correctly or may cause unexpected behavior.
- Some terminal emulators may not fully support all ncurses features, leading to inconsistent behavior across different terminals.

It's important to consider these limitations when using ncurses in your Rust project, especially if you need to work with wide characters or require consistent behavior across different terminals.

AppCUI uses ncursesw for wide character support, in order to render multiple Unicode chars.
