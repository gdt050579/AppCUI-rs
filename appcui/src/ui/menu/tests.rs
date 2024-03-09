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
    let mut a = App::debug(60, 15, script).menu_bar().build().unwrap();
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
    let mut a = App::debug(60, 15, script).menu_bar().build().unwrap();
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
    let mut a = App::debug(60, 15, script).menu_bar().build().unwrap();
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
    let mut a = App::debug(60, 24, script).menu_bar().build().unwrap();
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
    let mut a = App::debug(60, 24, script).menu_bar().build().unwrap();
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
    let mut a = App::debug(60, 24, script).menu_bar().build().unwrap();
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

#[test]
fn check_popup_menu() {
    pub(crate) mod mycustomcontrol {
        use crate::prelude::*;

        #[CustomControl(events = MenuEvents, overwrite = OnPaint+OnMouseEvent, commands = Red+Green+Black+Aqua+Magenta+Yellow+Blue+Gray+White+LightRed+LightGreen, internal: true)]
        pub struct MyCustomControl {
            col: Color,
            h_menu: Handle<Menu>,
            small_menu: bool,
        }
        impl MyCustomControl {
            pub fn new(layout: Layout) -> Self {
                let mut obj = Self {
                    base: ControlBase::new(layout, true),
                    col: Color::Red,
                    h_menu: Handle::None,
                    small_menu: false,
                };
                let m = menu!(
                    "ColorControl,class:MyCustomControl,items=[
                    {&Red,selected:true,cmd:Red},
                    {&Green,selected:false,cmd:Green},
                    {Black,selected:false,cmd:Black},
                    {&Aqua,selected:false,cmd:Aqua},
                    {&Magenta,selected:false,cmd:Magenta},
                    {&Yellow,selected:false,cmd:Yellow},
                    {&Blue,selected:false,cmd:Blue},
                    {Gray,selected:false,cmd:Gray},
                    {White,selected:false,cmd:White},
                    {'Light red',selected:false,cmd:LightRed},
                    {'Light green',selected:false,cmd:LightGreen},
                    ]"
                );
                obj.h_menu = obj.register_menu(m);
                obj
            }
            pub fn enable_small_menu(&mut self, value: bool) {
                self.small_menu = value;
            }
        }
        impl OnPaint for MyCustomControl {
            fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
                surface.clear(Character::new(' ', Color::Black, self.col, CharFlags::None));
                let sz = self.get_client_size();
                let attr = CharAttribute::with_fore_color(Color::White);
                let line = if self.has_focus() { LineType::Double } else { LineType::Single };
                let r = Rect::with_size(0, 0, sz.width as u16, sz.height as u16);
                surface.draw_rect(r, line, attr);
            }
        }
        impl MenuEvents for MyCustomControl {
            fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<menu::SingleChoice>, command: mycustomcontrol::Commands) {
                match command {
                    mycustomcontrol::Commands::Red => self.col = Color::DarkRed,
                    mycustomcontrol::Commands::Green => self.col = Color::DarkGreen,
                    mycustomcontrol::Commands::Black => self.col = Color::Black,
                    mycustomcontrol::Commands::Aqua => self.col = Color::Aqua,
                    mycustomcontrol::Commands::Magenta => self.col = Color::Magenta,
                    mycustomcontrol::Commands::Yellow => self.col = Color::Yellow,
                    mycustomcontrol::Commands::Blue => self.col = Color::Blue,
                    mycustomcontrol::Commands::Gray => self.col = Color::Gray,
                    mycustomcontrol::Commands::White => self.col = Color::White,
                    mycustomcontrol::Commands::LightRed => self.col = Color::Red,
                    mycustomcontrol::Commands::LightGreen => self.col = Color::Green,
                }
            }
        }
        impl OnMouseEvent for MyCustomControl {
            fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
                if let MouseEvent::Pressed(ev) = event {
                    if ev.button == MouseButton::Right {
                        self.show_menu(self.h_menu, ev.x, ev.y, if self.small_menu { Some(Size::new(20, 5)) } else { None });
                        return EventProcessStatus::Processed;
                    }
                }
                EventProcessStatus::Ignored
            }
        }
    }

    #[Window(events: CheckBoxEvents, internal: true)]
    pub struct MyWindow {
        hc: Handle<mycustomcontrol::MyCustomControl>,
        cb: Handle<CheckBox>,
    }
    impl MyWindow {
        pub fn new() -> Self {
            let mut w = MyWindow {
                base: Window::new("Test", Layout::new("d:c,w:76,h:10"), window::Flags::None),
                hc: Handle::None,
                cb: Handle::None,
            };
            w.hc = w.add(mycustomcontrol::MyCustomControl::new(Layout::new("x:50%,y:6,a:c,w:16,h:4")));
            w.add(label!(
                "'Press the right mouse button on the square below to show a popup menu',x:37,y:1,a:c,w:70,h:1"
            ));
            w.cb = w.add(checkbox!("'&Limit the meniu size to 3 items',x:2,y:2,w:30,checked:false"));

            w
        }
    }
    impl CheckBoxEvents for MyWindow {
        fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
            if handle == self.cb {
                let h = self.hc;
                if let Some(obj) = self.get_control_mut(h) {
                    obj.enable_small_menu(checked);
                }
                return EventProcessStatus::Processed;
            }
            return EventProcessStatus::Ignored;
        }
    }
    let script = "
            Paint.Enable(false)
            Paint('initial_state')
            CheckHash(0xca08a561329e08e0)
            Mouse.Move(39,13)
            Mouse.Click(39,13,right)
            Paint('popup menu on top')
            CheckHash(0xbc698d1fff6c047c)
            Mouse.Move(48,5)
            Mouse.Click(48,5,left)
            Paint('color is aqua')
            CheckHash(0xbadca93818a37db4)
            Mouse.Move(43,13)
            Mouse.Click(43,13,right)
            Paint('menu again')
            CheckHash(0x614a550849150d1f)
            Mouse.Move(53,8)
            Mouse.Click(53,8,left)
            Mouse.Move(53,7)
            Paint('menu is blue')
            CheckHash(0xdeb1cb68b9415d4)
            Mouse.Move(39,7)
            Mouse.Drag(39,7,39,2)
            Paint('window moved on top')
            CheckHash(0x7daeebcf9fc73b54)
            Mouse.Move(40,8)
            Mouse.Click(40,8,right)
            Paint('now menu is on bottom')
            CheckHash(0x4c2e4aa471f44b21)
            Mouse.Move(35,8)
            Mouse.Click(35,8,left)
            Mouse.Move(28,8)
            Paint('no menu showed')
            CheckHash(0x7daeebcf9fc73b54)
            Mouse.Move(5,5)
            Mouse.Click(5,5,left)
            Paint('enable small menus')
            CheckHash(0xca41721d2facc991)
            Mouse.Move(38,8)
            Mouse.Click(38,8,right)
            Paint('small menu with 3 items')
            CheckHash(0xcd82bea014628957)
            Mouse.Move(47,12)
            Mouse.Click(47,12,left)
            Paint('scroll one item')
            CheckHash(0xae6744d7d3a78b88)
            Mouse.Move(47,10)
            Mouse.Click(47,10,left)
            Paint('menu is now black')
            CheckHash(0x496f95b6093f7b49)
            Mouse.Move(21,5)
            Mouse.Click(21,5,left)
            Mouse.Move(22,5)
            Paint('remove the limit of 3 items')
            CheckHash(0x8c66b5179d508670)
            Mouse.Move(40,2)
            Mouse.Drag(40,2,40,7)
            Mouse.Move(38,13)
            Mouse.Click(38,13,right)
            Mouse.Move(38,14)
            Mouse.Click(38,14,right)
            Mouse.Click(38,14,right)
            Mouse.Move(35,12)
            Mouse.Click(35,12,right)
            Mouse.Click(35,12,right)
            Paint('menu on top, but scrolled (up to light red)')
            CheckHash(0x4191bdc942455569)
            Mouse.Move(44,12)
            Mouse.Click(44,12,left)
            Paint('menu on top scrolled (up to light green)')
            CheckHash(0xd8069e838766bdfb)
            Mouse.Move(43,11)
            Mouse.Click(43,11,left)
            Paint('color is light green')
            CheckHash(0x8997a3906f032270)    
        ";
    let mut a = App::debug(80, 24, script).menu_bar().build().unwrap();
    a.add_window(MyWindow::new());
    a.run();
}

#[test]
fn check_popup_menu_with_keys() {
    pub(crate) mod mycustomcontrol {
        use crate::prelude::*;

        #[CustomControl(events = MenuEvents, overwrite = OnPaint+OnMouseEvent, commands = Red+Green+Black+Aqua+Magenta+Yellow+Blue+Gray+White+LightRed+LightGreen, internal: true)]
        pub struct MyCustomControl {
            col: Color,
            h_menu: Handle<Menu>,
            small_menu: bool,
        }
        impl MyCustomControl {
            pub fn new(layout: Layout) -> Self {
                let mut obj = Self {
                    base: ControlBase::new(layout, true),
                    col: Color::Red,
                    h_menu: Handle::None,
                    small_menu: false,
                };
                let m = menu!(
                    "ColorControl,class:MyCustomControl,items=[
                    {&Red,selected:true,cmd:Red},
                    {&Green,selected:false,cmd:Green},
                    {Black,selected:false,cmd:Black},
                    {&Aqua,selected:false,cmd:Aqua},
                    {&Magenta,selected:false,cmd:Magenta},
                    {&Yellow,selected:false,cmd:Yellow},
                    {&Blue,selected:false,cmd:Blue},
                    {Gray,selected:false,cmd:Gray},
                    {White,selected:false,cmd:White},
                    {'Light red',selected:false,cmd:LightRed},
                    {'Light green',selected:false,cmd:LightGreen},
                    ]"
                );
                obj.h_menu = obj.register_menu(m);
                obj
            }
            pub fn enable_small_menu(&mut self, value: bool) {
                self.small_menu = value;
            }
        }
        impl OnPaint for MyCustomControl {
            fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
                surface.clear(Character::new(' ', Color::Black, self.col, CharFlags::None));
                let sz = self.get_client_size();
                let attr = CharAttribute::with_fore_color(Color::White);
                let line = if self.has_focus() { LineType::Double } else { LineType::Single };
                let r = Rect::with_size(0, 0, sz.width as u16, sz.height as u16);
                surface.draw_rect(r, line, attr);
            }
        }
        impl MenuEvents for MyCustomControl {
            fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<menu::SingleChoice>, command: mycustomcontrol::Commands) {
                match command {
                    mycustomcontrol::Commands::Red => self.col = Color::DarkRed,
                    mycustomcontrol::Commands::Green => self.col = Color::DarkGreen,
                    mycustomcontrol::Commands::Black => self.col = Color::Black,
                    mycustomcontrol::Commands::Aqua => self.col = Color::Aqua,
                    mycustomcontrol::Commands::Magenta => self.col = Color::Magenta,
                    mycustomcontrol::Commands::Yellow => self.col = Color::Yellow,
                    mycustomcontrol::Commands::Blue => self.col = Color::Blue,
                    mycustomcontrol::Commands::Gray => self.col = Color::Gray,
                    mycustomcontrol::Commands::White => self.col = Color::White,
                    mycustomcontrol::Commands::LightRed => self.col = Color::Red,
                    mycustomcontrol::Commands::LightGreen => self.col = Color::Green,
                }
            }
        }
        impl OnMouseEvent for MyCustomControl {
            fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
                if let MouseEvent::Pressed(ev) = event {
                    if ev.button == MouseButton::Right {
                        self.show_menu(self.h_menu, ev.x, ev.y, if self.small_menu { Some(Size::new(20, 5)) } else { None });
                        return EventProcessStatus::Processed;
                    }
                }
                EventProcessStatus::Ignored
            }
        }
    }

    #[Window(events: CheckBoxEvents, internal: true)]
    pub struct MyWindow {
        hc: Handle<mycustomcontrol::MyCustomControl>,
        cb: Handle<CheckBox>,
    }
    impl MyWindow {
        pub fn new() -> Self {
            let mut w = MyWindow {
                base: Window::new("Test", Layout::new("d:c,w:76,h:10"), window::Flags::None),
                hc: Handle::None,
                cb: Handle::None,
            };
            w.hc = w.add(mycustomcontrol::MyCustomControl::new(Layout::new("x:50%,y:6,a:c,w:16,h:4")));
            w.add(label!(
                "'Press the right mouse button on the square below to show a popup menu',x:37,y:1,a:c,w:70,h:1"
            ));
            w.cb = w.add(checkbox!("'&Limit the meniu size to 3 items',x:2,y:2,w:30,checked:false"));

            w
        }
    }
    impl CheckBoxEvents for MyWindow {
        fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
            if handle == self.cb {
                let h = self.hc;
                if let Some(obj) = self.get_control_mut(h) {
                    obj.enable_small_menu(checked);
                }
                return EventProcessStatus::Processed;
            }
            return EventProcessStatus::Ignored;
        }
    }
    let script = "
            Paint.Enable(false)
            Paint('initial_state')
            CheckHash(0xca08a561329e08e0)
            Mouse.Click(39,13,right)
            Paint('popup menu on top')
            CheckHash(0xBC698D1FFF6C047C)
            Key.Pressed('G')
            Paint('Green color selected')
            CheckHash(0x1AECE5B91EDFACD0)
            Mouse.Click(39,13,right)
            Key.Pressed('M')
            Paint('Magenta color selected')
            CheckHash(0x60580F11EE435534)
            Mouse.Click(39,13,right)
            Key.Pressed(Down,3)
            Paint('Black selected')
            CheckHash(0x23506C27C5D8377F)
            Key.Pressed(Enter)
            Paint('Black color selected')
            CheckHash(0x6D59577FB15D2468)
            Mouse.Click(39,13,right)
            Key.Pressed(Down,5)
            Paint('Magenta selected')
            CheckHash(0x360C3880A88E84EB)
            Key.Pressed(Escape)
            Paint('Normal state')
            CheckHash(0x6D59577FB15D2468)

        ";
    let mut a = App::debug(80, 24, script).menu_bar().build().unwrap();
    a.add_window(MyWindow::new());
    a.run();
}

#[test]
fn check_menubar_with_keys() {
    #[Window(events : MenuEvents, commands  : A, internal: true)]
    struct MyWindow {
        h_file: Handle<Menu>,
        h_edit: Handle<Menu>,
        h_help: Handle<Menu>,
        lb: Handle<Label>,
    }
    impl MyWindow {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,d:c,w:40,h:8"),
                h_file: Handle::None,
                h_edit: Handle::None,
                h_help: Handle::None,
                lb: Handle::None,
            };
            w.lb = w.add(label!("None,d:c,w:30,h:1"));
            // construct a popup menu
            w.h_file = w.register_menu(menu!(
                "&File,class: MyWindow, items=[
                {New,cmd:A},
                {&Save,cmd:A},
                {'&Save As ...',cmd:A},
                {&Open,cmd:A},
                {-},
                {E&xit,Alt+F4,cmd:A}
            ]"
            ));
            w.h_edit = w.register_menu(menu!(
                "&Edit,class: MyWindow, items=[
                {&Copy,cmd:A},
                {&Paste,cmd:A},
                {&Cut,cmd:A},
                {-},
                {&Special,items=[
                    {'Slot &1',cmd:A},
                    {'Slot &2',cmd:A},
                    {'Slot &3',cmd:A},
                    {'Slot &4',cmd:A},
                    {'Slot &5',cmd:A},
                ]}            
            ]"
            ));
            w.h_help = w.register_menu(menu!(
                "&Help,class: MyWindow, items=[
                {&About,cmd:A},
                {&Update,cmd:A},
                {-},
                {&Tutorials,items=[
                    {'&Usage',cmd:A},
                    {'&Download',cmd:A},
                    {&Time,items=[
                        {'Day &1',cmd:A},
                        {'Day &2',cmd:A},
                        {'Day &3',cmd:A},
                    ]}            
                ]}            
            ]"
            ));
            w
        }
    }
    impl MenuEvents for MyWindow {
        fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, _: mywindow::Commands) {
            if let Some(i) = self.get_menuitem(menu, item) {
                let s = String::from(i.get_caption());
                let h = self.lb;
                if let Some(l) = self.get_control_mut(h) {
                    l.set_caption(&s);
                }
            }
        }

        fn on_update_menubar(&self, menubar: &mut MenuBar) {
            menubar.add(self.h_file);
            menubar.add(self.h_edit);
            menubar.add(self.h_help);
        }
    }

    let script = "
            Paint.Enable(false)
            Paint('Initial state')
            CheckHash(0x91b83be85febb5c)
            Paint('State_3')
            CheckHash(0x91b83be85febb5c)
            Key.Pressed(Alt+F)
            Paint('State_4')
            CheckHash(0xa918009138fe8394)
            Key.Pressed(Right)
            Paint('State_5')
            CheckHash(0xd06ed5592e610e0f)
            Key.Pressed(Right)
            Paint('State_6')
            CheckHash(0xa2e5920f376a7c9)
            Key.Pressed(U)
            Paint('State_7')
            CheckHash(0x17a048b709c71033)
            Key.Pressed(Alt+F)
            Paint('State_8')
            CheckHash(0xe284e496b7d64afb)
            Key.Pressed(S)
            Paint('State_9')
            CheckHash(0x75e666eee2ec2383)
            Key.Pressed(Alt+E)
            Paint('State_10')
            CheckHash(0x5bccabd112b0ed28)
            Key.Pressed(S)
            Paint('State_11')
            CheckHash(0xc8c4a1989978790e)
            Key.Pressed(Down)
            Paint('State_12')
            CheckHash(0xe3e988ca07ae6fe6)
            Key.Pressed(Down)
            Paint('State_13')
            CheckHash(0xd2df5bfbc348f96)
            Key.Pressed(Down)
            Paint('State_14')
            CheckHash(0x2d2b5de8d5a7fe6)
            Key.Pressed(Down)
            Paint('State_15')
            CheckHash(0x205ef8f927624f76)
            Key.Pressed(Down)
            Paint('State_16')
            CheckHash(0xce1e7f26f590f866)
            Key.Pressed(Enter)
            Paint('State_17')
            CheckHash(0x7e95895334949bc7)
            Key.Pressed(Alt+H)
            Paint('State_18')
            CheckHash(0xf476bd281922256a)
            Key.Pressed(T)
            Paint('State_19')
            CheckHash(0xf6d7db13b0b1b332)
            Key.Pressed(T)
            Paint('State_20')
            CheckHash(0xa68414070d2a191c)
            Key.Pressed(Down)
            Paint('State_21')
            CheckHash(0x97b5cc6f878adc34)
            Key.Pressed(Down)
            Paint('State_22')
            CheckHash(0x85047c827afb8e74)
            Key.Pressed(Enter)
            Paint('State_23')
            CheckHash(0x29d76feb6df37f57)
            Key.Pressed(Alt+F)
            Paint('State_24')
            CheckHash(0xd6801a161b7f144f)
            Key.Pressed(Down)
            Paint('State_25')
            CheckHash(0x530cc0e8ac7b3207)
            Key.Pressed(Down)
            Paint('State_26')
            CheckHash(0xad73304bd2c81b7)
            Key.Pressed(Right)
            Paint('State_27')
            CheckHash(0x9de3637f03dcd744)
            Key.Pressed(Left)
            Paint('State_28')
            CheckHash(0xd6801a161b7f144f)
            Key.Pressed(Right)
            Paint('State_29')
            CheckHash(0x9de3637f03dcd744)
            Key.Pressed(Down)
            Paint('State_30')
            CheckHash(0x73be6d2e4e3bea5c)
            Key.Pressed(Down)
            Paint('State_31')
            CheckHash(0xc1dbc3b0e5ffa5c)
            Key.Pressed(Down)
            Paint('State_32')
            CheckHash(0xf09bd9750d5dcdfc)
            Key.Pressed(Down)
            Paint('State_33')
            CheckHash(0x37ce7965152d471c)
            Key.Pressed(Right)
            Paint('State_34')
            CheckHash(0x3430b51df070dcd)
            Key.Pressed(Down)
            Paint('State_35')
            CheckHash(0xe64221bacee7efe5)
            Key.Pressed(Down)
            Paint('State_36')
            CheckHash(0x422121deb76b6df5)
            Key.Pressed(Down)
            Paint('State_37')
            CheckHash(0x52b4ecf5493ffe5)
            Key.Pressed(Left)
            Paint('State_38')
            CheckHash(0x37ce7965152d471c)
            Key.Pressed(Left)
            Paint('State_39')
            CheckHash(0xd6801a161b7f144f)
            Key.Pressed(Right)
            Paint('State_40')
            CheckHash(0x9de3637f03dcd744)
            Key.Pressed(Right)
            Paint('State_41')
            CheckHash(0x10f873625ddc4f52)
            Key.Pressed(T)
            Paint('State_42')
            CheckHash(0x8c36a2dd1f0b78a)
            Key.Pressed(Right)
            Paint('State_43')
            CheckHash(0xd6801a161b7f144f)
            Key.Pressed(Alt+H)
            Paint('State_44')
            CheckHash(0x10f873625ddc4f52)
            Key.Pressed(T)
            Paint('State_45')
            CheckHash(0x8c36a2dd1f0b78a)
            Key.Pressed(T)
            Paint('State_46')
            CheckHash(0xe9823256d89c85ec)
            Key.Pressed(Down)
            Paint('State_47')
            CheckHash(0xef869c71dcb89a94)
            Key.Pressed(Down)
            Paint('State_48')
            CheckHash(0xc3544620e83d04)
            Key.Pressed(Down)
            Paint('State_49')
            CheckHash(0x1e7a2ed30fdad484)
            Key.Pressed(Left)
            Paint('State_50')
            CheckHash(0x37ee79faf757c172)
            Key.Pressed(Up)
            Paint('State_51')
            CheckHash(0x7e0dd88d668d7372)
            Key.Pressed(Up)
            Paint('State_52')
            CheckHash(0x1de0a99c3bca602)
            Key.Pressed(Left)
            Paint('State_53')
            CheckHash(0xd980b4cfb1a79f7a)
            Key.Pressed(Up)
            Paint('State_54')
            CheckHash(0xa4ab426dd6c7a96a)
            Key.Pressed(Up)
            Paint('State_55')
            CheckHash(0x5f67cd6dac41f2ba)
            Key.Pressed(Left)
            Paint('State_56')
            CheckHash(0x9de3637f03dcd744)
            Key.Pressed(P)
            Paint('State_57')
            CheckHash(0xf45c01a4988b0fe2)
            Key.Pressed(Escape)
            Paint('State_58')
            CheckHash(0x7d77e1090489150e)
        ";
    let mut a = App::debug(60, 20, script).menu_bar().build().unwrap();
    a.add_window(MyWindow::new());
    a.run();
}

#[test]
fn check_menubar_recursive_shortcuts() {
    #[Window(events : MenuEvents, commands  : A, internal: true)]
    struct MyWindow {
        h_file: Handle<Menu>,
        h_edit: Handle<Menu>,
        h_help: Handle<Menu>,
        lb: Handle<Label>,
    }
    impl MyWindow {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,d:c,w:40,h:8"),
                h_file: Handle::None,
                h_edit: Handle::None,
                h_help: Handle::None,
                lb: Handle::None,
            };
            w.lb = w.add(label!("None,d:c,w:30,h:1"));
            // construct a popup menu
            w.h_file = w.register_menu(menu!(
                "&File,class: MyWindow, items=[
                {New,F1,cmd:A},
                {&Save,F2,cmd:A},
                {'&Save As ...',Alt+F2,cmd:A},
                {&Open,F3,cmd:A},
                {-},
                {E&xit,Alt+F4,cmd:A}
            ]"
            ));
            w.h_edit = w.register_menu(menu!(
                "&Edit,class: MyWindow, items=[
                {&Copy,Ctrl+Ins,cmd:A},
                {&Paste,Shift+Ins,cmd:A},
                {&Cut,Ctrl+X,cmd:A},
                {-},
                {&Special,items=[
                    {'Slot &1',Alt+1,cmd:A},
                    {'Slot &2',Alt+2,cmd:A},
                    {'Slot &3',Alt+3,cmd:A},
                    {'Slot &4',Alt+4,cmd:A},
                    {'Slot &5',Alt+5,cmd:A},
                ]}            
            ]"
            ));
            w.h_help = w.register_menu(menu!(
                "&Help,class: MyWindow, items=[
                {&About,Ctrl+Shift+A,cmd:A},
                {&Update,F10,cmd:A},
                {-},
                {&Tutorials,items=[
                    {'&Usage',Alt+U,cmd:A},
                    {'&Download',Ctrl+D,cmd:A},
                    {&Time,items=[
                        {'Day &1',Ctrl+Alt+Shift+1,cmd:A},
                        {'Day &2',Ctrl+Alt+Shift+2,cmd:A},
                        {'Day &3',Ctrl+Alt+Shift+3,cmd:A},
                    ]}            
                ]}            
            ]"
            ));
            w
        }
    }
    impl MenuEvents for MyWindow {
        fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, _: mywindow::Commands) {
            if let Some(i) = self.get_menuitem(menu, item) {
                let s = String::from(i.get_caption());
                let h = self.lb;
                if let Some(l) = self.get_control_mut(h) {
                    l.set_caption(&s);
                }
            }
        }

        fn on_update_menubar(&self, menubar: &mut MenuBar) {
            menubar.add(self.h_file);
            menubar.add(self.h_edit);
            menubar.add(self.h_help);
        }
    }

    let script = "
            Paint.Enable(false)
            Paint('Initial State')
            CheckHash(0x91b83be85febb5c)
            Paint('State_3')
            CheckHash(0x91b83be85febb5c)
            Key.Pressed(F1)
            Paint('State_4')
            CheckHash(0x991cc1c2da3524e5)
            Key.Pressed(F2)
            Paint('State_5')
            CheckHash(0x75e666eee2ec2383)
            Key.Pressed(F3)
            Paint('State_6')
            CheckHash(0xbd478c6ec1d61d7a)
            Key.Pressed(F10)
            Paint('State_7')
            CheckHash(0x17a048b709c71033)
            Key.Pressed(Ctrl+Insert)
            Paint('State_8')
            CheckHash(0x6a44a1c00095e983)
            Key.Pressed(Shift+Insert)
            Paint('State_9')
            CheckHash(0xf45c01a4988b0fe2)
            Key.Pressed(Ctrl+X)
            Paint('State_10')
            CheckHash(0x74119be5c3c516b)
            Key.Pressed(Alt+1)
            Paint('State_11')
            CheckHash(0x2ac25f636c040343)
            Key.Pressed(Alt+2)
            Paint('State_12')
            CheckHash(0x1538278948c78240)
            Key.Pressed(Alt+3)
            Paint('State_13')
            CheckHash(0xa5e95649068b8b0d)
            Key.Pressed(Alt+4)
            Paint('State_14')
            CheckHash(0xcc5a96a3371609a2)
            Key.Pressed(Alt+5)
            Paint('State_15')
            CheckHash(0x7e95895334949bc7)
            Key.Pressed(Ctrl+Alt+Shift+1)
            Paint('State_16')
            CheckHash(0x58dec6ef41fcf9ec)
            Key.Pressed(Ctrl+Alt+Shift+2)
            Paint('State_17')
            CheckHash(0x29d76feb6df37f57)
            Key.Pressed(Ctrl+Alt+Shift+3)
            Paint('State_18')
            CheckHash(0x18e2245507c5d476)
            Key.Pressed(Alt+D)
            Key.Pressed(Alt+H)
            Paint('State_19')
            CheckHash(0x12ed0204088005fd)
            Key.Pressed(A)
            Paint('State_20')
            CheckHash(0x36480d44d0d8e490)
            Key.Pressed(F10)
            Paint('State_21')
            CheckHash(0x17a048b709c71033)
            Key.Pressed(Ctrl+Alt+Shift+A)
            Key.Pressed(Alt+G)
            Key.Pressed(Alt+H)
            Paint('State_22')
            CheckHash(0x1083ad875a04b658)
            Key.Pressed(Escape)
            Paint('State_23')
            CheckHash(0x17a048b709c71033)
            Key.Pressed(Ctrl+Shift+A)
            Paint('State_24')
            CheckHash(0x36480d44d0d8e490)
            Key.Pressed(Escape)
            Paint('State_25')
            CheckHash(0x7d77e1090489150e)    
        ";
    let mut a = App::debug(60, 20, script).menu_bar().build().unwrap();
    a.add_window(MyWindow::new());
    a.run();
}
