# Main Window Mode

A main-window mode is an AppCUI application that hosts exactly one window, docked to fill the desktop. Start it with `App::main_window(factory)`. The result is a [`MainWindowAppBuilder`](application.md#builder) that you configure and then start with `.run()`.

This mode is meant for tools that do not need a window list, a custom desktop, or overlapping frames. The factory is invoked once, after the runtime is created, and must return a `MainWindow` or a struct declared with `#[MainWindow]`. `Escape` ends the application when `WindowEvents::on_cancel` allows it.

The factory can be a **closure** or a **function**:

1. A **closure** — use this when you build the window inline, add children, or `new` takes arguments (`|| MyWin::new(start_value)`):

```rs
fn main() -> Result<(), appcui::system::Error> {
    App::main_window(|| {
        let mut win = MainWindow::new();
        win.add(label!("'Hello World !',a:c,w:13,h:1"));
        win
    })
    .size(Size::new(40, 10))
    .run()
}
```

2. A **function** or constructor with no arguments — pass `MyWin::new` or `MainWindow::new` (no parentheses). The constructor is invoked later, after the runtime exists. Do **not** write `MyWin::new()` here; that would create the window immediately.

```rs
fn main() -> Result<(), appcui::system::Error> {
    App::main_window(MyWin::new).size(Size::new(40, 10)).run()
}
```

A free function works the same way: `App::main_window(hello_world_window)`.

`MainWindow::new()` takes no arguments. You do not pass a title, a layout, or window flags. The window is borderless and always fills the visible desktop (leaving room for the [application bar](../chapter-4/app_bar.md) and the [command bar](../chapter-4/command_bar.md) when they are enabled). Set the terminal title with `.title(...)` on the builder.

`window!(...)` and `Window::new(...)` build a desktop window for [multi-window mode](multi_window.md). `App::main_window` does not accept them.

A custom window is a struct marked with `#[MainWindow(...)]`. The attribute inserts `base: MainWindow` as the first field. Initialize that field with `MainWindow::new()`:

```rs
#[MainWindow(events = ButtonEvents)]
struct MyWin {
    info: Handle<Label>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = Self {
            base: MainWindow::new(),
            info: Handle::None,
        };
        win.info = win.add(label!("'Hello',a:c,w:5,h:1"));
        win
    }
}
```

`#[Window]` and `base: window!(...)` do not produce a main window. The `events` list works the same way as for [`#[Window]`](../chapter-3/event_loop.md): name the child-control traits you implement yourself.

Read more about this layout in [Main Window Apps](../chapter-3/event-loop/main_window.md).

## Additional constructor methods

Besides the methods described in [Builder](application.md#builder) the following methods are available for a main-window mode setup:
* `.app_bar()` to enable the application top app bar. Read more in [Application bar](../chapter-4/app_bar.md).
* `.command_bar()` to enable the application command bar. Read more in [Command bar](../chapter-4/command_bar.md).
* `.theme(custom_theme)` to set up a custom theme or another predefined theme. Read more on themes in the [Themes](../chapter-6/themes.md) section.
* `.timers_count(count)` to set up the number of timers that can be used in the application (if not specified the default value is 4)

There is no `.window(...)` method on this builder. The one window is the factory passed to `App::main_window(...)`. There is also no `.desktop(...)` method; a custom desktop can only be set in [multi-window mode](multi_window.md).

## Window behavior

`MainWindow::new()` creates a borderless window with no title bar, no close button, and a fixed position. It cannot be moved or resized, and it has no maximize button or resize grip. The visible area is the terminal minus the app bar (top) and the command bar (bottom), if those bars are enabled.

`Escape` calls `WindowEvents::on_cancel`. If that method returns `ActionRequest::Allow` (the default), the application exits. Return `ActionRequest::Deny` when you need a confirmation dialog. `Enter` calls `WindowEvents::on_accept`.

Because there is only one window, desktop actions such as `Tab` / `Ctrl+Tab` window cycling and window hotkeys have no effect.

## Remarks

* Main-window mode is the opposite of [multi-window mode](multi_window.md). Here you cannot register a second window, use a custom desktop, or give the window a layout.
* `.desktop(...)` is **not** available on this builder. A custom desktop can only be set in multi-window mode, via `App::new().desktop(...)`.
* The factory passed to `App::main_window(...)` must return a `MainWindow` or a `#[MainWindow]` struct. Open modal dialogs later from the running window, not from the factory.


## Example

The following example is a small calculator that fills the terminal. Closing the window asks for confirmation. The caption `"Square root"` is the terminal title, set with `.title(...)`.

```rs
use appcui::prelude::*;

#[MainWindow(events = ButtonEvents+WindowEvents)]
struct MyWindow {
    info: Handle<Label>,
    number: Handle<TextField>,
}
impl MyWindow {
    fn new() -> Self {
        let mut w = Self {
            base: MainWindow::new(),
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
    App::main_window(MyWindow::new).title("Square root").size(Size::new(40, 10)).run()
}
```
