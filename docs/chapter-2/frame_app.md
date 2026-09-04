# FrameApp Mode

A frame app is an AppCUI application that paints the full surface on a fixed timer. There are no windows, menus, or stock controls. You implement [`FrameApp`](#trait) and pass an instance to `App::frame_app(...)`. The result is a [`FrameAppBuilder`](application.md#builder) that you configure and then start with `.run()`.

Use this mode for games, animations, visualizations, or anything that must redraw on a clock (a clock display, a sprite loop, a simulation). For custom drawing that only needs to refresh after a key or mouse event, use [InputApp mode](input_app.md) instead.

```rs
struct HelloWorld;
impl FrameApp for HelloWorld {
    fn on_paint(&self, surface: &mut Surface) {
        surface.write_string(0, 0, "Hello World !", charattr!("white"), false);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::frame_app(HelloWorld {}).fps(30).run()
}
```

Only `on_paint` is required. The other trait methods have empty defaults. You draw directly on the [surface](surface.md); clip and origin are already reset before each paint.

The loop is: `on_start` once, then at the configured frame rate `on_update(ticks)` followed by `on_paint`. Keyboard and mouse events are delivered in between ticks. Pressing `Escape` closes the application by default.

## Additional constructor methods

Besides the methods described in [Builder](application.md#builder) the following methods are available for a frame app mode setup:
* `.fps(count)` to set the frame rate. The value is clamped to `1..=120`. The default is **30**.
* `.auto_close(enabled)` to close the application when `Escape` is pressed. This is **enabled** by default. When disabled, `Escape` is forwarded to `on_key_event`.
* `.clear_char(ch)` to set the character used to fill the surface before each paint. The default is a space with a white foreground and a black background. Pass `None` to skip the fill and only reset clip and origin (the previous frame stays on screen).

This builder does not have `.window(...)`, `.desktop(...)`, `.app_bar()`, `.command_bar()`, `.theme(...)`, or `.timers_count(...)`. A frame app owns the whole surface.

## Trait

Each frame app should implement the following trait:



```rs
pub trait FrameApp {
    fn on_start(&mut self) {}
    fn on_resize(&mut self, new_size: Size) {}
    fn on_update(&mut self, ticks: u64) {}
    fn on_key_event(&mut self, key: Key, ch: char) {}
    fn on_mouse_event(&mut self, ev: &MouseEvent) {}
    fn on_paint(&self, surface: &mut Surface);
    fn on_close(&mut self) -> ActionRequest { ActionRequest::Allow }
}
```

| Method                  | Called when                                                                                                                                                          |
| ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `on_start`              | Once, after the application is created and before the first paint. Use it to initialize state.                                                                       |
| `on_resize`             | When the terminal size changes. `new_size` is the new surface size in character cells.                                                                               |
| `on_update`             | Once per frame, before `on_paint`. `ticks` is the number of timer ticks since the application started. Drive animations from here.                                   |
| `on_key_event`          | When a key is pressed. If `.auto_close(true)` (the default), `Escape` closes the application **before** this method is called. See [Keyboard](keyboard.md).          |
| `on_mouse_event`        | When a mouse event occurs (move, press, release, drag, or wheel). See [Mouse](mouse.md).                                                                             |
| `on_paint`              | After `on_update` (and after the optional clear from `.clear_char(...)`). This method is `&self`; mutate state in `on_update` or in the input handlers.              |
| `on_close`              | When the application is about to close. Return `ActionRequest::Allow` to close, or `ActionRequest::Deny` to keep running.                                            |

## Remarks

* A frame app has no window chrome. You are responsible for drawing every character you want on the screen.
* `on_paint` takes `&self`. Keep mutable state on the struct and update it in `on_update`, `on_key_event`, or `on_mouse_event`.
* `on_key_event` and `on_mouse_event` do **not** return `EventProcessStatus`. The surface is painted on the next timer tick regardless of whether you handled the event.
* There can be only one application per process. A second `App::frame_app(...).run()` while another instance is running returns an error.

## Example

The following example paints the current time in the center of the screen and refreshes once per second.

```rs
use appcui::prelude::*;
use chrono::Local;

struct Clock;

impl FrameApp for Clock {
    fn on_paint(&self, surface: &mut Surface) {
        let size = surface.size();
        let txt = Local::now().format("%H:%M:%S").to_string();
        surface.write_string(
            (size.width / 2 - 4) as i32,
            (size.height / 2) as i32,
            &txt,
            charattr!("white"),
            false,
        );
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::frame_app(Clock {}).fps(1).run()
}
```
