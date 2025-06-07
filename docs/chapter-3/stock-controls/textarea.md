# TextArea

Represent a control where you can add/modify a text:

<img src="img/textarea.png" width=400/>

To create a textarea use `TextArea::new` method (with 3 parameters: a caption, a layout and initialization flags).
```rs
let tx = TextArea::new("Some text", Layout::new("x:10,y:5,w:15"), textarea::Flags::None);
```

or use the macro `textarea!()`
```rs
let textarea1 = textarea!("text='some text to edit',d:c,h:100%");
let textarea2 = textarea!("'some text to print',d:c,h:100%,flags:ReadOnly");
```

A textarea supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name      | Type   | Positional parameter                | Purpose                                                                                                              |
| ------------------- | ------ | ----------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `text` | String | **Yes** (first postional parameter) | The text from a text area. If ommited an empty string will be considered as the caption of the textarea. |
| `flags`             | List   | **No**                              | TextArea initialization flags that control how the TextArea should look and behave(ReadOnly, having line numbers)                 |

Text Area supports the following initialization flags:
* `textarea::Flags::ShowLineNumber` or `ShowLineNumber` (for macro initialization) - This flag enables the display of line numbers in the text area, typically in a gutter on the left side. It helps users keep track of their position within the text, making navigation and debugging easier. This feature is especially useful for programming and document editing, where line references are important.
* `textarea::Flags::ReadOnly` or `ReadOnly` (for macro initialization) - When this flag is set, the text area becomes non-editable, meaning users can view but not modify the text. This is useful for displaying logs, reference documents, or any content where accidental modifications should be prevented. Although users cannot change the text, they may still be able to select and copy it.
* `textarea::Flags::ScrollBars` or `ScrollBars` (for macro initialization)- This flag enables scrollbars in the text area when the content exceeds the visible space. It ensures smooth navigation by allowing users to scroll horizontally or vertically as needed.
* `textarea::Flags::HighlightCursor` or `HughlightCursor` (for macro initialization) - When enabled, this flag highlights the current cursor position within the text. It can be useful for visually tracking the insertion point while typing or editing. The highlight will appear as a different background color.

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a textfield also has the following aditional methods:

| Method                                      | Purpose |
|---------------------------------------------|---------|
| **set_text** | Replaces the current content with the specified text. |
| **insert_text** | Inserts the given text at the specified cursor position. |
| **remove_text** | Removes a portion of the text between specified positions. |
| **text** | Returns the full content of the text editor. |
| **select_text** | Selects a range of text with the given start position and size. |
| **clear_selection**      | Clears any active text selection. |
| **has_selection**      | Returns true if there is an active text selection. |
| **selection**      | Returns the currently selected text, if any. |
| **delete_selection**      | Deletes the currently selected text. |
| **is_read_only**      | Returns true if the text editor is in read-only mode. |
| **set_cursor_position**      | Moves the cursor to the specified position. |
| **cursor_position**      | Returns the current position of the cursor. |


## Key association

The following keys are processed by a TextField control if it has focus:

| Key                     | Purpose |
|-------------------------|---------|
| **Arrow Keys**          | Move the cursor left, right, up, or down by one character or line. |
| **Shift + Arrows**      | Extends the text selection in the direction of the arrow key. |
| **Ctrl + Right**        | Moves the cursor to the beginning of the next word. |
| **Ctrl + Left**         | Moves the cursor to the beginning of the previous word. |
| **Ctrl + Shift + Right** | Extends the selection to the beginning of the next word. |
| **Ctrl + Shift + Left**  | Extends the selection to the beginning of the previous word. |
| **Ctrl + C**            | Copies the selected text to the clipboard. |
| **Ctrl + V**            | Pastes the clipboard content at the cursor position. |
| **Backspace**           | Deletes the character before the cursor. |
| **Delete**             | Deletes the character after the cursor. |
| **Ctrl + Backspace**    | Deletes the entire previous word. |
| **Ctrl + Delete**       | Deletes the entire next word. |
| **Enter**              | Inserts a new line at the cursor position. |
| **Page Up**            | Moves the view up by one page, scrolling the text accordingly. |
| **Page Down**          | Moves the view down by one page, scrolling the text accordingly. |

Aditionally, all printable characters can be used to insert / modify or edit the current text.

## Mouse actions

Mouse cursor can be used to select the text.

## Example

The following code creates multiple text areas with both unicode and regular text.

```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:11"), window::Flags::None);
    w.add(TextArea::new("I ❤️ Rust Language", Layout::new("d:c,h:100%"), textarea::Flags::None));
    w.add(TextArea::new("Read only text", Layout::new("d:c,h:100%"), textarea::Flags::ReadOnly));
    w.add(TextArea::new("Line Numbers tab functional", Layout::new("d:c,h:100%"), textarea::Flags::ShowLineNumber | textarea::Flags::ReadOnly));
    w.add(TextArea::new("I also have scrollbars ❤️", Layout::new("d:c,h:100%"), textarea::Flags::ScrollBars));
    a.add_window(w);
    a.run();
    Ok(())
}
```