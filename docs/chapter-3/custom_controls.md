# Custom controls

While the existing stock controls should suffice for most apps, there is sometimes a need to create a custom control. This can be done using a special attribute macro, `#[CustomControl(...)]`, as follows:

```rs
#[CustomControl(...)]
struct MyCustomControl {
    // additional fields
}
```

A custom control accepts the following attributes (via the `#[CustomControl(...)]` macro):
* `events` with two possible values or combinations: [MenuEvents](../chapter-4/menu.md) and/or [CommandBarEvents](../chapter-4/command_bar.md):
    ```rs
    #[CustomControl(events = MenuEvent+CommandBarEvent)]
    struct MyCustomControl {
        // additional fields
    }
    ```
* `overwrite` to allow one to overwrite certain traits (for painting or resizing):
    ```rs
    #[CustomControl(overwrite = OnPaint+OnResize)]
    struct MyCustomControl {
        // additional fields
    }
    ```
* `emit` to describe a list of events that the current control can emit towards the event loop:
    ```rs
    #[CustomControl(emit = Player1Wins+Player2Wins+GameOver)]
    struct MyCustomControl {
        // additional fields
    }
    ```
* `commands` (as described in the [Commands](../chapter-4/commands.md) section)

## A simple example

The following example creates a simple custom control with the `X` character written in `Yellow` over `Red` background and a `White` double border.

```rs
use appcui::prelude::*;

#[CustomControl(overwrite = OnPaint)]
struct MyControl {}
impl MyControl {
    fn new(layout: Layout) -> Self {
        Self { base: ControlBase::new(layout, true) }
    }
}

impl OnPaint for MyControl {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(char!("'X',Yellow,DarkRed"));
        let size = self.size();
        surface.draw_rect(
            Rect::with_point_and_size(Point::ORIGIN, size),
            LineType::Double,
            CharAttribute::with_fore_color(Color::White),
        );
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("caption:'Custom Control',a:c,w:30,h:10");
    w.add(MyControl::new(layout!("l:1,t:1,r:1,b:1")));
    a.add_window(w);
    a.run();
    Ok(())
}
```

**Remarks:** Notice that a new data member `base` has been created by the `#[CustomControl]` macro. This data member provides all standard methods that every control has (related to visibility, enablement, etc.). This data member must be instantiated in one of the following two ways:

```rs
ControlBase::new(layout: Layout, accept_input: bool)
```

or

```rs
ControlBase::with_focus_overlay(layout: Layout)
```


where:
* `layout` is the [Layout](../chapter-3/layout.md) of the control
* `accept_input` is either **true** if we want the new custom control to receive events from the mouse and/or keyboard, or **false** otherwise (the latter case is usual when a control similar to a label is being created). 

The second method (`ControlBase::with_focus_overlay`) is used when we want to create a custom control that will extend its size one character to the bottom and one character to the right.