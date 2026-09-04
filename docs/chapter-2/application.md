# Application

An application in `AppCUI` is the **context** that holds all of the framework data together (it keeps all controls, passes messages between controls, manages terminals and system events). There can be only one application per program that uses `AppCUI` (this is enforced by the framework: subsequent attempts to create an application will fail).

To create an application, one of the following constructors can be used:
1. `App::new()`. This creates a builder for a classic desktop with one or more windows and chooses the best-fit terminal available on the current operating system.

2. `App::single_window(factory)`. This creates a builder for an application with exactly one window that fills the desktop. The `factory` is invoked once, after the runtime is created, and must return that window. It can be a closure (`|| window!("Demo,d:f")`) or a function / constructor with no arguments (`MyWin::new`).

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

Using a constructor such as `App::new` , `App::single_window`, `App::frame_app` or `App::input_app` creates a different builder object that can further be used to set up how the application will be constructed. For example, you can change the terminal size, colors, font, etc. using this object. Keep in mind that not all settings apply to each terminal, and using the wrong configuration might lead to an initialization error. 

Regarless of the app type, the following methods are available for each builder:
* `.size(terminal_size)` to set up a terminal size
* `.title(terminal_title)` to set up a terminal title
* `.backend(backend_type)` to select a specific terminal backend
* `.log_file(path,append)` to set up a log file where logs will be displayed. This option will only be valid in **debug mode**. Once the file was specified, any call to [log!](logging.md) macro will be recorded in that file.
* `.color_schema(enabled)` if set this flag will try to use the terminal color schema, otherwise it will use AppCUI predefined values (e.g. for `Color::DarkBlue` will use `RGB(0,0,128)`). This flag is enabled by default.
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
        .color_schema(false)          // use AppCUI predefined colors
        .restore_screen(true)         // restore original screen when finished
        .window(|| {                  // add the following window
            window!("'Demo',a:c,w:40,h:10")
        })
        .run()
}
```

## Errors

If the `.run()` method from the Builder object fails, an error is returned. You can use the `.kind` member to identify the type of error. Currently, the following error classes are provided:
* `ErrorKind::InitializationFailure` — a failure occurred when initializing the backend API (this is usually due to some OS constraints).
* `ErrorKind::InvalidFeature` — an invalid feature (configuration option) that is not compatible with the current terminal was used. For example, an attempt to set up DirectX for the NCurses backend will be invalid.
* `ErrorKind::InvalidParameter` — a valid feature but with invalid parameters was used. For example, an attempt to instantiate a terminal with the size of **(0x0)** will trigger such an error.

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
2. Register one or more windows with the `.window(...)` method (or pass the window factory to `App::single_window(...)`). The factory can be a closure (`.window(|| { ... })`) or a function / constructor with no arguments (`.window(MyWin::new)`).
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
