# Single Window Mode

A single-window mode is an AppCUI application that hosts exactly one window, docked to fill the desktop. Start it with `App::single_window(factory)`. The result is a [`SingleWindowAppBuilder`](application.md#builder) that you configure and then start with `.run()`.

This mode is meant for tools that do not need a window list, a custom desktop, or overlapping frames. The factory is invoked once, after the runtime is created, and must return a non-modal window (`T` implements `Control`, `WindowControl`, and `NotModalWindow`). Closing that window ends the application.

```rs
fn main() -> Result<(), appcui::system::Error> {
    App::single_window(|| window!("title:'Demo',d:f"))
        .size(Size::new(40, 10))
        .run()
}
```

The factory can be a function or a closure. Building children inside the factory is the usual pattern:

```rs
fn main() -> Result<(), appcui::system::Error> {
    App::single_window(|| {
        let mut win = window!("title:'Demo',d:f");
        win.add(label!("'Hello World !',a:c,w:13,h:1"));
        win
    })
    .run()
}
```

The layout you pass to the window is ignored. AppCUI replaces it so that the window occupies the entire visible desktop (leaving room for the [application bar](../chapter-4/app_bar.md) and [command bar](../chapter-4/command_bar.md) when they are enabled). Using `d:f` (docked, fill) in the `window!` macro makes that intent explicit.

Read more about creating windows and handling their events in the [Window](../chapter-3/event-loop/window.md) section, and about this layout in [Single Window Apps](../chapter-3/event-loop/single_window.md).

## Additional constructor methods

Besides the methods described in [Builder](application.md#builder) the following methods are available for a single-window mode setup:
* `.app_bar()` to enable the application top app bar. Read more in [Application bar](../chapter-4/app_bar.md).
* `.command_bar()` to enable the application command bar. Read more in [Command bar](../chapter-4/command_bar.md).
* `.theme(custom_theme)` to set up a custom theme or another predefined theme. Read more on themes in the [Themes](../chapter-6/themes.md) section.
* `.timers_count(count)` to set up the number of timers that can be used in the application (if not specified the default value is 4)

There is no `.window(...)` method on this builder. The one window is the factory passed to `App::single_window(...)`. There is also no `.desktop(...)` method; a custom desktop can only be set in [multi-window mode](multi_window.md).

## Window behavior

The single window is created with the `FixedPosition` flag. It cannot be moved or resized, and it has no maximize or resize grip. The visible area is the terminal minus the app bar (top) and the command bar (bottom), if those bars are enabled.

Closing the window (the close button, `Escape`, or `close()`) closes the application. You can intercept that with `WindowEvents::on_cancel` and return `ActionRequest::Deny` if you need a confirmation dialog.

Because there is only one window, desktop actions such as `Tab` / `Ctrl+Tab` window cycling and window hotkeys have no effect.

## Remarks

* Single-window mode is the opposite of [multi-window mode](multi_window.md). Here you cannot register a second window, use a custom desktop, or allow the window to move or resize.
* `.desktop(...)` is **not** available on this builder. A custom desktop can only be set in multi-window mode, via `App::new().desktop(...)`.
* The `Sizeable` window flag is **not** allowed. If it is set, the code panics:
    ```rs
    // the following line will panic
    App::single_window(|| window!("Test,a:c,flags: Sizeable")).run()
    ```
* The factory passed to `App::single_window(...)` must return a **non-modal** window. Open modal dialogs later from the running window, not from the factory.


## Example

The following example is a small calculator that fills the terminal. Closing the window asks for confirmation.

```rs
use appcui::prelude::*;

#[Window(events = ButtonEvents+WindowEvents)]
struct MyWindow {
    info: Handle<Label>,
    number: Handle<TextField>,
}
impl MyWindow {
    fn new() -> Self {
        let mut w = Self {
            base: window!("title:'Square root',d:f"),
            info: Handle::None,
            number: Handle::None,
        };
        w.number = w.add(textfield!("l:1,t:1,r:1,h:1"));
        w.info = w.add(label!("'',l:1,t:3,r:1,h:1"));
        w.add(button!("Compute,x:50%,y:100%,w:20,p:b"));
        w
    }
}
impl ButtonEvents for MyWindow {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        let value: Option<f64> = if let Some(txt) = self.control(self.number) {
            txt.text().parse().ok()
        } else {
            None
        };
        let h = self.info;
        if let (Some(v), Some(i)) = (value, self.control_mut(h)) {
            i.set_caption(format!("SQRT({})={}", v, v.sqrt()).as_str());
        }
        EventProcessStatus::Processed
    }
}
impl WindowEvents for MyWindow {
    fn on_cancel(&mut self) -> ActionRequest {
        if dialogs::validate("Close", "Do you want to close the application ?") {
            ActionRequest::Allow
        } else {
            ActionRequest::Deny
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::single_window(|| MyWindow::new()).size(Size::new(40, 10)).run()
}
```
