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