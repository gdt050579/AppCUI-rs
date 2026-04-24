# Application

An application in `AppCUI` is the **context** that holds all of the framework data together (it keeps all controls, passes messages between controls, manages terminals and system events). There can be only one application per program that uses `AppCUI` (this is enforced by the framework: subsequent attempts to create an application will fail).

To create an application, three APIs can be used:
1. `App::new()`. This will create an application and choose the best-fit terminal available on the current operating system. The result of the `new()` method is a `Builder` object that can further be used to configure how the terminal looks.

2. `App::with_backend(backend_type)`. This will create a `Builder` object, but you will choose the backend to be used instead of having one chosen for you automatically. You can read more about backend availability and types in the [Backends](backends.md) section.

3. `App::debug(width, height, script)`. This is designed to help you with unit testing (see more on this way of initializing `AppCUI` in the [Debug scenarios](debug_scenarious.md) section). 

**Example** (using the default backend):
```rs
let mut a = App::new().build().expect("Failed to create an AppCUI application");
```

**Example** (using the windows console backend):
```rs
let mut a = App::with_backend(appcui::backend::WindowsConsole)
                 .build()
                 .expect("Failed to create an AppCUI application with WindowsConsole backend");
```

## Builder

Using `App::new` or `App::with_backend` creates a builder object that can further be used to set up how the application will be constructed. For example, you can change the terminal size, colors, font, etc. using this object. Keep in mind that not all settings apply to each terminal, and using the wrong configuration might lead to an initialization error. Currently, the Builder supports the following methods:
* `.size(terminal_size)` to set up a terminal size
* `.title(terminal_title)` to set up a terminal title
* `.desktop(custom_desktop)` if you want to use a custom desktop instead of the default one
* `.single_window()` if you want a single window application
* `.app_bar()` to enable the application top app bar
* `.command_bar()` to enable the application command bar
* `.theme(custom_theme)` to set up a custom theme or another predefined theme. Read more on themes in the [Themes](chapter-6/themes.md) section.
* `.timers_count(count)` to set up the number of timers that can be used in the application (if not specified the default value is 4)
* `.log_file(path,append)` to set up a log file where logs will be displayed. This option will only be valid in **debug mode**. Once the file was specified, any call to [log!](logging.md) macro will be recorded in that file.
* `.color_schema(enabled)` if set this flag will try to use the terminal color schema, otherwise it will use AppCUI predefined values (e.g. for `Color::DarkBlue` will use `RGB(0,0,128)`). This flag is enabled by default.
* `.restore_screen(enabled)` if set the backend will attempt to restore the original screen status (content, cursor position, etc.) as it was before the application started. This option is **enabled** by default. Keep in mind that not all backends have this kind of support.

After setting up the configuration for an application, just call the `build()` method to create an application. This method returns a result of type `Result<App,Error>` from where the appcui application can be obtained via several methods such as:
* `unwrap()` or `expect(...)` methods
* `?` operator
* `if let` construct

A typical example of using these settings is as follows:
```rs
let mut a = App::new().size(Size::new(80,40))       // size should be 80x40 chars
                      .app_bar()                    // top application bar should be enabled
                      .command_bar()                // command bar should be enabled
                      .log_file("debug.log", false) // log into debug.log
                      .color_schema(false)          // use AppCUI predefined colors
                      .restore_screen(true)         // restore original screen when finished
                      .build()
                      .expect("Failed to create an AppCUI application");
```

## Errors

If the `.build()` method from the `Builder` object fails, an error is returned. You can use the `.kind` member to identify the type of error. Currently, the following error classes are provided:
* `ErrorKind::InitializationFailure` — a failure occurred when initializing the backend API (this is usually due to some OS constraints).
* `ErrorKind::InvalidFeature` — an invalid feature (configuration option) that is not compatible with the current terminal was used. For example, an attempt to set up DirectX for the NCurses backend will be invalid.
* `ErrorKind::InvalidParameter` — a valid feature but with invalid parameters was used. For example, an attempt to instantiate a terminal with the size of **(0x0)** will trigger such an error.

To get a more detailed description of the error, use the `description()` method from the `Error` class, as in the following code snippet:
```rs
let result = App::new().size(Size::new(0,0)).build();
if let Err(error) = result {
    // we have an Error - let's print it
    println!("Fail to instantiate AppCUI");
    println!("Error: {}",error.description());
}
```

## Execution flow

Usually, each AppCUI program consists of the following steps:
1. Create an application.
2. Add one or more windows to that application (use the `add_window` method on struct `App`).
3. Run the application via the `run` method. This method consumes the object, so you cannot use the application anymore after this method ends.

A typical `main.rs` file that uses `AppCUI` framework looks like this:
```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    // 1. build an application
    let mut app = App::new().build()?;
    // 2. add one or several windows
    app.add_window(/* a window */);
    // 3. run the application
    app.run();
    Ok(())
}
```