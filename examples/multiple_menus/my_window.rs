use appcui::prelude::*;
use appcui::ui::appbar::*;

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
            "class:MyWindow,items=[
            {New,cmd:New},
            {Save,cmd:Save},
            {Open,cmd:Open},
        ]"
        );
        w.h_menu = w.appbar_mut().add(MenuEntry::new("File", m, 0, Side::Left));

        w
    }
}
impl MenuEvents for MyWindow {
    fn on_update_menubar(&self, menubar: &mut AppBar) {
        menubar.show(self.h_menu);
    }
}
