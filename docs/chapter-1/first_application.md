# First Application

Let's start by building a simple window that prints `Hello World` 
on the screen.

Firts, make sure that you have the following dependency added in your
project `cargo.toml` file:

```ini
[dependencies]
appcui = <version>
```

Then, replace your `main.rs` with the following snippet:
```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut win = Window::new(
        "First Window",
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
or using macros to compact the code:
```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut win = window!("'First Window',a:c,w:30,h:9");
    win.add(label!("'Hello World !',a:c,w:13,h:1"));
    app.add_window(win);
    app.run();
    Ok(())
}
```

After compiling and executing this code you should see something like this:

<img src="img/hello_world.png" width=300/>

**Remarks**: Keep in mind that depending on your terminal and other settings this image might look differently.
