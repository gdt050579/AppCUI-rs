use crate::controls::common::*;
use crate::controls::menu::*;
use super::super::command_bar::*;
use crate::controls::*;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;
use AppCUIProcMacro::AppCUIDesktop;

#[AppCUIDesktop(overwrite = OnPaint)]
struct MyDesktop {}
impl MyDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
        }
    }
}
impl OnPaint for MyDesktop {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(Character::new(
            'x',
            Color::Red,
            Color::Green,
            CharFlags::None,
        ));
    }
}

#[test]
fn check_custom_paint_for_desktop() {
    let script = "
        //Paint('desktop with red and green')
        CheckHash(0xD490E8FF2EC89965)
    ";
    let a = App::debug(60, 10, InitializationFlags::None, MyDesktop::new(), script).unwrap();
    a.run();
}
