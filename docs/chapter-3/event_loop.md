# Event loop

`AppCUI` is an event driven framework, meaning that each control can emit events to reflect various acions or changes that occur. For example, whenever you push a button, an event will be raise. All events are process at Window level by implementing various traits. To build a Window that supports event handling, you must use a special procedural macro call `Window`, defined in the the following way:

```rs
#[Window(events=..., overwrite=...)]
struct MyWindow {
    // specific fields
}
```

where the attribute `events` and `overwrite` have the following forms:
* `events=EventTrait-1 + EventTrait-2 + EventTrait-3 + ... EventTrait-n`
* `overwrite=OverwriteTrait-1 + OverwriteTrait-2 + OverwriteTrait-3 + ... OverwriteTrait-n` 

