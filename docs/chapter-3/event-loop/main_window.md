# Main Window Apps

A main window app is an AppCUI application with one window that fills the desktop. Build that window with `MainWindow::new()`, or with a struct marked `#[MainWindow(...)]` whose `base` field is initialized with `MainWindow::new()`. Pass the factory to `App::main_window(...)`. When `WindowEvents::on_cancel` allows it, `Escape` closes the app.

`window!(...)` and `#[Window]` build a desktop window for [multi-window mode](../../chapter-2/multi_window.md). `App::main_window` does not accept them. `MainWindow::new()` takes no title, layout, or flags.

The factory can be a **closure** or a **function** / constructor with no arguments (`MyWin::new`):
```rs
App::main_window(|| {
    let mut win = MainWindow::new();
    win.add(label!("'Hello',a:c,w:5,h:1"));
    win
}).run()

// equivalent when `new` takes no arguments:
App::main_window(MyWin::new).run()
```

A custom window declares its own fields and handles events through `#[MainWindow]`:
```rs
#[MainWindow(events = ButtonEvents+WindowEvents)]
struct MyWin {
    info: Handle<Label>,
}
impl MyWin {
    fn new() -> Self {
        Self {
            base: MainWindow::new(),
            info: Handle::None,
        }
    }
}
```

The `events` list works the same way as for [`#[Window]`](../event_loop.md). The full setup, including the app bar, the command bar, and a worked example, is in [Main Window Mode](../../chapter-2/main_window.md).

## Remarks
* in a `Main Window` mode you can not set a custom desktop. `.desktop(...)` is only available on the multi-window builder (`App::new()`).
* Since in a `Main Window` mode there is only one window, the factory passed to `App::main_window(...)` is invoked once and must return that window. You can not register a second window.
* `MainWindow::new()` takes no layout. The window is borderless, has no title bar and no close button, and always fills the visible desktop (leaving room for the app bar and the command bar when they are enabled). It cannot be moved or resized.
