use appcui::{prelude::*, ui::appbar::MenuButton};
pub(crate) struct RegularMenus {
    h_file: Handle<appbar::MenuButton>,
    h_edit: Handle<appbar::MenuButton>,
    h_help: Handle<appbar::MenuButton>,
}

impl RegularMenus {
    pub(crate) fn new<T>(appbar: &mut AppBar, cmdid: T) -> Self
    where
        T: CommandID + Copy,
        u32: From<T>,
    {
        let mut menu = Menu::new();
        menu.add(menu::Command::new("&New", key!("Ctrl+N"), cmdid));
        menu.add(menu::Command::new("&Save", key!("Ctrl+S"), cmdid));
        menu.add(menu::Command::new("Save &As...", Key::None, cmdid));
        menu.add(menu::Command::new("&Open", key!("Ctrl+O"), cmdid));
        menu.add(menu::Separator::new());
        menu.add(menu::Command::new("E&xit", key!("Alt+F4"), cmdid));
        let h_file = appbar.add(MenuButton::new("&File", menu, 0, appbar::Side::Left));

        let mut menu = Menu::new();
        menu.add(menu::Command::new("Copy", key!("Ctrl+c"), cmdid));
        menu.add(menu::Command::new("Cut", key!("Ctrl+X"), cmdid));
        menu.add(menu::Command::new("Paste", key!("Ctrl+V"), cmdid));
        menu.add(menu::Separator::new());
        menu.add(menu::Command::new("UpperCase", Key::None, cmdid));
        menu.add(menu::Command::new("LowerCase", Key::None, cmdid));
        menu.add(menu::Command::new("CamelCase", Key::None, cmdid));
        let h_edit = appbar.add(MenuButton::new("&Edit", menu, 0, appbar::Side::Left));

        let mut menu = Menu::new();
        menu.add(menu::Command::new("About", Key::None, cmdid));;
        menu.add(menu::Command::new("Online web page", Key::None, cmdid));;
        let h_help = appbar.add(MenuButton::new("&Help", menu, 0, appbar::Side::Left));        

        Self { h_file, h_edit, h_help }
    }
    pub(crate) fn activate(&self, appbar: &mut AppBar) {
        appbar.show(self.h_file);
        appbar.show(self.h_edit);
        appbar.show(self.h_help);
    }
}
