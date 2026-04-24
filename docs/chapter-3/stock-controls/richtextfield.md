# RichTextField

Represents a text editing control with parser-based, per-character styling.

<img src="img/richtextfield.png" width=300/>

To create a rich text field, use `RichTextField::new` (with 3 parameters: text, layout and flags):
```rs
let rtx = RichTextField::new("some text", layout!("x:10,y:5,w:20"), richtextfield::Flags::None);
```

You can also build it with a parser callback via `RichTextField::with_parser`:
```rs
fn my_parser(t: &mut richtextfield::AttributeText, theme: &Theme) {
    let n = t.count();
    for i in 0..n {
        if let Some(c) = t.char(i) {
            if c.is_ascii_digit() {
                t.set_attr(i, CharAttribute::new(Color::Yellow, Color::Transparent, CharFlags::Bold));
            } else {
                t.set_attr(i, theme.text.normal);
            }
        }
    }
}

let rtx = RichTextField::with_parser(
    "v1 build 42",
    layout!("x:1,y:1,w:30,h:1"),
    richtextfield::Flags::None,
    my_parser,
);
```

Or via macro:
```rs
let rtx1 = richtextfield!("text='some text',x:10,y:5,w:20");
let rtx2 = richtextfield!("'some text',x:10,y:5,w:20,parser:my_parser");
```

A `RichTextField` supports all common parameters (see [Instantiate via Macros](../instantiate_via_macros.md)). Besides them, the following named parameters are also accepted:

| Parameter name      | Type     | Positional parameter                 | Purpose                                                                 |
| ------------------- | -------- | ------------------------------------ | ----------------------------------------------------------------------- |
| `text` or `caption` | String   | **Yes** (first positional parameter) | Initial text content. If omitted, an empty text is used.                |
| `flags`             | List     | **No**                               | Initialization flags controlling read-only mode and Enter processing.   |
| `parser`            | Function | **No**                               | Parser callback (`fn(&mut AttributeText, &Theme)`) applied after edits. |

A rich text field supports the following initialization flags:
- `richtextfield::Flags::Readonly` or `Readonly` (macro) - allows viewing/selecting/copying but prevents text edits.
- `richtextfield::Flags::ProcessEnter` or `ProcessEnter` (macro) - captures `Enter` and triggers `RichTextFieldEvents::on_validate(...)`.
- `richtextfield::Flags::DisableAutoSelectOnFocus` or `DisableAutoSelectOnFocus` (macro) - disables auto-select-all when focus is received.

## Events

To intercept events from a rich text field, implement:

```rs
pub trait RichTextFieldEvents {
    fn on_validate(&mut self, handle: Handle<RichTextField>, text: &str) -> EventProcessStatus { ... }
    fn on_text_changed(&mut self, handle: Handle<RichTextField>) -> EventProcessStatus { ... }
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md), `RichTextField` has:

| Method            | Purpose                                                              |
| ----------------- | -------------------------------------------------------------------- |
| `set_text(...)`   | Replaces the full text and recomputes parser styling.                |
| `undo()`          | Undo last action.                                                    |
| `redo()`          | Redo last action.                                                    |
| `text()`          | Returns current text.                                                |
| `is_readonly()`   | Returns `true` when the control is in read-only mode.                |
| `set_parser(...)` | Installs/replaces parser callback and reapplies styling immediately. |
| `reset_parser()`  | Removes parser callback and resets character styles to defaults.     |

## Key association

The following keys are processed when a `RichTextField` has focus:

| Key                                  | Purpose                                                     |
| ------------------------------------ | ----------------------------------------------------------- |
| `Left`, `Right`, `Up`, `Down`        | Move cursor through text                                    |
| `Shift`+{`Left`,`Right`,`Up`,`Down`} | Extend/reduce selection                                     |
| `Ctrl`+`Left`                        | Move to beginning of previous word                          |
| `Shift`+`Ctrl`+`Left`                | Select from current position to beginning of previous word  |
| `Ctrl`+`Right`                       | Move to beginning of next word                              |
| `Shift`+`Ctrl`+`Right`               | Select from current position to beginning of next word      |
| `Home` / `End`                       | Move to start / end of text                                 |
| `Shift` + `Home` / `Shift` + `End`   | Select from cursor to start / end                           |
| `Delete` / `Backspace`               | Delete current / previous character (or current selection)  |
| `Ctrl`+`A`                           | Select all text                                             |
| `Ctrl`+`U`                           | Convert selection (or current word) to lowercase            |
| `Ctrl`+`Shift`+`U`                   | Convert selection (or current word) to uppercase            |
| `Ctrl`+`C` or `Ctrl`+`Insert`        | Copy selection                                              |
| `Ctrl`+`V` or `Shift`+`Insert`       | Paste clipboard text                                        |
| `Ctrl`+`X` or `Shift`+`Delete`       | Cut selection                                               |
| `Ctrl`+`Z`                           | Undo last action                                            |
| `Ctrl`+`Shift`+`Z`                   | Redo last action                                            |
| `Enter`                              | Triggers `on_validate(...)` only when `ProcessEnter` is set |

Additionally, printable characters can be inserted directly into the text.

## Mouse actions

Mouse cursor can be used for caret positioning and selection. Double click selects the clicked word.

## Example

The following example applies markdown-like styling with a parser callback:

```rs
use appcui::prelude::*;

fn markdown_like_colors(t: &mut richtextfield::AttributeText, _theme: &Theme) {
    let n = t.count();
    let mut ticks = false;
    let mut bold = false;
    let mut italic = false;
    let mut i = 0;
    while i < n {
        let c = t.char(i).unwrap_or('\0');
        if i + 1 < n && c == '*' && t.char(i + 1).unwrap_or('\0') == '*' {
            bold = !bold;
            i += 2;
            continue;
        }
        if c == '`' {
            ticks = !ticks;
            i += 1;
            continue;
        }
        if c == '*' {
            italic = !italic;
            i += 1;
            continue;
        }

        if ticks {
            t.set_attr(i, CharAttribute::new(Color::Aqua, Color::Transparent, CharFlags::None));
        } else if bold {
            t.set_attr(i, CharAttribute::new(Color::Yellow, Color::Transparent, CharFlags::Bold));
        } else if italic {
            t.set_attr(i, CharAttribute::new(Color::Green, Color::Transparent, CharFlags::Italic));
        }
        i += 1;
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = window!("'RichTextField example',a:c,w:80,h:12");
    w.add(label!("'Use **, * and ` to toggle styles',l:1,t:1,r:1,h:1"));
    w.add(richtextfield!(
        "'A `code` and **bold** plus *italic* demo',l:1,t:3,r:1,h:2,parser:markdown_like_colors"
    ));
    app.add_window(w);
    app.run();
    Ok(())
}
```
