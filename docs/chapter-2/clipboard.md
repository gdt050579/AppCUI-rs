# Clipboard

Access to clipboard can be done via a special non-instantiable class called `Clipboard`. This class provides the basic functionality to work with the clipboard, as follows:

| Method                     | Purpose                                                                                |
| -------------------------- | -------------------------------------------------------------------------------------- |
| `Clipboard::clear()`       | Cleans the content of the clipboard                                                    |
| `Clipboard::set_text(...)` | Sets a new text to the clipboard                                                       |
| `Clipboard::has_text()`    | Returns `true` if the clipboard contains a text or `false` otherwise                   |
| `Clipboard::text()`        | Returns an option with a String that contains the text that is stored in the clipboard |

Access to clipboard depends on the type of backend you are using (e.g. `WindowsConsole` backend relies on low level APIs like `OpenClipboard`, `GetClipboardData`, `EmptyClipboard`, `SetClipboardData` and `CloseClipboard`). As such, you will only be able to use this class after the application has been initialized (e.g. after a call to `App::new()`). Calling static methods from this class before that moment will have no action.

## Example

A typical example on how to use the clipboard looks like the following:

```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    // fist initialize the application
    let mut a = App::new().build()?;
    // now use the clipboard
    if let Some(text) = Clipboard::text() {
        // do something with the text from the clipboard
    }
    // now set a new text into the clipboard:
    Clipboard::set_text("Hello world");
    Ok(())
}

```

**Remarks**: Keep in mind that calling `Clipboard::text()` will always create a `String` object containing the content of the clipboad. If you just want to check if something exists in a clipboard (for example to enable/disable some menu items - use `Clipboard::has_text()` method instead).


## Limitations

Depending on the type of terminal, the clipboard comes with some limitations (for example in case of `WindowsConsole` backend, the clipboard can not store unicode characters that are not in WTF-16 format - within the range 0..0xFFFF). 