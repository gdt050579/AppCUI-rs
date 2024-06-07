# HLine

Represent a horizontal line:

<img src="img/hline.png" width=300/>

To create a horizontal line use `HLine::new` method (with 3 parameters: a title, a layout and a set of flags). The flags let you choose if the line has text or if it is a double line.
```rs
let a = HLine::new("TestLine", Layout::new("x:1,y:3,w:30"), Flags::None);
let b = HLine::new("TestLine", Layout::new("x:1,y:3,w:30"), Flags::DoubleLine | Flags::HasTitle);
```
or the macro `hline!`
```rs
let hl1 = hline!("x:1,y:1,w:10");
let hl2 = hline!("TestLine,x:1,y:3,w:30,flags:DoubleLine+HasTitle");
```

A horizontal line supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                | Type   | Positional parameter                | Purpose                                 |
| ----------------------------- | ------ | ----------------------------------- | --------------------------------------- |
| `text` or `title`             | String | **Yes** (first postional parameter) | The title (text) written on the line    |
| `flags`                       | Enum   | **No**                              | HasText in order to set if the line has text or DoubleLine to draw a double line    |



## Events
A horizontal line emits no events.

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a horizontal line also has the following aditional methods:

| Method             | Purpose                                  |
| ------------------ | ---------------------------------------- |
| `set_title(...)`   | Set the new title for a horizontal line. |
| `title()`          | Returns the current title of a label     |

## Key association

A horizontal line does not receive any input and as such it has no key associated with it.

## Example

The following code creates a window with a horizontal line that contains the text `Hello world !`.
```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    
    w.add(HLine::new("Hello world !", Layout::new("x:1,y:3,w:30"), Flags::DoubleLine | Flags::HasTitle));
    app.add_window(w);
    app.run();
    Ok(())
}
```