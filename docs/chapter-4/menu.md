# Menu

A menu is a list of items (that represents commands, checkboxes and single choice elements) that can be displayed over the existing controls.

<img src="img/menu.png" width=400/>

To create a menu, use `Menu::new()` method or the macro `menu!` (this can be used to quickly create complex static menus). 

```rs
let m = Menu::new()
```

Menus can be displayed in two ways:
- As part of an **AppBar** using `MenuButton` components (recommended for desktop applications)
- As standalone popup menus using the traditional registration method

The menu button's name might include the special character `&`. This designates the next character as the hot key needed to activate the menu (in the previous example this will be `Alt+F`).

## Registration

There are two ways to register and display menus in AppCUI:

### Method 1: AppBar Integration (Recommended)

For desktop applications, menus are typically integrated into the AppBar using `MenuButton` components:

```rs
#[Window(events = MenuEvents+AppBarEvents, commands=[New, Open, Save, Exit])]
struct MyWin {
    file_menu: Handle<appbar::MenuButton>,
}
impl MyWin {
    fn new() -> Self {
        let mut w = MyWin {
            base: window!("Example,a:c,w:40,h:10"),
            file_menu: Handle::None,
        };
        
        // Create a menu and add items to it
        let mut file_menu = Menu::new();
        file_menu.add(menu::Command::new("&New", key!("Ctrl+N"), mywin::Commands::New));
        file_menu.add(menu::Command::new("&Open", key!("Ctrl+O"), mywin::Commands::Open));
        file_menu.add(menu::Command::new("&Save", key!("Ctrl+S"), mywin::Commands::Save));
        file_menu.add(menu::Separator::new());
        file_menu.add(menu::Command::new("E&xit", key!("Alt+F4"), mywin::Commands::Exit));
        
        // Add menu to AppBar as a MenuButton
        w.file_menu = w.appbar().add(
            appbar::MenuButton::new("&File", file_menu, 0, appbar::Side::Left)
        );
        
        w
    }
}
```

### Method 2: Traditional Registration

Each menu can also be registered directly into the **AppCUI** framework using the `.register_menu(...)` method. This is useful for popup menus or when you need direct menu handles:

```rs
#[Window(events = MenuEvents, commands=[<list of commands>])]
struct MyWin {
    menu_handle_1: Handle<Menu>,
    menu_handle_2: Handle<Menu>,
    // other handles
}
impl MyWin {
    fn new() -> Self {
        let mut w = MyWin {
            base: window!(...),
            menu_handle_1: Handle::None,
            menu_handle_2: Handle::None,
            // other handle initialization,
        };
        // first menu
        let m1 = Menu::new();
        // add items to menu 'm1'
        w.menu_handle_1 = w.register_menu(m1);

        // second menu
        let m2 = Menu::new();
        // add items to menu 'm2'
        w.menu_handle_2 = w.register_menu(m2);

        w
    }
}
```

## Events

Using a menu implies that you will need to implement `MenuEvents` into the desktop / window or a custom control to receive the associated action from a menu. 

### MenuEvents Trait

The `MenuEvents` trait provides the following methods:

```rs
trait MenuEvents {
    fn on_menu_open(&self, menu: &mut Menu) {
        // called whenever a menu is being opened
        // by AppCUI framework
        // This method can be used to change 
        // certain menu related aspects, such as
        // - enable/disable menu items
        // - add new items
    }

    fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, command: u32) {
        // this is called whenever a Command menu 
        // item is being clicked
    }

    fn on_check(&mut self, menu: Handle<Menu>, item: Handle<menu::CheckBox>, command: u32, checked: bool) {
        // this is called whenever a CheckBox menu 
        // item is being clicked
    }

    fn on_select(&mut self, menu: Handle<Menu>, item: Handle<menu::SingleChoice>, command: u32) {
        // this is called whenever a SingleChoice menu 
        // item is being clicked
    }
}
```

### AppBarEvents Trait (for AppBar Integration)

When using menus with AppBar, you'll also need to implement `AppBarEvents`:

```rs
trait AppBarEvents {
    fn on_update(&self, appbar: &mut AppBar) {
        // Called when the app bar needs to be updated
        // Use appbar.show(handle) to display menu buttons
    }
}
```

### Complete Example

```rs
#[Window(events = MenuEvents+AppBarEvents, commands=[New, Open, Save, Exit])]
struct MyWindow {
    file_menu: Handle<appbar::MenuButton>,
}

impl MenuEvents for MyWindow {
    fn on_command(&mut self, _menu: Handle<Menu>, _item: Handle<menu::Command>, command: u32) {
        match command {
            cmd if cmd == mywindow::Commands::New as u32 => { /* Handle New */ },
            cmd if cmd == mywindow::Commands::Open as u32 => { /* Handle Open */ },
            cmd if cmd == mywindow::Commands::Save as u32 => { /* Handle Save */ },
            cmd if cmd == mywindow::Commands::Exit as u32 => { /* Handle Exit */ },
            _ => {}
        }
    }
}

impl AppBarEvents for MyWindow {
    fn on_update(&self, appbar: &mut AppBar) {
        // Show the file menu in the app bar
        appbar.show(self.file_menu);
    }
}
```

## Methods

The following methods are available for every `Menu` object

| Method         | Purpose                                                               |
| -------------- | --------------------------------------------------------------------- |
| `add(...)`     | Adds a new menu item to the existing menu and returns a Handle for it |
| `get(...)`     | Returns an immutable reference to a menu item                         |
| `get_mut(...)` | Returns a mutable reference to a menu item                            |

Besides this the following [methods](../chapter-3/common_methods.md#menu-related-methods) are available in each control and allow menu manipulation.