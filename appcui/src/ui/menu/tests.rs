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

#[test]
fn check_scroll_button_activation() {
    #[Window(events = MenuEvents, commands=A+B+C, internal: true)]
    struct MyWin {
        m_file: Handle<Menu>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,d:c,w:40,h:8"),
                m_file: Handle::None,
            };
            let m = menu!(
                "&Keywords, class:MyWin, items=[
                    {1,cmd:A},
                    {2,cmd:A},
                    {3,cmd:A},
                    {4,cmd:A},
                    {5,cmd:A},
                    {6,cmd:A},
                    {7,cmd:A},
                    {8,cmd:A},
                    {9,cmd:A},
                    {10,cmd:A},
                    {11,cmd:A},
                    {12,cmd:A},
                ]"
            );
            w.m_file = w.register_menu(m);
            w
        }
    }
    impl MenuEvents for MyWin {
        fn on_update_menubar(&self, menubar: &mut MenuBar) {
            menubar.add(self.m_file);
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0x8a3517e84f3258c3)
        Mouse.Move(3,0)
        Mouse.Click(3,0,left)
        Paint('Meniu opened')
        CheckHash(0x49de03a8810dbb75)
        Mouse.Move(4,11)
        Mouse.Click(4,11,left)
        Paint('2 to 10')
        CheckHash(0xd6509540ca0c1de1)
        Mouse.Click(4,11,left)
        Paint('3 to 11')
        CheckHash(0x44283c0cfe0beadb)
        Mouse.Click(4,11,left)
        Paint('4 to 12 (down button disabled)')
        CheckHash(0x3359a06bc50d8ae3)
        Mouse.Move(3,1)
        Paint('hover over top scroll button')
        CheckHash(0xc8508353e83bddcb)
        Mouse.Click(3,1,left)
        Mouse.Move(4,1)
        Mouse.Click(4,1,left)
        Mouse.Move(9,2)
        Paint('2 to 10 (both butons are enabled)')
        CheckHash(0x8a69f56b56a4ee59)
        Mouse.Move(4,1)
        Mouse.Click(4,1,left)
        Paint('top button is disabled')
        CheckHash(0x49de03a8810dbb75)
    ";
    let mut a = App::debug(60, 15, script).menu().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_submenus_open() {
    #[Window(events = MenuEvents, commands=A+B+C, internal: true)]
    struct MyWin {
        m_file: Handle<Menu>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,d:c,w:40,h:8"),
                m_file: Handle::None,
            };
            let m = menu!(
                "&Menu, class:MyWin, items=[
                {1,items=[
                    {1,cmd:A},
                    {2,cmd:A},
                    {3,cmd:A},
                    {4,items=[
                        {1,cmd:A},
                        {2,cmd:A},
                        {3,cmd:A},
                        {10,items=[
                            {1,cmd:A},
                            {2,cmd:A},
                            {3,cmd:A},
                            {11,items=[
                                {1,cmd:A},
                                {---},
                                {2,cmd:A},
                                {3,cmd:A},
                            ]
                            }
                        ]
                        }        
                    ]
                    }
                ]},
                {2,items=[
                    {'a long submenu',items=[
                        {1,cmd:A},
                        {2,cmd:A},
                        {3,cmd:A},
                        {'another long submenu',items=[
                            {'value 1 for command',cmd:A},
                            {'value 2 for command',cmd:A},
                            {'value 3 for command',cmd:A},
                            {---},
                            {'yet another long submenu',items=[
                                {'really long name to be used in a menu',key:Ctrl+Alt+Shift+Space,cmd:A},
                                {'second',cmd:A},
                                {'third',cmd:A},
                            ]
                            }  
                        ]
                        }    
                    ]
                    },
                    {1,cmd:A},
                    {2,cmd:A},
                    {3,cmd:A},
                ]},    
                {10,cmd:A},
                {11,cmd:A},
                {12,cmd:A},
            ]"
            );
            w.m_file = w.register_menu(m);
            w
        }
    }
    impl MenuEvents for MyWin {
        fn on_update_menubar(&self, menubar: &mut MenuBar) {
            menubar.add(self.m_file);
        }
    }
    let script = "
    Paint.Enable(false)
    Paint('initial state')
    CheckHash(0xf51c6661294d459a)
    Mouse.Move(3,0)
    Mouse.Click(3,0,left)
    Paint('menu opened')
    CheckHash(0x839d47292a2cd17a)
    Mouse.Move(3,2)
    Mouse.Click(3,2,left)
    Mouse.Move(10,6)
    Mouse.Click(10,6,left)
    Mouse.Move(13,7)
    Paint('menu: 1-4-1(selected)')
    CheckHash(0x5f3a5bf7a04a8f40)
    Mouse.Move(15,10)
    Mouse.Click(15,10,left)
    Mouse.Move(22,9)
    Mouse.Click(22,9,left)
    Mouse.Move(28,13)
    Paint('menu:1-4-10-11 and (3) selected')
    CheckHash(0xb3c2dd92383de67)
    Mouse.Move(2,3)
    Mouse.Click(2,3,left)
    Mouse.Move(5,3)
    Mouse.Click(5,3,left)
    Mouse.Move(13,4)
    Mouse.Click(13,4,left)
    Mouse.Move(31,8)
    Mouse.Click(31,8,left)
    Paint('a long submenu test-1')
    CheckHash(0x6c2bc9be57a3bc06)
    Mouse.Move(36,12)
    Paint('yet another long submenu selected')
    CheckHash(0x6c2bc9be57a3bc06)
    Mouse.Move(37,13)
    Mouse.Click(37,13,left)
    Mouse.Move(38,10)
    Paint('really long submenu ... with key selected')
    CheckHash(0x1210c55dc6934605)
    Mouse.Move(4,2)
    Mouse.Click(4,2,left)
    Mouse.Click(4,2,left)
    Mouse.Move(12,6)
    Mouse.Click(12,6,left)
    Mouse.Move(17,10)
    Mouse.Click(17,10,left)
    Mouse.Move(26,9)
    Mouse.Click(26,9,left)
    Mouse.Move(28,13)
    Paint('last menu from 1 selected')
    CheckHash(0xb3c2dd92383de67)
    Mouse.Move(6,2)
    Mouse.Click(6,2,left)
    Mouse.Click(6,2,left)
    Mouse.Move(11,6)
    Mouse.Click(11,6,left)
    Mouse.Move(16,10)
    Mouse.Click(16,10,left)
    Mouse.Move(24,9)
    Mouse.Click(24,9,left)
    Mouse.Move(10,3)
    Mouse.Click(10,3,left)
    Paint('Go from last menu to the second one')
    CheckHash(0x60340250ec2ef1c2)
    
    ";
    let mut a = App::debug(60, 15, script).menu().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_dynamic_change_menu() {
    #[Window(events = MenuEvents, commands=Increment, internal: true)]
    struct MyWin {
        m_counter: Handle<menu::Command>,
        some_menu: Handle<Menu>,
        counter: u32,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,d:c,w:40,h:8"),
                m_counter: Handle::None,
                some_menu: Handle::None,
                counter: 0,
            };
            let mut m = Menu::new("Some menu");
            w.m_counter = m.add(menuitem!("'Increment (0)',cmd:Increment,class:MyWin"));
            w.some_menu = w.register_menu(m);

            w
        }
    }
    impl MenuEvents for MyWin {
        fn on_update_menubar(&self, menubar: &mut MenuBar) {
            menubar.add(self.some_menu);
        }
        fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, _: mywin::Commands) {
            if item == self.m_counter {
                self.counter += 1;
                let new_text = format!("Increment ({})", self.counter);
                if let Some(menuitem) = self.get_menuitem_mut(menu, item) {
                    menuitem.set_caption(&new_text.as_str());
                }
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0xf788ef470502e34a)
        Mouse.Move(6,0)
        Paint('hover over menu bar')
        CheckHash(0x3d94307e4fc9bd2)
        Mouse.Click(6,0,left)
        Paint('increment (0)')
        CheckHash(0x288a35a870df748e)
        Mouse.Move(6,2)
        Paint('hover over increment')
        CheckHash(0x1ec2bf22389e4636)
        Mouse.Click(6,2,left)
        Mouse.Move(4,0)
        Mouse.Click(4,0,left)
        Mouse.Move(6,2)
        Paint('increment (1)')
        CheckHash(0xf24b54cf300890fb)
        Mouse.Click(6,2,left)
        Mouse.Move(4,0)
        Mouse.Click(4,0,left)
        Mouse.Move(6,2)
        Paint('increment (2)')
        CheckHash(0x16c4c2a4544f97f4)
    ";
    let mut a = App::debug(60, 24, script).menu().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_dynamic_change_menu_2() {
    #[Window(events = MenuEvents, commands=Increment, internal: true)]
    struct MyWin {
        m_counter: Handle<menu::Command>,
        some_menu: Handle<Menu>,
        counter: u32,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,d:c,w:40,h:8"),
                m_counter: Handle::None,
                some_menu: Handle::None,
                counter: 0,
            };
            let mut m = Menu::new("Some menu");
            w.m_counter = m.add(menuitem!("'Increment (0)',cmd:Increment,class:MyWin"));
            w.some_menu = w.register_menu(m);

            w
        }
    }
    impl MenuEvents for MyWin {
        fn on_update_menubar(&self, menubar: &mut MenuBar) {
            menubar.add(self.some_menu);
        }
        fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, _: mywin::Commands) {
            if item == self.m_counter {
                self.counter += 1;
                let new_text = format!("Increment ({})", self.counter);
                if let Some(menuitem) = self.get_menuitem_mut(menu, item) {
                    menuitem.set_caption(&new_text.as_str());
                }
            }
        }
    }
    let script = "
            //Paint.Enable(false)
            Paint('State_2')
            CheckHash(0xf788ef470502e34a)
            Mouse.Move(5,0)
            Paint('State_3')
            CheckHash(0x3d94307e4fc9bd2)
            Mouse.Hold(5,0,left)
            Paint('State_4')
            CheckHash(0x288a35a870df748e)
            Mouse.Release(5,0,left)
            Mouse.Move(6,2)
            Paint('State_5')
            CheckHash(0x1ec2bf22389e4636)
            Mouse.Hold(6,2,left)
            Paint('State_6')
            CheckHash(0xf788ef470502e34a)
            Mouse.Release(6,2,left)
            Mouse.Move(6,0)
            Paint('State_7')
            CheckHash(0x3d94307e4fc9bd2)
            Mouse.Hold(6,0,left)
            Paint('State_8')
            CheckHash(0xe93455b60e606693)
            Mouse.Release(6,0,left)
            Mouse.Move(6,2)
            Paint('State_9')
            CheckHash(0xf24b54cf300890fb)
            Mouse.Move(7,2)
            Mouse.Hold(7,2,left)
            Paint('State_10')
            CheckHash(0xf788ef470502e34a)
            Mouse.Release(7,2,left)
            Mouse.Move(27,8)
            Mouse.Hold(27,8,left)
            Paint('State_11')
            CheckHash(0xd450c0ed0c207903)
            Mouse.Move(26,8)
            Paint('State_12')
            CheckHash(0x4ad39e8322170c33)
            Mouse.Move(26,7)
            Paint('State_13')
            CheckHash(0xb4910b018c89ecd3)
            Mouse.Move(25,7)
            Paint('State_14')
            CheckHash(0x3057a1de4971a143)
            Mouse.Move(24,7)
            Paint('State_15')
            CheckHash(0xe7433e46723ac883)
            Mouse.Move(24,6)
            Paint('State_16')
            CheckHash(0xe3fc949366a39c3)
            Mouse.Move(23,6)
            Paint('State_17')
            CheckHash(0x653fabf98a7ac023)
            Mouse.Move(22,5)
            Paint('State_18')
            CheckHash(0xeffa5205aa4cd3b3)
            Mouse.Move(21,5)
            Paint('State_19')
            CheckHash(0x754ad1aa27d31a3)
            Mouse.Move(21,4)
            Paint('State_20')
            CheckHash(0xe67813564aa69c43)
            Mouse.Move(20,4)
            Paint('State_21')
            CheckHash(0xcdd22f80e181b343)
            Mouse.Move(19,4)
            Paint('State_22')
            CheckHash(0xc4d5e36fe9d637c3)
            Mouse.Move(19,3)
            Paint('State_23')
            CheckHash(0x7954fdc89a730463)
            Mouse.Release(19,3,left)
            Paint('State_24')
            CheckHash(0xe2a5511bab6d8d1a)
            Mouse.Move(8,0)
            Paint('State_25')
            CheckHash(0xc0e18c69137d13e2)
            Mouse.Move(6,0)
            Mouse.Hold(6,0,left)
            Paint('State_26')
            CheckHash(0xda8c2e3d9ced61a5)
            Mouse.Release(6,0,left)
            Mouse.Move(7,2)
            Paint('State_27')
            CheckHash(0x27f040ba302e5f9d)
            Mouse.Hold(7,2,left)
            Paint('State_28')
            CheckHash(0xe2a5511bab6d8d1a)
            Mouse.Release(7,2,left)
            Mouse.Move(24,10)
            Key.Pressed(Escape)
            Paint('State_29')
            CheckHash(0x86cfc913da83fa16)
        ";
    let mut a = App::debug(60, 24, script).menu().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}