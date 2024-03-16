# Single Choice (menu item)

 A checkbox menu item is an equivalent of a checkbo but for menus. 
 
 <img src="img/singlechoice.png" width=400/>
 
 You can create it using either `menu::SingleChoice::new(...)` method or via the `menuitem!` macro.

 ```rs
 let sc = menu::SingleChoice::new("Choice", Key::new(KeyCode::F1,KeyModifier::None), <module>::Command::Content);
 ```
 or
 ```rs
 let sc = menu::SingleChoice::new("Choice", key!("F1"), <module>::Command::Content);
 ```
 or
 ```rs
 let sc = menuitem!("Choice,F1,'<module>::Command::Content',type:SinghleChoice");
 ```
 or
 ```rs
 let sc = menuitem!("Choice,F1,cmd:Content,class:<class-name>,selected:true");
 ```

## Macro build

The following parameters are accepted by `menuitem!` when building a checkbox menu item:

| Parameter name                                 | Type   | Positional parameter                  | Purpose                                                                                                                                                                                                                                                       |
| ---------------------------------------------- | ------ | ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `text` or `caption`                            | String | **Yes** (first postional parameter)   | The caption (text) of the single choice item. If the caption contains the special character `&` the next character after that will act as a short key (meaning that pressing that character while that menu is opened is equivalent to clicking on that item) |
| `key` or `shortcut` or `shortcutket`           | String | **Yes** (second positional parameter) | The shortcut associated with the single choice item. If not specified it will be considered `Key::None`                                                                                                                                                       |
| `cmd` or `cmd-id` or `command` or `command-id` | String | **Yes** (third positional parameter)  | The associated command id for this item                                                                                                                                                                                                                       |
| `select` or `selected`                         | Bool   | **No**                                | `true` if the choice is the selected one, `false` otherwise                                                                                                                                                                                                   |
| `type`                                         | String | **No**                                | The type of the item (for a single choice item if this filed is being specified its value must be `singlechoice`)                                                                                                                                             |
| `class`                                        | String | **No**                                | The name of the class where the menu is being implemented                                                                                                                                                                                                     |
| `enable` or `enabled`                          | Bool   | **No**                                | Use this to disable or enable a menu item                                                                                                                                                                                                                     |

## Events
To intercept events this item, the following trait and method have to be implemented to the Window that processes the event loop:
```rs
trait MenuEvents {
    fn on_select(&mut self, menu: Handle<Menu>, item: Handle<menu::CheckBox>, command: <module>::Commands) {
        // this is whenever a single choice item is selected
    }
}
```

## Methods

The following methods are availble for a `menu::SingleChoice` object:

| Method              | Purpose                                                                                                                                                                                                                                                               |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `set_caption(...)`  | Set the new caption for the item. If the string provided contains the special character `&`, this method also sets the hotkey associated with an item. If the string provided does not contain the `&` character, this method will clear the current hotkey (if any). |
| `caption()`         | Returns the current caption of an item                                                                                                                                                                                                                                |
| `set_checked(...)`  | Checkes or uncheckes current ite,                                                                                                                                                                                                                                     |
| `is_selected()`     | `true` if the item is checked, `false` otherwise                                                                                                                                                                                                                      |
| `set_selected()`    | Selects the current item                                                                                                                                                                                                                                              |
| `is_enabled()`      | `true` if the item is enables, `false` otherwise                                                                                                                                                                                                                      |
| `set_shortcut(...)` | Sets a new shortcut for the current item                                                                                                                                                                                                                              |
| `get_shortcut()`    | Returns the shortcut for the current item                                                                                                                                                                                                                             |

## Groups

All single choice items are implicetely gouped based on their index. A consequitive set of single choice items forms a group. Whenever a single choice item is selected, the rest of the items from the group will be unselected.

Let's consider the following example (menu):
```
Single choice A
Single choice B
Single choice C
---------------
Single choice D
Single choice E
Single choice F
```

This menu has 7 items (the first three are of type single choice, then we have a separator and then another three single choice items). As such, `2` groups will be create:
- **First group** - created out of single choice items `A`, `B` and `C`
- **Second group** - created out of single choice items `D`, `E` and `F`

Whenever an item from the first group is being selected, the rest of the items will be unselected (ex: if we slect item `F`, then item `D` and item `E` will be unselected by default).

## Example

The following code creates a menu with 3 menu items (of type checkbox). Notice that we had to initialize the application with support for menus.

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
        let mut m = Menu::new("Single choices");
        m.add(menu::SingleChoice::new("Choice &A", Key::None, mywin::Commands::Cmd1,false));
        m.add(menu::SingleChoice::new("Choice &B", Key::None, mywin::Commands::Cmd2,false));
        m.add(menuitem!("'Choice &C',F1,cmd:Cmd3,class:MyWin,selected:true"));
        w.m_commands = w.register_menu(m);

        w
    }
}
impl MenuEvents for MyWin {

    fn on_select(&mut self,menu:Handle<Menu>,item:Handle<menu::SingleChoice>,command:mywin::Commands) {
        match command {
            mywin::Commands::Cmd1 => { /* do something with option 1 */ },
            mywin::Commands::Cmd2 => { /* do something with option 2 */ },
            mywin::Commands::Cmd3 => { /* do something with option 3 */ },
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