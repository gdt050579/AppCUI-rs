use appcui::prelude::*;
use appcui::ui::appbar::*;

#[Window(events = AppBarEvents, commands=New+Save+Open)]
pub struct MyWindow {
    h_menu: Handle<MenuButton>,
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
        w.h_menu = w.appbar().add(MenuButton::new("File", m, 0, Side::Left));

        w
    }
}
impl AppBarEvents for MyWindow {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.h_menu);
    }
}
