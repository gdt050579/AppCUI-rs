# Application

An application in `AppCUI` is the **context** that holds all of the framework data together (it keeps all controls, passes messages between controls, manages terminals and system events). There can be only one application per program that uses `AppCUI` (this is enforced by the framework: subsequent attempts to create an application will fail).

To create an application, one of the following constructors can be used:
1. `App::new()`. This creates a builder for a classic desktop with one or more windows and chooses the best-fit terminal available on the current operating system.

2. `App::main_window(factory)`. This creates a builder for an application with exactly one window that fills the desktop. The `factory` is invoked once, after the runtime is created, and must return that window. It can be a closure (`|| window!("Demo,d:f")`) or a function / constructor with no arguments (`MyWin::new`).

3. `App::frame_app(frame_app)`. This creates a builder for games or animations that paint the full surface every frame. The provided type must implement `FrameApp`.

4. `App::input_app(input_app)`. This creates a builder for custom paint and input without window chrome. The provided type must implement `InputApp`.

The result of each constructor is a builder that can further be used to configure how the terminal looks. After configuration, call `.run()` to start the event loop.

To pick a specific backend instead of the default one, chain `.backend(backend_type)` on the builder. You can read more about backend availability and types in the [Backends](backends.md) section.

For unit testing, chain `.size(...)` and `.debug_script(script)` on the builder (see more in the [Debug scenarios](debug_scenarious.md) section).

**Example** (using the default backend):
```rs
fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| window!("'First Window',a:c,w:30,h:9"))
        .run()
}
```

**Example** (using the windows console backend):
```rs
fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .backend(appcui::backend::Type::WindowsConsole)
        .window(|| window!("'First Window',a:c,w:30,h:9"))
        .run()
}
```

## Builder

Using a constructor such as `App::new` , `App::main_window`, `App::frame_app` or `App::input_app` creates a different builder object that can further be used to set up how the application will be constructed. For example, you can change the terminal size, colors, font, etc. using this object. Keep in mind that not all settings apply to each terminal, and using the wrong configuration might lead to an initialization error. 

Regarless of the app type, the following methods are available for each builder:
* `.size(terminal_size)` to set up a terminal size
* `.title(terminal_title)` to set up a terminal title
* `.backend(backend_type)` to select a specific terminal backend
* `.log_file(path,append)` to set up a log file where logs will be displayed. This option will only be valid in **debug mode**. Once the file was specified, any call to [log!](logging.md) macro will be recorded in that file.
* `.palette(palette)` to choose the RGB values of the 16 named colors. The default is `Palette::Terminal`. See [Palette](#palette).
* `.palette_as_rgb(enabled)` to paint those named colors as 24-bit RGB. This is **disabled** by default, and it requires `Palette::BuiltIn` or `Palette::Custom`. See [Palette](#palette).
* `.restore_screen(enabled)` if set the backend will attempt to restore the original screen status (content, cursor position, etc.) as it was before the application started. This option is **enabled** by default. Keep in mind that not all backends have this kind of support.
* `.debug_script(script)` to simulate input for unit tests. Combine this with `.size(...)` to set the simulated terminal dimensions.

Besides these, each build has aditional methods specific to it.

After setting up the configuration for an application, call the `run()` method to create the runtime, instantiate every registered window, and start the event loop. This method returns a result of type `Result<(),Error>` that can be obtained via several methods such as:
* `unwrap()` or `expect(...)` methods
* `?` operator
* `if let` construct

A typical example of using these settings is as follows:
```rs
fn main() -> Result<(), appcui::system::Error> {
    App::new()                        // multi-window mode
        .size(Size::new(80,40))       // size should be 80x40 chars
        .app_bar()                    // top application bar should be enabled
        .command_bar()                // command bar should be enabled
        .log_file("debug.log", false) // log into debug.log
        .palette(appcui::backend::Palette::BuiltIn)          // use AppCUI predefined colors
        .restore_screen(true)         // restore original screen when finished
        .window(|| {                  // add the following window
            window!("'Demo',a:c,w:40,h:10")
        })
        .run()
}
```

## Palette

Named colors (`Color::Black` through `Color::White`) are drawn from a palette of 16 RGB values. `Color::Transparent` is not part of that table. Two builder methods control the palette. They exist on every application builder, and the order you call them in does not matter: both values are read when `run()` starts.

### `.palette(palette)`

This method chooses **which** RGB values the 16 named colors have.

| Variant | Meaning |
| ------- | ------- |
| `Palette::Terminal` | Default. The backend keeps its current colors and does not install a palette. |
| `Palette::BuiltIn` | Install the standard AppCUI colors (the table below). |
| `Palette::Custom(table)` | Install RGB values you supply. Build the table with `ColorPalette::from_pairs`. |

`Palette::BuiltIn` assigns each variant of the [`Color`](screen.md) enum the RGB value in the table below. These are the same 16 colors described in [Screen area and sizes](screen.md). `Color::Transparent` is documented there as well and is not a palette slot. `Color::Silver` is `(192, 192, 192)`.

| Color      | Enum variant       | RGB                                      | Color                                                                                                        |
| ---------- | ------------------ | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| Black      | `Color::Black`     | Red=**0**, Green=**0**, Blue=**0**       | <div style="width: 24px; height: 24px; background-color: rgb(0, 0, 0);border: 2px solid white;"></div>       |
| Dark Blue  | `Color::DarkBlue`  | Red=**0**, Green=**0**, Blue=**128**     | <div style="width: 24px; height: 24px; background-color: rgb(0, 0, 128);border: 2px solid white;"></div>     |
| Dark Green | `Color::DarkGreen` | Red=**0**, Green=**128**, Blue=**0**     | <div style="width: 24px; height: 24px; background-color: rgb(0, 128, 0);border: 2px solid white;"></div>     |
| Dark Red   | `Color::DarkRed`   | Red=**128**, Green=**0**, Blue=**0**     | <div style="width: 24px; height: 24px; background-color: rgb(128, 0, 0);border: 2px solid white;"></div>     |
| Teal       | `Color::Teal`      | Red=**0**, Green=**128**, Blue=**128**   | <div style="width: 24px; height: 24px; background-color: rgb(0, 128, 128);border: 2px solid white;"></div>   |
| Magenta    | `Color::Magenta`   | Red=**128**, Green=**0**, Blue=**128**   | <div style="width: 24px; height: 24px; background-color: rgb(128, 0, 128);border: 2px solid white;"></div>   |
| Olive      | `Color::Olive`     | Red=**128**, Green=**128**, Blue=**0**   | <div style="width: 24px; height: 24px; background-color: rgb(128, 128, 0);border: 2px solid white;"></div>   |
| Silver     | `Color::Silver`    | Red=**192**, Green=**192**, Blue=**192** | <div style="width: 24px; height: 24px; background-color: rgb(192, 192, 192);border: 2px solid white;"></div> |
| Gray       | `Color::Gray`      | Red=**128**, Green=**128**, Blue=**128** | <div style="width: 24px; height: 24px; background-color: rgb(128, 128, 128);border: 2px solid white;"></div> |
| Blue       | `Color::Blue`      | Red=**0**, Green=**0**, Blue=**255**     | <div style="width: 24px; height: 24px; background-color: rgb(0, 0, 255);border: 2px solid white;"></div>     |
| Green      | `Color::Green`     | Red=**0**, Green=**255**, Blue=**0**     | <div style="width: 24px; height: 24px; background-color: rgb(0, 255, 0);border: 2px solid white;"></div>     |
| Red        | `Color::Red`       | Red=**255**, Green=**0**, Blue=**0**     | <div style="width: 24px; height: 24px; background-color: rgb(255, 0, 0);border: 2px solid white;"></div>     |
| Aqua       | `Color::Aqua`      | Red=**0**, Green=**255**, Blue=**255**   | <div style="width: 24px; height: 24px; background-color: rgb(0, 255, 255);border: 2px solid white;"></div>   |
| Pink       | `Color::Pink`      | Red=**255**, Green=**0**, Blue=**255**   | <div style="width: 24px; height: 24px; background-color: rgb(255, 0, 255);border: 2px solid white;"></div>   |
| Yellow     | `Color::Yellow`    | Red=**255**, Green=**255**, Blue=**0**   | <div style="width: 24px; height: 24px; background-color: rgb(255, 255, 0);border: 2px solid white;"></div>   |
| White      | `Color::White`     | Red=**255**, Green=**255**, Blue=**255** | <div style="width: 24px; height: 24px; background-color: rgb(255, 255, 255);border: 2px solid white;"></div> |

On Windows Console, `BuiltIn` and `Custom` replace the console color table for the lifetime of the application. The previous table is restored when the application ends. On Windows VT, the same values are used for the colors the backend paints. On [Termios](backends/termios.md#palette) and [CrossTerm](backends/crossterm.md#palette), `BuiltIn` and `Custom` are written into the terminal's 16 color slots with OSC 4 when `.palette_as_rgb(false)` (the default), and each character is painted with 24-bit RGB when `.palette_as_rgb(true)`. On exit, those backends send OSC 104 to ask the terminal to restore those slots. That reset depends on the terminal: some terminals, including WSL, leave the new colors in place. NCurses and the Web Terminal ignore the palette.

A custom table lists only the colors you want to change. Each entry is `(color, red, green, blue)`. The table starts from the built-in AppCUI colors above, so a color you leave out keeps that value. A later entry for the same color replaces the earlier one. `Color::Transparent` has no slot and panics.

```rs
use appcui::backend::{ColorPalette, Palette};
use appcui::prelude::*;

fn custom_palette() -> ColorPalette {
    ColorPalette::from_pairs(&[
        (Color::Black, 40, 42, 54),
        (Color::Red, 255, 85, 85),
        (Color::White, 248, 248, 242),
    ])
}

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .palette(Palette::Custom(custom_palette()))
        .window(|| window!("'Demo',a:c,w:40,h:10"))
        .run()
}
```

### `.palette_as_rgb(enabled)`

This method chooses **how** the palette is sent to the terminal.

* `false` (the default). Named colors use the terminal's 16 color slots. `Palette::BuiltIn` and `Palette::Custom` are loaded into those slots. `Palette::Terminal` leaves the slots unchanged.
* `true`. Each character is painted with the 24-bit RGB value from the palette. This needs a palette that actually contains those values, so it must be paired with `Palette::BuiltIn` or `Palette::Custom`.

`palette_as_rgb(true)` together with `Palette::Terminal` (including the case where you never call `.palette(...)`) makes `run()` return `ErrorKind::InvalidParameter`. `palette_as_rgb(false)` with `Palette::Terminal` is the default and is valid.

Windows VT, Termios, and CrossTerm honor this flag. The Windows Console backend ignores it and always uses its 16-color table. The `Palette::Terminal` check still runs for every backend, including ones that ignore the flag.

```rs
fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .palette(appcui::backend::Palette::BuiltIn)
        .palette_as_rgb(true)
        .window(|| window!("'Demo',a:c,w:40,h:10"))
        .run()
}
```

## Errors

If the `.run()` method from the Builder object fails, an error is returned. You can use the `.kind` member to identify the type of error. Currently, the following error classes are provided:
* `ErrorKind::InitializationFailure` — a failure occurred when initializing the backend API (this is usually due to some OS constraints).
* `ErrorKind::InvalidFeature` — an invalid feature (configuration option) that is not compatible with the current terminal was used. For example, an attempt to set up DirectX for the NCurses backend will be invalid.
* `ErrorKind::InvalidParameter` — a valid feature but with invalid parameters was used. For example, an attempt to instantiate a terminal with the size of **(0x0)** will trigger such an error. The same kind is returned when `.palette_as_rgb(true)` is used with `Palette::Terminal` (see [Palette](#palette)).

To get a more detailed description of the error, use the `description()` method from the `Error` class, as in the following code snippet:
```rs
let result = App::new().size(Size::new(0,0)).run();
if let Err(error) = result {
    // we have an Error - let's print it
    println!("Fail to instantiate AppCUI");
    println!("Error: {}",error.description());
}
```

## Execution flow

Usually, each AppCUI program consists of the following steps:
1. Create a builder via `App::new()` (or another constructor).
2. Register one or more windows with the `.window(...)` method (or pass the window factory to `App::main_window(...)`). The factory can be a closure (`.window(|| { ... })`) or a function / constructor with no arguments (`.window(MyWin::new)`).
3. Run the application via the `run` method. This method consumes the builder, so you cannot use it anymore after this method ends.

A typical `main.rs` file that uses `AppCUI` framework looks like this:
```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| /* a window */)
        .run()
}
```

## Static methods

After an application is executed, you can use the following static methods:
* `App::set_theme(theme)` to set up the theme of the application
* `App::close()` to close the application
