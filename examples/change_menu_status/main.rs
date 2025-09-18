use appcui::prelude::*;

#[Window(events = MenuEvents+AppBarEvents, commands=Increment)]
struct MyWin {
    m_counter: Handle<menu::Command>,
    some_menu: Handle<appbar::MenuButton>,
    counter: u32,
}
impl MyWin {
    fn new() -> Self {
        let mut w = MyWin {
            base: window!("Test,a:c,w:40,h:8"),
            m_counter: Handle::None,
            some_menu: Handle::None,
            counter: 0
        };
        let mut m = Menu::new();
        w.m_counter = m.add(menuitem!("'Increment (0)',cmd:Increment,class:MyWin"));
        w.some_menu = w.appbar().add(appbar::MenuButton::new("Some menu",m,0,appbar::Side::Left));

        w
    }
}
impl MenuEvents for MyWin {
    fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, _: mywin::Commands) {
        if item == self.m_counter {
            self.counter += 1;
            let new_text = format!("Increment ({})",self.counter);
            if let Some(menuitem) = self.menuitem_mut(menu, item) {
                menuitem.set_caption(new_text.as_str());
            }
        }
    }
}
impl AppBarEvents for MyWin {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.some_menu);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().app_bar().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
