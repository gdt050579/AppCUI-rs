# Custom containers

While the existing stock containers ([panel](../stock-controls/panel.md), [tab](../stock-controls/tab.md), [accordion](../stock-controls/accordion.md), etc.) should suffice for most apps, there are times when you need to create a custom container that can hold and manage other controls. 

This can be done using a special macro: `#[CustomContainer(...)]` as follows:
```rs
#[CustomContainer(...)]
struct MyCustomContainer {
    // additional fields
}
```

A custom container accepts the same attributes as custom controls (via `#[CustomContainer(...)]` macro):
* `events` with two possible values or combinations: [MenuEvents](../../chapter-4/menu.md) and/or [CommandBarEvents](../../chapter-4/command_bar.md):
    ```rs
    #[CustomContainer(events = MenuEvent+CommandBarEvent)]
    struct MyCustomContainer {
        // additional fields
    }
    ```
* `overwrite` to allow one to overwrite certain traits (for painting or resizing):
    ```rs
    #[CustomContainer(overwrite = OnPaint+OnResize)]
    struct MyCustomContainer {
        // additional fields
    }
    ```
* `emit` to describe a list of events that the current container can emit towards the event loop:
    ```rs
    #[CustomContainer(emit = DataChanged+ItemSelected+ContainerResized)]
    struct MyCustomContainer {
        // additional fields
    }
    ```
* `commands` (as they are described in [Commands](../../chapter-4/commands.md) section)

## Difference between Custom Controls and Custom Containers

While both `#[CustomControl]` and `#[CustomContainer]` generate very similar code, they serve different conceptual purposes:

- **Custom Controls** are typically leaf elements in the UI hierarchy - they represent individual interactive or display elements (buttons, labels, input fields, etc.)
- **Custom Containers** are designed to hold and manage other controls - they act as layout managers or grouping elements that can contain child controls

When creating a custom container, two aditional methods are available:

| Method             | Description                                                                                                                                                                                      |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `add(...)`         | Adds a child control to the container (much like the methods from a panel or window controls)                                                                                                    |
| `set_margins(...)` | Sets the margins of the container. The margins are the space between the container and its child controls. By default they are 0 and are meaningless for a regular control that has no children. |



## A simple container example

The following example creates a simple custom container that acts as a bordered panel with a title bar and can hold other controls:

```rs
use appcui::prelude::*;

#[CustomContainer(overwrite = OnPaint)]
struct TitledContainer {
    title: String,
}

impl TitledContainer {
    fn new(layout: Layout, title: &str) -> Self {
        let mut me = Self {
            base: ContainerBase::new(layout, true),
            title: title.to_string(),
        };
        me.set_margins(1, 1, 1, 1);
        me
    }
}

impl OnPaint for TitledContainer {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        let size = self.size();
        surface.draw_rect(
            Rect::with_point_and_size(Point::ORIGIN, size), 
            LineType::AsciiRound, 
            charattr!("yellow"));
        surface.fill_horizontal_line(2, 
                                     0, 
                                     size.width as i32 - 3, 
                                     ~~Character~~::with_attributes(' ', charattr!("black,yellow")));

        let format = TextFormatBuilder::new()
                                        .position(size.width as i32/2, 0)
                                        .align(TextAlignment::Center)
                                        .attribute(charattr!("black,yellow"))
                                        .build();
        surface.write_text(&self.title, &format);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("caption:'Container Example',a:c,w:50,h:20");

    // Create the custom container
    let mut container = TitledContainer::new(layout!("l:2,t:2,r:2,b:2"), "My Custom Container");

    // Add child controls to the container
    container.add(label!("'Hello from inside!', x:2, y:2, w:20"));
    container.add(button!("'Click Me', x:2, y:4, w:12"));

    w.add(container);
    a.add_window(w);
    a.run();
    Ok(())
}
```

