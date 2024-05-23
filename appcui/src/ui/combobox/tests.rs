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