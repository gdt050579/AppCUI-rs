# KeySelector

Represent a control that can be used to select a key (including modifiers such as `Alt`, `Shift`, ...)

<img src="img/keyselector.png" width=300/>

To create a keyselector use `KeySelector::new` method (with 3 parameters: a key, a layout and initialization flags).
```rs
let k = KeySelector::new(Key::from(KeyCode::F1), Layout::new("x:10,y:5,w:15"),keyselector::Flags::None);
```
or the macro `keyselector!`
```rs
let b1 = keyselector!("F1,x:10,y:5,w:15");
let b2 = keyselector!("key:Ctrl+Alt+F1,x:10,y:5,w:15,flags:ReadOnly");
```

A keyselector supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name | Type   | Positional parameter                | Purpose                                                        |
| -------------- | ------ | ----------------------------------- | -------------------------------------------------------------- |
| `key`          | String | **Yes** (first postional parameter) | The key (including modifiers such as `Alt`,`Ctrl` or `Shift`)  |
| `flags`        | String | **No**                              | Initialization flags that describe the behavior of the control |

A keyselector supports the following initialization flags:
* `keyselector::Flags::AcceptEnter` or `AcceptEnter` (for macro initialization) - this will intercept the `Enter` key (with or without modifiers) if pressed while the control has focus
* `keyselector::Flags::AcceptEscape` or `AcceptEscape` (for macro initialization) - this will intercept the `Escape` key (with or without modifiers) if pressed while the control has focus. Enabling this feture  must be done carefully, as `Escape` key is used by window or desktop to exit and intercepting it might change this behavior.
* `keyselector::Flags::AcceptTab` or `AcceptTab` (for macro initialization) - this will intercept the `Tab` key (with or without modifiers) if pressed while the control has focus. Be carefull when intercepting this key, as it is being used to switch between controls.
* `keyselector::Flags::ReadOnly` or `ReadOnly` (for macro initialization) - this will set the internal state of the control to a read-only state (meaning that keys are being intercepted, but the selection of the new key will not be possible).


Some examples that uses these paramateres:
```rs
let intercept_enter = keyselector!("Enter,x:10,y:5,w:15,flags=AcceptEnter");
let readonly_all_keys = keyselector!("x:1,y:1,w:10,flags:[AcceptEnter,AcceptTab,AcceptEscape,ReadOnly]");
```

## Events
To intercept events from a keyselector, the following trait has to be implemented to the Window that processes the event loop:
```rs
pub trait KeySelectorEvents {
    fn on_key_changed(&mut self, handle: Handle<KeySelector>, new_key: Key, old_key: Key) -> EventProcessStatus { ... }
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a keyselector also has the following aditional methods:

| Method         | Purpose                                                                                    |
| -------------- | ------------------------------------------------------------------------------------------ |
| `set_key(...)` | Set the new key for a keyselector. You can also use `Key::None` here to infer no selection |
| `key()`        | Returns the current key of a keyselector                                                   |

## Key association

There are no specific key associations (all keys are intercepted expect for `Enter`, `Escape` and `Tab` that can be intercepted if some flags are set).

## Example

The following code creates a window with a keyselector and a button that can be used to reset the keyselector key to `None`.

```rs
use appcui::prelude::*;
#[Window(events = ButtonEvents+KeySelectorEvents)]
struct MyWin {
    reset: Handle<Button>,
    ks: Handle<KeySelector>,
    lb: Handle<Label>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'Key Selector example',a:c,w:40,h:9"),
            reset: Handle::None,
            ks: Handle::None,
            lb: Handle::None,
        };
        win.reset = win.add(button!("&Reset,x:50%,y:6,a:c,w:15"));
        win.ks = win.add(keyselector!("x:1,y:3,w:36"));
        win.lb = win.add(label!("<none>,x:1,y:1,w:35"));
        win
    }
    fn update_info(&mut self) {
        let key = self.control(self.ks).map(|obj| obj.key()).unwrap_or(Key::None);
        let s = if key == Key::None {
            "<none>".to_string()
        } else {
            format!("New key is: {}{}", key.modifier.name(), key.code.name())
        };
        let h = self.lb;
        if let Some(label) = self.control_mut(h) {
            label.set_caption(&s);
        }
    }
}

impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
        // reset button was pressed
        let h = self.ks;
        if let Some(k) = self.control_mut(h) {
            k.set_key(Key::None);
        }
        self.update_info();
        EventProcessStatus::Processed
    }
}
impl KeySelectorEvents for MyWin {
    fn on_key_changed(&mut self, _handle: Handle<KeySelector>, _new_key: Key, _old_key: Key) -> EventProcessStatus {
        self.update_info();
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWin::new());
    app.run();
    Ok(())
}
```