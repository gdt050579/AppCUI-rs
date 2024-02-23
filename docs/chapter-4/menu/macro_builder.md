# Build a menu with macros

Builing a menu is not a complicated task, but it envolves multiple operations over the menu items. Let's consider the folowing menu :
- Menu name: `Test`
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
let mut m = Menu::new("Test");
// build the color submenu
let mut m_colors = Menu::new("Colors");
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
m.add(menu::SubMenu::new(m_colors));

// build the clipboard submenu
let mut m_clipboard = Menu::new("&Clipboard");
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
m.add(menu::SubMenu::new(m_clipboard));

// add the last items
m.add(menu::Separator::new());
m.add(menu::Command::new("Exit", 
                         Key::None, 
                         mywin::Commands::Exit));
```

Notice that the code is correct but is quite bloated and hard to read.

## Build this menu using menuitem! macro