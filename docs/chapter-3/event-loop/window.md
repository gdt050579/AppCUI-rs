# Window

A window is the core component of an application and it is the object where all events from children controls are being processed.

<img src="img/window.png"/>

To create a Window use `Window::new` method (with 3 parameters: a title, a layout and initialization flags). Keep in mind that window will **NOT** handle any events from its children. 
```rs
let w = Window::new("Title", Layout::new("x:10,y:5,w:15,h:9"),window::Flags::None);
```

To create a window that will handle events from its children, use `#[Window(...)]` method:
```rs
#[Window(events=..., )]
struct MyWindow {
    // specific fields
}
```


A window supports the following initialization flags:
* `window::Flags::None` - regular window (with a close button)
* `window::Flags::Sizeable` - a window that has the resize grip and the maximize button
* `window::Flags::NoCloseButton` - a window without a close button
* `window::Flags::FixedPosition` - a window that can not be moved

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a button also has the following aditional methods:

| Method                 | Purpose                                                                             |
|------------------------|-------------------------------------------------------------------------------------|
| `add(...)`             | Adds a new control as a child control for current window |
| `get_contol(...)`      | Returns an immutable reference to a control based on its handle |
| `get_contol_mut(...)`  | Returns a mutable reference to a control based on its handle |
| `get_toolbar()`        | Returns a mutable reference to current window toolbar |
| `set_title(...)`       | Sets the title of Window.<br>Example: `win.set_title("Title")` - this will set the title of the window to `Title` |
| `get_title()`          | Returns the title of the current window |
| `set_tag(...)`         | Sets the tag of Window.<br>Example: `win.set_tag("ABC")` - this will set the tag of the window to `ABC` |
| `get_tag()`            | Returns the tag of the current window |
| `clear_tag()`          | Clears the current tag. Its equivalent to `set_tag("")` |

## Key association

In terms of key association, a Window has two modes:
* Normal mode (for whem a window is has focus)
* Resize/Move mode (in this mode you can use arrows and various combinations to move the window)

### For normal mode

| Key                   | Purpose                                                                             |
|-----------------------|-------------------------------------------------------------------------------------|
| `Ctrl`+`Alt`+`M` or <br> `Ctrl`+`Alt`+`R` | Switch the window to resize/move mode                               |

### For resize/move mode

| Key                   | Purpose                                                                             |
|-----------------------|-------------------------------------------------------------------------------------|
| `Escape`              | Switch back to the normal mode                                                      |
