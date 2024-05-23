use crate::prelude::*;

#[test]
fn check_creation() {
    let script = "
        //Paint.Enable(false)
        Error.Disable(true)
        Paint('initial state')   
        CheckHash(0x8E402A80F606DBF1)
        //Key.Pressed(Space)
        Paint('Opened')   
        CheckHash(0x8E402A80F606DBF1)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:40,h:7");
    let mut c = ComboBox::new(Layout::new("x:1,y:1,w:30"),combobox::Flags::None);
    w.add(c);
    a.add_window(w);
    a.run();
}