# Button

Represent a clickable button control. To create a button use `Button::new` method or the macro `button!`. 
```rs
let b = Button::new("&Start", Layout::new("x:10,y:5,w:15"),botton::Flags::None);
```
or 
```rs
let b1 = button!("caption=&Start,x:10,y:5,w:15");
let b2 = button!("&Start,x:10,y:5,w:15");
```

The caption of a button may contain the special character `&` that indicates a hot-key. For example, constructing a button with the following caption `&Start` will set up the text of the button to `Start` and will set up character `S` as the hot key for that button (pressing `Alt+S` will be equivalent to pressing that button);

A button supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following name parameters are also accepted:

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
        win.reset = win.add(Button::new("Reset", Layout::new("x:75%,y:2,w:13,a:c",), button::Flags::None));
        win
    }
    fn update_add_button(&mut self) {
        let h = self.add;
        let new_text = format!("Add ({})",self.counter);
        if let Some(button) = self.get_control_mut(h) {
            button.set_text(new_text.as_str());
        }
    }
}

impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, button_handle: Handle<Button>) -> EventProcessStatus {
        if button_handle == self.add {
            self.counter+=1;
            self.update_add_button();
            return EventProcessStatus::Processed;
        }
        if button_handle == self.reset {
            self.counter = 0;
            self.update_add_button();
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    a.add_window(MyWin::new());
    app.run();
    Ok(())
}
```