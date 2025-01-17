use crate::prelude::*;

#[test]
fn check_aspect() {
    let script = "
        Paint.Enable(false)
        Paint('Show one numeric slider')   
        CheckHash(0x117FB20F2E02572F)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test, d:c, w:40, h:9");
    w.add(hnumericslider!("i32, 5, 1, 9, 1, x:1, y:1, w:100%"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_aspect_up() {
    let script = "
        Paint.Enable(false)
        Paint('Show one numeric slider with values above the line')   
        CheckHash(0x17E1A7FD4BC305E1)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test, d:c, w:40, h:9");
    w.add(hnumericslider!("i32, 7, 1, 9, 1, x:1, y:1, w:100%, flags:OnTop"));
    a.add_window(w);
    a.run();
}


#[test]
fn check_click() {
    let script = "
        Paint.Enable(false)
        Paint('Test if clicks work as intended')
        CheckHash(0x117FB20F2E02572F)
        Mouse.Click(20, 4, left)
        Paint('After one click')
        CheckHash(0x275A9F3BD09DDA2F)
        Mouse.Click(40, 4, left)
        Paint('After second click')
        CheckHash(0x88C2C451AD53502F)
        Mouse.Drag(40, 4, 19, 4)
        Paint('After mouse drag')
        CheckHash(0x30D888DEAD435D2F)
        Mouse.Drag(19, 5, 34, 5)
        Paint('After mouse drag nr 2')
        CheckHash(0xD8003CB1DB99552F)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test, d:c, w:40, h:9");
    w.add(hnumericslider!("i32, 5, 1, 9, 1, x:1, y:1, w:100%"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_smaller_window_resize() {
    let script = "
        Paint.Enable(true)
        Paint('Show slider in small window')   
        CheckHash(0x249D67EEA5FA0F64)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test, d:c, w:100%, h:100%");
    w.add(hnumericslider!("i32, 40, 10, 100, 5, x:1, y:1, w:10"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_buttons() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xE9AC758297342943)
        Key.Pressed(Right,3)
        Paint('After right 3 times')
        CheckHash(0x623902DBB654EA43)
        Key.Pressed(Left,6)
        Paint('After left 6 times')
        CheckHash(0x34F1F8CE04509343)
        Key.Pressed(End,2)
        Paint('After End 2 times')
        CheckHash(0x623902DBB654EA43)
        Key.Pressed(Home,2)
        Paint('After Home 2 times')
        CheckHash(0xB9CAD40BECBB7F43)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test, d:c, w:60, h:9");
    w.add(hnumericslider!("i32, 40, 10, 50, 5, x:1, y:1, w:100%"));
    a.add_window(w);
    a.run();
}

