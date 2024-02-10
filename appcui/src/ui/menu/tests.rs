use crate::prelude::*;

#[test]
fn check_view() {
    #[Window(events = MenuEvents, commands=A+B+C, internal: true)]
    struct MyWin {
        m_file: Handle<Menu>,
        m_edit: Handle<Menu>,
        m_help: Handle<Menu>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,d:c,w:40,h:8"),
                m_file: Handle::None,
                m_help: Handle::None,
                m_edit: Handle::None,
            };
            let mut m = Menu::new("&File");
            m.add(menu::Command::new("&New", key!("F1"), mywin::Commands::A));
            m.add(menu::Command::new("&Save", key!("F2"), mywin::Commands::A));
            m.add(menu::Command::new("&Open", key!("F3"), mywin::Commands::A));
            m.add(menu::Separator::new());
            m.add(menu::Command::new("E&xit", key!("Alt+F4"), mywin::Commands::A));
            w.m_file = w.register_menu(m);

            let mut m = Menu::new("&Edit");
            m.add(menu::Command::new("&Copy", Key::None, mywin::Commands::B));
            m.add(menu::Command::new("C&ut", Key::None, mywin::Commands::B));
            m.add(menu::Command::new("&Paste", Key::None, mywin::Commands::B));
            m.add(menu::Separator::new());
            m.add(menu::SingleChoice::new("Paste only text", Key::None, mywin::Commands::B, true));
            m.add(menu::SingleChoice::new("Paste only images", Key::None, mywin::Commands::B, false));
            m.add(menu::SingleChoice::new("Paste everything", Key::None, mywin::Commands::B, false));
            w.m_edit = w.register_menu(m);

            let mut m = Menu::new("&Help");
            m.add(menu::Command::new("About", Key::None, mywin::Commands::C));
            w.m_help = w.register_menu(m);
            w
        }
    }
    impl MenuEvents for MyWin {
        fn on_menu_open(&self, menu: &mut Menu) {}

        fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, command: mywin::Commands) {}

        fn on_check(&mut self, menu: Handle<Menu>, item: Handle<menu::CheckBox>, command: mywin::Commands, checked: bool) {}

        fn on_select(&mut self, menu: Handle<Menu>, item: Handle<menu::SingleChoice>, command: mywin::Commands) {}

        fn on_update_menubar(&self, menubar: &mut MenuBar) {
            menubar.add(self.m_file);
            menubar.add(self.m_edit);
            menubar.add(self.m_help);
        }
    }
    let script = "
    Paint.Enable(false)
    Paint('initial state')
    CheckHash(0x2d58dca91073d56a)
    Mouse.Move(1,0)
    Paint('File menu hovered')
    CheckHash(0xc9c1e1197e4657da)
    Mouse.Move(8,0)
    Paint('Edit menu hovered')
    CheckHash(0xbb3ea536e852b9fa)
    Mouse.Move(13,0)
    Paint('help menu hovered')
    CheckHash(0x96b291f14af7f03a)
    Mouse.Click(1,0,left)
    Paint('file menu opened')
    CheckHash(0x96f6e4a4ec7daa91)
    Mouse.Move(3,4)
    Paint('File::open selected')
    CheckHash(0x4d2bc1c918c53be9)
    Mouse.Move(8,0)
    Paint('Edit menu opened')
    CheckHash(0x9a40c2590374c766)
    Mouse.Move(13,0)
    Paint('Help menu opened')
    CheckHash(0xf1bc8d9499199c48)
    Mouse.Move(3,0)
    Mouse.Move(3,6)
    Paint('File::exit menu selected')
    CheckHash(0xbab31a8b40b618a9)    
    ";
    let mut a = App::debug(60, 15, script).menu().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}
