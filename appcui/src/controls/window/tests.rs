use crate::{
    controls::{Desktop, Layout},
    system::{App, InitializationFlags},
};

use super::{Window, WindowFlags};

#[test]
fn check_window_full_title_1() {
    let script = "
        //Paint('full title')
        CheckHash(0xF410B9650F4ADF18)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add(Window::new(
        "1234567890A",
        Layout::new("d:c,w:20,h:10"),
        WindowFlags::None,
    ));
    a.run();
}
#[test]
fn check_window_full_title_2() {
    let script = "
        //Paint('full title')
        CheckHash(0xA0CFD68A45B1786C)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add(Window::new(
        "Title",
        Layout::new("d:c,w:20,h:10"),
        WindowFlags::None,
    ));
    a.run();
}
#[test]
fn check_window_full_title_3() {
    let script = "
        //Paint('full title')
        CheckHash(0xEEBF652BB26E9C4C)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add(Window::new(
        "ABC",
        Layout::new("d:c,w:12,h:8"),
        WindowFlags::None,
    ));
    a.run();
}
#[test]
fn check_window_minimize_title_1() {
    let script = "
        //Paint('Title = ABCD...IJKL')
        CheckHash(0x671DB3CA4AD392AE)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add(Window::new(
        "ABCDEFGHIJKL",
        Layout::new("d:c,w:20,h:10"),
        WindowFlags::None,
    ));
    a.run();
}
#[test]
fn check_window_minimize_title_2() {
    let script = "
        //Paint('Title = AB...KL')
        CheckHash(0x7F7F1F564130F50E)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add(Window::new(
        "ABCDEFGHIJKL",
        Layout::new("d:c,w:16,h:8"),
        WindowFlags::None,
    ));
    a.run();
}
#[test]
fn check_window_minimize_title_3() {
    let script = "
        //Paint('Title = A...L')
        CheckHash(0x6CB0EAB5DDA0E087)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add(Window::new(
        "ABCDEFGHIJKL",
        Layout::new("d:c,w:14,h:6"),
        WindowFlags::None,
    ));
    a.run();
}
#[test]
fn check_window_minimize_title_4() {
    let script = "
        //Paint('no title !')
        CheckHash(0xB4E69D08EB82C07B)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add(Window::new(
        "ABCDEFGHIJKL",
        Layout::new("d:c,w:12,h:6"),
        WindowFlags::None,
    ));
    a.run();
}

#[test]
fn check_window_tag() {
    let script = "
        Paint('full title')
        //CheckHash(0xA0CFD68A45B1786C)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_tag("ABC");
    a.add(w);
    a.run();
}
