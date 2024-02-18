# Commands

All custom controls, windows (including modal) and desktop supports a set of commands that are associated with their functionality.

To define such commands, use the attribute `commands` when define a window, modal window, desktop or custom control:

```rs
#[Window(commands = <list of commands>)]
```

The list of commands can be added in two ways:
- use `[command1, command2, ... command-n]` format
- use `command1+command2+...command-n` format

## Example

```rs
#[Window(commands = [Save,Load,New]])]
```
and
```rs
#[Window(commands = Save+Load+New)]
```

are identical and create 3 commands that are supported by the new windows (`Save`, `Load` and `New`).

## Enum

Including a `command` attribute implicitly generates an enum within a module. The module's name is derived from the desktop, window, or custom control for which the commands are created, using its name in lowercase.

For example, the following definition:
```rs
#[Window(commands = Save+Load+New)]
struct MyWindow { /* data memebers */ }
```

will create the following:

```rs
mod mywindow {
    #[repr(u32)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Commands {
        Save = 0,
        Open = 1,
        New = 2
    }
}
```

Notice that the module name `mywindow` is the lowercase name of the window `MyWindow`.

You can further use these commands for menus or the command bar (via `mywindow::Commands::Save`, or `mywindow::Commands::Open`, ...).

You can also use them for parameter `cmd` in the macro definition for menus or menu items. If the macro parameter defines the name of the class via the parameter `class` (e.g. `class = MyWindow`) you don't have to write the full qualifier in the macro, you can write the name of the command alone.

For example, the following are equivalent:
```rs
cmd="mywin::Command::Save"
```

and
```rs
cmd="Save", class="MyWin"
```
