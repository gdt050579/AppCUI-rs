# Build a menu with macros

Builing a menu is not a complicated task, but it envolves multiple operations over the menu items. Let's consider the folowing menu :
- Items:
  - `Colors` -> a sub-menu that contains the following sub-items:
    - `Red` (a single choice sub-item)
    - `Green` (a single choice sub-item)
    - `Blue` (a single choice sub-item)
  - `Clipboard` -> another sub-menu that contains the following sub-items:
    - `Copy` (a command, with `Ctrl+C` shortcut associated )
    - `Cut` (a command, with `Ctrl+X` shortcut associated )
    - `Paste` ( a command with `Ctrl+V` shortcut associated )
    - a separator
    - `Paste Special` (also a command, with no shortcut associated )
  - a separator
  - `Exit` (a command with no shortcut associated)
  
We will also considered that the following commands were added via the `command` attribute: 
```rs
#[Window(... commands=Red+Green+Blue+Copy+Paste+Cut+PasteSpecial+Exit)]
struct MyWindow { /* data memebers */ }
```

Let's see several ways this menu can be created.

## Build this menu without any macros

``` rs
let mut m = Menu::new();
// build the color submenu
let mut m_colors = Menu::new();
m_colors.add(menu::SingleChoice::new("Red",
                                     Key::None,
                                     mywin::Commands::Red, 
                                     true));
m_colors.add(menu::SingleChoice::new("Green",
                                     Key::None,
                                     mywin::Commands::Green, 
                                     true));
m_colors.add(menu::SingleChoice::new("Blue",
                                     Key::None,
                                     mywin::Commands::Blue, 
                                     true));
m.add(menu::SubMenu::new("Colors", m_colors));

// build the clipboard submenu
let mut m_clipboard = Menu::new();
m_clipboard.add(menu::Command::new("Copy",
                                   Key::new(KeyCode::C, KeyModifier::Ctrl),
                                   mywin::Commands::Copy));
m_clipboard.add(menu::Command::new("Cut",
                                   Key::new(KeyCode::X, KeyModifier::Ctrl),
                                   mywin::Commands::Cut));
m_clipboard.add(menu::Command::new("Paste",
                                   Key::new(KeyCode::V, KeyModifier::Ctrl),
                                   mywin::Commands::Paste));
m_clipboard.add(menu::Separator::new());
m_clipboard.add(menu::Command::new("Paste Special",
                                   Key::None,
                                   mywin::Commands::PasteSpecial));
m.add(menu::SubMenu::new("&Clipboard", m_clipboard));

// add the last items
m.add(menu::Separator::new());
m.add(menu::Command::new("Exit", 
                         Key::None, 
                         mywin::Commands::Exit));
```

Notice that the code is correct but is quite bloated and hard to read.

## Build this menu using menuitem! macro

```rs
let mut m = Menu::new();
// build the color submenu
let mut m_colors = Menu::new();
m_colors.add(menuitem!("Red,selected:true,cmd:Red,class:MyWin"));
m_colors.add(menuitem!("Green,selected:true,cmd:Green,class:MyWin"));
m_colors.add(menuitem!("Blue,selected:true,cmd:Blue,class:MyWin"));
m.add(menu::SubMenu::new("Colors", m_colors));

// build the clipboard submenu
let mut m_clipboard = Menu::new();
m_clipboard.add(menuitem!("Copy,Ctrl+C,cmd:Copy,class:MyWin"));
m_clipboard.add(menuitem!("Cut,Ctrl+X,cmd:Cut,class:MyWin"));
m_clipboard.add(menuitem!("Paste,Ctrl+V,cmd:Paste,class:MyWin"));
m_clipboard.add(menuitem!("---"));
m_clipboard.add(menuitem!("'Paste Special',None,cmd:PasteSpecial,class:MyWin"));
m.add(menu::SubMenu::new("&Clipboard", m_clipboard));

// add the last items
m.add(menuitem!("---"));
m.add(menuitem!("Exit,cmd:Exit,class:MyWin"));
```

The code is more readable, but we can make it even more smaller.

## Building a menu using the menu! macro

In this case we will use the `menu!` macro to condense the code even more:

```rs
let m = menu!("items=[
    { Colors,items=[
        { Red,selected:true,cmd:Red,class:MyWin },
        { Green,selected:true,cmd:Green,class:MyWin },
        { Blue,selected:true,cmd:Blue,class:MyWin }
    ]},
    { &Clipboard,items=[
        { Copy,Ctrl+C,cmd:Copy,class:MyWin },
        { Cut,Ctrl+X,cmd:Cut,class:MyWin },
        { Paste,Ctrl+V,cmd:Paste,class:MyWin },
        { --- },
        { 'Paste Special',None,cmd:PasteSpecial,class:MyWin }
    ]},
    { --- },
    { Exit,cmd:Exit,class:MyWin }
]");
```
Notice that in this case, the description of a menu item looks is more condense (and easier to read) and it looks like a JSON files.

However, there are still some duplicate data in this form (for example: attribute `class` with value `MyWin` is present for each of the actionable items). In this case we can use the inherit properties of a menu, an specify this item only once and reduce the code even more by adding the `class` attribute to the top level menu description and we get the most compressed way of quickly creating a menu.

```rs
let m = menu!("class:MyWin,items=[
    { Colors,items=[
        { Red,selected:true,cmd:Red },
        { Green,selected:true,cmd:Green },
        { Blue,selected:true,cmd:Blue }
    ]},
    { &Clipboard,items=[
        { Copy,Ctrl+C,cmd:Copy },
        { Cut,Ctrl+X,cmd:Cut },
        { Paste,Ctrl+V,cmd:Paste },
        { --- },
        { 'Paste Special',None,cmd:PasteSpecial }
    ]},
    { --- },
    { Exit,cmd:Exit }
]");
```
**Remarks**: Keep in mind that this method will not allow you obtain any menu item handle. If they are neccesary to change some attributes (like enable/disable status) you will not be able to do so. However, if your menu only has commands, or checboxes and assigning a command is enough for you to react to an event, this is the prefered way to create a menu.
