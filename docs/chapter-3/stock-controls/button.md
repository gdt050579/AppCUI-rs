# Button

Represent a clickable button control. To create a button use `Button::new` method.

**Example**:
```rs
let b = Button::new("&Start", Layout::new("..."),botton::Flags::None);
```

## Events

## Example

The following code creates a window with two buttons (`Add` and `Reset`). When `Add` button is pressed a number is incremented and set as the text of the `Add` button. When `Reset` button is being pressed, the counter is reset to 0.

```rs
use appcui::prelude::*;

#[Window(events = ButtonEvents, internal=true)]
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