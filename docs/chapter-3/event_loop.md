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
* WindowEvents
* ButtonEvents
* CheckBoxEvents
* CommandBarEvents
* MenuEvents
* ToolBarEvents

These events can be implemented to receive notification on various actions that children controls are performing. 

