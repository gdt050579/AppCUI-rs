use crate::prelude::*;

#[test]
fn check_label_position() {
    let script = "
        Paint.Enable(false)
        Paint('nine labels across al corners and center')   
        CheckHash(0xF7D704CAB062ED5C)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(Label::new("TopLeft", Layout::new("d:tl,w:7,h:1")));
    w.add(Label::new("Top", Layout::new("d:t,w:3,h:1")));
    w.add(Label::new("TopRight", Layout::new("d:tr,w:8,h:1")));
    w.add(Label::new("Right", Layout::new("d:r,w:5,h:1")));
    w.add(Label::new("BottomRight", Layout::new("d:br,w:11,h:1")));
    w.add(Label::new("Bottom", Layout::new("d:b,w:6,h:1")));
    w.add(Label::new("BottomLeft", Layout::new("d:bl,w:10,h:1")));
    w.add(Label::new("Left", Layout::new("d:l,w:4,h:1")));
    w.add(Label::new("Center", Layout::new("d:c,w:6,h:1")));

    a.add_window(w);
    a.run();
}
#[test]
fn check_label_multiline() {
    let script = "
        Paint.Enable(false)
        Paint('a multi-line label')   
        CheckHash(0xD4FE75C904BD13F9)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(Label::new("This is a multi-line label", Layout::new("d:tl,w:10,h:3")));
    a.add_window(w);
    a.run();
}
#[test]
fn check_label_with_hotkey() {
    let script = "
        Paint.Enable(false)
        Paint('label with a hot ket')   
        CheckHash(0xD2356769850743E5)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(Label::new("A &hot ket label", Layout::new("d:tl,w:30")));
    a.add_window(w);
    a.run();
}
