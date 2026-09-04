# Multi Window Mode

A multi-window mode setup is the default mode of AppCUI and creates a classic desktop that can host one or more independent windows. Start it with `App::new()`. The result is a [`MultiWindowAppBuilder`](application.md#builder) that you configure and then start with `.run()`.

In this mode each window has its own title bar, layout, and focus. Windows can overlap, move, and (when created with the `Sizeable` flag) resize. Closing one window does not close the application. You can also start with no windows at all and add them later from a [custom desktop](../chapter-4/custom_desktop.md).

```rs
fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| window!("'First Window',a:c,w:30,h:9"))
        .run()
}
```

Register every startup window with `.window(factory)`. The factory is a `FnOnce() -> T` that runs after the runtime is created and must return a non-modal window (`T` implements `Control`, `WindowControl`, and `NotModalWindow`). Call `.window(...)` once per window. Windows are created in the order they were registered.

The factory can be a **closure** or a **function**:

1. A **closure** — use this when you build the window inline, add children, or `new` takes arguments (`|| MyWin::new("Title")`):

```rs
fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'Demo',a:c,w:40,h:10");
            win.add(label!("'Hello World !',a:c,w:13,h:1"));
            win
        })
        .window(|| window!("'Second Window',x:2,y:2,w:24,h:8"))
        .run()
}
```

2. A **function** or constructor with no arguments — pass `MyWin::new` (no parentheses). `fn() -> T` implements `FnOnce() -> T`, so the constructor is invoked later, after the runtime exists. Do **not** write `MyWin::new()` here; that would create the window immediately.

```rs
fn main() -> Result<(), appcui::system::Error> {
    App::new().window(MyWin::new).run()
}
```

A free function works the same way: `.window(hello_world_window)`.

Read more about creating windows and handling their events in the [Window](../chapter-3/event-loop/window.md) section.

## Additional constructor methods

Besides the methods described in [Builder](application.md#builder) the following methods are available for a multi-window mode setup:
* `.desktop(custom_desktop)` if you want to use a custom desktop instead of the default one. This method is only available on the multi-window builder. Read more in [Custom desktops](../chapter-4/custom_desktop.md).
* `.window(factory)` to register a window that will be created when the application starts (multi-window mode; call once per window). `factory` can be a closure (`.window(|| { ... })`) or a function / constructor with no arguments (`.window(MyWin::new)`)
* `.app_bar()` to enable the application top app bar. Read more in [Application bar](../chapter-4/app_bar.md).
* `.command_bar()` to enable the application command bar. Read more in [Command bar](../chapter-4/command_bar.md).
* `.theme(custom_theme)` to set up a custom theme or another predefined theme. Read more on themes in the [Themes](../chapter-6/themes.md) section.
* `.timers_count(count)` to set up the number of timers that can be used in the application (if not specified the default value is 4)

## Desktop behavior

The default [desktop](../chapter-4/desktop.md) is created automatically. It fills the terminal (minus the app bar and command bar, if they are enabled) and manages the window list.

| Key                             | Purpose                                                                                          |
| ------------------------------- | ------------------------------------------------------------------------------------------------ |
| `Tab` or `Ctrl+Tab`             | Moves the focus to the next window                                                               |
| `Shift+Tab` or `Ctrl+Shift+Tab` | Moves the focus to the previous window                                                           |
| `Escape`                        | Closes the desktop and the application (unless a custom desktop denies this in `on_close`)       |
| `Alt`+{hotkey}                  | Moves the focus to the window that has that hotkey (see [Window hot keys](../chapter-3/event-loop/window.md#window-hot-key)) |

A window can also be activated by clicking it. Closing the last window leaves the desktop running; press `Escape` on the desktop (or call `App::close()`) to exit.

To add windows after the application has started, use a custom desktop and call `add_window(...)` from `on_start` or from a desktop command. The default desktop has no UI for creating windows on its own.

## Remarks

* Multi-window mode is the opposite of [single-window mode](single_window.md). Here you can register several windows, use a custom desktop, and allow windows to move or resize.
* `.window(...)` is optional. A desktop-only application (for example one that creates windows from menus or the command bar) is valid:
    ```rs
    App::new().desktop(MyDesktop::new()).app_bar().command_bar().run()
    ```
* The factory passed to `.window(...)` must return a **non-modal** window. Open modal dialogs later from a running window, not from the builder.
* Window flags such as `Sizeable` and `FixedPosition` are allowed. In single-window mode `Sizeable` is not.
* There can be only one application per process. A second `App::new().run()` while another instance is running returns an error.

## Example

The following example starts two windows on the default desktop. The first is centered and contains a label. The second is placed in the top-left corner and has an `Alt+2` hotkey so you can switch to it from the keyboard.

```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .size(Size::new(80, 24))
        .window(|| {
            let mut win = window!("'First Window',a:c,w:30,h:9");
            win.add(label!("'Hello World !',a:c,w:13,h:1"));
            win
        })
        .window(|| {
            let mut win = window!("'Second Window',x:2,y:2,w:28,h:8");
            win.set_hotkey(key!("Alt+2"));
            win.add(label!("'Press Alt+2 to focus me',a:c,w:24,h:1"));
            win
        })
        .run()
}
```

A more complete application usually uses a `#[Window(...)]` type so that child-control events are handled on the window. If `new` takes arguments, wrap the call in a closure. If `new` takes none, pass `CounterWindow::new` directly:

```rs
use appcui::prelude::*;

#[Window(events = ButtonEvents)]
struct CounterWindow {
    value: i32,
    label: Handle<Label>,
}
impl CounterWindow {
    fn new(title: &str, layout: Layout) -> Self {
        let mut w = Self {
            base: Window::new(title, layout, window::Flags::Sizeable),
            value: 0,
            label: Handle::None,
        };
        w.label = w.add(label!("'Count: 0',a:c,w:16,h:1"));
        w.add(button!("Increment,x:50%,y:100%,w:14,p:b"));
        w
    }
}
impl ButtonEvents for CounterWindow {
    fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
        self.value += 1;
        let h = self.label;
        if let Some(lbl) = self.control_mut(h) {
            lbl.set_caption(&format!("Count: {}", self.value));
        }
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| CounterWindow::new("Left", layout!("x:2,y:2,w:32,h:8")))
        .window(|| CounterWindow::new("Right", layout!("x:36,y:2,w:32,h:8")))
        .run()
}
```
