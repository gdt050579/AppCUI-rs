# TextField

Represent a control where you can add/modify a text:

<img src="img/textfield.png" width=300/>

To create a textfield use `TextField::new` method (with 3 parameters: a caption, a layout and initialization flags).
```rs
let tx = TextField::new("some text", layout!("x:10,y:5,w:15"),textfield::Flags::None);
```
or the macro `textfield!`
```rs
let tx1 = textfield!("text='some text',x:10,y:5,w:15");
let tx2 = textfield!("some_text,x:10,y:5,w:15");
```

A textfield supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name      | Type   | Positional parameter                | Purpose                                                                                                              |
| ------------------- | ------ | ----------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `text` or `caption` | String | **Yes** (first postional parameter) | The caption (text) from a text field. If ommited an empty string will be considered as the caption of the textfield. |
| `flags`             | List   | **No**                              | TextField initialization flags that control how Enter is process, if the textfield is readonly, etc                  |

A textfield supports the following initialization flags:
* `textfield::Type::Readonly` or `Readonly` (for macro initialization) - thils will allow you to view or copy the text but not to modify it
* `textfield::Type::ProcessEnter` or `ProcessEnter` (for macro initialization) - by default the `Enter` key is not processed by this control. However, if this flag is being used, `Enter` key is being captured and when pressed the `TextFieldEvents::on_validate(...)` method is being called.
* `textfield::Type::DisableAutoSelectOnFocus` or `DisableAutoSelectOnFocus` (for macro initialization) - by default, a textfield will automatically select its content when it receives the focus. This behavior can be disabled by adding this flag to the initialization flags.
  
Some examples that uses these paramateres:
```rs
let no_auto_focus = textfield!("caption='no auto focus',x:10,y:5,w:15,flags:DisableAutoSelectOnFocus");
let read_only = textfield!("text='a read only text',x=9,y:1,align:center,w:9,flags: ReadOnly");
let expty_text = textfield!("x:1,y:1,w:10");
```

## Events
To intercept events from a textfield, the following trait has to be implemented to the Window that processes the event loop:

```rs
pub trait TextFieldEvents {
    fn on_validate(&mut self, handle: Handle<TextField>, text: &str) -> EventProcessStatus {...}
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a textfield also has the following aditional methods:

| Method          | Purpose                                                                                                                  |
| --------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `set_text(...)` | Set the new text for a textfield.                                                                                        |
| `text()`        | Returns the current text from a textfield                                                                                |
| `is_readonly()` | Returns `true` if the current textfield is in a readonly state (was created with the readonlu flag) or `false` otherwise |

## Key association

The following keys are processed by a TextField control if it has focus:

| Key                                  | Purpose                                                                                                                                   |
| ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `Left`, `Right`, `Up`, `Down`        | Navigate through the text from the textfield                                                                                              |
| `Shift`+{`Left`,`Right`,`Up`,`Down`} | Selects part of the text textfield                                                                                                        |
| `Ctrl`+`Left`                        | Moves to the begining of the previous word                                                                                                |
| `Shift`+`Ctrl`+`Left`                | Selects the text from the begining of the previous word until the current position                                                        |
| `Ctrl`+`Right`                       | Moves to the begining of the next word                                                                                                    |
| `Shift`+`Ctrl`+`Right`               | Selects the text from current postion until the start of the next word                                                                    |
| `Home`                               | Move to the begining of the text                                                                                                          |
| `Shift`+`Home`                       | Selects the text from the beging of the text until the current position                                                                   |
| `End`                                | Moves to the end of the text                                                                                                              |
| `Shift` + `End`                      | Selects the text from current position until the end of the text                                                                          |
| `Delete`                             | Deletes the current character. If a selection exists, it deletes it first                                                                 |
| `Backspace`                          | Deletes the previous charactr. If a selection exists, it deletes it first                                                                 |
| `Ctrl`+`A`                           | Selects the entire text                                                                                                                   |
| `Ctrl`+`U`                           | Converts the current selection to lower case. If no selection is present the curent word will be selected and then converted to lowercase |
| `Ctrl`+`Shift`+`U`                   | Converts the current selection to upper case. If no selection is present the curent word will be selected and then converted to uppercase |
| `Ctrl`+`C` or `Ctrl`+`Insert`        | Copy the current selection to clipboard                                                                                                   |
| `Ctrl`+`V` or `Shift`+`Insert`       | Paste the text from the clipboard (if any) to current position                                                                            |
| `Ctrl`+`X` or `Shift`+`Delete`       | If a selection is present, it copies it into the clipboard and then delets it (acts like a `Cut` command)                                 |
| `Enter`                              | Only if the  flag `textfield::Type::ProcessEnter` is present will trigger a call to `TextFieldEvents::on_validate(...)`                   |

Aditionally, al printable characters can be used to insert / modify or edit the current text.

## Mouse actions

Mouse cursor can be used to select the text. Aditionally, a double click over a word will select it.

## Example

The following code creates multiple text fields with both unicode and regular text.

```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = Window::new("Title", layout!("a:c,w:40,h:11"), window::Flags::None);
    w.add(textfield!("text:'I ❤️ Rust Language',x:1,y:1,w:36,h:1"));
    w.add(textfield!("'Read only text',x:1,y:3,w:36,h:1, flags: Readonly"));
    w.add(textfield!("Inactive,x:1,y:5,w:36,h:1,enable: false"));
    w.add(textfield!("'No auto selection',x:1,y:7,w:36,h:1, flags: DisableAutoSelectOnFocus"));
    a.add_window(w);
    a.run();
    Ok(())
}
```