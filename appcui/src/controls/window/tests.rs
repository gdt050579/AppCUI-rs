use AppCUIProcMacro::key;

use crate::{
    controls::{Desktop, Layout},
    system::{App, InitializationFlags},
    input::{Key, KeyModifier, KeyCode}
};

use super::{Window, WindowFlags};


#[test]
fn check_window_title() {
    let script = "
        //Paint('title')
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
        //Paint('Shorten title')
        CheckHash(0x3A1C142AE9968A2F)
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
fn check_window_tag_1() {
    let script = "
        //Paint('tags')
        CheckHash(0xBB2962251DDB2240)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_tag("ABC");
    a.add(w);
    a.run();
}
#[test]
fn check_window_tag_2() {
    let script = "
        //Paint('title should be visible')
        CheckHash(0xE2CB87CCC6FD9E4A)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_tag("ABCD");
    a.add(w);
    a.run();
}
#[test]
fn check_window_tag_and_split_title_1() {
    let script = "
        //Paint('title split with 3 special chars')
        CheckHash(0x34902E0B6D58F035)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_tag("ABCDE");
    a.add(w);
    a.run();
}
#[test]
fn check_window_tag_and_split_title_2() {
    let script = "
        //Paint('title split with 3 special chars')
        CheckHash(0xA52995587B045766)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_tag("ABCDEF");
    a.add(w);
    a.run();
}
#[test]
fn check_window_tag_and_title_first_letter() {
    let script = "
        //Paint('title first letter and special char with 3 points')
        CheckHash(0x6F914F802B3B7B5D)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_tag("ABCDEFG");
    a.add(w);
    a.run();
}
#[test]
fn check_window_tag_and_title_not_visible() {
    let script = "
        //Paint('title not visible')
        CheckHash(0xA2C91CB6A1484009)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_tag("ABCDEFGH");
    a.add(w);
    a.run();
}


#[test]
fn check_window_hotkey_1() {
    let script = "
        //Paint('hotkey')
        CheckHash(0x4454159FD9AA73E9)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_hotkey(key!("Alt+F1"));
    a.add(w);
    a.run();
}
#[test]
fn check_window_hotkey_2() {
    let script = "
        //Paint('hotkey')
        CheckHash(0xC9D2F0E450475385)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_hotkey(KeyCode::Enter);
    a.add(w);
    a.run();
}

#[test]
fn check_window_hotkey_and_tag() {
    let script = "
        //Paint('hotkey & tag')
        CheckHash(0x8F6D9DF3500A2D7A)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_hotkey(key!("Alt+1"));
    w.set_tag("XYZ");
    a.add(w);
    a.run();
}

#[test]
fn check_window_resize() {
    let script = "
        //Paint('initial state')
        CheckHash(0xCA2429E8E2E892CE)
        Mouse.Move(39,7)
        //Paint('over the resize handler')
        CheckHash(0xA36090DA42BA1E4)
        Mouse.Hold(39,7,left)
        //Paint('click on resize handler')
        CheckHash(0x8A07C3A15BDC6726)
        Mouse.Move(41,8)
        //Paint('resized')
        CheckHash(0x49D487762DD0A3F2)
        Mouse.Move(51,8)
        //Paint('even bigger')
        CheckHash(0xB71562AEB43E3BCE)
        Mouse.Move(47,6)
        //Paint('smaller')
        CheckHash(0x54A4BBBC93FEE)
        Mouse.Release(47,6,left)
        //Paint('after release of handle')
        CheckHash(0x3002AE84BFE16A36)
    ";
    let mut a = App::debug(60, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let w = Window::new("Title", Layout::new("d:c,w:20,h:5"), WindowFlags::Sizeable);
    a.add(w);
    a.run();
}