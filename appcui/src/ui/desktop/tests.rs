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
    let a = App::debug(60, 10, InitializationFlags::None, MyDesktop::new(), script).unwrap();
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
    let a = App::debug(60, 10, InitializationFlags::None, MyDesktop::new(), script).unwrap();
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
    let a = App::debug(30, 7, InitializationFlags::None, MyDesktop::new(), script).unwrap();
    a.run();
}
