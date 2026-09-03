# Clipboard

Access to the clipboard can be done via a special non-instantiable class called `Clipboard`. This class provides the basic functionality to work with the clipboard, as follows:

| Method                     | Purpose                                                                                |
| -------------------------- | -------------------------------------------------------------------------------------- |
| `Clipboard::clear()`       | Clears the content of the clipboard                                                    |
| `Clipboard::set_text(...)` | Sets a new text to the clipboard                                                       |
| `Clipboard::has_text()`    | Returns `true` if the clipboard contains a text or `false` otherwise                   |
| `Clipboard::text()`        | Returns an option with a String that contains the text that is stored in the clipboard |

Access to the clipboard depends on the type of backend you are using (e.g. the `WindowsConsole` backend relies on low-level APIs such as `OpenClipboard`, `GetClipboardData`, `EmptyClipboard`, `SetClipboardData`, and `CloseClipboard`). As such, you will only be able to use this class after the application has been initialized (for example from a window factory passed to `.window(...)`, or from an event handler once `.run()` has started). Calling static methods from this class before that moment will have no effect.

## Example

A typical example of how to use the clipboard looks like the following:

```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            // the application is initialized when this factory runs
            if let Some(text) = Clipboard::text() {
                // do something with the text from the clipboard
            }
            Clipboard::set_text("Hello world");
            window!("Clipboard,a:c,w:40,h:9")
        })
        .run()
}
```

**Remarks:** Keep in mind that calling `Clipboard::text()` will always create a `String` object containing the content of the clipboard. If you just want to check whether something exists in the clipboard (for example, to enable or disable some menu items), use the `Clipboard::has_text()` method instead.


## Limitations

Depending on the type of terminal, the clipboard comes with some limitations (for example, with the `WindowsConsole` backend, the clipboard cannot store Unicode characters that are not in WTF-16 format—within the range 0..0xFFFF). 