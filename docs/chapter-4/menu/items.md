# Menu Items

Each menu is form out of menu items. AppCUI supports the following menu items:
- [Command]() : a command (clicking this item will send a command)
- [CheckBox]() : a item that has two states (`checked` or `unchecked`). Clicking on this item will change the state (from `checked` to `unchecked` and vice-versa) and will send a command.
- [SingleChoice]() : a item that is part of a group of items from which only one can be selected at a moment of time. Clicking on this item will select it (an implicelely unselect any other selected SingleChoice item from the group) and send a command.
- [SubMenu]() : a item that contains another menu. Clicking on this item will open the sub-menu.
- [Separator](): a item that has no input and is represented by a horizontal line that separates groups or commands withing a menu.

## Macro

All menu items can be build via `menuitem!` macro. The following attributes are can be used:

| Attribute  | Command   | CheckBox  | SingleChoice | Sub-Menu  | Separator |
| ---------- | --------- | --------- | ------------ | --------- | --------- |
| `caption`  | Yes       | Yes       | Yes          | Yes       | Yes       |
| `shortcut` | Yes       | Yes       | Yes          |           |           |
| `command`  | Yes       | Yes       | Yes          |           |           |
| `checked`  |           | Yes       |              |           |           |
| `selected` |           |           | Yes          |           |           |
| `items`    |           |           |              | Yes       |           |
| `enabled`  | Yes (opt) | Yes (opt) | Yes (opt)    | Yes (opt) |           |
| `type`     | Yes (opt) | Yes (opt) | Yes (opt)    | Yes (opt) | Yes (opt) |
| `class`    | Yes (opt) | Yes (opt) | Yes (opt)    | Yes (opt) | Yes (opt) |

The `type` attribute is not optional. If not present , the type of the menu item is determine as follows:
- a menuitem with only one attribute of type caption that consists only in multiple characters `-` will be consider a `Separator`
- a menuitem that has the attribute `checked` will be considered a `CheckBox`
- a menuitem that has the attribute `selected` will be considered a `SingleChoice`
- a menuitem that has the attribute `items` wil be considered a `SubMenu`
- otherwise, the menuitem will be considered of type `Command`

Similarly, the attribute `class` can be used to simplify the command value. Typically, the `command` attribute must include a format that resambles the following form:

```rs
"command='<module-name>::Command::<Command-ID>'"
```

where `<module-name>` is the lowercase name of the struct that registers the menu.
To simplify the previous form, one can use the following:


```rs
"command=<Command-ID>, class=<class-name>"
```

For example - assuming we have the following window:
```rust
#[Window(... commands=Save+Open+New)]
struct MyWindow { /* data members */ }
```

then we can define a menu item for command `Save` in one of the following ways:

```rust
let item = menuitem!("... command='mywindow::Commands::Save'");
```

or

```rust
let item = menuitem!("... command=Save, class=MyWindow");
```

It is also important to notice that `class` attribute will be inherit (meaning that if you specify it for a menu item that hase sub menus, the sub menu items will inherit it and as such you don't have to add it to their definition).
