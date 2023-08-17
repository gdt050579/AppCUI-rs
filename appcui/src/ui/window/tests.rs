use AppCUIProcMacro::*;

use crate::{
    input::{Key, KeyCode, KeyModifier},
    system::{App, InitializationFlags},
    ui::{common::ControlHandle, Desktop, Layout},
};

use super::{
    toolbar::{self, Gravity},
    Window, WindowFlags,
};

#[test]
fn check_window_title() {
    let script = "
        Paint.Enable(false)
        Paint('title')
        CheckHash(0xA0CFD68A45B1786C)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add_window(Window::new(
        "Title",
        Layout::new("d:c,w:20,h:10"),
        WindowFlags::None,
    ));
    a.run();
}
#[test]
fn check_window_full_title_1() {
    let script = "
        Paint.Enable(false)
        Paint('full title')
        CheckHash(0xF410B9650F4ADF18)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add_window(Window::new(
        "1234567890A",
        Layout::new("d:c,w:20,h:10"),
        WindowFlags::None,
    ));
    a.run();
}
#[test]
fn check_window_full_title_2() {
    let script = "
        Paint.Enable(false)
        Paint('full title')
        CheckHash(0xA0CFD68A45B1786C)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add_window(Window::new(
        "Title",
        Layout::new("d:c,w:20,h:10"),
        WindowFlags::None,
    ));
    a.run();
}
#[test]
fn check_window_full_title_3() {
    let script = "
        Paint.Enable(false)
        Paint('full title')
        CheckHash(0xEEBF652BB26E9C4C)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add_window(Window::new(
        "ABC",
        Layout::new("d:c,w:12,h:8"),
        WindowFlags::None,
    ));
    a.run();
}
#[test]
fn check_window_minimize_title_1() {
    let script = "
        Paint.Enable(false)
        Paint('Title = ABCD...IJKL')
        CheckHash(0x671DB3CA4AD392AE)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add_window(Window::new(
        "ABCDEFGHIJKL",
        Layout::new("d:c,w:20,h:10"),
        WindowFlags::None,
    ));
    a.run();
}
#[test]
fn check_window_minimize_title_2() {
    let script = "
        Paint.Enable(false)
        Paint('Title = AB...KL')
        CheckHash(0x7F7F1F564130F50E)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add_window(Window::new(
        "ABCDEFGHIJKL",
        Layout::new("d:c,w:16,h:8"),
        WindowFlags::None,
    ));
    a.run();
}
#[test]
fn check_window_minimize_title_3() {
    let script = "
        Paint.Enable(false)
        Paint('Title = A...L')
        CheckHash(0x6CB0EAB5DDA0E087)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add_window(Window::new(
        "ABCDEFGHIJKL",
        Layout::new("d:c,w:14,h:6"),
        WindowFlags::None,
    ));
    a.run();
}
#[test]
fn check_window_minimize_title_4() {
    let script = "
        Paint.Enable(false)
        Paint('Shorten title')
        CheckHash(0x3A1C142AE9968A2F)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add_window(Window::new(
        "ABCDEFGHIJKL",
        Layout::new("d:c,w:12,h:6"),
        WindowFlags::None,
    ));
    a.run();
}

#[test]
fn check_window_tag_1() {
    let script = "
        Paint.Enable(false)
        Paint('tags')
        CheckHash(0xBB2962251DDB2240)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_tag("ABC");
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_tag_2() {
    let script = "
        Paint.Enable(false)
        Paint('title should be visible')
        CheckHash(0xE2CB87CCC6FD9E4A)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_tag("ABCD");
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_tag_and_split_title_1() {
    let script = "
        Paint.Enable(false)
        Paint('title split with 3 special chars')
        CheckHash(0x34902E0B6D58F035)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_tag("ABCDE");
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_tag_and_split_title_2() {
    let script = "
        Paint.Enable(false)
        Paint('title split with 3 special chars')
        CheckHash(0xA52995587B045766)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_tag("ABCDEF");
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_tag_and_title_first_letter() {
    let script = "
        Paint.Enable(false)
        Paint('title first letter and special char with 3 points')
        CheckHash(0x6F914F802B3B7B5D)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_tag("ABCDEFG");
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_tag_and_title_not_visible() {
    let script = "
        Paint.Enable(false)
        Paint('title not visible')
        CheckHash(0xA2C91CB6A1484009)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_tag("ABCDEFGH");
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_hotkey_1() {
    let script = "
        Paint.Enable(false)
        Paint('hotkey')
        CheckHash(0x4454159FD9AA73E9)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_hotkey(key!("Alt+F1"));
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_hotkey_2() {
    let script = "
        Paint.Enable(false)
        Paint('hotkey')
        CheckHash(0xC9D2F0E450475385)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_hotkey(KeyCode::Enter);
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_hotkey_and_tag() {
    let script = "
        Paint.Enable(false)
        Paint('hotkey & tag')
        CheckHash(0x8F6D9DF3500A2D7A)
    ";
    let mut a = App::debug(20, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), WindowFlags::None);
    w.set_hotkey(key!("Alt+1"));
    w.set_tag("XYZ");
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_resize() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0x6E5585BA8803D312)
        Mouse.Move(39,7)
        Paint('over the resize handler')
        CheckHash(0x18BDFDDEEC0AAC26)
        Mouse.Hold(39,7,left)
        Paint('click on resize handler')
        CheckHash(0x1B338A6C37E2546A)
        Mouse.Move(41,8)
        Paint('resized')
        CheckHash(0x36E28C864436D546)
        Mouse.Move(51,8)
        Paint('even bigger')
        CheckHash(0xF74E3A7D92E695E2)
        Mouse.Move(47,6)
        Paint('smaller')
        CheckHash(0x1628BC4542D3DC12)
        Mouse.Release(47,6,left)
        Paint('after release of handle')
        CheckHash(0x6A9DA986C039579A)
    ";
    let mut a = App::debug(60, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let w = Window::new("Title", Layout::new("d:c,w:20,h:5"), WindowFlags::Sizeable);
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_size() {
    #[Window(overwrite = OnResize,internal = true)]
    struct MyWin {
        info: ControlHandle<Label>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", Layout::new("d:c,w:40,h:7"), WindowFlags::None),
                info: ControlHandle::None,
            };
            me.info = me.add(Label::new("", Layout::new("x:1,y:1,w:36,h:3")));
            me
        }
    }
    impl OnResize for MyWin {
        fn on_resize(&mut self, old_size: Size, new_size: Size) {
            self.base.on_resize(old_size, new_size);
            let label_handle = self.info;
            let size = self.get_size();
            let client_size = self.get_client_size();
            if let Some(label) = self.get_control_mut(label_handle) {
                label.set_text(
                    format!(
                        "Previous size : {}x{}\nNew size      : {}x{}\nClient size   : {}x{}",
                        old_size.width,
                        old_size.height,
                        size.width,
                        size.height,
                        client_size.width,
                        client_size.height
                    )
                    .as_str(),
                );
            }
            assert!(size == new_size);
            assert!(old_size.width == 12); // minim window width
            assert!(old_size.height == 3); // minim window height (set by bounds)
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0x8471BBDBD6D2B1E)
    ";
    let mut a = App::debug(60, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_window_move() {
    let script = "
        Paint.Enable(false)
        Mouse.Move(30,3)
        Paint('over the title')
        CheckHash(0x6E5585BA8803D312)
        Mouse.Hold(30,3,left)
        Paint('press on window title')
        CheckHash(0x5EEA06B578ABD9A)
        Mouse.Move(15,1)
        Paint('window moved')
        CheckHash(0x1EBF887394C43672)
        Mouse.Move(1,1)
        Paint('window moved outside')
        CheckHash(0x8C8185EC85BF07AE)
        Mouse.Release(1,1,left)
        Paint('after release of handle')
        CheckHash(0x922FBAECBC6B2C2)
    ";
    let mut a = App::debug(60, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let w = Window::new("Title", Layout::new("d:c,w:20,h:5"), WindowFlags::Sizeable);
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_toolbar_label() {
    let script = "
        Paint.Enable(false)
        Paint('multiple label')
        CheckHash(0x7DF82A0072CCF28F)
    ";
    let mut a = App::debug(60, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:8"), WindowFlags::None);
    w.get_toolbar()
        .add(toolbar::Label::new(Gravity::BottomLeft, "Label 1"));
    w.get_toolbar()
        .add(toolbar::Label::new(Gravity::BottomLeft, "Label 2"));
    w.get_toolbar()
        .add(toolbar::Label::new(Gravity::BottomRight, "Label 3"));

    a.add_window(w);
    a.run();
}

#[test]
fn check_window_toolbar_button() {
    let script = "
        Paint.Enable(false)
        Paint('buttons')
        CheckHash(0x52243E492A4813E1)
        Mouse.Move(16,8)
        Paint('Mouse on button start')
        CheckHash(0x11C054FAECF9D51F)
        Mouse.Move(21,8)
        Paint('Mouse on button stop')
        CheckHash(0x63DCB4AE11E49B9)
        Mouse.Hold(21,8,left)
        Paint('Mouse press over stop button')
    ";
    let mut a = App::debug(60, 10, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:8"), WindowFlags::None);
    w.get_toolbar()
        .add(toolbar::Button::new(Gravity::BottomLeft, "Start", 1));
    w.get_toolbar()
        .add(toolbar::Button::new(Gravity::BottomLeft, "Stop", 2));
    w.get_toolbar()
        .add(toolbar::Button::new(Gravity::BottomRight, "Exit", 3));

    a.add_window(w);
    a.run();
}
