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
 let cmd = menuitem!("Content,F1,'<module>::Command::Content');
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
| `class`                                        | String | **No**                                | The name of the class where the menu is being implements (see **Remarks** for more details)                                                                                                                                                        |
