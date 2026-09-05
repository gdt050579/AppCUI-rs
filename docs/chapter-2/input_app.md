# InputApp Mode

An input app is an AppCUI application that paints the full surface and handles keyboard and mouse events directly, with no windows, menus, or stock controls. You implement [`InputApp`](#trait) and pass an instance to `App::input_app(...)`. The result is an [`InputAppBuilder`](application.md#builder) that you configure and then start with `.run()`.

Unlike [FrameApp mode](frame_app.md), there is **no periodic tick**. The surface is painted at start (and after a resize), and again only when a key or mouse handler returns `EventProcessStatus::Processed`. Use this mode when the screen should sit idle until the user does something.

```rs
struct HelloWorld;
impl InputApp for HelloWorld {
    fn on_paint(&self, surface: &mut Surface) {
        surface.write_string(0, 0, "Hello World !", charattr!("white"), false);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::input_app(HelloWorld {}).run()
}
```

Only `on_paint` is required. The other trait methods have defaults that return `EventProcessStatus::Ignored`. You draw directly on the [surface](surface.md); clip and origin are already reset before each paint.

Pressing `Escape` closes the application by default.

## Additional constructor methods

Besides the methods described in [Builder](application.md#builder) the following methods are available for an input app mode setup:
* `.auto_close(enabled)` to close the application when `Escape` is pressed. This is **enabled** by default. When disabled, `Escape` is forwarded to `on_key_event`.
* `.clear_char(ch)` to set the character used to fill the surface before each paint. The default is a space with a white foreground and a black background. Pass `None` to skip the fill and only reset clip and origin (the previous frame stays on screen).

There is no `.fps(...)` method. This builder also does not have `.window(...)`, `.desktop(...)`, `.app_bar()`, `.command_bar()`, `.theme(...)`, or `.timers_count(...)`. An input app owns the whole surface.

## Trait

Each input app should implement the following trait:


```rs
pub trait InputApp {
    fn on_start(&mut self) {}
    fn on_resize(&mut self, new_size: Size) {}
    fn on_key_event(&mut self, key: Key, ch: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_mouse_event(&mut self, ev: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
    fn on_paint(&self, surface: &mut Surface);
    fn on_close(&mut self) -> ActionRequest { ActionRequest::Allow }
}
```

| Method           | Called when                                                                                                                                                                                                 |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `on_start`       | Once, after the application is created and before the first paint. Use it to initialize state.                                                                                                              |
| `on_resize`      | When the terminal size changes. The runtime always repaints after a resize, so this method has no return value.                                                                                             |
| `on_key_event`   | When a key is pressed. Return `EventProcessStatus::Processed` to request a repaint, or `EventProcessStatus::Ignored` if nothing visible changed. If `.auto_close(true)` (the default), `Escape` closes the application **before** this method is called. See [Keyboard](keyboard.md). |
| `on_mouse_event` | When a mouse event occurs (move, press, release, drag, or wheel). Return `EventProcessStatus::Processed` to request a repaint. See [Mouse](mouse.md).                                                       |
| `on_paint`       | After the optional clear from `.clear_char(...)`. This method is `&self`; mutate state in the input handlers.                                                                                               |
| `on_close`       | When the application is about to close. Return `ActionRequest::Allow` to close, or `ActionRequest::Deny` to keep running.                                                                                   |

## Differences between FrameApp and InputApp

|                                 | FrameApp                                                                 | InputApp                                                                                          |
| ------------------------------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------- |
| Constructor                     | `App::frame_app(...)`                                                    | `App::input_app(...)`                                                                             |
| Update loop                     | Timer at `.fps(...)` (default 30)                                        | None. The app waits for input                                                                     |
| `on_update`                     | Called every frame with a tick count                                     | Not part of the trait                                                                             |
| When `on_paint` runs            | Every frame, after `on_update`                                           | At start, after a resize, and when a handler returns `EventProcessStatus::Processed`              |
| `on_key_event` / `on_mouse_event` | No return value. The next timer tick paints anyway                     | Return `EventProcessStatus` to decide whether to repaint                                          |
| `.fps(...)`                     | Yes (`1..=120`)                                                          | No                                                                                                |
| `.auto_close(...)`              | Yes (default `true`)                                                     | Yes (default `true`)                                                                              |
| `.clear_char(...)`              | Yes                                                                      | Yes                                                                                               |
| Typical use                     | Games, animations, clocks, anything that must move while idle            | Editors, menus, puzzles, or tools that only change when the user presses a key or clicks          |

If you need a timed animation (for example cards that flip back after a delay), use FrameApp. If the screen should not refresh until the next key or click, use InputApp.

## Remarks

* An input app has no window chrome. You are responsible for drawing every character you want on the screen.
* `on_paint` takes `&self`. Keep mutable state on the struct and update it in `on_key_event` or `on_mouse_event`.
* Return `EventProcessStatus::Processed` only when something visible changed. Returning `Ignored` leaves the current surface on screen and avoids extra paints.
* There can be only one application per process. A second `App::input_app(...).run()` while another instance is running returns an error.

## Example

The following example prints a status line. Press `Enter` or click the surface to update it. `Escape` closes the application.

```rs
use appcui::prelude::*;

struct Demo {
    message: String,
}
impl InputApp for Demo {
    fn on_key_event(&mut self, key: Key, _ch: char) -> EventProcessStatus {
        if key.value() == key!("Enter") {
            self.message = "Enter pressed".to_string();
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }

    fn on_mouse_event(&mut self, ev: &MouseEvent) -> EventProcessStatus {
        if let MouseEvent::Pressed(data) = ev {
            self.message = format!("Click {},{}", data.x, data.y);
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }

    fn on_paint(&self, surface: &mut Surface) {
        surface.write_string(1, 1, "Input app", charattr!("aqua,black"), false);
        surface.write_string(1, 3, &self.message, charattr!("white,black"), false);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::input_app(Demo {
        message: "Waiting...".to_string(),
    })
    .run()
}
```
