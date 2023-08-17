use crate::{
    ui::{Desktop, Label, Layout, Window, WindowFlags},
    system::{App, InitializationFlags},
};

#[test]
fn check_label_position() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0xF7D704CAB062ED5C)   
    ";
    let mut a = App::debug(60, 11, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), WindowFlags::None);
    w.add(Label::new("TopLeft", Layout::new("d:tl,w:7")));
    w.add(Label::new("Top", Layout::new("d:t,w:3")));
    w.add(Label::new("TopRight", Layout::new("d:tr,w:8")));
    w.add(Label::new("Right", Layout::new("d:r,w:5")));
    w.add(Label::new("BottomRight", Layout::new("d:br,w:11")));
    w.add(Label::new("Bottom", Layout::new("d:b,w:6")));
    w.add(Label::new("BottomLeft", Layout::new("d:bl,w:10")));
    w.add(Label::new("Left", Layout::new("d:l,w:4")));
    w.add(Label::new("Center", Layout::new("d:c,w:6")));

    a.add_window(w);
    a.run();
}
#[test]
fn check_label_multiline() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0xD4FE75C904BD13F9)   
    ";
    let mut a = App::debug(60, 11, InitializationFlags::None, Desktop::new(), script).unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), WindowFlags::None);
    w.add(Label::new("This is a multi-line label", Layout::new("d:tl,w:10,h:3")));
    a.add_window(w);
    a.run();
}
