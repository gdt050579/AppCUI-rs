use crate::{
    controls::{Desktop, Layout},
    system::{App, InitializationFlags},
};

use super::{Window, WindowFlags};

#[test]
fn check_window_1() {
    let script = "
        Paint('desktop with red and green')
        //CheckHash(0xD490E8FF2EC89965)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add(Window::new(
        "Title",
        Layout::new("d:c,w:20,h:10"),
        WindowFlags::None,
    ));
    a.run();
}
