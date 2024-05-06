# Custom controls

While the existing stock controls should suffice for most apps, there is sometines a need to create a custom control. This can be done using a special macro: `#[CustomControl(...)] as follows:
```rs
#[CustomControl(...)]
struct MyCustomControl {
    // aditional fields
}
```

A custom control accepts the following atributes (via `#[CustomControl(...)]` macro):
* `events` with two possible values or combinations: [MenuEvents](../chapter-4/menu.md) and/or [CommandBarEvents](../chapter-4/command_bar.md):
    ```rs
    #[CustomControl(events = MenuEvent+CommandBarEvent)]
    struct MyCustomControl {
        // aditional fields
    }
    ```
* `overwrite` to allow one to overwrite certain traits (for painting or resizing):
    ```rs
    #[CustomControl(overwrite = OnPaint+OnReisze)]
    struct MyCustomControl {
        // aditional fields
    }
* `emit` to describe a list of events that the current control can emit towards the event loop:
    ```rs
    #[CustomControl(emit = Playe1Wins+Playe2Wins+GameOver)]
    struct MyCustomControl {
        // aditional fields
    }
* `commands` (as they are described in [Commands](../chapter-4/commands.md) section)

## Overwrites

The following traits can be overwritten in a custom control:
* OnPaint
* OnResize
* OnFocus
* OnExpand
* OnDefaultAction
* OnKeyPressed
* OnMouseEvent
