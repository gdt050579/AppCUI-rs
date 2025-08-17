# CharPicker

A CharPicker is an expandable control that allows you to select a character from various Unicode character sets:

<img src="img/charpicker.png" width=300/>

It can be created using `CharPicker::new(...)`, `CharPicker::with_set(...)` or the `charpicker!` macro.

```rs
let c1 = CharPicker::new(None, layout!("x:1,y:1,w:30"));
let c2 = CharPicker::with_set(Some('A'), layout!("x:1,y:1,w:30"), charpicker::Set::with_interval("ASCII", 0x20, 0x7E).unwrap());
let c3 = charpicker!("char=A,x:1,y:1,w:30,sets=[Ascii]");
let c4 = charpicker!("code=65,x:1,y:1,w:30,sets=[Ascii,Arrows]");
```

A CharPicker supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name | Type    | Positional parameter                 | Purpose                                                                                       |
| -------------- | ------- | ------------------------------------ | --------------------------------------------------------------------------------------------- |
| `char` or `ch` | String  | **Yes** (first positional parameter) | The initial character to select in the CharPicker                                             |
| `code`         | Numeric | **No**                               | The Unicode code point of the initial character to select                                     |
| `sets`         | List    | **No**                               | A list of character sets to populate the CharPicker with (e.g., `[Ascii, Arrows, Emoticons]`) |

The following sets are available:

| Name           | Description                              | Example                          |
| -------------- | ---------------------------------------- | -------------------------------- |
| `Animals`      | Animal emoji and symbols                 | &#128000;, &#128060;, &#129408;  |
| `Arabic`       | Arabic script characters                 | &#1575;, &#1576;, &#1578;        |
| `Arrows`       | Various arrow symbols                    | &#8593;, &#8594;, &#8596;        |
| `Ascii`        | ASCII characters (0x20-0x7E)             | A, B, C, !, @, #                 |
| `Blocks`       | Block drawing characters (0x2580-0x259F) | &#9600;, &#9604;, &#9608;        |
| `BoxDrawing`   | Box drawing characters (0x2500-0x257F)   | &#9556;, &#9559;, &#9562;        |
| `Braille`      | Braille patterns (0x2800-0x28FF)         | &#10241;, &#10243;, &#10249;     |
| `Chinese`      | Chinese (CJK) characters (0x4E00-0x9FFF) | &#20013;, &#25991;, &#23383;     |
| `Currency`     | Currency symbols (0x20A0-0x20CF)         | &#8364;, &#163;, &#8377;         |
| `Cyrillic`     | Cyrillic script characters               | &#1040;, &#1041;, &#1042;        |
| `Emoticons`    | Emoticons and emoji (0x1F600-0x1F64F)    | &#128512;, &#128513;, &#128514;  |
| `Games`        | Gaming symbols (cards, dice, chess)      | &#9812;, &#9823;, &#9856;        |
| `Greek`        | Greek script characters                  | &#913;, &#914;, &#915;           |
| `Latin`        | Extended Latin characters                | &#192;, &#193;, &#194;           |
| `Math`         | Mathematical symbols                     | &#8704;, &#8706;, &#8707;        |
| `Numbers`      | Number-related symbols                   | &#8532;, &#8533;, &#9312;        |
| `Pictographs`  | Miscellaneous symbols and pictographs    | &#9728;, &#9729;, &#127744;      |
| `Punctuation`  | Various punctuation marks                | &#8211;, &#8212;, &#8230;        |
| `Shapes`       | Geometric shapes                         | &#9632;, &#9633;, &#9650;        |
| `Subscripts`   | Subscript characters (0x2080-0x209C)     | &#8320;, &#8321;, &#8322;        |
| `Superscripts` | Superscript characters                   | &#8304;, &#185;, &#178;          |
| `Transport`    | Transportation symbols (0x1F680-0x1F6FF) | &#128640;, &#128641;, &#128642;  |
| `Unicode`      | Full Unicode range (0x0020-0x10FFFF)     | All available Unicode characters |

**Remark:** The `sets` parameter can also be set to `[*]` to select all available sets.
```rs
let cp = charpicker!("x:1,y:1,w:30,sets:[*]");
```

## Events

To intercept events from a CharPicker control, the following trait has to be implemented in the Window that processes the event loop:
```rs
pub trait CharPickerEvents {
    fn on_char_changed(&mut self, handle: Handle<CharPicker>, code: Option<char>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
```

The `code` parameter contains `Some(char)` when a character is selected, or `None` when no character is selected.

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a CharPicker also has the following additional methods:

| Method             | Purpose                                                                                      |
| ------------------ | -------------------------------------------------------------------------------------------- |
| `add_set(...)`     | Adds a new character set to the CharPicker                                                   |
| `clear_sets()`     | Removes all character sets from the CharPicker and unselects the current character           |
| `select_char(...)` | Selects a specific character in the CharPicker (the character must exist in one of the sets) |
| `unselect_char()`  | Unselects the current character (sets selection to None)                                     |
| `char()`           | Returns the current character selected in the CharPicker (if any)                            |

## Key association

The following keys are processed by a CharPicker control if it has focus:

| Key                           | Purpose                                                                                                                      |
| ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `Space` or `Enter`            | Expands or collapses the CharPicker control.                                                                                 |
| `Up`, `Down`, `Left`, `Right` | Navigates through the available characters in the current character set.                                                     |
| `Home`                        | Move the selection to the first character in the current set                                                                 |
| `End`                         | Move the selection to the last character in the current set                                                                  |
| `PageUp`, `PageDown`          | Navigates through the characters page by page when the control is expanded                                                   |
| `Ctrl+Up`, `Alt+Up`           | Scrolls the view up when expanded (without changing selection)                                                               |
| `Ctrl+Down`, `Alt+Down`       | Scrolls the view down when expanded (without changing selection)                                                             |
| `Ctrl+Left`, `Alt+Left`       | Switches to the previous character set                                                                                       |
| `Ctrl+Right`, `Alt+Right`     | Switches to the next character set                                                                                           |
| `Ctrl+C`, `Ctrl+Insert`       | Copies the current selected character to the clipboard                                                                       |
| `Ctrl+V`, `Shift+Insert`      | Pastes the first character from the clipboard and selects it in the CharPicker (if that character exists in one of the sets) |
| `Escape`                      | Collapses the control. If the CharPicker is already collapsed, this key will not be captured                                 |

Besides these keys, typing any printable character will automatically search for and select that character in the current character sets.

When the CharPicker is expanded, you can also use the mouse to:
- Click on characters to select them
- Click the left/right arrow buttons to switch between character sets
- Click the `[None]` button to unselect the current character
- Use the mouse wheel to scroll through characters or switch between sets

## Sets

A CharPicker can be populated with multiple character sets. The sets are displayed in the control in a tabbed interface.
By default (using the `CharPicker::new(...)` constructor), the CharPicker is populated with the entire Unicode character set. However, you can add your own sets using the `add_set(...)` method.

To create a set of characters, you can use the `charpicker::Set` struct. The following methods are available:
* `with_interval(...)` - Creates a set of characters from a Unicode interval (e.g., `0x20-0x7E`)
* `from_unicode_symbols(...)` - Creates a set of characters from a Unicode symbols (e.g., `Ascii`, `Arrows`, `Animals`, ... )
* `new(...)` - Creates a set of characters from a list of characters (this allows one to create a custom set of characters - e.g. just numbers, or just vowels, etc)


## Example

The following example creates a Window with a CharPicker that allows selecting from ASCII and Arrow character sets. When a character is selected, the window title is updated to show the character and its Unicode code point.

```rs
use appcui::prelude::*;

#[Window(events = CharPickerEvents)]
struct MyWin {
    picker: Handle<CharPicker>,
}

impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: window!("x:1,y:1,w:40,h:8,caption:'Character Picker Demo'"),
            picker: Handle::None,
        };
        w.add(label!("'Select a character:',x:1,y:1,w:30"));
        
        // Create CharPicker with ASCII and Arrows character sets
        let mut cp = CharPicker::new(None, layout!("x:1,y:2,w:30"));
        cp.clear_sets(); // Remove the default Unicode set
        cp.add_set(charpicker::Set::from_unicode_symbols("ASCII", charpicker::UnicodeSymbols::Ascii));
        cp.add_set(charpicker::Set::from_unicode_symbols("Arrows", charpicker::UnicodeSymbols::Arrows));
        
        // Or use the macro for the same result:
        // let cp = charpicker!("x:1,y:2,w:30,sets:[Ascii,Arrows]");
        
        w.picker = w.add(cp);
        w
    }
}

impl CharPickerEvents for MyWin {
    fn on_char_changed(&mut self, _handle: Handle<CharPicker>, code: Option<char>) -> EventProcessStatus {
        let title = if let Some(ch) = code {
            format!("Selected: '{}' (U+{:04X})", ch, ch as u32)
        } else {
            String::from("No character selected")
        };
        self.set_title(&title);
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
```