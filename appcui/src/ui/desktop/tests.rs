use crate::prelude::*;

#[test]
fn check_custom_paint_for_desktop() {
    #[Desktop(overwrite = OnPaint, internal = true)]
    struct MyDesktop {}
    impl MyDesktop {
        fn new() -> Self {
            Self { base: Desktop::new() }
        }
    }
    impl OnPaint for MyDesktop {
        fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
            surface.clear(Character::new('x', Color::Red, Color::Green, CharFlags::None));
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('desktop with red and green')
        CheckHash(0xD490E8FF2EC89965)
    ";
    let a = App::debug(60, 10, script).desktop(MyDesktop::new()).build().unwrap();
    a.run();
}

#[test]
fn check_on_start_for_desktop() {
    #[Desktop(overwrite = OnPaint, events = DesktopEvents, internal = true)]
    struct MyDesktop {
        info: String,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                info: String::new(),
            }
        }
    }
    impl OnPaint for MyDesktop {
        fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
            surface.clear(Character::new('.', Color::Yellow, Color::Black, CharFlags::None));
            surface.write_string(
                1,
                1,
                self.info.as_str(),
                CharAttribute::new(Color::White, Color::DarkRed, CharFlags::None),
                false,
            );
        }
    }
    impl DesktopEvents for MyDesktop {
        fn on_start(&mut self) {
            self.info.push_str("started");
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('desktop with yellow dots and started written with red background')
        CheckHash(0x7B0B399907719797)
    ";
    let a = App::debug(60, 10, script).desktop(MyDesktop::new()).build().unwrap();
    a.run();
}

#[test]
fn check_on_resize_for_desktop() {
    #[Desktop(overwrite = OnPaint+OnResize,  internal = true)]
    struct MyDesktop {
        info: String,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                info: String::new(),
            }
        }
    }
    impl OnPaint for MyDesktop {
        fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
            surface.clear(Character::new('.', Color::Yellow, Color::Black, CharFlags::None));
            surface.write_string(
                1,
                1,
                self.info.as_str(),
                CharAttribute::new(Color::White, Color::DarkRed, CharFlags::None),
                true,
            );
        }
    }
    impl OnResize for MyDesktop {
        fn on_resize(&mut self, old_size: Size, new_size: Size) {
            self.info.clear();
            self.info.push_str(
                format!(
                    "Old size: {}x{}\nNew size: {}x{}",
                    old_size.width, old_size.height, new_size.width, new_size.height
                )
                .as_str(),
            );
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('Initial state (30x10)')
        CheckHash(0x66D6684907F9EA24)
        Resize(40,5)
        Paint('Old: 30x10, New: 40x5')
        CheckHash(0x6CDE4060C8AB8E26)
    ";
    let a = App::debug(30, 7, script).desktop(MyDesktop::new()).build().unwrap();
    a.run();
}

#[test]
fn check_menus() {
    #[Desktop(events = DesktopEvents + MenuEvents,  commands: [A,B,C], internal = true)]
    struct MyDesktop {
        file_menu: Handle<Menu>,
    }
    impl MyDesktop {
        fn new() -> Self {
            Self {
                base: Desktop::new(),
                file_menu: Handle::None,
            }
        }
    }
    impl DesktopEvents for MyDesktop {
        fn on_start(&mut self) {
            self.file_menu = self.register_menu(menu!(
                "&File,class: MyDesktop, items=[
                    {New,F1,cmd:A},
                    {&Save,F2,cmd:B},
                    {'&Save As ...',Alt+F2,cmd:C},
                    {&Open,F3,cmd:A},
                    {-},
                    {E&xit,Alt+F4,cmd:C}
                ]"
            ));
        }
    }
    impl MenuEvents for MyDesktop {
        fn on_update_menubar(&self, menubar: &mut MenuBar) {
            menubar.add(self.file_menu);
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('Initial state (with menus)')
        CheckHash(0x8FCFD286F8A503DB)
        Mouse.Click(3,0,left)
        Paint('Menu opem')
        CheckHash(0x610ADBDB875BBED1)
    ";
    let a = App::debug(40, 12, script).desktop(MyDesktop::new()).menu_bar().build().unwrap();
    a.run();
}

#[test]
fn check_on_close() {
    #[Desktop(events = DesktopEvents, internal = true)]
    struct MyDesktop {}
    impl MyDesktop {
        fn new() -> Self {
            Self { base: Desktop::new() }
        }
    }
    impl DesktopEvents for MyDesktop {
        fn on_close(&mut self) -> ActionRequest {
            if dialogs::validate("Exit", "Close application ?") {
                ActionRequest::Allow
            } else {
                ActionRequest::Deny
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('Initial state (with menus)')
        CheckHash(0xAB06844D69595285)
        Key.Pressed(Escape)
        Paint('Exit question')
        CheckHash(0xD156AD73229C5DB6)
        Key.Pressed(Escape)
        Paint('Back to original state')
        CheckHash(0xAB06844D69595285)
        Key.Pressed(Escape)
        Paint('Exit question')
        CheckHash(0xD156AD73229C5DB6)
        Key.Pressed(Alt+Y)
    ";
    let a = App::debug(40, 6, script).desktop(MyDesktop::new()).build().unwrap();
    a.run();
}
