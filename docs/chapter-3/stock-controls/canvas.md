# Canvas

Represent a surface that can be drawn under a view-port:

<img src="img/canvas.png" width=300/>

To create a canvas use `Canvas::new` method (with 3 parameters: a size, a layout and initialization flags).
```rs
let b = Canvas::new(Size::new(30,10), Layout::new("x:10,y:5,w:15"),canvas::Flags::None);
```
or the macro `canvas!`
```rs
let b1 = canvas!("30x10,x:10,y:5,w:15");
let b2 = canvas!("'30,10',x:10,y:5,w:15");
```

A canvas supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                | Type         | Positional parameter                | Purpose                                                                                                                                                                                         |
| ----------------------------- | ------------ | ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `size` or `sz` or `surface`   | Size         | **Yes** (first postional parameter) | The size of the surface within the canvas                                                                                                                                                       |
| `flags`                       | String       | **No**                              | canvas initialization flags                                                                                                                                                                     |
| `back` or `backgroud`         | char! format | **No**                              | A character as describes in [Macro Builds](../../chapter-2/screen.md#macro-builds) - the same as with the  `char!` macro format                                                                 |
| `lsm` or `left-scroll-margin` | Numeric      | **No**                              | The left margin of the bottom scroll bar in characters. If not provided the default value is 0. This should be a positive number and it only has an effect if the flag `Scrollbars` was set up. |
| `tsm` or `top-scroll-margin`  | Numeric      | **No**                              | The top margin of the right scroll bar in characters. If not provided the default value is 0. This should be a positive number and it only has an effect if the flag `Scrollbars` was set up.   |

A canvas supports the following initialization flags:
* `canvas::Flags::ScrollBars` or `ScrollBars` (for macro initialization) - thils enable a set of scrollbars that can be used to change the view of the inner surface, but only when the control has focus, as described in [Components](../components.md) section.

Some examples that uses these paramateres:

1. A canvas with a backgroud that consists in the character `X` in with `Aqua` and `DarkBlue` colors.
    ```rs
    let c = canvas!("size:10x5,x:10,y:5,w:15,back={X,fore:aqua,back:darkblue}");
    ```
2. A canvas with scrollbars with different margins
    ```rs
    let c = canvas!("sz:'10 x 5',x:10,y:5,w:15,flags:Scrollbars,lsm:5,tsm:1");
    ```

## Events
A canvas emits no events.

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a canvas also has the following aditional methods:

| Method                     | Purpose                                                            |
| -------------------------- | ------------------------------------------------------------------ |
| `drawing_surface_mut(...)` | Returns the inner surface that can be dranw into the canvas        |
| `resize_surface(...)`      | Resizes the inner surface of the canvas                            |
| `set_backgound(...)`       | Sets the character used for background                             |
| `clear_background()`       | Remove the background character making the background transparent. |

## Key association

The following keys are processed by a canvas control if it has focus:

| Key                                 | Purpose                                                                                                                                |
| ----------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `Left`,`Right`,`Up`,`Down`          | Move the view port to a specified direction by one character.                                                                          |
| `Shift+Left`                        | Moves the horizontal view port coordonate to 0                                                                                         |
| `Shift+Up`                          | Moves the vertical view port coordonate to 0                                                                                           |
| `Shift+Right`                       | Moves the horizontal view port coordonate so that the right side of the inner surface is displayed                                     |
| `Shift+Down`                        | Moves the vertical view port coordonate so that the bottom side of the inner surface is displayed                                      |
| `Ctrl`+{`Left`,`Right`,`Up`,`Down`} | Move the view port to a specified direction by a number of characters that is equal to the width for Left/Right or height for Up/Down. |
| `PageUp`, `PageDown`                | has the same effect as `Ctrl`+{`Up` or `Down`}                                                                                         |
| `Home`                              | Moves the view port to the coordonates (0,0)                                                                                           |
| `End`                               | Moves the view port so that the bottom-right part of the inner surface is visible                                                      |

## Example

The following code uses a canvas to create a viewer over the Rust language definition from wikipedia:

```rs
use appcui::prelude::*;

static text: &str = r"--- From Wiki ----
Rust is a multi-paradigm, general-purpose 
programming language that emphasizes performance, 
type safety, and concurrency. It enforces memory 
safety—meaning that all references point to valid 
memory—without a garbage collector. To 
simultaneously enforce memory safety and prevent 
data races, its 'borrow checker' tracks the object 
lifetime of all references in a program during 
compilation. Rust was influenced by ideas from 
functional programming, including immutability, 
higher-order functions, and algebraic data types. 
It is popular for systems programming.

From: https://en.wikipedia.org/wiki/Rust_(programming_language)
";
fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(60,20)).build()?;
    let mut w = window!("Title,d:c,w:40,h:8,flags:Sizeable");
    let mut c = canvas!("'60x15',d:c,w:100%,h:100%,flags=ScrollBars,lsm:3,tsm:1");
    let s = c.drawing_surface_mut();
    s.write_string(0, 0, text, CharAttribute::with_color(Color::White, Color::Black), true);
    w.add(c);
    a.add_window(w);
    a.run();
    Ok(())
}
```