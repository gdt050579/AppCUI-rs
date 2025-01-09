# Application

An application in `AppCUI` is the **context** that holds all of the framework data together (it keeps all controls, passes messages between controls, manages terminals and system events). There can be only one application per program that uses `AppCUI` (this is enforced by the framework: subsequenct attempts to create an application will fail).

To create an application three APIs can be used:
1. `App::new()`. This will create an application and chose the best fit terminal available on the current operating system. The result of `new()` method is a `Builder` object that can further be used to configure how the terminal looks like.

2. `App::with_terminal(terminal_type)`. This will create `Builder` object, but you will chose the terminal to be used instead of having one chosed for you automatically. You can check more on terminals availability and types on section [Terminals](terminals.md)

3. `App::debug(width, height, script)`. This is designed to help you with unit testing (see more on this way of initializing `AppCUI` on section [Debug scenarios](debug_scenarious.md)) 

**Example** (using the default terminal):
```rs
let mut a = App::new().build().expect("Fail to create an AppCUI application");
```

**Example** (using the windows terminal):
```rs
let mut a = App::with_terminal(TerminalType::WindowsConsole).build().expect("Fail to create an AppCUI application with WindowsConsole terminal");
```

## Builder

Using `App::new` or `App::with_terminal` creates a builder object that can further be used set up how the application will be constructed. For example, you can change the terminal size, colors, font, etc using this object. Keep in mind that not all settings apply for each terminal, and using the wrong configuration might led to an initialization error. Curently, the Builder supports the following methods:
* `.size(terminal_size)` to set up a terminal size
* `.title(terminal_title)` to set up a terminal title
* `.desktop(custom_desktop)` if you want to use a custom desktop instead of the default one
* `.single_window()` if you want a single window application
* `.menu_bar()` to enable the application top menu bar
* `.command_bar()` to enable the application command bar
* `.theme(custom_theme)` to set up a custom theme or another predefined theme. Read more on themes in section [Themes](chapter-6/themes.md)
* `.timers_count(count)` to set up the number of timers that can be used in the application (if not specified the default value is 4)

After setting up the configuration for an application, just call the `build()` method to create an application. This methods returns a result of type `Result<App,Error>` from where the appcui application can be obtained via several methods such as:
* `unwrap()` or `expect(...)` methods
* `?` opertor
* `if let` construct

A typical example of using this settings is as follows:
```rs
let mut a = App::new().size(Size::new(80,40))  // size should be 80x25 chars
                      .menu_bar()              // top menu bar should be enabled
                      .command_bar()           // command bar should be enabled
                      .build()
                      .expect("Fail to create an AppCUI application");
```

## Errors

If the `.build()` method from the `Builder` object fails, an error is returned. You can use `.kind` member to identify the type of error. Curently, the following error class are provided:
* `ErrorKind::InitializationFailure` a failure occured when initializing the terminal (this is usually due to some OS constranits). 
* `ErrorKind::InvalidFeature` an invalid feature (configuration option) that is not compatible with the current terminal was used. For example, an attemp to set up DirectX for NCurses terminal will be invalid.
* `ErrorKind::InvalidParameter` a valid feature but with invalid parameters was used. For example, an attempt to instantiate a terminal with the size of **(0x0)** will trigger such an error.

To get a more detailed description of the Error, use the `description()` method from class `Error` just like in the next code snipped:
```rs
let result = App::new().size(Size::new(0,0)).build();
if let Err(error) = result {
    // we have an Error - let's print it
    println!("Fail to instantiate AppCUI");
    println!("Error: {}",error.description());
}
```

## Execution flows

Usually, each AppCUI program consists in the following steps:
1. create an application
2. add on or multiple windows to that application (to do this use the `add_window` method from struct `App`)
3. run the application via method `run`. This method consumes the object and as such you can not use the application anymore after this method ends.

A typical `main.rs` file that uses `AppCUI` framework looks like this:
```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    // 1. build an application
    let mut app = App::new().build()?;
    // 2. add one or several windows
    app.add_window(/* a window */);
    // 3. run the aplication
    app.run();
    Ok(())
}
```