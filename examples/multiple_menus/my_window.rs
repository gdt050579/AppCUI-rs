use appcui::prelude::*;
use appcui::ui::menubar::*;

#[Window(events = MenuEvents, commands=New+Save+Open)]
pub struct MyWindow {
    h_menu: Handle<MenuEntry>,
}
impl MyWindow {
    pub fn new(title: &str, layout: Layout) -> Self {
        let mut w = MyWindow {
            base: Window::new(title, layout, window::Flags::None),
            h_menu: Handle::None,
        };
        let m = menu!(
            "File,class:MyWindow,items=[
            {New,cmd:New},
            {Save,cmd:Save},
            {Open,cmd:Open},
        ]"
        );
        w.h_menu = w.menubar_mut().add(MenuEntry::new(m, 0, MenuBarPosition::Left));

        w
    }
}
impl MenuEvents for MyWindow {
    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.show(self.h_menu);
    }
}
