use crate::prelude::*;

#[test]
fn check_empty_combobox() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0xD3338F888B5B6D69)
        Key.Pressed(Space)
        Paint('Opened')   
        CheckHash(0xB34B803556F629E9)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:40,h:7");
    let c = ComboBox::new(Layout::new("x:1,y:1,w:30"),combobox::Flags::None);
    w.add(c);
    a.add_window(w);
    a.run();
}

#[test]
fn check_open_unselected_combobox() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0xD3338F888B5B6D69)
        Key.Pressed(Space)
        Paint('Opened (nothing selected)')   
        CheckHash(0xB6B348402DAF8E22)
        Key.Pressed(Space)
        Paint('closed (nothing selected)')   
        CheckHash(0xD3338F888B5B6D69)
        Key.Pressed(Space)
        Paint('Opened (nothing selected)')   
        CheckHash(0xB6B348402DAF8E22)
        Key.Pressed(Down)
        Paint('First item selected')   
        CheckHash(0xB28AF987ED8D20B0)
        Key.Pressed(Down)
        Paint('Second item selected')   
        CheckHash(0x43E2E38DE5C81DC7)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:40,h:7");
    let mut c = ComboBox::new(Layout::new("x:1,y:1,w:30"),combobox::Flags::None);
    c.add("option 1");
    c.add("option 2");
    c.add("option 3");
    w.add(c);
    a.add_window(w);
    a.run();
}