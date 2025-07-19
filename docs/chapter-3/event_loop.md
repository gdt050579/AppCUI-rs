# Event loop

`AppCUI` is an event driven framework, meaning that each control can emit events to reflect various acions or changes that occur. For example, whenever you push a button, an event will be raise. All events are process at Window level by implementing various traits. To build a Window that supports event handling, you must use a special procedural macro call `Window`, defined in the the following way:

```rs
#[Window(events=..., )]
struct MyWindow {
    // specific fields
}
```

where the attribute `events` has the following form:
* `events=EventTrait-1 + EventTrait-2 + EventTrait-3 + ... EventTrait-n`

and an `event trait` can be one of the following:
* ButtonEvents
* CheckBoxEvents
* RadioBoxEvents
* WindowEvents
* MenuEvents
* CommandBarEvents
* ToolBarEvents
* ColorPickerEvents
* ThreeStateBoxEvents
* PasswordEvents
* KeySelectorEvents
* TextFieldEvents


These events can be implemented to receive notification on various actions that children controls are performing. 

When creating a window that supports event loop in this manner, you will need to instantiate it. A common approach is the following:
```rs
#[Window(events=..., )]
struct MyWindow {
    // specific fields
}
impl MyWindow {
    fn new(/* extra parameters */) -> Self {
        let mut obj = MyWindow {
            base: Window::new(title, layout, flags);
            // initialization other fileds from MyWindow i
        }
        // other initialization (such as creating children)
        return obj;
    }
}
```

The initializaton `base: Window::new(title, layout, flags);` is mandatory. As for the `title`, `layout` and `flags` you can provide them as parameters in the **new** method or you can infer them / or hardcode them in a different way. More on how a Window can be created on [Window](event-loop/window.md) page.

Once you create an event loop you can add it to your application using `add_window(...)` method.
```rs
fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWindow::new(/* parameters */));
    app.run();
    Ok(())
}
```

## A simple example

Let's start with a simple example that creates such a window that has a fixed sized of `40x20` characters and two internal `i32` values.

```rs
use appcui::prelude::*;

#[Window()]
struct MyWindow {
    value1: i32,
    value2: i32
}
impl MyWindow {
    fn new(title: &str) -> Self {
        MyWindow {
            base: Window::new(title, Layout::new("a:c,w:40,h:20"), window::Flags::None);
            value1: 0,
            value2: 1
        }
    }
}
fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWindow::new("Some title"));
    app.run();
    Ok(())
}
```

## Intercepting events from a child control

Usually, a window that processes events mentains a handle to various controls and enable event processing in the `#[Window(...)]` declaration.

```rs
use appcui::prelude::*;

#[Window(events = /*Events specific to a control */)]
struct MyWindow {
    value1: i32,
    control: Handle</*control type*/>
}
impl MyWindow {
    fn new(/* parameters */) -> Self {
        let mut mywin = MyWindow {
            base: Window::new(/*...*/);
            control: Handle::None
        }
        // now we create the control
        mywin.control =  mywin.add(/* Code that creates a control */);

        return mywin;
    }
}
impl /*Control event*/ for MyWindow {
    // add logic for event
}
fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWindow::new(/* parameters */));
    app.run();
    Ok(())
}
```

For every control described in [Stock Controls](stock_controls.md) an example on how that control can be used with the event loop and the type of events it emits will be presented.
