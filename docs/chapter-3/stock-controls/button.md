# Button

Represent a clickable button control:

<img src="img/button.png" width=300/>

To create a button use `Button::new` method (with 3 parameters: a caption, a layout and initialization flags).
```rs
let b = Button::new("&Start", Layout::new("x:10,y:5,w:15"),botton::Flags::None);
```
or the macro `button!`
```rs
let b1 = button!("caption=&Start,x:10,y:5,w:15");
let b2 = button!("&Start,x:10,y:5,w:15");
```

The caption of a button may contain the special character `&` that indicates that the next character is a hot-key. For example, constructing a button with the following caption `&Start` will set up the text of the button to `Start` and will set up character `S` as the hot key for that button (pressing `Alt+S` will be equivalent to pressing that button).

A button supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name | Type | Positional parameter  | Purpose |
|----------------|------|-----------------------|---------|
| `name` or `text` or `caption` | String | **Yes** (first postional parameter) | The caption (text) written on a button |
| `flags` | String or List| **No** | Button initialization flags |

A button supports the following initialization flags:
* `button::Flags::Flat` or `flat` (for macro initialization) - thils will hide the shaddow of the button makeing it flat.

Some examples that uses these paramateres:
```rs
let disabled_button = button!("caption=&Disabled,x:10,y:5,w:15,enable=false");
let hidden_button = button!("text='&Hidden',x=9,y:1,align:center,w:9,visible=false");
let flat_button = button!("&flat,x:1,y:1,w:10,flags:flat");
```

## Events
To intercept events from a button, the following trait has to be implemented to the Window that processes the event loop:
```rs
pub trait ButtonEvents {
    fn on_pressed(&mut self, button: Handle<Button>) -> EventProcessStatus {...}
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a button also has the following aditional methods:

| Method             | Purpose                                                                             |
|--------------------|-------------------------------------------------------------------------------------|
| `set_caption(...)` | Set the new caption for a button. If the string provided contains the special character `&`, this method also sets the hotkey associated with a control. If the string provided does not contain the `&` character, this method will clear the current hotkey (if any).<br>Example: `button.set_caption("&Start")` - this will set the caption of the button cu `Start` and the hotket to `Alt+S` |
| `get_caption()`    | Returns the current caption of a button |

## Key association

The following keys are processed by a Button control if it has focus:

| Key           | Purpose                                                                             |
|---------------|-------------------------------------------------------------------------------------|
| `Space`       | Clicks / pushes the button and emits `ButtonEvents::on_pressed(...)` event. It has the same action clicking the checkbox with the mouse.  |
| `Enter`       | Clicks / pushes the button and emits `ButtonEvents::on_pressed(...)` event. It has the same action clicking the checkbox with the mouse.  |

Aditionally, `Alt`+**letter or number** will have the same action (even if the Button does not have a focus) if that letter or nunber was set as a hot-key for a button via its caption. For example, creating a value with the following caption: `"My b&utton"` (notice the `&` character before letter `u`) will enable `Alt+U` to be a hot-key associated with this button. Pressing this combination while the button is enabled and part of the current focused window, will change the focus to that button and will emit the `ButtonEvents::on_pressed(...)` event.

## Example

The following code creates a window with two buttons (`Add` and `Reset`). When `Add` button is pressed a number is incremented and set as the text of the `Add` button. When `Reset` button is being pressed, the counter is reset to 0.

```rs
use appcui::prelude::*;

#[Window(events = ButtonEvents)]
struct MyWin {
    add: Handle<Button>,
    reset: Handle<Button>,
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
        win.add = win.add(Button::new("Add (0)", Layout::new("x:25%,y:2,w:13,a:c"), button::Flags::None));
        win.reset = win.add(Button::new("&Reset", Layout::new("x:75%,y:2,w:13,a:c",), button::Flags::None));
        win
    }
    fn update_add_button_caption(&mut self) {
        let h = self.add;
        let new_text = format!("Add ({})",self.counter);
        if let Some(button) = self.get_control_mut(h) {
            button.set_caption(new_text.as_str());
        }
    }
}

impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, button_handle: Handle<Button>) -> EventProcessStatus {
        if button_handle == self.add {
            // 'Add' button was pressed - lets increment the counter
            self.counter += 1;
            self.update_add_button_caption();
            return EventProcessStatus::Processed;
        }
        if button_handle == self.reset {
            // 'Reset` button was pressed - counter will become 0
            self.counter = 0;
            self.update_add_button_caption();
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