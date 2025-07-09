use appcui::prelude::*;

#[Window(events = MenuEvents, commands=New+Save+Open)]
pub struct MyWindow {
    h_menu: Handle<Menu>
}
impl MyWindow {
    pub fn new(title: &str, layout: Layout) -> Self {
        let mut w = MyWindow {
            base: Window::new(title, layout, window::Flags::None),
            h_menu: Handle::None
        };
        let m = menu!("File,class:MyWindow,items=[
            {New,cmd:New},
            {Save,cmd:Save},
            {Open,cmd:Open},
        ]");
        w.h_menu = w.register_menu(m);

        w
    }
}
impl MenuEvents for MyWindow {
    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.add(self.h_menu, 0);
    }
}