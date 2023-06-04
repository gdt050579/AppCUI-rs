use AppCUIProcMacro::AppCUIDesktop;
use crate::controls::*;
use crate::controls::events::*;
use crate::input::*;
use crate::graphics::*;
use crate::system::*;
use crate::controls::menu::*;

#[AppCUIDesktop(overwrite = OnPaint)]
struct MyDesktop {}
impl MyDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new()
        }
    }
}
impl OnPaint for MyDesktop {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(Character::new('x',Color::Red,Color::Green,CharFlags::None));
    }
}

#[test]
fn check_custom_paint_for_desktop() {
    let a = App::debug(60, 20, InitializationFlags::None, MyDesktop::new(), "Paint(desktop)").unwrap();
    a.run();
}