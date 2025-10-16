# Panel

Represent a panel (a container that can have multiple children):

<img src="img/panel.png" width=300/>

To create a panel use `Panel::new` method (with 2 parameters: a title and a layout) or method `Panel::with_type` with 3 parameters (a title, a layout and the panel type).
```rs
let p1 = Panel::new("My panel", layout!("x:10,y:5,w:15"));
let p2 = Panel::with_type("My panel", layout!("x:10,y:5,w:15"), panel::Type::Border);
```
or the macro `panel!`
```rs
let p1 = panel!("caption='a panel',x:10,y:5,w:15");
let p2 = panel!("MyPanel,x:10,y:5,w:15,type:Border");
```

A panel supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                 | Type   | Positional parameter                | Purpose                                                               |
| ------------------------------ | ------ | ----------------------------------- | --------------------------------------------------------------------- |
| `title` or `text` or `caption` | String | **Yes** (first postional parameter) | The title of the panel                                                |
| `type`                         | String | **No**                              | Panel type. If not provided, **Border** type is considered as default |

A pabel supports the following types:
* `panel::Type::Border` or `border` (for macro initialization) - this will create a panel surrounded by a border (with the title left allined).
* `panel::Type::Window` or `window` (for macro initialization) - this will create a panel surrounded by a border (with the title centered allined).
* `panel::Type::Page` or `page` (for macro initialization) - this will create a panel without any border or title
* `panel::Type::TopBar` or `topbar` (for macro initialization) - this will create a panel with a top bar and centered titled
* `panel::Type::Raised` or `raised` (for macro initialization) - this will create 3D panel (with a raised look)
* `panel::Type::Sunken` or `sunken` (for macro initialization) - this will create 3D panel (with a sunken look)



## Events
A panel emits no events.

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a button also has the following aditional methods:

| Method           | Purpose                                                                                                                             |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `set_title(...)` | Set the new title of the panel                                                                                                      |
| `title()`        | Returns the current title of the panel                                                                                              |
| `panel_type()`   | Returns type of the panel                                                                                                           |
| `add(...)`       | Adds a new control as a child for the panel. It returns a handle for the new control or `Handle::None` if the control was not added |

## Key association

A panel does not receive any input and as such it has no key associated with it.

## Example

The following code creates a panel with the title `Options`.
```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = Window::new("Title", layout!("a:c,w:40,h:10"), window::Flags::None);
    w.add(Panel::new("Options", layout!("l:1,t:1,r:1,b:2")));
    app.add_window(w);
    app.run();
    Ok(())
}
```