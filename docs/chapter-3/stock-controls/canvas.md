# Canvas

Represent a surface that can be drawn and scrolled as part of a control:

<img src="img/control.png" width=300/>

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
| `get_drawing_surface(...)` | Returns the inner surface that can be dranw into the canvas        |
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
| `Home`                              | Moves the view port to the coordonates (0,0)                                                                                           |
| `End`                               | Moves the view port so that the bottom-right part of the inner surface is visible                                                      |

## Example

The following code creates a window with two canvass (`Add` and `Reset`). When `Add` canvas is pressed a number is incremented and set as the text of the `Add` canvas. When `Reset` canvas is being pressed, the counter is reset to 0.

```rs
use appcui::prelude::*;

#[Window(events = canvasEvents)]
struct MyWin {
    add: Handle<canvas>,
    reset: Handle<canvas>,
    counter: i32,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: Window::new("My Win", Layout::new("d:c,w:40,h:6"), window::Flags::None),
            add: Handle::None,
            reset: Handle::None,
            counter: 0,
        };
        win.add = win.add(canvas::new("Add (0)", Layout::new("x:25%,y:2,w:13,a:c"), canvas::Type::Normal));
        win.reset = win.add(canvas::new("&Reset", Layout::new("x:75%,y:2,w:13,a:c",), canvas::Type::Normal));
        win
    }
    fn update_add_canvas_caption(&mut self) {
        let h = self.add;
        let new_text = format!("Add ({})",self.counter);
        if let Some(canvas) = self.get_control_mut(h) {
            canvas.set_caption(new_text.as_str());
        }
    }
}

impl canvasEvents for MyWin {
    fn on_pressed(&mut self, canvas_handle: Handle<canvas>) -> EventProcessStatus {
        if canvas_handle == self.add {
            // 'Add' canvas was pressed - lets increment the counter
            self.counter += 1;
            self.update_add_canvas_caption();
            return EventProcessStatus::Processed;
        }
        if canvas_handle == self.reset {
            // 'Reset` canvas was pressed - counter will become 0
            self.counter = 0;
            self.update_add_canvas_caption();
            return EventProcessStatus::Processed;
        }
        // unknown handle - we'll ignore this event
        EventProcessStatus::Ignored
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWin::new());
    app.run();
    Ok(())
}
```