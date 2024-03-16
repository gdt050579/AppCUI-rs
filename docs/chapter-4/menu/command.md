# Command (menu item)

 A command menu item is an equivalent of a button but for menus. 
 
 <img src="img/command.png" width=400/>
 
 You can create it using either `menu::Command::new(...)` method or via the `menuitem!` macro.

 ```rs
 let cmd = menu::Command::new("Content", Key::new(KeyCode::F1,KeyModifier::None), <module>::Command::Content);
 ```
 or
 ```rs
 let cmd = menu::Command::new("Content", key!("F1"), <module>::Command::Content);
 ```
 or
 ```rs
 let cmd = menuitem!("Content,F1,'<module>::Command::Content'");
 ```
 or
 ```rs
 let cmd = menuitem!("Content,F1,cmd:Content,class:<class-name>");
 ```

## Macro build

The following parameters are accepted by `menuitem!` when building a command menu item:

| Parameter name                                 | Type   | Positional parameter                  | Purpose                                                                                                                                                                                                                                            |
| ---------------------------------------------- | ------ | ------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `text` or `caption`                            | String | **Yes** (first postional parameter)   | The caption (text) of the command. If the caption contains the special character `&` the next character after that will act as a short key (meaning that pressing that character while that menu is opened is equivalent to clicking on that item) |
| `key` or `shortcut` or `shortcutket`           | String | **Yes** (second positional parameter) | The shortcut associated with the command. If not specified it will be considered `Key::None`                                                                                                                                                       |
| `cmd` or `cmd-id` or `command` or `command-id` | String | **Yes** (third positional parameter)  | The associated command id for this item                                                                                                                                                                                                            |
| `type`                                         | String | **No**                                | The type of the item (for a command item if this filed is being specified its value must be `command`)                                                                                                                                             |
| `class`                                        | String | **No**                                | The name of the class where the menu is being implemented                                                                                                                                                                                          |
| `enable` or `enabled`                          | Bool   | **No**                                | Use this to disable or enable a menu item                                                                                                                                                                                                          |

## Events
To intercept events this item, the following trait and method have to be implemented to the Window that processes the event loop:
```rs
trait MenuEvents {
    fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, command: <module>::Commands) {
        // add logic here
    }
}
```

## Methods

The following methods are availble for a `menu::Command` object:

| Method              | Purpose                                                                                                                                                                                                                                                               |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `set_caption(...)`  | Set the new caption for the item. If the string provided contains the special character `&`, this method also sets the hotkey associated with an item. If the string provided does not contain the `&` character, this method will clear the current hotkey (if any). |
| `caption()`         | Returns the current caption of an item                                                                                                                                                                                                                                |
| `set_enables(...)`  | Enables or disables current item                                                                                                                                                                                                                                      |
| `is_enabled()`      | `true` if the item is enables, `false` otherwise                                                                                                                                                                                                                      |
| `set_shortcut(...)` | Sets a new shortcut for the current item                                                                                                                                                                                                                              |
| `shortcut()`        | Returns the shortcut for the current item                                                                                                                                                                                                                             |

## Example

The following code creates a menu with 3 menu items (of type command). Notice that we had to initialize the application with support for menus.

```rs
use appcui::prelude::*;

#[Window(events = MenuEvents, commands=Cmd1+Cmd2+Cmd3)]
struct MyWin {
    m_commands: Handle<Menu>,
}
impl MyWin {
    fn new() -> Self {
        let mut w = MyWin {
            base: window!("Test,d:c,w:40,h:8"),
            m_commands: Handle::None,
        };
        let mut m = Menu::new("Commands");
        m.add(menu::Command::new("Command-1", Key::None, mywin::Commands::Cmd1));
        m.add(menu::Command::new("Command-2", Key::None, mywin::Commands::Cmd2));
        m.add(menuitem!("Command-3,F1,cmd:Cmd3,class:MyWin"));
        w.m_commands = w.register_menu(m);

        w
    }
}
impl MenuEvents for MyWin {

    fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, command: mywin::Commands) {
        match command {
            mywin::Commands::Cmd1 => { /* do something with command 1 */ },
            mywin::Commands::Cmd2 => { /* do something with command 2 */ },
            mywin::Commands::Cmd3 => { /* do something with command 3 */ },
        }
    }
    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.add(self.m_commands);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().menu_bar().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
```