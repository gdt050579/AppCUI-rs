# Macros

AppCUI has its own macros, but **does not depend** on them. Every macro in this book is a short form of a constructor, a method call, or a trait implementation you can write yourself. `use appcui::prelude::*;` brings both the macros and those types into scope. Pick either form.

For example - a character with a red glyph on a yellow background can be written `with macros` as:

```rs
let ch = char!("A,red,yellow");
```

or `without macros` as:

```rs
let ch = Character::new('A', Color::Red, Color::Yellow, CharFlags::None);
```

The same pairing holds for the other macros. The [first application](first_application.md) builds one window both ways.

| Macro | Same value, written out |
| --- | --- |
| `charattr!("white")` | `CharAttribute::with_fore_color(Color::White)` |
| `layout!("a:c,w:30,h:9")` | `LayoutBuilder::new().alignment(Alignment::Center).width(30).height(9).build()` |
| `window!("'Title',a:c,w:30,h:9")` | `Window::new("Title", layout, window::Flags::None)` |
| `label!("'Hello',a:c,w:5,h:1")` | `Label::new("Hello", layout)` |
| `menu!(...)` | `Menu::new()`, then `add` for each item. [Build a menu with macros](../chapter-4/menu/macro_builder.md) shows both. |

`#[Window]`, `#[MainWindow]`, `#[ModalWindow]`, `#[CustomControl]`, and `#[Desktop]` insert a `base` field and the trait implementations the runtime expects. That struct can be written by hand. [Event loop](../chapter-3/event_loop.md) shows what `#[Window]` generates and the version without the attribute. `#[MainWindow]` is the attribute for [main-window mode](../chapter-2/main_window.md): its `base` field is a `MainWindow`, initialized with `MainWindow::new()`.

The string inside a function-like macro (`window!`, `button!`, `layout!`, and one macro per stock control) is described in [Instantiate via Macros](../chapter-3/instantiate_via_macros.md).

## FrameApp and InputApp

A [FrameApp](../chapter-2/frame_app.md) or an [InputApp](../chapter-2/input_app.md) that only draws and handles input needs no macro. `on_paint` receives a `Surface`. Keys and mouse events arrive as `Key` and `MouseEvent`. Colors are ordinary `CharAttribute` values:

```rs
use appcui::prelude::*;

struct HelloWorld;
impl FrameApp for HelloWorld {
    fn on_paint(&self, surface: &mut Surface) {
        surface.write_string(
            0,
            0,
            "Hello World !",
            CharAttribute::with_fore_color(Color::White),
            false,
        );
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::frame_app(HelloWorld {}).run()
}
```

An `InputApp` is the same idea: implement `on_paint`, and `on_key_event` or `on_mouse_event` when the screen should react to the user. `charattr!("white")` can stand in for the `CharAttribute` above. Nothing in this loop requires `window!` or `#[Window]`.

Windows, controls, and menus can be used in these modes as well. Once you add them, the macros are available (`window!`, `button!`, `menu!`, `#[Window]`, and the others). They stay optional. `Window::new`, `Label::new`, `Menu::new`, and a struct you write yourself (the one `#[Window]` would otherwise generate) build that UI too.

## Debugging macros

Sometimes macros don't work as expected and the errors they produce might be hard to undestand (while most of the constructor macros don't have this problem, with the exception of `#[Window]` and `#[ModalWindow]` which might produce errors that are not very helpful due to the fact that they change the structure code and the Rust compiler is not able to provide a helpful error message).
In this case, there are several things you can do to help debug the problem:

### Read and evaluate the macro message

Constructor macros (`char!`, `button!`, `window!`, and the rest) usually stop with a message that already says what is wrong. Read the text under `proc macro panicked`. The line that starts with `Error:` is the reason. `Code :` repeats a slice of the parameter string, and the `^` characters sit under the token that failed.

A background color `char!` does not accept:

```rs
let ch = char!("A,red,purple");
```

`purple` is not a color name. The build stops with:

```text
error: proc macro panicked
 --> src/main.rs:1:14
  |
1 | let ch = char!("A,red,purple");
  |          ^^^^^^^^^^^^^^^^^^^^^
  |
  = help: message: called `Result::unwrap()` on an `Err` value:
          Error: Invalid color 'purple' for parameter 'back'. Expected a color name (Black, DarkBlue, DarkGreen, Teal, DarkRed, Magenta, Olive, Silver, Gray, Blue, Green, Aqua, Red, Pink, Yellow, White, Transparent), a hex color (#RGB or #RRGGBB, e.g. #F00 or #FF0000) or an RGB color (rgb(r, g, b) with values from 0 to 255, e.g. rgb(255, 128, 0))
          Code : A,red,purple
               :       ^^^^^^
```

The parameter is `back` (the second color in `A,red,purple`). The carets cover `purple`. The names in parentheses are the colors that parameter accepts, and so are `#RGB` / `#RRGGBB` and `rgb(r, g, b)`. Replace `purple` with one of those (for example `yellow` or `#A020F0`) and compile again.

A button type outside the enum is just as direct:

```rs
let btn = button!("'OK', a:c, w:10, h:1, type: Round");
```

```text
error: proc macro panicked
 --> src/main.rs:1:15
  |
1 | let btn = button!("'OK', a:c, w:10, h:1, type: Round");
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: message:
          Error: Unknwon enum variant: Round !
          Available variants are: Flat, Normal, Raised
          Code : c, w:10, h:1, type: Round
               :                     ^^^^^
```

`Round` is not a button type. The variants that are accepted are `Flat`, `Normal`, and `Raised`, and the carets sit on `Round`. Write `type: Flat` (or `Normal`, or `Raised`). The word `Unknwon` is the spelling the macro prints.

The same shape shows up for a bad flag, an unknown parameter name, or a layout key. When `Error:` names the value and lists the legal ones, edit the string. The next two sections are for the other kind of failure: the message talks about a field or a trait the macro added, and the parameter string itself is fine.

### Comment the macro out

When you do use a macro, the compiler expands it and then type-checks that expansion. The error often sits on the macro line, and it can name a field or a trait the macro added. Two tricks separate that generated code from the struct you wrote.

Comment out the attribute and compile again:

```rs
// #[Window(events = ButtonEvents)]
struct MyWin {
    count: i32,
}
```

The messages from this build are about the struct as it stands in the file: field types, method bodies, and the `base` initializer you wrote.

Uncomment the attribute and compile again. Messages that show up on this second build are about the expansion. Two that come up often:

* `missing field 'base' in initializer` — the attribute added `base`, and a struct literal still skips it. Initialize it with `Window::new(...)` or `window!(...)`.
* `conflicting implementations of trait 'ButtonEvents'` — the struct implements `ButtonEvents`, and the macro implemented it as well. List the trait in `events`: `#[Window(events = ButtonEvents)]`.

The same two compiles work for `#[ModalWindow]`, `#[CustomControl]`, and `#[Desktop]`. A longer list of `#[Window]` messages, and the change that fixes each one, is in [Event loop](../chapter-3/event_loop.md).

A build that stops with `proc-macro panicked` is the macro rejecting an argument. The text after the panic is the reason (a misspelled trait name, a missing `commands` list). Comment the attribute out and compile once more to check the struct on its own. The panic stays until the argument is valid.

### Expand macro recursively

[rust-analyzer](https://rust-analyzer.github.io/) can show the expansion in the editor. Cursor and VS Code both use it.

1. Put the cursor on the macro name: `window` in `window!(...)`, or `Window` in `#[Window(...)]`.
2. Open the command palette and run **Expand macro recursively**. The same command is on the right-click menu.
3. A read-only editor opens with the generated Rust. Nested macros are expanded as well, so `window!("'First Window',a:c,w:30,h:9")` appears as the `Window::with_type(...)` call it turns into.

That text is the code `rustc` type-checks. For an attribute, it is the struct with `base` inserted and the generated `impl` blocks. For a function-like macro, it is the constructor call. Pasting it over the macro is a way to keep that constructor and drop the macro.

To have the compiler point at those lines, paste the expansion in place of the macro and compile, then restore the macro if you still want the short form. Commenting a call such as `window!(...)` out removes the value, and the following errors name the missing variable.

From a terminal, [`cargo expand`](https://github.com/dtolnay/cargo-expand) prints the same expansion. Install it with `cargo install cargo-expand`, then run `cargo expand` on the module that contains the macro.
