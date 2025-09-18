use crate::{
    prelude::*,
    ui::appbar::{Side, MenuButton},
};

#[test]
fn check_view() {
    #[Window(events = AppBarEvents, commands=A+B+C, internal: true)]
    struct MyWin {
        m_file: Handle<MenuButton>,
        m_edit: Handle<MenuButton>,
        m_help: Handle<MenuButton>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,a:c,w:40,h:8"),
                m_file: Handle::None,
                m_help: Handle::None,
                m_edit: Handle::None,
            };
            let mut m = Menu::new();
            m.add(menu::Command::new("&New", key!("F1"), mywin::Commands::A));
            m.add(menu::Command::new("&Save", key!("F2"), mywin::Commands::A));
            m.add(menu::Command::new("&Open", key!("F3"), mywin::Commands::A));
            m.add(menu::Separator::new());
            m.add(menu::Command::new("E&xit", key!("Alt+F4"), mywin::Commands::A));
            w.m_file = w.appbar().add(MenuButton::new("&File", m, 0, Side::Left));

            let mut m = Menu::new();
            m.add(menu::Command::new("&Copy", Key::None, mywin::Commands::B));
            m.add(menu::Command::new("C&ut", Key::None, mywin::Commands::B));
            m.add(menu::Command::new("&Paste", Key::None, mywin::Commands::B));
            m.add(menu::Separator::new());
            m.add(menu::SingleChoice::new("Paste only text", Key::None, mywin::Commands::B, true));
            m.add(menu::SingleChoice::new("Paste only images", Key::None, mywin::Commands::B, false));
            m.add(menu::SingleChoice::new("Paste everything", Key::None, mywin::Commands::B, false));
            w.m_edit = w.appbar().add(MenuButton::new("&Edit", m, 0, Side::Left));

            let mut m = Menu::new();
            m.add(menu::Command::new("About", Key::None, mywin::Commands::C));
            w.m_help = w.appbar().add(MenuButton::new("&Help", m, 0, Side::Left));
            w
        }
    }
    impl AppBarEvents for MyWin {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.m_file);
            appbar.show(self.m_edit);
            appbar.show(self.m_help);
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
    let mut a = App::debug(60, 15, script).app_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_scroll_button_activation() {
    #[Window(events = AppBarEvents, commands=A+B+C, internal: true)]
    struct MyWin {
        m_file: Handle<MenuButton>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,a:c,w:40,h:8"),
                m_file: Handle::None,
            };
            let m = menu!(
                "class:MyWin, items=[
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
            w.m_file = w.appbar().add(MenuButton::new("&Keywords", m, 0, Side::Left));
            w
        }
    }
    impl AppBarEvents for MyWin {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.m_file);
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
    let mut a = App::debug(60, 15, script).app_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_submenus_open() {
    #[Window(events = AppBarEvents, commands=A+B+C, internal: true)]
    struct MyWin {
        m_file: Handle<MenuButton>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,a:c,w:40,h:8"),
                m_file: Handle::None,
            };
            let m = menu!(
                "class:MyWin, items=[
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
            w.m_file = w.appbar().add(MenuButton::new("&Menu", m, 0, Side::Left));
            w
        }
    }
    impl AppBarEvents for MyWin {
        fn on_update(&self, menubar: &mut AppBar) {
            menubar.show(self.m_file);
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
    let mut a = App::debug(60, 15, script).app_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_dynamic_change_menu() {
    #[Window(events = MenuEvents+AppBarEvents, commands=Increment, internal: true)]
    struct MyWin {
        m_counter: Handle<menu::Command>,
        some_menu: Handle<MenuButton>,
        counter: u32,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,a:c,w:40,h:8"),
                m_counter: Handle::None,
                some_menu: Handle::None,
                counter: 0,
            };
            let mut m = Menu::new();
            w.m_counter = m.add(menuitem!("'Increment (0)',cmd:Increment,class:MyWin"));
            w.some_menu = w.appbar().add(MenuButton::new("Some menu", m, 0, Side::Left));

            w
        }
    }
    impl MenuEvents for MyWin {
        fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, _: mywin::Commands) {
            if item == self.m_counter {
                self.counter += 1;
                let new_text = format!("Increment ({})", self.counter);
                if let Some(menuitem) = self.menuitem_mut(menu, item) {
                    menuitem.set_caption(new_text.as_str());
                }
            }
        }
    }
    impl AppBarEvents for MyWin {
        fn on_update(&self, menubar: &mut AppBar) {
            menubar.show(self.some_menu);
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
    let mut a = App::debug(60, 24, script).app_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_dynamic_change_menu_2() {
    #[Window(events = AppBarEvents+MenuEvents, commands=Increment, internal: true)]
    struct MyWin {
        m_counter: Handle<menu::Command>,
        some_menu: Handle<MenuButton>,
        counter: u32,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,a:c,w:40,h:8"),
                m_counter: Handle::None,
                some_menu: Handle::None,
                counter: 0,
            };
            let mut m = Menu::new();
            w.m_counter = m.add(menuitem!("'Increment (0)',cmd:Increment,class:MyWin"));
            w.some_menu = w.appbar().add(MenuButton::new("Some menu", m, 0, Side::Left));

            w
        }
    }
    impl MenuEvents for MyWin {
        fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, _: mywin::Commands) {
            if item == self.m_counter {
                self.counter += 1;
                let new_text = format!("Increment ({})", self.counter);
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
        Paint('State_22 (desktop is empty, no menu in menubar)')
        CheckHash(0xDB84D57C7804761D)
    ";
    let mut a = App::debug(60, 24, script).app_bar().build().unwrap();
    a.add_window(MyWin::new());
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
                    "class:MyCustomControl,items=[
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
                let sz = self.client_size();
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
                base: Window::new("Test", layout!("a:c,w:76,h:10"), window::Flags::None),
                hc: Handle::None,
                cb: Handle::None,
            };
            w.hc = w.add(mycustomcontrol::MyCustomControl::new(layout!("x:50%,y:6,p:c,w:16,h:4")));
            w.add(label!(
                "'Press the right mouse button on the square below to show a popup menu',x:37,y:1,p:c,w:70,h:1"
            ));
            w.cb = w.add(checkbox!("'&Limit the meniu size to 3 items',x:2,y:2,w:30,checked:false"));

            w
        }
    }
    impl CheckBoxEvents for MyWindow {
        fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
            if handle == self.cb {
                let h = self.hc;
                if let Some(obj) = self.control_mut(h) {
                    obj.enable_small_menu(checked);
                }
                return EventProcessStatus::Processed;
            }
            EventProcessStatus::Ignored
        }
    }
    let script = "
            Paint.Enable(false)
            Paint('initial_state')
            CheckHash(0xD8785911D8A3E144)
            Mouse.Move(39,13)
            Mouse.Click(39,13,right)
            Paint('popup menu on top')
            CheckHash(0x27E1510C1208B360)
            Mouse.Move(48,5)
            Mouse.Click(48,5,left)
            Paint('color is aqua')
            CheckHash(0xE159A8823308D660)
            Mouse.Move(43,13)
            Mouse.Click(43,13,right)
            Paint('menu again')
            CheckHash(0x154B1288966A77AB)
            Mouse.Move(53,8)
            Mouse.Click(53,8,left)
            Mouse.Move(53,7)
            Paint('menu is blue')
            CheckHash(0x1A5B4549FC2ACC0)
            Mouse.Move(39,7)
            Mouse.Drag(39,7,39,2)
            Paint('window moved on top')
            CheckHash(0x5E2E275A2BEC81C0)
            Mouse.Move(40,8)
            Mouse.Click(40,8,right)
            Paint('now menu is on bottom')
            CheckHash(0xB391F5104AC11BDD)
            Mouse.Move(35,8)
            Mouse.Click(35,8,left)
            Mouse.Move(28,8)
            Paint('no menu showed')
            CheckHash(0x5E2E275A2BEC81C0)
            Mouse.Move(5,5)
            Mouse.Click(5,5,left)
            Paint('enable small menus')
            CheckHash(0x2055B958836FA86E)
            Mouse.Move(38,8)
            Mouse.Click(38,8,right)
            Paint('small menu with 3 items')
            CheckHash(0x2B309344AE720CA4)
            Mouse.Move(47,12)
            Mouse.Click(47,12,left)
            Paint('scroll one item')
            CheckHash(0xABBE597E9C0F7D53)
            Mouse.Move(47,10)
            Mouse.Click(47,10,left)
            Paint('menu is now black')
            CheckHash(0xCEAD492B41918786)
            Mouse.Move(21,5)
            Mouse.Click(21,5,left)
            Mouse.Move(22,5)
            Paint('remove the limit of 3 items')
            CheckHash(0xE2E7E5BCDEB2414)
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
            CheckHash(0xC729D9D22633EBC5)
            Mouse.Move(44,12)
            Mouse.Click(44,12,left)
            Paint('menu on top scrolled (up to light green)')
            CheckHash(0x9B59E08491100CA7)
            Mouse.Move(43,11)
            Mouse.Click(43,11,left)
            Paint('color is light green')
            CheckHash(0x8A597798E864C55C)    
        ";
    let mut a = App::debug(80, 24, script).app_bar().build().unwrap();
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
                    "class:MyCustomControl,items=[
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
                let sz = self.client_size();
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
                base: Window::new("Test", layout!("a:c,w:76,h:10"), window::Flags::None),
                hc: Handle::None,
                cb: Handle::None,
            };
            w.hc = w.add(mycustomcontrol::MyCustomControl::new(layout!("x:50%,y:6,p:c,w:16,h:4")));
            w.add(label!(
                "'Press the right mouse button on the square below to show a popup menu',x:37,y:1,p:c,w:70,h:1"
            ));
            w.cb = w.add(checkbox!("'&Limit the meniu size to 3 items',x:2,y:2,w:30,checked:false"));

            w
        }
    }
    impl CheckBoxEvents for MyWindow {
        fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
            if handle == self.cb {
                let h = self.hc;
                if let Some(obj) = self.control_mut(h) {
                    obj.enable_small_menu(checked);
                }
                return EventProcessStatus::Processed;
            }
            EventProcessStatus::Ignored
        }
    }
    let script = "
            Paint.Enable(false)
            Paint('initial_state')
            CheckHash(0xD8785911D8A3E144)
            Mouse.Click(39,13,right)
            Paint('popup menu on top')
            CheckHash(0x27E1510C1208B360)
            Key.Pressed('G')
            Paint('Green color selected')
            CheckHash(0xEA78E87B12F04C0C)
            Mouse.Click(39,13,right)
            Key.Pressed('M')
            Paint('Magenta color selected')
            CheckHash(0xE67A114A46E98160)
            Mouse.Click(39,13,right)
            Key.Pressed(Down,3)
            Paint('Black selected')
            CheckHash(0xDE77E1A4F67D5A0B)
            Key.Pressed(Enter)
            Paint('Black color selected')
            CheckHash(0xC71AEEECAC9C2C54)
            Mouse.Click(39,13,right)
            Key.Pressed(Down,5)
            Paint('Magenta selected')
            CheckHash(0xEC93049622378D37)
            Key.Pressed(Escape)
            Paint('Normal state')
            CheckHash(0xC71AEEECAC9C2C54)

        ";
    let mut a = App::debug(80, 24, script).app_bar().build().unwrap();
    a.add_window(MyWindow::new());
    a.run();
}

#[test]
fn check_menu_checkbox_methods() {
    #[Desktop(events =  AppBarEvents+MenuEvents+DesktopEvents,  commands: [A,B,C], internal = true)]
    struct MyDesktop {
        m_desktop: Handle<MenuButton>,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                m_desktop: Handle::None,
            }
        }
    }

    impl DesktopEvents for MyDesktop {
        fn on_start(&mut self) {
            let m = menu!(
                "class:MyDesktop,items=[
                    {&A,cmd:A, checked: true},
                    {&B,cmd:B, checked: false},
                    {&C,cmd:C, checked: true}
                ]"
            );
            self.m_desktop = self.appbar().add(MenuButton::new("Desktop", m, 0, Side::Left));
        }
    }
    impl MenuEvents for MyDesktop {
        fn on_check(&mut self, menu: Handle<Menu>, item: Handle<menu::CheckBox>, _: mydesktop::Commands, _: bool) {
            if let Some(i) = self.menuitem_mut(menu, item) {
                assert!(i.is_checked());
                assert!(i.is_enabled());
                assert_eq!(i.caption(), "B");
                i.set_caption("B is checked");
                i.set_enabled(false);
                i.set_shortcut(key!("Ctrl+B"));
                assert_eq!(i.shortcut(), Key::new(KeyCode::B, KeyModifier::Ctrl));
            }
        }
    }
    impl AppBarEvents for MyDesktop {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.m_desktop);
        }        
    }
    let script = "
        Paint.Enable(false)
        Paint('1.Initial state')
        CheckHash(0x769320EBFCBC2E03)
        Mouse.Click(4,0,left)
        Paint('2.Menu open')
        CheckHash(0xD1BCA6516F85A1E7)
        Mouse.Move(4,3)
        Paint('3.Move over B')
        CheckHash(0x88D7FBBBE7F71D1F)
        Mouse.Click(4,3,left)
        Paint('4.Click over B')
        CheckHash(0x769320EBFCBC2E03)
        Mouse.Click(4,0,left)
        Paint('5.Menu open again (all are checked and B title is <B is checked>, B is disabled, B shortcut is Ctrl+B)')
        CheckHash(0xC5EB204AA2303162)
    ";
    App::debug(60, 15, script).desktop(MyDesktop::new()).app_bar().build().unwrap().run();
}

#[test]
fn check_menu_singlechoice_methods() {
    #[Desktop(events =  AppBarEvents+MenuEvents+DesktopEvents,  commands: [A,B,C], internal = true)]
    struct MyDesktop {
        m_desktop: Handle<MenuButton>,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                m_desktop: Handle::None,
            }
        }
    }

    impl DesktopEvents for MyDesktop {
        fn on_start(&mut self) {
            let m = menu!(
                "class:MyDesktop,items=[
                    {&A,cmd:A, selected: true},
                    {&B,cmd:B, selected: false},
                    {&C,cmd:C, selected: false}
                ]"
            );
            self.m_desktop = self.appbar().add(MenuButton::new("Desktop", m, 0, Side::Left));
        }
    }
    impl MenuEvents for MyDesktop {
        fn on_select(&mut self, menu: Handle<Menu>, item: Handle<menu::SingleChoice>, _: mydesktop::Commands) {
            if let Some(i) = self.menuitem_mut(menu, item) {
                assert!(i.is_selected());
                assert!(i.is_enabled());
                assert_eq!(i.caption(), "B");
                i.set_caption("B is selected");
                i.set_enabled(false);
                i.set_shortcut(key!("Ctrl+B"));
                assert_eq!(i.shortcut(), Key::new(KeyCode::B, KeyModifier::Ctrl));
            }
        }
    }
    impl AppBarEvents for MyDesktop {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.m_desktop);
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1.Initial state')
        CheckHash(0x769320EBFCBC2E03)
        Mouse.Click(4,0,left)
        Paint('2.Menu open')
        CheckHash(0xA65B33445CC2260D)
        Mouse.Move(4,3)
        Paint('3.Move over B')
        CheckHash(0xF2F67373DCFDD2F6)
        Mouse.Click(4,3,left)
        Paint('4.Click over B')
        CheckHash(0x769320EBFCBC2E03)
        Mouse.Click(4,0,left)
        Paint('5.Menu open again (B title is <B is selected>, B is disabled, B shortcut is Ctrl+B)')
        CheckHash(0x66D7A77E4F1A6760)
    ";
    App::debug(60, 15, script).desktop(MyDesktop::new()).app_bar().build().unwrap().run();
}

#[test]
fn check_menu_set_status_checkbox_and_singlechoice() {
    #[Desktop(events =  MenuEvents+DesktopEvents+AppBarEvents,  commands: [A,B,C], internal = true)]
    struct MyDesktop {
        m_desktop: Handle<MenuButton>,
        m_cb: Handle<menu::CheckBox>,
        m_sc: Handle<menu::SingleChoice>,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                m_desktop: Handle::None,
                m_cb: Handle::None,
                m_sc: Handle::None,
            }
        }
    }

    impl DesktopEvents for MyDesktop {
        fn on_start(&mut self) {
            let mut m = Menu::new();
            self.m_cb = m.add(menu::CheckBox::new("Check Item", Key::None, mydesktop::Commands::A, false));
            m.add(menu::Separator::new());
            self.m_sc = m.add(menu::SingleChoice::new("Choice One", Key::None, mydesktop::Commands::B, false));
            m.add(menu::SingleChoice::new("Choice Two", Key::None, mydesktop::Commands::C, false));
            self.m_desktop = self.appbar().add(MenuButton::new("Desktop", m, 0, Side::Left));
        }
    }
    impl MenuEvents for MyDesktop {
        fn on_menu_open(&self, menu: &mut Menu) {
            let h1 = self.m_cb;
            let h2 = self.m_sc;
            if let Some(i) = menu.get_mut(h1) {
                i.set_checked(true);
            }
            if let Some(i) = menu.get_mut(h2) {
                i.set_selected();
            }
        }
    }
    impl AppBarEvents for MyDesktop {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.m_desktop);
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1.Initial state')
        CheckHash(0x769320EBFCBC2E03)
        Mouse.Click(4,0,left)
        Paint('2.Menu open (A is checked, B is selected)')
        CheckHash(0x9078AC513ABAC7F5)
        Mouse.Move(4,3)
    ";
    App::debug(60, 15, script).desktop(MyDesktop::new()).app_bar().build().unwrap().run();
}

#[test]
fn check_menu_command_methods() {
    #[Desktop(events =  AppBarEvents+MenuEvents+DesktopEvents,  commands: [A,B,C], internal = true)]
    struct MyDesktop {
        m_desktop: Handle<MenuButton>,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                m_desktop: Handle::None,
            }
        }
    }

    impl DesktopEvents for MyDesktop {
        fn on_start(&mut self) {
            let m = menu!(
                "class:MyDesktop,items=[
                    {&A,cmd:A},
                    {&B,cmd:B},
                    {&C,cmd:C}
                ]"
            );
            self.m_desktop = self.appbar().add(MenuButton::new("Desktop",m, 0, Side::Left));
        }
    }
    impl MenuEvents for MyDesktop {
        fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, _: mydesktop::Commands) {
            if let Some(i) = self.menuitem_mut(menu, item) {
                assert!(i.is_enabled());
                assert_eq!(i.caption(), "B");
                i.set_caption("B is pressed");
                i.set_enabled(false);
                i.set_shortcut(key!("Ctrl+B"));
                assert_eq!(i.shortcut(), Key::new(KeyCode::B, KeyModifier::Ctrl));
            }
        }
    }
    impl AppBarEvents for MyDesktop {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.m_desktop);
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1.Initial state')
        CheckHash(0x769320EBFCBC2E03)
        Mouse.Click(4,0,left)
        Paint('2.Menu open')
        CheckHash(0x40BC654D81173EBF)
        Mouse.Move(4,3)
        Paint('3.Move over B')
        CheckHash(0xF9B77C225B39E477)
        Mouse.Click(4,3,left)
        Paint('4.Click over B')
        CheckHash(0x769320EBFCBC2E03)
        Mouse.Click(4,0,left)
        Paint('5.Menu open again (B title is <B is pressed>, B is disabled, B shortcut is Ctrl+B)')
        CheckHash(0x3906086C97012BD3)
    ";
    App::debug(60, 15, script).desktop(MyDesktop::new()).app_bar().build().unwrap().run();
}
