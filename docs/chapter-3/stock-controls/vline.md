# VLine

Represent a vertical line:

<img src="img/vline.png" width=300/>

To create a vertical line use `VLine::new` method (with 2 parameters: a layout and a set of flags). The flags let you choose if it is a double line.
```rs
let a = VLine::new(Layout::new("x:1,y:1,h:10"), Flags::None);
let b = VLine::new(Layout::new("x:3,y:1,h:20"), Flags::DoubleLine);
```
or the macro `vline!`
```rs
let hl1 = vline!("x:1,y:1,h:10");
let hl2 = vline!("x:3,y:1,h:20,flags:DoubleLine");
```

A vertical line supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                | Type   | Positional parameter                | Purpose                                 |
| ----------------------------- | ------ | ----------------------------------- | --------------------------------------- |
| `flags`                       | Enum   | **No**                              | DoubleLine to draw a double line        |



## Events
A vertical line emits no events.

## Methods
A vertical line emits no aditional methods.

## Key association
A vertical line does not receive any input and as such it has no key associated with it.

## Example

The following code creates a window with a vertical line that contains the text `Hello world !`.
```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:20"), window::Flags::None);
    
    w.add(VLine::new(Layout::new("x:3,y:1,h:15"), Flags::DoubleLine));
    app.add_window(w);
    app.run();
    Ok(())
}
```