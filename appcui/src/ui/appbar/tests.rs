use crate::{
    prelude::*,
    ui::appbar::{MenuButton, Separator, Side},
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
            w.m_file = w.appbar().add(MenuButton::new("&File", Menu::new(), 2, Side::Left));
            w.m_edit = w.appbar().add(MenuButton::new("&Edit", Menu::new(), 1, Side::Left));
            w.m_help = w.appbar().add(MenuButton::new("&Help", Menu::new(), 0, Side::Left));
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
                    obj.h_menu = obj.appbar().add(MenuButton::new("ControlMenu", m, 2, appbar::Side::Left));
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
                w.h_menu = w.appbar().add(MenuButton::new("WindowMenu", m, 1, Side::Left));
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
            self.m_desktop = self.appbar().add(MenuButton::new("DesktopMenu", m, 0, Side::Left));
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
                    obj.h_menu = obj.appbar().add(MenuButton::new("ControlMenu", m, 0, Side::Left));
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
                w.h_menu = w.appbar().add(MenuButton::new("WindowMenu", m, 1, Side::Left));
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
            self.m_desktop = self.appbar().add(MenuButton::new("DesktopMenu", m, 2, Side::Left));
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
                w.h_menu = w.appbar().add(MenuButton::new("File", m, 0, Side::Left));

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
                obj.h_menu = obj.appbar().add(MenuButton::new("ColorControl", m, 0, Side::Left));
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
                obj.h_menu = obj.appbar().add(MenuButton::new("Text", m, 0, Side::Left));
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
            w.h_file = w.appbar().add(MenuButton::new(
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
            w.h_edit = w.appbar().add(MenuButton::new(
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
            w.h_help = w.appbar().add(MenuButton::new(
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
            Paint('State_58 (no menu visible in the app bar)')
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
            w.h_file = w.appbar().add(MenuButton::new(
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
            w.h_edit = w.appbar().add(MenuButton::new(
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
            w.h_help = w.appbar().add(MenuButton::new(
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
            Paint('State_25 (no menu visible in the appbar)')
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
            w.m_file = w.appbar().add(MenuButton::new("&File", Menu::new(), 0, Side::Left));
            w.m_edit = w.appbar().add(MenuButton::new("&Edit", Menu::new(), 0, Side::Left));
            w.m_help = w.appbar().add(MenuButton::new("&Help", Menu::new(), 0, Side::Right));
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
        Paint.Enable(false)
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
        CheckHash(0xE5CF1A3F8CBE868E)
        Key.Pressed(F1)
        Paint('5. Enabled (and automatically hovered)')
        CheckHash(0x31DB14049BEB45AE)
        Mouse.Move(20,0);
        Paint('6. not hovered')
        CheckHash(0xB1A6546E0A2D22B6)
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
            me.h = me.appbar().add(MenuButton::new("My Menu", Menu::new(), 0, Side::Left));
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
                if let Some(m) = self.appbar().get_mut(h) {
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

#[test]
fn check_mouse_over_disable() {
    let script = "
        Paint.Enable(false)
        Paint('1. initial state order:(File,Option,Editor ... Help,About)')
        CheckHash(0xA38A52B34C90BD7C)
        Mouse.Move(3,0)
        Paint('2. Hover over File')
        CheckHash(0xFABDCA63D641602C)
        Mouse.Move(9,0)
        Paint('3. Hover over Option (not selected as inactiv)')
        CheckHash(0xA38A52B34C90BD7C)
        Mouse.Move(17,0)
        Paint('4. Hover over Editor')
        CheckHash(0x64831F72959A7B3C)
        Mouse.Move(50,0)
        Paint('5. Hover over Help')
        CheckHash(0xD7094A72EECA762C)
        Mouse.Move(56,0)
        Paint('6. Hover over About (not selected as inactiv)')
        CheckHash(0xA38A52B34C90BD7C)        
    ";

    #[Window(events = AppBarEvents, internal=true)]
    struct MyWin {
        h_file: Handle<appbar::MenuButton>,
        h_opt: Handle<appbar::MenuButton>,
        h_edit: Handle<appbar::MenuButton>,
        h_help: Handle<appbar::MenuButton>,
        h_about: Handle<appbar::MenuButton>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", layout!("x:1,y:1,w:20,h:7"), window::Flags::None),
                h_file: Handle::None,
                h_opt: Handle::None,
                h_edit: Handle::None,
                h_help: Handle::None,
                h_about: Handle::None,
            };
            me.h_file = me.appbar().add(MenuButton::new("File", Menu::new(), 0, Side::Left));
            let mut m = MenuButton::new("Option", Menu::new(), 0, Side::Left);
            m.set_enabled(false);
            me.h_opt = me.appbar().add(m);
            me.h_edit = me.appbar().add(MenuButton::new("Editor", Menu::new(), 0, Side::Left));
            me.h_help = me.appbar().add(MenuButton::new("Help", Menu::new(), 1, Side::Right));
            let mut m = MenuButton::new("About", Menu::new(), 0, Side::Right);
            m.set_enabled(false);
            me.h_about = me.appbar().add(m);

            me
        }
    }
    impl AppBarEvents for MyWin {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h_file);
            appbar.show(self.h_opt);
            appbar.show(self.h_edit);
            appbar.show(self.h_help);
            appbar.show(self.h_about);
        }
    }

    let mut a = App::debug(60, 10, script).command_bar().app_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_mouse_click_and_hover_disable() {
    let script = "
        Paint.Enable(false)
        Paint('1. initial state order:(File,Option,Editor ... Help,About)')
        CheckHash(0xA38A52B34C90BD7C)
        Mouse.Click(3,0,left)
        Mouse.Move(3,0)
        Paint('2. Hover over File (File is opened)')
        CheckHash(0xF92F32321FFA49D5)
        Mouse.Move(9,0)
        Paint('3. Hover over Option (not selected but File remains open)')
        CheckHash(0xF92F32321FFA49D5)
        Mouse.Move(17,0)
        Paint('4. Hover over Editor (Editor is opened)')
        CheckHash(0xA83E700EAA75E308)
        Mouse.Move(50,0)
        Paint('5. Hover over Help (Help is opened)')
        CheckHash(0xB8A25E5D4D92412C)
        Mouse.Move(56,0)
        Paint('6. Hover over About (not selected as inactiv but Help remains open)')
        CheckHash(0xB8A25E5D4D92412C)        
    ";

    #[Window(events = AppBarEvents, internal=true)]
    struct MyWin {
        h_file: Handle<appbar::MenuButton>,
        h_opt: Handle<appbar::MenuButton>,
        h_edit: Handle<appbar::MenuButton>,
        h_help: Handle<appbar::MenuButton>,
        h_about: Handle<appbar::MenuButton>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", layout!("x:1,y:1,w:20,h:7"), window::Flags::None),
                h_file: Handle::None,
                h_opt: Handle::None,
                h_edit: Handle::None,
                h_help: Handle::None,
                h_about: Handle::None,
            };
            me.h_file = me.appbar().add(MenuButton::new("File", Menu::new(), 0, Side::Left));
            let mut m = MenuButton::new("Option", Menu::new(), 0, Side::Left);
            m.set_enabled(false);
            me.h_opt = me.appbar().add(m);
            me.h_edit = me.appbar().add(MenuButton::new("Editor", Menu::new(), 0, Side::Left));
            me.h_help = me.appbar().add(MenuButton::new("Help", Menu::new(), 1, Side::Right));
            let mut m = MenuButton::new("About", Menu::new(), 0, Side::Right);
            m.set_enabled(false);
            me.h_about = me.appbar().add(m);

            me
        }
    }
    impl AppBarEvents for MyWin {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h_file);
            appbar.show(self.h_opt);
            appbar.show(self.h_edit);
            appbar.show(self.h_help);
            appbar.show(self.h_about);
        }
    }

    let mut a = App::debug(60, 10, script).command_bar().app_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_separator() {
    let script = "
        Paint.Enable(false)
        Paint('1. initial state order:(File | Option)')
        CheckHash(0x6B9252EAA3B8099)    
        Mouse.Move(3,0)
        Paint('2. Hover over File')
        CheckHash(0x6CEB644FC4509049)    
        Mouse.Move(6,0)
        Paint('3. Hover over separator (nothing is selected)')
        CheckHash(0x6B9252EAA3B8099)    
        Mouse.Click(6,0,left)
        Paint('4. Click on separator (nothing happenes)')
        CheckHash(0x6B9252EAA3B8099)    
        Mouse.Move(10,0)
        Paint('5. Hover over Option')
        CheckHash(0x68337B1696A38369)    
    ";

    #[Window(events = AppBarEvents, internal=true)]
    struct MyWin {
        h_file: Handle<appbar::MenuButton>,
        h_opt: Handle<appbar::MenuButton>,
        h_sep: Handle<appbar::Separator>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", layout!("x:1,y:1,w:20,h:7"), window::Flags::None),
                h_file: Handle::None,
                h_opt: Handle::None,
                h_sep: Handle::None,
            };
            me.h_file = me.appbar().add(MenuButton::new("File", Menu::new(), 0, Side::Left));
            me.h_opt = me.appbar().add(MenuButton::new("Option", Menu::new(), 0, Side::Left));
            me.h_sep = me.appbar().add(Separator::new(0, Side::Left));

            me
        }
    }
    impl AppBarEvents for MyWin {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h_file);
            appbar.show(self.h_sep);
            appbar.show(self.h_opt);
        }
    }

    let mut a = App::debug(60, 10, script).command_bar().app_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_label() {
    let script = "
        Paint.Enable(false)
        Paint('1. initial state order:(File ++123++ Option )')
        CheckHash(0xD552B3F72E3CC4D1)    
        Mouse.Move(3,0)
        Paint('2. Hover over File')
        CheckHash(0xF46D6B222B2BD0C1)    
        Mouse.Move(6,0)
        Paint('3. Hover over label (nothing is selected)')
        CheckHash(0xD552B3F72E3CC4D1)    
        Mouse.Click(6,0,left)
        Paint('4. Click on label (nothing happenes)')
        CheckHash(0xD552B3F72E3CC4D1)    
        Mouse.Move(17,0)
        Paint('5. Hover over Option')
        CheckHash(0xBF17FD1669336B81)    
    ";

    #[Window(events = AppBarEvents, internal=true)]
    struct MyWin {
        h_file: Handle<appbar::MenuButton>,
        h_opt: Handle<appbar::MenuButton>,
        h_l: Handle<appbar::Label>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", layout!("x:1,y:1,w:20,h:7"), window::Flags::None),
                h_file: Handle::None,
                h_opt: Handle::None,
                h_l: Handle::None,
            };
            me.h_file = me.appbar().add(MenuButton::new("File", Menu::new(), 0, Side::Left));
            me.h_opt = me.appbar().add(MenuButton::new("Option", Menu::new(), 0, Side::Left));
            me.h_l = me.appbar().add(appbar::Label::new("++123++", 0, Side::Left));

            me
        }
    }
    impl AppBarEvents for MyWin {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h_file);
            appbar.show(self.h_l);
            appbar.show(self.h_opt);
        }
    }

    let mut a = App::debug(60, 10, script).command_bar().app_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_label_tooltip() {
    let script = "
        Paint.Enable(false)
        Paint('1. initial state order:(File ++123++ Option )')
        CheckHash(0xD552B3F72E3CC4D1)    
        Mouse.Move(3,0)
        Paint('2. Hover over File')
        CheckHash(0xF46D6B222B2BD0C1)    
        Mouse.Move(6,0)
        Paint('3. Hover over label (nothing is selected, tooltip is visible)')
        CheckHash(0x718D6D41A0A0ECB4)    
        Mouse.Click(6,0,left)
        Paint('4. Click on label (nothing happenes, no tooltip)')
        CheckHash(0xD552B3F72E3CC4D1)    
        Mouse.Move(17,0)
        Paint('5. Hover over Option')
        CheckHash(0xBF17FD1669336B81)    
    ";

    #[Window(events = AppBarEvents, internal=true)]
    struct MyWin {
        h_file: Handle<appbar::MenuButton>,
        h_opt: Handle<appbar::MenuButton>,
        h_l: Handle<appbar::Label>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", layout!("x:1,y:1,w:20,h:7"), window::Flags::None),
                h_file: Handle::None,
                h_opt: Handle::None,
                h_l: Handle::None,
            };
            me.h_file = me.appbar().add(MenuButton::new("File", Menu::new(), 0, Side::Left));
            me.h_opt = me.appbar().add(MenuButton::new("Option", Menu::new(), 0, Side::Left));
            let mut lb = appbar::Label::new("++123++", 0, Side::Left);
            lb.set_tooltip("This is a label");
            me.h_l = me.appbar().add(lb);

            me
        }
    }
    impl AppBarEvents for MyWin {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h_file);
            appbar.show(self.h_l);
            appbar.show(self.h_opt);
        }
    }

    let mut a = App::debug(60, 10, script).command_bar().app_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_move_left_right() {
    let script = "
        Paint.Enable(false)
        Paint('1. initial state Left:(M-1,M-2,L-1,|), Right:(M-5,B-1,M-4,M-3)')
        CheckHash(0x7E08D0E9FC227B2C)   
        Mouse.Click(2,0,left)    
        Paint('2. M-1 selected')
        CheckHash(0x624003D52D053481)  
        Key.Pressed(Right) 
        Paint('3. M-2 selected')
        CheckHash(0x42A24CF684C9687F)  
        Key.Pressed(Right) 
        Paint('4. M-5 selected')
        CheckHash(0x576E13E8CA158650)  
        Key.Pressed(Right) 
        Paint('5. M-3 selected')
        CheckHash(0x584FF83E53552C48)  
        Key.Pressed(Right) 
        Paint('6. M-1 selected')
        CheckHash(0x624003D52D053481)  
        Key.Pressed(Left) 
        Paint('7. M-3 selected')
        CheckHash(0x584FF83E53552C48)  
        Key.Pressed(Left) 
        Paint('8. M-5 selected')
        CheckHash(0x576E13E8CA158650)  
        Key.Pressed(Left) 
        Paint('9. M-2 selected')
        CheckHash(0x42A24CF684C9687F)  
        Key.Pressed(Left)  
        Paint('10. M-1 selected')
        CheckHash(0x624003D52D053481)                
    ";

    #[Window(events = AppBarEvents, internal=true)]
    struct MyWin {
        h_1: Handle<appbar::MenuButton>,
        h_2: Handle<appbar::MenuButton>,
        h_3: Handle<appbar::Label>,
        h_4: Handle<appbar::Separator>,
        h_5: Handle<appbar::MenuButton>,
        h_6: Handle<appbar::MenuButton>,
        h_7: Handle<appbar::Button>,
        h_8: Handle<appbar::MenuButton>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", layout!("x:1,y:1,w:20,h:7"), window::Flags::None),
                h_1: Handle::None,
                h_2: Handle::None,
                h_3: Handle::None,
                h_4: Handle::None,
                h_5: Handle::None,
                h_6: Handle::None,
                h_7: Handle::None,
                h_8: Handle::None,
            };
            me.h_1 = me.appbar().add(MenuButton::new("M-1", Menu::new(), 0, Side::Left));
            me.h_2 = me.appbar().add(MenuButton::new("M-2", Menu::new(), 0, Side::Left));
            me.h_3 = me.appbar().add(appbar::Label::new("L-1", 0, Side::Left));
            me.h_4 = me.appbar().add(appbar::Separator::new(0, Side::Left));
            me.h_5 = me.appbar().add(MenuButton::new("M-3", Menu::new(), 0, Side::Right));
            let mut mm = MenuButton::new("M-4", Menu::new(), 0, Side::Right);
            mm.set_enabled(false);
            me.h_6 = me.appbar().add(mm);
            me.h_7 = me.appbar().add(appbar::Button::new("<B1>", 0, Side::Right));
            me.h_8 = me.appbar().add(MenuButton::new("M-5", Menu::new(), 0, Side::Right));

            me
        }
    }
    impl AppBarEvents for MyWin {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h_1);
            appbar.show(self.h_2);
            appbar.show(self.h_3);
            appbar.show(self.h_4);
            appbar.show(self.h_5);
            appbar.show(self.h_6);
            appbar.show(self.h_7);
            appbar.show(self.h_8);
        }
    }

    let mut a = App::debug(80, 10, script).command_bar().app_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_button_with_menu_opened() {
    #[Window(events = AppBarEvents, commands: A, internal: true)]
    pub(crate) struct Win {
        cnt: u8,
        h_menu: Handle<appbar::MenuButton>,
        h_minus: Handle<appbar::Button>,
        h_plus: Handle<appbar::Button>,
        h_label: Handle<appbar::Label>,
        h_sep: Handle<appbar::Separator>,
    }
    impl Win {
        pub(crate) fn new() -> Self {
            let mut w = Win {
                base: window!("Test,a:c,w:40,h:8,Flags: Sizeable"),
                h_menu: Handle::None,
                h_label: Handle::None,
                h_plus: Handle::None,
                h_minus: Handle::None,
                h_sep: Handle::None,
                cnt: 2,
            };
            let m = menu!(
                "class: Win, items=[
                    { &New, cmd: A , key: Ctrl+N },
                    { &Save, cmd: A, key: Ctrl+S },
                    { 'Save &as...', cmd: A },
                    { &Open, cmd: A, key: Ctrl+O },
                    { --- },
                    { E&xit, cmd: A, key: Alt+F4 },
                ]"
            );
            let mb = appbar::MenuButton::new("&File", m, 1, appbar::Side::Left);
            w.h_menu = w.appbar().add(mb);
            w.h_sep = w.appbar().add(appbar::Separator::new(1, appbar::Side::Left));
            w.h_label = w.appbar().add(appbar::Label::new("2/5", 1, appbar::Side::Left));
            w.h_minus = w.appbar().add(appbar::Button::with_tooltip(" < ", "Previous", 1, appbar::Side::Left));
            w.h_plus = w.appbar().add(appbar::Button::with_tooltip(" > ", "Next", 1, appbar::Side::Left));

            w
        }
    }
    impl AppBarEvents for Win {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h_menu);
            appbar.show(self.h_sep);
            appbar.show(self.h_minus);
            appbar.show(self.h_label);
            appbar.show(self.h_plus);
        }

        fn on_button_click(&mut self, button: Handle<appbar::Button>) {
            if button == self.h_minus {
                self.cnt = (self.cnt - 1).clamp(1, 5);
            }
            if button == self.h_plus {
                self.cnt = (self.cnt + 1).clamp(1, 5);
            }
            let cnt = self.cnt;
            let h = self.h_minus;
            self.appbar().get_mut(h).unwrap().set_enabled(cnt > 1);
            let h = self.h_plus;
            self.appbar().get_mut(h).unwrap().set_enabled(cnt < 5);
            let v: [u8; 3] = [cnt + 48, b'/', b'5'];
            let h = self.h_label;
            self.appbar().get_mut(h).unwrap().set_caption(str::from_utf8(&v).unwrap());
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. initial state : File | < 2/5 >')
        CheckHash(0x6B79439B12DD53E3)  
        Mouse.Click(3,0,left)             
        Paint('2. File menu opened')
        CheckHash(0x9D8C28A442A6C70A)  
        Mouse.Move(8,0)             
        Paint('3. Hover over < , File remains opened')
        CheckHash(0x881DF7D4E9EC04C2)  
        Mouse.Click(8,0,left)             
        Paint('4. Click < , File is closed, Status: < 1/5 >, < becomes inactive, tooltip is shown')
        CheckHash(0xD4E21DFDF8961BA4)  
        Mouse.Click(8,0,left)             
        Paint('5. Click < , nothing happens - is inactive, tooltip is hidden')
        CheckHash(0x9DB1C581E6A2C950)  
        Mouse.Click(8,0,left)             
        Paint('6. Click < , nothing happens (same hash as 5)')
        CheckHash(0x9DB1C581E6A2C950)  
        Mouse.Move(14,0)             
        Paint('7. Hover over > , tooltip is shown')
        CheckHash(0x7E584097D7F321D6)  
        Mouse.Hold(14,0,left)             
        Paint('8. Press over > , tooltip is hidden, status is < 2/5 >')
        CheckHash(0x19ACF49C9D7F43DF)  
        Mouse.Release(14,0,left)             
        Paint('9. Release > , tooltip is hidden, status is < 2/5 > , > remains hovered')
        CheckHash(0x488DA51B6121B78B)  
        Mouse.Click(3,0,left)             
        Paint('10. File menu opened')
        CheckHash(0x9D8C28A442A6C70A)  
        Key.Pressed(Left)
        Paint('11. Nothing happens, File menu remains selected and opened')
        CheckHash(0x9D8C28A442A6C70A)  
        Key.Pressed(Right)
        Paint('12. Nothing happens, File menu remains selected and opened')
        CheckHash(0x9D8C28A442A6C70A)  

    ";

    let mut a = App::debug(80, 10, script).command_bar().app_bar().build().unwrap();
    a.add_window(Win::new());
    a.run();
}

#[test]
fn check_menu_aligned_right_opened() {
    #[Window(events = AppBarEvents, commands=A, internal = true)]
    pub(crate) struct Win {
        h_sep: Handle<appbar::Separator>,
        h_left: Handle<appbar::MenuButton>,
        h_color: Handle<appbar::MenuButton>,
        h_opt: Handle<appbar::MenuButton>,
    }
    impl Win {
        pub(crate) fn new() -> Self {
            let mut w = Win {
                base: window!("'Aligned menus',a:c,w:40,h:8,Flags: Sizeable"),
                h_sep: Handle::None,
                h_left: Handle::None,
                h_color: Handle::None,
                h_opt: Handle::None,
            };

            w.add(label!("'Three manus: Left, Color and Options, and a separator from the desktop specific menus to these one. Left menu is aligned to the left, while Color and Options are aligned to th right of the application bar.',d:f"));

            let m = menu!(
                "class: Win, items=[
                { Command-1, cmd: A , key: Ctrl+N },
                { Command-2, cmd: A, key: Ctrl+S },
                { Command-3, cmd: A },
                { --- },
                { E&xit, cmd: A, key: Alt+F4 },
            ]"
            );
            w.h_left = w.appbar().add(appbar::MenuButton::new("&Left", m, 1, appbar::Side::Left));
            let m = menu!(
                "class: Win, items=[
                { &Red, cmd: A , selected: true},
                { &Green, cmd: A, selected: false },
                { &Blue, cmd: A, selected: false },
            ]"
            );
            w.h_color = w.appbar().add(appbar::MenuButton::new("&Color", m, 2, appbar::Side::Right));
            let m = menu!(
                "class: Win, items=[
                { 'Option &1', cmd: A, checked: true },
                { 'Option &2', cmd: A, checked: false },
                { 'Option &3', cmd: A, checked: true },
                { --- },
                { 'Option &1', cmd: A, checked: false },
                { 'Option &1', cmd: A, checked: true },
            ]"
            );
            w.h_opt = w.appbar().add(appbar::MenuButton::new("&Options", m, 1, appbar::Side::Right));
            w.h_sep = w.appbar().add(appbar::Separator::new(1, appbar::Side::Left));
            w
        }
    }
    impl AppBarEvents for Win {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h_sep);
            appbar.show(self.h_left);
            appbar.show(self.h_color);
            appbar.show(self.h_opt);
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. initial state : Left ...... Color Option')
        CheckHash(0x696029ACD06801A3)  
        Mouse.Click(3,0,left)             
        Paint('2. Left menu opened (direction from left to right)')
        CheckHash(0x9B21B1A17B0C6B0B)  
        Key.Pressed(Right)             
        Paint('3. Color menu opened (direction from left to right)')
        CheckHash(0x1349F1A3A7A0C6E6)  
        Key.Pressed(Right)             
        Paint('4. Help menu opened (direction from right to left)')
        CheckHash(0xEA4995BF7547A2A0)  
    ";

    let mut a = App::debug(80, 10, script).command_bar().app_bar().build().unwrap();
    a.add_window(Win::new());
    a.run();
}

#[test]
fn check_menu_aligned_right_opened_large_secondary_right_most_menu() {
    #[Window(events = AppBarEvents, commands=A, internal = true)]
    pub(crate) struct Win {
        h_sep: Handle<appbar::Separator>,
        h_left: Handle<appbar::MenuButton>,
        h_color: Handle<appbar::MenuButton>,
        h_opt: Handle<appbar::MenuButton>,
    }
    impl Win {
        pub(crate) fn new() -> Self {
            let mut w = Win {
                base: window!("'Aligned menus',a:c,w:40,h:8,Flags: Sizeable"),
                h_sep: Handle::None,
                h_left: Handle::None,
                h_color: Handle::None,
                h_opt: Handle::None,
            };

            w.add(label!("'Three manus: Left, Color and Options, and a separator from the desktop specific menus to these one. Left menu is aligned to the left, while Color and Options are aligned to th right of the application bar.',d:f"));

            let m = menu!(
                "class: Win, items=[
                { Command-1, cmd: A , key: Ctrl+N },
                { Command-2, cmd: A, key: Ctrl+S },
                { Command-3, cmd: A },
                { --- },
                { E&xit, cmd: A, key: Alt+F4 },
            ]"
            );
            w.h_left = w.appbar().add(appbar::MenuButton::new("&Left", m, 1, appbar::Side::Left));
            let m = menu!(
                "class: Win, items=[
                { 'Red with nuances of gray', cmd: A , selected: true},
                { &Green, cmd: A, selected: false },
                { &Blue, cmd: A, selected: false },
            ]"
            );
            w.h_color = w.appbar().add(appbar::MenuButton::new("&Color", m, 2, appbar::Side::Right));
            let m = menu!(
                "class: Win, items=[
                { 'Option &1', cmd: A, checked: true },
                { 'Option &2', cmd: A, checked: false },
                { 'Option &3', cmd: A, checked: true },
                { --- },
                { 'Option &1', cmd: A, checked: false },
                { 'Option &1', cmd: A, checked: true },
            ]"
            );
            w.h_opt = w.appbar().add(appbar::MenuButton::new("&Options", m, 1, appbar::Side::Right));
            w.h_sep = w.appbar().add(appbar::Separator::new(1, appbar::Side::Left));
            w
        }
    }
    impl AppBarEvents for Win {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h_sep);
            appbar.show(self.h_left);
            appbar.show(self.h_color);
            appbar.show(self.h_opt);
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. initial state : Left ...... Color Option')
        CheckHash(0x696029ACD06801A3)  
        Mouse.Click(3,0,left)             
        Paint('2. Left menu opened (direction from left to right)')
        CheckHash(0x9B21B1A17B0C6B0B)  
        Key.Pressed(Right)             
        Paint('3. Color menu opened (direction from right to left)')
        CheckHash(0xD93C87922F5D3946)  
        Key.Pressed(Right)             
        Paint('4. Help menu opened (direction from right to left)')
        CheckHash(0xEA4995BF7547A2A0)  
    ";

    let mut a = App::debug(80, 10, script).command_bar().app_bar().build().unwrap();
    a.add_window(Win::new());
    a.run();
}

#[test]
fn check_desktop_resize() {
    // Desktop with menu
    #[Desktop(events = AppBarEvents+DesktopEvents, commands = Settings+About, internal: true)]
    struct MyDesktop {
        m_1: Handle<MenuButton>,
        m_2: Handle<MenuButton>,
        m_3: Handle<MenuButton>,
        m_4: Handle<MenuButton>,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                m_1: Handle::None,
                m_2: Handle::None,
                m_3: Handle::None,
                m_4: Handle::None,
            }
        }
    }
    impl DesktopEvents for MyDesktop {
        fn on_start(&mut self) {
            self.m_1 = self.appbar().add(MenuButton::new("Left-Menu-1", Menu::new(), 0, Side::Left));
            self.m_2 = self.appbar().add(MenuButton::new("Left-Menu-2", Menu::new(), 1, Side::Left));
            self.m_3 = self.appbar().add(MenuButton::new("Right-Menu-1", Menu::new(), 0, Side::Right));
            self.m_4 = self.appbar().add(MenuButton::new("Right-Menu-2", Menu::new(), 1, Side::Right));
        }
    }
    impl AppBarEvents for MyDesktop {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.m_1);
            appbar.show(self.m_2);
            appbar.show(self.m_3);
            appbar.show(self.m_4);
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state - LM-1,LM-2,RM-2,RM-1')
        CheckHash(0x23B78E48A70060F5)
        Resize(50,10)
        Paint('2. New width (50) - LM-1,LM-2,RM-1')
        CheckHash(0x32F5182C91D2DD0C)
        Resize(40,10)
        Paint('3. New width (40) - LM-1,RM-1')
        CheckHash(0x4490C2EE9C85A226)
        Resize(30,10)
        Paint('4. New width (30) - LM-1,LM-2,RM-1')
        CheckHash(0xE68F2CCD42F41D0C)
    ";
    App::debug(60, 10, script).desktop(MyDesktop::new()).app_bar().build().unwrap().run();
}

#[test]
fn check_menu_button_api() {
    // Desktop with menu
    #[Desktop(events = AppBarEvents+DesktopEvents, internal: true)]
    struct MyDesktop {
        h: Handle<MenuButton>,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                h: Handle::None,
            }
        }
    }
    impl DesktopEvents for MyDesktop {
        fn on_start(&mut self) {
            self.h = self.appbar().add(MenuButton::new("Menu", Menu::new(), 0, Side::Left));
        }
    }
    impl AppBarEvents for MyDesktop {
        fn on_update(&self, appbar: &mut AppBar) {
            let h = self.h;
            if let Some(mb) = appbar.get_mut(h) {
                let mut s = mb.caption().to_string();
                s.push_str("+1");
                mb.set_caption(&s);
            }
            appbar.show(self.h);
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state - menu name is <Menu+1>')
        CheckHash(0xADA1CFB8DAF362DC)
    ";
    App::debug(60, 10, script).desktop(MyDesktop::new()).app_bar().build().unwrap().run();
}

#[test]
fn check_label_api() {
    // Desktop with menu
    #[Desktop(events = AppBarEvents+DesktopEvents, internal: true)]
    struct MyDesktop {
        h: Handle<appbar::Label>,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                h: Handle::None,
            }
        }
    }
    impl DesktopEvents for MyDesktop {
        fn on_start(&mut self) {
            self.h = self.appbar().add(appbar::Label::new("Label", 0, Side::Left));
        }
    }
    impl AppBarEvents for MyDesktop {
        fn on_update(&self, appbar: &mut AppBar) {
            let h = self.h;
            if let Some(mb) = appbar.get_mut(h) {
                let mut s = mb.caption().to_string();
                s.push_str("+1");
                mb.set_caption(&s);
            }
            appbar.show(self.h);
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state - menu name is <Label+1>')
        CheckHash(0xF0C393E648D75BAD)
    ";
    App::debug(60, 10, script).desktop(MyDesktop::new()).app_bar().build().unwrap().run();
}

#[test]
fn check_button_api() {
    // Desktop with menu
    #[Desktop(events = AppBarEvents+DesktopEvents, internal: true)]
    struct MyDesktop {
        h: Handle<appbar::Button>,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                h: Handle::None,
            }
        }
    }
    impl DesktopEvents for MyDesktop {
        fn on_start(&mut self) {
            self.h = self.appbar().add(appbar::Button::new("Button", 0, Side::Left));
        }
    }
    impl AppBarEvents for MyDesktop {
        fn on_update(&self, appbar: &mut AppBar) {
            let h = self.h;
            if let Some(mb) = appbar.get_mut(h) {
                let mut s = mb.caption().to_string();
                s.push_str("+1");
                mb.set_caption(&s);
                s = format!("Enabled: {}", mb.is_enabled());
                mb.set_tooltip(&s);
                assert_eq!(mb.hotkey(), Key::None);
            }
            appbar.show(self.h);
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state - menu name is <Button+1>')
        CheckHash(0xBB16027424F5097D)
        Mouse.Move(3,0)
        Paint('2. Hover - Tooltip is <Enabled: true>')
        CheckHash(0x578201E79429C78D)
    ";
    App::debug(60, 10, script).desktop(MyDesktop::new()).app_bar().build().unwrap().run();
}

#[test]
fn check_button_shortcut() {
    // Desktop with menu
    #[Desktop(events = AppBarEvents+DesktopEvents, internal: true)]
    struct MyDesktop {
        h: Handle<appbar::Button>,
        l: Handle<appbar::Label>,
        cnt: i32,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                h: Handle::None,
                l: Handle::None,
                cnt: 1,
            }
        }
    }
    impl DesktopEvents for MyDesktop {
        fn on_start(&mut self) {
            self.h = self.appbar().add(appbar::Button::new(" &Add ", 0, Side::Left));
            self.l = self.appbar().add(appbar::Label::new("1", 0, Side::Left));
        }
    }
    impl AppBarEvents for MyDesktop {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h);
            appbar.show(self.l);
        }

        fn on_button_click(&mut self, _: Handle<appbar::Button>) {
            let h = self.l;
            self.cnt += 1;
            let s = format!("{}", self.cnt);
            if let Some(lb) = self.appbar().get_mut(h) {
                lb.set_caption(&s)
            }
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xCF08088F9E5191F9)
        Key.Pressed(Alt+A)
        Paint('2. Now cnt = 2')
        CheckHash(0xE22138588EE3A3E6)
        Mouse.Click(3,0,left)
        Paint('3. Now cnt = 3')
        CheckHash(0xC636FD44BF9F062F)        
    ";
    App::debug(60, 10, script).desktop(MyDesktop::new()).app_bar().build().unwrap().run();
}

#[test]
fn check_toggle_button() {
    #[Window(events = AppBarEvents, internal: true)]
    pub(crate) struct Win {
        lb: Handle<Label>,
        h_bold: Handle<appbar::ToggleButton>,
        h_italic: Handle<appbar::ToggleButton>,
        h_underline: Handle<appbar::ToggleButton>,
        h_sep: Handle<appbar::Separator>,
    }
    impl Win {
        pub(crate) fn new() -> Self {
            let mut w = Win {
                base: window!("'Toggle Buttons',a:c,w:40,h:8,Flags: Sizeable"),
                lb: Handle::None,
                h_bold: Handle::None,
                h_italic: Handle::None,
                h_underline: Handle::None,
                h_sep: Handle::None,
            };

            w.lb = w.add(label!("'Toggle buttons can change their state from selected or not',d:f"));
            w.h_sep = w.appbar().add(appbar::Separator::new(1, appbar::Side::Left));
            w.h_bold = w
                .appbar()
                .add(appbar::ToggleButton::with_tooltip(" B ", "Bold", false, 1, appbar::Side::Left));
            w.h_italic = w
                .appbar()
                .add(appbar::ToggleButton::with_tooltip(" I ", "Italc", false, 1, appbar::Side::Left));
            w.h_underline = w.appbar().add(appbar::ToggleButton::new(" U ", false, 1, appbar::Side::Left));

            w
        }
    }
    impl AppBarEvents for Win {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h_sep);
            appbar.show(self.h_bold);
            appbar.show(self.h_italic);
            appbar.show(self.h_underline);
        }

        fn on_togglebutton_state_changed(&mut self, togglebutton: Handle<appbar::ToggleButton>, selected: bool) {
            let s = match () {
                _ if togglebutton == self.h_bold => "Bold is :",
                _ if togglebutton == self.h_italic => "Italic is :",
                _ if togglebutton == self.h_underline => "Underline is :",
                _ => "",
            };
            let txt = format!("{s} {selected}");
            let h = self.lb;
            if let Some(lb) = self.control_mut(h) {
                lb.set_caption(&txt);
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. initial state : B, I and U not checked')
        CheckHash(0x9A3ED2DA126C8874)  
        Mouse.Move(2,0)
        Paint('2. Bold is hovered (not checked)')
        CheckHash(0x75803A4441D6FCAC)  
        Mouse.Click(2,0,left)
        Paint('3. Bold is selected')
        CheckHash(0x87849312084B4FC7)  
        Mouse.Move(8,0)
        Paint('4. Underline is hovered (not checked) - no tooltip is shown')
        CheckHash(0xE5ADA779B42E403F)          
        Mouse.Click(8,0,left)
        Paint('5. Underline is selected')
        CheckHash(0x924F21DEEC6A0F2B)  
        Mouse.Click(2,0,left)
        Paint('6. Bold is un-selected')
        CheckHash(0x21A77127D6AE5AEB)  
    ";

    let mut a = App::debug(80, 10, script).command_bar().app_bar().build().unwrap();
    a.add_window(Win::new());
    a.run();
}

#[test]
fn check_togglebutton_api() {
    // Desktop with menu
    #[Desktop(events = AppBarEvents+DesktopEvents, internal: true)]
    struct MyDesktop {
        h: Handle<appbar::ToggleButton>,
        h_inactive: Handle<appbar::ToggleButton>,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                h: Handle::None,
                h_inactive: Handle::None,
            }
        }
    }
    impl DesktopEvents for MyDesktop {
        fn on_start(&mut self) {
            self.h = self.appbar().add(appbar::ToggleButton::new("ToggleButton", false, 0, Side::Left));
            let mut m = appbar::ToggleButton::new("Inactive", true, 1, Side::Left);
            m.set_enabled(false);
            self.h_inactive = self.appbar().add(m);
        }
    }
    impl AppBarEvents for MyDesktop {
        fn on_update(&self, appbar: &mut AppBar) {
            let h = self.h;
            if let Some(mb) = appbar.get_mut(h) {
                let mut s = mb.caption().to_string();
                s.push_str("+1");
                mb.set_caption(&s);
                s = format!("Enabled: {},{}", mb.is_enabled(), mb.is_selected());
                mb.set_tooltip(&s);
                assert_eq!(mb.hotkey(), Key::None);
            }
            appbar.show(self.h);
            appbar.show(self.h_inactive);
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state - menu name is <ogleButton+1>')
        CheckHash(0x903A5C75FD798E9C)
        Mouse.Move(3,0)
        Paint('2. Hover - Tooltip is <Enabled: true>')
        CheckHash(0x9D71EE74E114C501)
        Mouse.Click(3,0,left)
        Paint('3. Click on togle')
        CheckHash(0x1F97C126CD55C6EC)
        Mouse.Move(20,0)
        Paint('4. Hover over inactive (same hash as 3)')
        CheckHash(0x1F97C126CD55C6EC)
        Mouse.Click(20,0,left)
        Paint('5. Click on inactive (same hash as 3)')
        CheckHash(0x1F97C126CD55C6EC)
    ";
    App::debug(60, 10, script).desktop(MyDesktop::new()).app_bar().build().unwrap().run();
}

#[test]
fn check_toggle_button_shortcut() {
    // Desktop with menu
    #[Desktop(events = AppBarEvents+DesktopEvents, internal: true)]
    struct MyDesktop {
        h: Handle<appbar::ToggleButton>,
        h2: Handle<appbar::Button>,
        l: Handle<appbar::Label>,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                h: Handle::None,
                l: Handle::None,
                h2: Handle::None,
            }
        }
    }
    impl DesktopEvents for MyDesktop {
        fn on_start(&mut self) {
            let mut tb = appbar::ToggleButton::new(" &Status ", false, 0, Side::Left);
            tb.set_selected(true);
            self.h = self.appbar().add(tb);
            self.l = self.appbar().add(appbar::Label::new("?", 0, Side::Left));
            self.h2 = self.appbar().add(appbar::Button::new(" &DoNothing ", 0, Side::Left));
        }
    }
    impl AppBarEvents for MyDesktop {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h);
            appbar.show(self.l);
            appbar.show(self.h2);
        }

        fn on_togglebutton_state_changed(&mut self, _: Handle<appbar::ToggleButton>, selected: bool) {
            let h = self.l;
            if let Some(lb) = self.appbar().get_mut(h) {
                if selected {
                    lb.set_caption("selected");
                } else {
                    lb.set_caption("not-selected");
                }
            }
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (Status,?,DoNothing)')
        CheckHash(0xE00B56C7D04BECFD)
        Key.Pressed(Alt+S)
        Paint('2. no-selected -> (Status,not-selected,DoNothing), status is not hovered')
        CheckHash(0x1BABD9712B74EC36)
        Mouse.Click(3,0,left)
        Paint('3. selected -> (Status,selected,DoNothing)')
        CheckHash(0xFDCE185A899DE157)   
        Mouse.Move(20,0)     
        Paint('4. Toggle remains selected, hover over DoNothing -> (Status,selected,DoNothing)')
        CheckHash(0xF8403DC8DDE7F36F)   
        Mouse.Click(20,0,left)     
        Paint('5. Same hash as 4, nothing hapens')
        CheckHash(0xF8403DC8DDE7F36F)   
    ";
    App::debug(60, 10, script).desktop(MyDesktop::new()).app_bar().build().unwrap().run();
}

#[test]
fn check_switch_button() {
    #[Window(events = AppBarEvents, internal: true)]
    pub(crate) struct Win {
        lb: Handle<Label>,
        h_s_1: Handle<appbar::SwitchButton>,
        h_s_2: Handle<appbar::SwitchButton>,
        h_s_3: Handle<appbar::SwitchButton>,
        h_sep: Handle<appbar::Separator>,
    }
    impl Win {
        pub(crate) fn new() -> Self {
            let mut w = Win {
                base: window!("'Switch Buttons',a:c,w:40,h:8,Flags: Sizeable"),
                lb: Handle::None,
                h_s_1: Handle::None,
                h_s_2: Handle::None,
                h_s_3: Handle::None,
                h_sep: Handle::None,
            };

            w.lb = w.add(label!("'Switch buttons',d:f"));
            w.h_sep = w.appbar().add(appbar::Separator::new(1, appbar::Side::Left));
            w.h_s_1 = w.appbar().add(appbar::SwitchButton::with_tooltip(
                "Yes",
                "No",
                appbar::SwitchButtonSymbol::CheckBox,
                "Tootip for switch 1",
                false,
                1,
                appbar::Side::Left,
            ));
            w.h_s_2 = w.appbar().add(appbar::SwitchButton::with_symbol(
                "Ok",
                "Not-Ok",
                appbar::SwitchButtonSymbol::CheckMark,
                true,
                1,
                appbar::Side::Left,
            ));
            w.h_s_3 = w
                .appbar()
                .add(appbar::SwitchButton::new(" State-1 ", " State-2 ", false, 1, appbar::Side::Left));

            w
        }
    }
    impl AppBarEvents for Win {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h_sep);
            appbar.show(self.h_s_1);
            appbar.show(self.h_s_2);
            appbar.show(self.h_s_3);
        }

        fn on_switchbutton_state_changed(&mut self, sb: Handle<appbar::SwitchButton>, selected: bool) {
            let s = match () {
                _ if sb == self.h_s_1 => "Switch 1 is :",
                _ if sb == self.h_s_2 => "Switch 2 is :",
                _ if sb == self.h_s_3 => "Switch 3 is :",
                _ => "",
            };
            assert_eq!(selected, self.appbar().get(sb).unwrap().is_selected());
            assert!(self.appbar().get(sb).unwrap().is_enabled());
            let txt = format!("{s} {selected}");
            let h = self.lb;
            if let Some(lb) = self.control_mut(h) {
                lb.set_caption(&txt);
            }
        }
    }
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('1. initial state ')
        CheckHash(0x34199CEF8A70B116)  
        Mouse.Move(3,0)
        Paint('2. Over switch 1 (tooltip is shown)')
        CheckHash(0xE6CA6E59CB2C35BD)  
        Mouse.Move(9,0)
        Paint('3. Over switch 2 ')
        CheckHash(0xD6347B2FA35EF386)  
        Mouse.Move(15,0)
        Paint('4. Over switch 3 (state is 2)')
        CheckHash(0xB540C1F0C0009B6E)  
        Mouse.Click(15,0,left)
        Paint('5. Click on switch 3 (state is 1)')
        CheckHash(0xD2D729A13BFF4A49)  
        Mouse.Click(9,0,left)
        Paint('6. Click on switch 2 (now is not-ok)')
        CheckHash(0x95281EF94C0AB084)  
        Mouse.Click(3,0,left)
        Paint('7. Click on switch 1 (now is yes)')
        CheckHash(0xEA70077163C07910)  
    ";

    let mut a = App::debug(80, 10, script).command_bar().app_bar().build().unwrap();
    a.add_window(Win::new());
    a.run();
}

#[test]
fn check_switchbutton_api() {
    // Desktop with menu
    #[Desktop(events = AppBarEvents+DesktopEvents, internal: true)]
    struct MyDesktop {
        h: Handle<appbar::SwitchButton>,
        h_inactive: Handle<appbar::SwitchButton>,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                h: Handle::None,
                h_inactive: Handle::None,
            }
        }
    }
    impl DesktopEvents for MyDesktop {
        fn on_start(&mut self) {
            self.h = self.appbar().add(appbar::SwitchButton::new("State-A", "State-B", false, 0, Side::Left));
            let mut m = appbar::SwitchButton::new("Inactive-A", "Inactive-B", true, 1, Side::Left);
            m.set_enabled(false);
            self.h_inactive = self.appbar().add(m);
        }
    }
    impl AppBarEvents for MyDesktop {
        fn on_update(&self, appbar: &mut AppBar) {
            let h = self.h;
            if let Some(mb) = appbar.get_mut(h) {
                let s = format!("Enabled: {},{}", mb.is_enabled(), mb.is_selected());
                mb.set_tooltip(&s);
                assert_eq!(mb.hotkey(), Key::None);
            }
            appbar.show(self.h);
            appbar.show(self.h_inactive);
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state - menu name is <ogleButton+1>')
        CheckHash(0xFAEAE5E5E717F22E)
        Mouse.Move(3,0)
        Paint('2. Hover - Tooltip is <Enabled: true>')
        CheckHash(0xA29CA62FD88E5797)
    ";
    App::debug(60, 10, script).desktop(MyDesktop::new()).app_bar().build().unwrap().run();
}

#[test]
fn check_switchbutton_hotkey() {
    // Desktop with menu
    #[Desktop(events = AppBarEvents+DesktopEvents, internal: true)]
    struct MyDesktop {
        h: Handle<appbar::SwitchButton>,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                h: Handle::None,
            }
        }
    }
    impl DesktopEvents for MyDesktop {
        fn on_start(&mut self) {
            let mut sb = appbar::SwitchButton::new(" State-&A ", " State-&B ", true, 0, Side::Left);
            sb.set_selected(false);
            self.h = self.appbar().add(sb); 
        }
    }
    impl AppBarEvents for MyDesktop {
        fn on_update(&self, appbar: &mut AppBar) {
            appbar.show(self.h);
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state - State is B')
        CheckHash(0x47A646967FF5C3C9)
        Key.Pressed(Alt+B)
        Paint('2. State is A')
        CheckHash(0xBD4CCE57AE86E536)
        Key.Pressed(Alt+A)
        Paint('3. State is now B again (same hash as 1)')
        CheckHash(0x47A646967FF5C3C9)
    ";
    App::debug(60, 10, script).desktop(MyDesktop::new()).app_bar().build().unwrap().run();
}
