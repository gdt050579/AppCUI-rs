use crate::prelude::*;

#[Desktop(overwrite = OnPaint, internal = true)]
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
        Paint.Enable(false)
        Paint('desktop with red and green')
        CheckHash(0xD490E8FF2EC89965)
    ";
    let a = App::debug(60, 10, InitializationFlags::None, MyDesktop::new(), script).unwrap();
    a.run();
}
