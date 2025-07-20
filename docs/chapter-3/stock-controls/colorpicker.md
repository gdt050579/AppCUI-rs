# ColorPicker

Represent a control from where you can choose a color:

<img src="img/colorpicker.png" width=300/>

To create a color picker use `ColorPicker::new` method (with 2 parameters: a color and a layout).
```rs
let c = ColorPicker::new(Color::Green, layout!("x:10,y:5,w:15"));
```
or the macro `colorpicker!`
```rs
let c1 = colorpicker!("color=Red,x:10,y:5,w:15");
let c2 = colorpicker!("Darkgreen,x:10,y:5,w:15");
let c3 = colorpicker!("Yellow,x:10,y:5,w:15,visible:false");
```

A ColorPicker control supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name | Type   | Positional parameter                | Purpose                                                                                                                                                                                                                                               |
| -------------- | ------ | ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `color`        | String | **Yes** (first postional parameter) | The color of the ColorPicker control (should be one of the following values: `Black`, `DarkBlue`, `DarkGreen`, `Teal`, `DarkRed`, `Magenta`, `Olive`, `Silver`,   `Gray`, `Blue`, `Green`, `Aqua`, `Red`, `Pink`, `Yellow`, `White` or `Transparent`) |

## Events
To intercept events from a ColorPicker control, the following trait has to be implemented to the Window that processes the event loop:
```rs
pub trait ColorPickerEvents {
    fn on_color_changed(&mut self, handle: Handle<ColorPicker>, color: Color) -> EventProcessStatus {...}
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a ColorPicker control also has the following aditional methods:

| Method           | Purpose                                                                                  |
| ---------------- | ---------------------------------------------------------------------------------------- |
| `set_color(...)` | Manually sets the color of the ColorPicker control. It receives an object of type Color. |
| `color()`        | Returns the current color selected in the ColorPicker control                            |


## Key association

The following keys are processed by a ColorPicker control if it has focus:

| Key                           | Purpose                                                                                                                                                                                                                           |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Space` or `Enter`            | Expands or packs (collapses) the ColorPicker control.                                                                                                                                                                             |
| `Up`, `Down`, `Left`, `Right` | Changes the current selected color from the ColorPicker. Using this keys will trigger a call to `ColorPickerEvents::on_color_changed(...)`                                                                                        |
| `Escape`                      | Only when the ColorPicker is expanded, it collapses the control. If the ColorPicker is already colapsed, this key will not be captured (meaning that one of the ColorPicker ancestors will be responsable with treating this key) |

## Example

The following example creates a Window with a ColorPicker and a label. Every time the color from the ColorPicker is being changed, the label caption will be modified with the name of the new color.

```rust,no_run
#[Window(events = ColorPickerEvents)]
struct MyWin {
    c: Handle<ColorPicker>,
    l: Handle<Label>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: Window::new("Test", layout!("a:c,w:40,h:10"), window::Flags::None),
            c: Handle::None,
            l: Handle::None,
        };
        win.l = win.add(label!("'',x:1,y:1,w:30,h:1"));
        win.c = win.add(colorpicker!("Black,x:1,y:3,w:30"));
        win
    }
}

impl ColorPickerEvents for MyWin {
    fn on_color_changed(&mut self, _handle: Handle<ColorPicker>, color: Color) -> EventProcessStatus {
        let h = self.l;
        if let Some(label) = self.control_mut(h) {
            label.set_caption(color.get_name());
            return EventProcessStatus::Processed;
        }
        return EventProcessStatus::Ignored;
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWin::new());
    app.run();
    Ok(())
}

```