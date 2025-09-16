use crate::{
    prelude::*,
    ui::appbar::{MenuButton, Side},
};

#[test]
fn check_order_parameter() {
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
            w.m_file = w.appbar_mut().add(MenuButton::new("&File", Menu::new(), 2, Side::Left));
            w.m_edit = w.appbar_mut().add(MenuButton::new("&Edit", Menu::new(), 1, Side::Left));
            w.m_help = w.appbar_mut().add(MenuButton::new("&Help", Menu::new(), 0, Side::Left));
            w
        }
    }
    impl AppBarEvents for MyWin {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.m_file);
            appbar.show(self.m_help);
            appbar.show(self.m_edit);
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('initial order - Help should be first, then Edit, then File')
        CheckHash(0xF06F449DDF16C472)
    ";
    let mut a = App::debug(60, 15, script).app_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_order_parameter_multi_controls() {
    // Window with menu
    pub mod mywindow {
        use crate::prelude::*;
        use crate::ui::appbar::*;

        // Custom control with menu
        pub mod mycustomcontrol {
            use crate::prelude::*;
            use crate::ui::appbar::*;

            #[CustomControl(events = AppBarEvents+MenuEvents, overwrite = OnPaint, commands = Red+Green+Blue, internal: true)]
            pub struct MyCustomControl {
                col: Color,
                h_menu: Handle<MenuButton>,
            }
            impl MyCustomControl {
                pub fn new(layout: Layout) -> Self {
                    let mut obj = Self {
                        base: ControlBase::new(layout, true),
                        col: Color::Red,
                        h_menu: Handle::None,
                    };
                    let m = menu!(
                        "class:MyCustomControl,items=[
                        {Red,selected:true,cmd:Red},
                        {Green,selected:false,cmd:Green},
                        {Blue,selected:false,cmd:Blue}
                    ]"
                    );
                    obj.h_menu = obj.appbar_mut().add(MenuButton::new("ControlMenu", m, 2, appbar::Side::Left));
                    obj
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
                        mycustomcontrol::Commands::Blue => self.col = Color::Blue,
                    }
                }
            }
            impl AppBarEvents for MyCustomControl {
                fn on_update(&self, appbar: &mut AppBar) {
                    // Custom control menu with order 2 (should appear third)
                    appbar.show(self.h_menu);
                }
            }
        }

        #[Window(events = AppBarEvents, commands = Copy+Paste+Cut, internal: true)]
        pub struct MyWindow {
            h_menu: Handle<MenuButton>,
            hc: Handle<mycustomcontrol::MyCustomControl>,
        }
        impl MyWindow {
            pub fn new() -> Self {
                let mut w = MyWindow {
                    base: window!("Test,a:c,w:30,h:10"),
                    h_menu: Handle::None,
                    hc: Handle::None,
                };
                let m = menu!(
                    "class:MyWindow,items=[
                        {Copy,cmd:Copy},
                        {Paste,cmd:Paste},
                        {Cut,cmd:Cut}
                    ]"
                );
                w.h_menu = w.appbar_mut().add(MenuButton::new("WindowMenu", m, 1, Side::Left));
                w.hc = w.add(mycustomcontrol::MyCustomControl::new(layout!("x:1,y:1,w:10,h:5")));
                w
            }
        }
        impl AppBarEvents for MyWindow {
            fn on_update(&self, appbar: &mut AppBar) {
                // Window menu with order 1 (should appear second)
                appbar.show(self.h_menu);
            }
        }
    }

    // Desktop with menu
    #[Desktop(events = AppBarEvents+DesktopEvents, commands = Settings+About, internal: true)]
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
                    {Settings,cmd:Settings},
                    {About,cmd:About}
                ]"
            );
            self.m_desktop = self.appbar_mut().add(MenuButton::new("DesktopMenu", m, 0, Side::Left));
        }
    }
    impl AppBarEvents for MyDesktop {
        fn on_update(&self, appbar: &mut AppBar) {
            // Desktop menu with order 0 (should appear first)
            appbar.show(self.m_desktop);
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('initial state - Desktop(0), Window(1), Control(2)')
        CheckHash(0x38778794A1D87753)
    ";
    let mut a = App::debug(60, 15, script).desktop(MyDesktop::new()).app_bar().build().unwrap();
    a.add_window(mywindow::MyWindow::new());
    a.run();
}

#[test]
fn check_order_parameter_multi_controls_reversed() {
    // Window with menu
    pub mod mywindow {
        use crate::prelude::*;
        use crate::ui::appbar::*;

        // Custom control with menu
        pub mod mycustomcontrol {
            use crate::prelude::*;
            use crate::ui::appbar::*;

            #[CustomControl(events = AppBar+MenuEvents, overwrite = OnPaint, commands = Red+Green+Blue, internal: true)]
            pub struct MyCustomControl {
                col: Color,
                h_menu: Handle<MenuButton>,
            }
            impl MyCustomControl {
                pub fn new(layout: Layout) -> Self {
                    let mut obj = Self {
                        base: ControlBase::new(layout, true),
                        col: Color::Red,
                        h_menu: Handle::None,
                    };
                    let m = menu!(
                        "class:MyCustomControl,items=[
                        {Red,selected:true,cmd:Red},
                        {Green,selected:false,cmd:Green},
                        {Blue,selected:false,cmd:Blue}
                    ]"
                    );
                    obj.h_menu = obj.appbar_mut().add(MenuButton::new("ControlMenu", m, 0, Side::Left));
                    obj
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
                        mycustomcontrol::Commands::Blue => self.col = Color::Blue,
                    }
                }                    
            }
            impl AppBarEvents for MyCustomControl {
                fn on_update(&self, appbar: &mut AppBar) {
                     // Custom control menu with order 2 (should appear first)
                    appbar.show(self.h_menu);                
                }
            }
        }

        #[Window(events = AppBarEvents, commands = Copy+Paste+Cut, internal: true)]
        pub struct MyWindow {
            h_menu: Handle<MenuButton>,
            hc: Handle<mycustomcontrol::MyCustomControl>,
        }
        impl MyWindow {
            pub fn new() -> Self {
                let mut w = MyWindow {
                    base: window!("Test,a:c,w:30,h:10"),
                    h_menu: Handle::None,
                    hc: Handle::None,
                };
                let m = menu!(
                    "class:MyWindow,items=[
                        {Copy,cmd:Copy},
                        {Paste,cmd:Paste},
                        {Cut,cmd:Cut}
                    ]"
                );
                w.h_menu = w.appbar_mut().add(MenuButton::new("WindowMenu", m, 1, Side::Left));
                w.hc = w.add(mycustomcontrol::MyCustomControl::new(layout!("x:1,y:1,w:10,h:5")));
                w
            }
        }
        impl AppBarEvents for MyWindow {
            fn on_update(&self, appbar: &mut AppBar) {
                // Window menu with order 1 (should appear second)
                appbar.show(self.h_menu);
            }
        }
    }

    // Desktop with menu
    #[Desktop(events = AppBarEvents+DesktopEvents, commands = Settings+About, internal: true)]
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
                    {Settings,cmd:Settings},
                    {About,cmd:About}
                ]"
            );
            self.m_desktop = self.appbar_mut().add(MenuButton::new("DesktopMenu", m, 2, Side::Left));
        }
    }
    impl AppBarEvents for MyDesktop {
        fn on_update(&self, appbar: &mut AppBar) {
            // Desktop menu with order 2 (should appear last)
            appbar.show(self.m_desktop);
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('initial state - Control(2), Window(1), Desktop(0)')
        CheckHash(0x47F641E3D761FF9B)
    ";
    let mut a = App::debug(60, 15, script).desktop(MyDesktop::new()).app_bar().build().unwrap();
    a.add_window(mywindow::MyWindow::new());
    a.run();
}

#[test]
fn check_update_multiple_menus() {
    mod mywin {
        use crate::prelude::*;
        use crate::ui::appbar::*;
        #[Window(events = AppBarEvents, commands=New+Save+Open, internal: true)]
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
                w.h_menu = w.appbar_mut().add(MenuButton::new("File", m, 0, Side::Left));

                w
            }
        }
        impl AppBarEvents for MyWindow {
            fn on_update(&self, appbar: &mut AppBar) {
                appbar.show(self.h_menu);
            }
        }
    }
    mod colorcustomcontrol {
        use crate::prelude::*;
        use crate::ui::appbar::*;

        #[CustomControl(events = AppBarEvents+MenuEvents, overwrite = OnPaint, commands = Red+Green+Blue, internal: true)]
        pub struct ColorCustomControl {
            col: Color,
            h_menu: Handle<MenuButton>,
        }
        impl ColorCustomControl {
            pub fn new(layout: Layout) -> Self {
                let mut obj = Self {
                    base: ControlBase::new(layout, true),
                    col: Color::Red,
                    h_menu: Handle::None,
                };
                let m = menu!(
                    "class:ColorCustomControl,items=[
                        {Red,F1,selected:true,cmd:Red},
                        {Green,F2,selected:false,cmd:Green},
                        {Blue,F3,selected:false,cmd:Blue}
                    ]"
                );
                obj.h_menu = obj.appbar_mut().add(MenuButton::new("ColorControl", m, 0, Side::Left));
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
        }
        impl AppBarEvents for ColorCustomControl {
            fn on_update(&self, appbar: &mut AppBar) {
                appbar.show(self.h_menu);
            }
        }

    }
    mod textcustomcontrol {
        use crate::prelude::*;
        use crate::ui::appbar::*;

        #[CustomControl(events = AppBarEvents+MenuEvents, overwrite = OnPaint, commands = Red+Green+Blue, internal: true)]
        pub struct TextCustomControl {
            text: &'static str,
            h_menu: Handle<MenuButton>,
        }
        impl TextCustomControl {
            pub fn new(layout: Layout) -> Self {
                let mut obj = Self {
                    base: ControlBase::new(layout, true),
                    text: "Red",
                    h_menu: Handle::None,
                };
                let m = menu!(
                    "class:TextCustomControl,items=[
                        {'Text->Red',F1,selected:true,cmd:Red},
                        {'Text->Green',F2,selected:false,cmd:Green},
                        {'Text->Blue',F3,selected:false,cmd:Blue}
                    ]"
                );
                obj.h_menu = obj.appbar_mut().add(MenuButton::new("Text", m, 0, Side::Left));
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
        }
        impl AppBarEvents for TextCustomControl {
            fn on_update(&self, appbar: &mut AppBar) {
                appbar.show(self.h_menu);
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
    let mut a = App::debug(60, 24, script).app_bar().build().unwrap();
    let mut w1 = mywin::MyWindow::new("Win-1", layout!("x:1,y:2,w:18,h:10"));
    w1.add(colorcustomcontrol::ColorCustomControl::new(layout!("x:1,y:1,w:10")));
    w1.add(button!("Button,x:1,y:3,w:10"));
    let mut w2 = mywin::MyWindow::new("Win-2", layout!("x:20,y:2,w:18,h:15"));
    w2.add(textcustomcontrol::TextCustomControl::new(layout!("x:1,y:1,w:10")));
    w2.add(button!("Button,x:1,y:3,w:10"));
    a.add_window(w1);
    a.add_window(w2);
    a.run();
}

#[test]
fn check_with_keys() {
    #[Window(events : AppBarEvents+MenuEvents, commands  : A, internal: true)]
    struct MyWindow {
        h_file: Handle<MenuButton>,
        h_edit: Handle<MenuButton>,
        h_help: Handle<MenuButton>,
        lb: Handle<Label>,
    }
    impl MyWindow {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,a:c,w:40,h:8"),
                h_file: Handle::None,
                h_edit: Handle::None,
                h_help: Handle::None,
                lb: Handle::None,
            };
            w.lb = w.add(label!("None,a:c,w:30,h:1"));
            // construct a popup menu
            w.h_file = w.appbar_mut().add(MenuButton::new(
                "&File",
                menu!(
                    "class: MyWindow, items=[
                    {New,cmd:A},
                    {&Save,cmd:A},
                    {'&Save As ...',cmd:A},
                    {&Open,cmd:A},
                    {-},
                    {E&xit,Alt+F4,cmd:A}
                ]"
                ),
                0,
                Side::Left,
            ));
            w.h_edit = w.appbar_mut().add(MenuButton::new(
                "&Edit",
                menu!(
                    "class: MyWindow, items=[
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
                ),
                0,
                Side::Left,
            ));
            w.h_help = w.appbar_mut().add(MenuButton::new(
                "&Help",
                menu!(
                    "class: MyWindow, items=[
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
                ),
                0,
                Side::Left,
            ));
            w
        }
    }
    impl MenuEvents for MyWindow {
        fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, _: mywindow::Commands) {
            if let Some(i) = self.menuitem(menu, item) {
                let s = String::from(i.caption());
                let h = self.lb;
                if let Some(l) = self.control_mut(h) {
                    l.set_caption(&s);
                }
            }
        }
    }
    impl AppBarEvents for MyWindow {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h_file);
            appbar.show(self.h_edit);
            appbar.show(self.h_help);
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
            Paint('State_58 (no menu visible in the menu bar)')
            CheckHash(0xD0C81ECF174389FD)
        ";
    let mut a = App::debug(60, 20, script).app_bar().build().unwrap();
    a.add_window(MyWindow::new());
    a.run();
}

#[test]
fn check_recursive_shortcuts() {
    #[Window(events : MenuEvents+AppBarEvents, commands : A, internal: true)]
    struct MyWindow {
        h_file: Handle<MenuButton>,
        h_edit: Handle<MenuButton>,
        h_help: Handle<MenuButton>,
        lb: Handle<Label>,
    }
    impl MyWindow {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,a:c,w:40,h:8"),
                h_file: Handle::None,
                h_edit: Handle::None,
                h_help: Handle::None,
                lb: Handle::None,
            };
            w.lb = w.add(label!("None,a:c,w:30,h:1"));
            // construct a popup menu
            w.h_file = w.appbar_mut().add(MenuButton::new(
                "&File",
                menu!(
                    "class: MyWindow, items=[
                    {New,F1,cmd:A},
                    {&Save,F2,cmd:A},
                    {'&Save As ...',Alt+F2,cmd:A},
                    {&Open,F3,cmd:A},
                    {-},
                    {E&xit,Alt+F4,cmd:A}
                ]"
                ),
                0,
                Side::Left,
            ));
            w.h_edit = w.appbar_mut().add(MenuButton::new(
                "&Edit",
                menu!(
                    "class: MyWindow, items=[
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
                ),
                0,
                Side::Left,
            ));
            w.h_help = w.appbar_mut().add(MenuButton::new(
                "&Help",
                menu!(
                    "class: MyWindow, items=[
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
                ),
                0,
                Side::Left,
            ));
            w
        }
    }
    impl MenuEvents for MyWindow {
        fn on_command(&mut self, menu: Handle<Menu>, item: Handle<menu::Command>, _: mywindow::Commands) {
            if let Some(i) = self.menuitem(menu, item) {
                let s = String::from(i.caption());
                let h = self.lb;
                if let Some(l) = self.control_mut(h) {
                    l.set_caption(&s);
                }
            }
        }
    }
    impl AppBarEvents for MyWindow {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h_file);
            appbar.show(self.h_edit);
            appbar.show(self.h_help);
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
            Paint('State_25 (no menu visible in the menubar)')
            CheckHash(0xD0C81ECF174389FD)    
        ";
    let mut a = App::debug(60, 20, script).app_bar().build().unwrap();
    a.add_window(MyWindow::new());
    a.run();
}

#[test]
fn check_side_parameter() {
    #[Window(events: AppBarEvents, commands: A, internal: true)]
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
            w.m_file = w.appbar_mut().add(MenuButton::new("&File", Menu::new(), 0, Side::Left));
            w.m_edit = w.appbar_mut().add(MenuButton::new("&Edit", Menu::new(), 0, Side::Left));
            w.m_help = w.appbar_mut().add(MenuButton::new("&Help", Menu::new(), 0, Side::Right));
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
        Paint('1. initial order - File,Edit on left, Help or Right')
        CheckHash(0x57E37E1F8A5C816A)
        Mouse.Move(59,0)
        Paint('2. Hover over Help')
        CheckHash(0xB305A201840FDCFA)
        Mouse.Move(0,0)
        Paint('3. Hover over File')
        CheckHash(0x1F36887325D95F9A)
        Mouse.Move(6,0)
        Paint('4. Hover over Edit')
        CheckHash(0x21EDFC4F0D6D293A)
    ";
    let mut a = App::debug(60, 15, script).app_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}


#[test]
fn check_enable_disable() {
    let script = "
        //Paint.Enable(false)
        Error.Disable(true)
        Paint('1. initial state')
        CheckHash(0xB1A6546E0A2D22B6)
        Key.Pressed(F1)
        Paint('2. Disabled')
        CheckHash(0xE5CF1A3F8CBE868E)
        Mouse.Move(2,0);
        Paint('3. nothig happens on hovernig')
        CheckHash(0xE5CF1A3F8CBE868E)
        Mouse.Click(2,0,left);
        Paint('4. nothig happens on click')
        CheckHash(0x1EC6A227F348D572)
        Key.Pressed(F1)
        Paint('5. Enabled')
        CheckHash(0x4CDD9A1678E2E012)
        Mouse.Move(3,0);
        Paint('6. now is hovered')
        CheckHash(0x4CDD9A1678E2E012)
    ";

    #[Window(events = CommandBarEvents+AppBarEvents, internal=true,commands=[DoSomething])]
    struct MyWin {
        h: Handle<appbar::MenuButton>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", layout!("x:1,y:1,w:20,h:7"), window::Flags::None),
                h: Handle::None,
            };
            me.h = me.appbar_mut().add(MenuButton::new("My Menu",Menu::new(),0,Side::Left));
            me
        }
    }
    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Enable/Disable", mywin::Commands::DoSomething);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            if command_id == mywin::Commands::DoSomething {
                let h = self.h;
                if let Some(m) = self.appbar_mut().get_mut(h) {
                    m.set_enabled(!m.is_enabled());
                }
            }            
        }
    }
    impl AppBarEvents for MyWin {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h);
        }
    }

    let mut a = App::debug(60, 10, script).command_bar().app_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}