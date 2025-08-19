# AppCUI-rs

<img src="https://raw.githubusercontent.com/gdt050579/AppCUI-rs/main/docs/chapter-1/img/logo.png" align="center" />

```                                                              
⯈ 𝗔𝗽𝗽𝗖𝗨𝗜-𝗿𝘀 🖳
```

![Windows Build Status](https://github.com/gdt050579/AppCUI-rs/actions/workflows/windows.yml/badge.svg)
![Linux Build Status](https://github.com/gdt050579/AppCUI-rs/actions/workflows/linux.yml/badge.svg)
![MacOS Build Status](https://github.com/gdt050579/AppCUI-rs/actions/workflows/macos.yml/badge.svg)
![Code Coverage](https://gist.githubusercontent.com/gdt050579/f7d7e7d56b2725a3b33a265e8a9d8e9c/raw/coverage.svg)
![License](https://img.shields.io/github/license/gdt050579/AppCUI-rs)
[![Crates.io](https://img.shields.io/crates/v/appcui.svg)](https://crates.io/crates/appcui)
[![Docs.rs](https://docs.rs/appcui/badge.svg)](https://docs.rs/appcui)
[![Gallery](https://img.shields.io/badge/See-Gallery-blue?style=for-the-badge)](./Gallery.md)

AppCUI-rs is a fast, cross-platform Rust library for building modern, text-based user interfaces (TUIs) with rich widgets, themes, and full Unicode support—an alternative to ncurses and other terminal UI frameworks.
* [Book](https://gdt050579.github.io/AppCUI-rs/)
* [Documentation](https://docs.rs/appcui)


## ✨ Features
- [x] multiple out-of-the-box controls (buttons, labels, text boxes, check boxes, radio buttons, list views, tree views, combo boxes, date/time pickers, color pickers, tabs, accordeons, etc.). A full list of controls can be found [here](https://gdt050579.github.io/AppCUI-rs/chapter-3/stock_controls.html) 
- [x] powerful layout system that allows you to position controls using absolute coordinates, relative coordinates, docking, alignment, anchors or pivot positioning (see more [here](https://gdt050579.github.io/AppCUI-rs/chapter-3/layout.html)) 
- [x] menus and toolbars
- [x] multi-platform support (Windows via API and virtual terminal, Linux via ncurses, MacOS via termios)
- [x] multi-threading support to allow background tasks
- [x] timers
- [x] mouse support
- [x] clipboard support
- [x] color themes
- [x] support for Unicode characters
- [x] predefined dialogs (message box, input box, color picker, save & open dialogs, folder navigator, etc)
- [x] true colors support (24 bits per pixel) for terminals that supports it.

## 📸 Screenshots 

<img src="https://github.com/gdt050579/AppCUI-rs/raw/main/docs/chapter-1/img/appcui-rs-demo.gif" align="center" />

👉 **Check out the [Gallery](./Gallery.md) for full demos of all controls!**

## 🖥️ Backends

AppCUI supports various backends based on the operating system it is being used for:

* **Windows Console** - based on Win32 low level API, design for clasical windows console
* **Windows VT** - based on ANSI sequances, designed for modern windows virtual terminals
* **NCurses** - based on NCurses API for linux envinronments
* **Termios** - based on ANSI sequances and low level APIs for MAC OSX
* **Web Terminal** - designed for Web implementation (based on webgl)
* **CrossTerm** - based on the `crossterm` crate, but enabled via a feature flag

More on the supported backends can be found [here](https://gdt050579.github.io/AppCUI-rs/chapter-2/backends.html)


## 🚀 Quick Start

Add the following to your `Cargo.toml`:

```toml
[dependencies]
appcui = "*"
```

Then create a new Rust project and add the following code:

```rust
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut win = Window::new(
        "Test",
        LayoutBuilder::new().alignment(Alignment::Center).width(30).height(9).build(),
        window::Flags::Sizeable,
    );
    win.add(Label::new(
        "Hello World !",
        LayoutBuilder::new().alignment(Alignment::Center).width(13).height(1).build(),
    ));
    app.add_window(win);
    app.run();
    Ok(())
}
```

or a more compact version using proc-macros:

```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut win = window!("Test,a:c,w:30,h:9");
    win.add(label!("'Hello World !',a:c,w:13,h:1"));
    app.add_window(win);
    app.run();
    Ok(())
}
```

Then run the project with `cargo run`. You should see a window with the title `Test` and the text `Hello World !` in the center.

## 🧪 Examples

AppCUI-rs comes with a set of examples to help you get started. You can find them in the [examples](examples) folder, including:
- **Games** such as [Tic Tac Toe](examples/tic-tac-toe/), [Snake](examples/snake/), [Flappy Bird](examples/flappy), [Minesweeper](examples/minesweeper/) or [Ram it](examples/ramit/)
- **Utilities** such as [Calculator](examples/calculator/), [CSV Viewer](examples/csv_viewer/), [Temperature Converter](examples/temperature_convertor/) or a [Timer](examples/timer/)
- **Animations** such as [Matrix](examples/matrix/), [Fractals](examples/fractals/) or [Spiral](examples/spiral/)
- **Controls**/**Widgets** such as [Button](examples/buttons/), [CheckBox](examples/checkboxes/), [ComboBox](examples/combobox/), [DatePicker](examples/datepicker/), [ListView](examples/listview/), [TreeView](examples/treeview/) and many more.
- **Dialogs** such as [Notification](examples/notification_dialogs/) or [Input](examples/input_dialog/)

## 🛠️ A more complex example

Am example that creates a window with a button that when pressed increases a counter.

```rust
use appcui::prelude::*;

// Create a window that handles button events and has a counter
#[Window(events = ButtonEvents)]
struct CounterWindow {
    counter: i32
}

impl CounterWindow {
    fn new() -> Self {
        let mut w = Self {
            // set up the window title and position
            base: window!("'Counter window',a:c,w:30,h:5"),
            // initial counter is 1
            counter: 1            
        };
        // add a single button with the caption "1" (like the counter)
        w.add(button!("'1',d:b,w:20"));
        w
    }
}
impl ButtonEvents for CounterWindow {
    // When the button is pressed, this function will be called
    // with the handle of the button that was pressed
    // Since we only have one button, we don't need to store its handle 
    // in the struct, as we will receive the handle via the on_pressed method
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        // increase the counter
        self.counter += 1;
        // create a text that containe the new counter
        let text = format!("{}",self.counter);
        // aquire a mutable reference to the button using its handle
        if let Some(button) = self.control_mut(handle) {
            // set the caption of the button to th new text
            button.set_caption(&text);
        }
        // Tell the AppCUI framework that we have processed this event
        // This allows AppCUI to repaint the button
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    // create a new application
    let mut a = App::new().build()?;
    // add a new window (of type CounterWindow) to the application
    a.add_window(CounterWindow::new());
    // Run AppCUI framework (this wil start the window loop and messaage passing)
    a.run();
    Ok(())
}
```

## 🛣️ Roadmap

- [x] Basic set of widgets and support for Windows, Linux and MacOS
- [x] WebGL support
- [ ] OpenGL / SDL / Vulkan support
- [ ] TextArea support for code highlighting

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!  
Check out [CONTRIBUTING.md](CONTRIBUTING.md) to get started.

Join the discussion in [GitHub Discussions](https://github.com/gdt050579/AppCUI-rs/discussions).
