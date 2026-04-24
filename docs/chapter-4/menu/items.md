# Menu Items

Each menu is formed from menu items. AppCUI supports the following menu items:
- [Command](command.md): a command (clicking this item will send a command)
- [CheckBox](checkbox.md): an item that has two states (`checked` or `unchecked`). Clicking this item changes the state (from `checked` to `unchecked` and vice versa) and sends a command.
- [SingleChoice](single_choice.md): an item that is part of a group from which only one can be selected at a time. Clicking this item selects it (implicitly unselecting any other selected `SingleChoice` item in the group) and sends a command.
- [SubMenu](submenu.md): an item that contains another menu. Clicking this item opens the submenu.
- [Separator](separator.md): an item that has no input and is represented by a horizontal line that separates groups or commands within a menu.

## Macro

All menu items can be built via the `menuitem!` macro. The following attributes can be used:

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

The `type` attribute is not optional. If not present, the type of the menu item is determined as follows:
- a menu item with only one attribute of type `caption` that consists only of multiple `-` characters will be considered a `Separator`
- a menu item that has the attribute `checked` will be considered a `CheckBox`
- a menu item that has the attribute `selected` will be considered a `SingleChoice`
- a menu item that has the attribute `items` will be considered a `SubMenu`
- otherwise, the menu item will be considered of type `Command`

Similarly, the `class` attribute can be used to simplify the command value. Typically, the `command` attribute must use a format that resembles the following:

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
#[Window(..., commands = Save+Open+New)]
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

It is also important to note that the `class` attribute is inherited (meaning that if you specify it for a menu item that has submenus, the submenu items inherit it, so you do not have to add it to their definitions).
