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
        Paint.Enable(false)
        Paint('State_2')
        CheckHash(0xf788ef470502e34a)
        Mouse.Move(8,0)
        Paint('State_3')
        CheckHash(0x3d94307e4fc9bd2)
        Mouse.Move(6,0)
        Mouse.Hold(6,0,left)
        Paint('State_4')
        CheckHash(0x288a35a870df748e)
        Mouse.Release(6,0,left)
        Mouse.Move(7,2)
        Paint('State_5')
        CheckHash(0x1ec2bf22389e4636)
        Mouse.Hold(7,2,left)
        Paint('State_6')
        CheckHash(0xf788ef470502e34a)
        Mouse.Release(7,2,left)
        Mouse.Move(7,0)
        Paint('State_7')
        CheckHash(0x3d94307e4fc9bd2)
        Mouse.Move(6,0)
        Mouse.Hold(6,0,left)
        Paint('State_8')
        CheckHash(0xe93455b60e606693)
        Mouse.Release(6,0,left)
        Mouse.Move(7,2)
        Paint('State_9')
        CheckHash(0xf24b54cf300890fb)
        Mouse.Move(8,2)
        Mouse.Hold(8,2,left)
        Paint('State_10')
        CheckHash(0xf788ef470502e34a)
        Mouse.Release(8,2,left)
        Mouse.Move(7,0)
        Paint('State_11')
        CheckHash(0x3d94307e4fc9bd2)
        Mouse.Hold(7,0,left)
        Paint('State_12')
        CheckHash(0x8741fbdd037efe3c)
        Mouse.Release(7,0,left)
        Mouse.Move(8,2)
        Paint('State_13')
        CheckHash(0x16c4c2a4544f97f4)
        Mouse.Hold(8,2,left)
        Paint('State_14')
        CheckHash(0xf788ef470502e34a)
        Mouse.Release(8,2,left)
        Mouse.Move(7,0)
        Paint('State_15')
        CheckHash(0x3d94307e4fc9bd2)
        Mouse.Move(6,0)
        Mouse.Hold(6,0,left)
        Paint('State_16')
        CheckHash(0xb62b7cc777749d19)
        Mouse.Release(6,0,left)
        Key.Pressed(Down)
        Paint('State_17')
        CheckHash(0xdf0e1894cf1d1d31)
        Key.Pressed(Enter)
        Paint('State_18')
        CheckHash(0xf788ef470502e34a)
        Mouse.Hold(6,0,left)
        Paint('State_19')
        CheckHash(0xc708e44f863d76ba)
        Mouse.Release(6,0,left)
        Key.Pressed(Down)
        Paint('State_20')
        CheckHash(0x6a9abad8f1616dd2)
        Key.Pressed(Enter)
        Paint('State_21')
        CheckHash(0xf788ef470502e34a)
        Key.Pressed(Escape)
        Paint('State_22')
        CheckHash(0x86cfc913da83fa16)
            ";
    let mut a = App::debug(60, 24, script).menu().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_menubar_update_multiple_menus() {
    mod mywin {
        use crate::prelude::*;
        #[Window(events = MenuEvents, commands=New+Save+Open, internal: true)]
        pub struct MyWindow {
            h_menu: Handle<Menu>,
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
                w.h_menu = w.register_menu(m);

                w
            }
        }
        impl MenuEvents for MyWindow {
            fn on_update_menubar(&self, menubar: &mut MenuBar) {
                menubar.add(self.h_menu);
            }
        }
    }
    mod colorcustomcontrol {
        use crate::prelude::*;

        #[CustomControl(events = MenuEvents, overwrite = OnPaint, commands = Red+Green+Blue, internal: true)]
        pub struct ColorCustomControl {
            col: Color,
            h_menu: Handle<Menu>,
        }
        impl ColorCustomControl {
            pub fn new(layout: Layout) -> Self {
                let mut obj = Self {
                    base: ControlBase::new(layout, true),
                    col: Color::Red,
                    h_menu: Handle::None,
                };
                let m = menu!(
                    "ColorControl,class:ColorCustomControl,items=[
            {Red,F1,selected:true,cmd:Red},
            {Green,F2,selected:false,cmd:Green},
            {Blue,F3,selected:false,cmd:Blue}
        ]"
                );
                obj.h_menu = obj.register_menu(m);
                obj
            }
        }
        impl OnPaint for ColorCustomControl {
            fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
                surface.clear(Character::new(' ', Color::Black, self.col, CharFlags::None));
                if self.has_focus() {
                    surface.write_string(1, 0, "Focus", CharAttribute::with_fore_color(Color::Yellow), false);
                }
            }
        }
        impl MenuEvents for ColorCustomControl {
            fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<menu::SingleChoice>, command: colorcustomcontrol::Commands) {
                match command {
                    colorcustomcontrol::Commands::Red => self.col = Color::Red,
                    colorcustomcontrol::Commands::Green => self.col = Color::Green,
                    colorcustomcontrol::Commands::Blue => self.col = Color::Blue,
                }
            }

            fn on_update_menubar(&self, menubar: &mut MenuBar) {
                menubar.add(self.h_menu);
            }
        }
    }
    mod textcustomcontrol {
        use crate::prelude::*;

        #[CustomControl(events = MenuEvents, overwrite = OnPaint, commands = Red+Green+Blue, internal: true)]
        pub struct TextCustomControl {
            text: &'static str,
            h_menu: Handle<Menu>,
        }
        impl TextCustomControl {
            pub fn new(layout: Layout) -> Self {
                let mut obj = Self {
                    base: ControlBase::new(layout, true),
                    text: "Red",
                    h_menu: Handle::None,
                };
                let m = menu!(
                    "Text,class:TextCustomControl,items=[
            {'Text->Red',F1,selected:true,cmd:Red},
            {'Text->Green',F2,selected:false,cmd:Green},
            {'Text->Blue',F3,selected:false,cmd:Blue}
        ]"
                );
                obj.h_menu = obj.register_menu(m);
                obj
            }
        }
        impl OnPaint for TextCustomControl {
            fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
                if self.has_focus() {
                    surface.clear(Character::new(' ', Color::Black, Color::Black, CharFlags::None));
                    surface.write_string(1, 0, self.text, CharAttribute::with_fore_color(Color::Yellow), false);
                } else {
                    surface.clear(Character::new(' ', Color::Blue, Color::Blue, CharFlags::None));
                    surface.write_string(1, 0, self.text, CharAttribute::with_fore_color(Color::Yellow), false);
                }
            }
        }
        impl MenuEvents for TextCustomControl {
            fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<menu::SingleChoice>, command: textcustomcontrol::Commands) {
                match command {
                    textcustomcontrol::Commands::Red => self.text = "Red",
                    textcustomcontrol::Commands::Green => self.text = "Green",
                    textcustomcontrol::Commands::Blue => self.text = "Blue",
                }
            }

            fn on_update_menubar(&self, menubar: &mut MenuBar) {
                menubar.add(self.h_menu);
            }
        }
    }
    let script = "
            Paint.Enable(false)
            Paint('initial_state')
            CheckHash(0xdcba473356cabc6)
            Key.Pressed(Tab)
            Paint('two_menus')
            CheckHash(0xc93f804f8356dbd7)
            Mouse.Move(8,0)
            Mouse.Click(8,0,left)
            Paint('file menu opened')
            CheckHash(0x6b2f0c6cde25d6c1)
            Mouse.Move(1,0)
            Mouse.Move(5,4)
            Paint()
            Mouse.Click(5,4,left)
            Paint('blue')
            CheckHash(0xab05af43acbaa8e4)
            Mouse.Move(10,9)
            Mouse.Click(10,9,left)
            Paint('win-1 focused')
            CheckHash(0xc842e40135eb3190)
            Mouse.Move(9,4)
            Mouse.Click(9,4,left)
            Paint('two menus')
            CheckHash(0xb325123cd8b36a5)
            Mouse.Move(10,0)
            Mouse.Click(10,0,left)
            Mouse.Move(8,3)
            Mouse.Click(8,3,left)
            Paint('green')
            CheckHash(0x39be1f5c9b1502f5)
            Mouse.Move(27,10)
            Mouse.Click(27,10,left)
            Paint('win-2 focused')
            CheckHash(0x22e770cdf16b33a4)
            Mouse.Move(3,0)
            Mouse.Click(3,0,left)
            Paint('text->blue selected')
            CheckHash(0xcb24ccb03a1993df)
            Mouse.Move(28,9)
            Mouse.Click(28,9,left)
            Key.Pressed(Tab)
            Paint('one menu')
            CheckHash(0x6ec113e98df3ca14)
            ";
    let mut a = App::debug(60, 24, script).menu().build().unwrap();
    let mut w1 = mywin::MyWindow::new("Win-1", Layout::new("x:1,y:2,w:18,h:10"));
    w1.add(colorcustomcontrol::ColorCustomControl::new(Layout::new("x:1,y:1,w:10")));
    w1.add(button!("Button,x:1,y:3,w:10"));
    let mut w2 = mywin::MyWindow::new("Win-2", Layout::new("x:20,y:2,w:18,h:15"));
    w2.add(textcustomcontrol::TextCustomControl::new(Layout::new("x:1,y:1,w:10")));
    w2.add(button!("Button,x:1,y:3,w:10"));
    a.add_window(w1);
    a.add_window(w2);
    a.run();
}
