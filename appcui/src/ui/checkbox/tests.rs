use crate::prelude::*;

#[test]
fn check_checkbox_1() {
    let script = "
        Paint.Enable(false)
        Paint('some checkboxes')   
        CheckHash(0xAA393B73091205A4)  
        CheckCursor(13,5) 
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(CheckBox::new("Single line", Layout::new("x:1,y:1,w:20"), false));
    w.add(CheckBox::new("Single line (checked initially)", Layout::new("x:1,y:2,w:40"), true));
    w.add(CheckBox::new("A &multi line checkbox that can be checked or not", Layout::new("x:1,y:3,w:20,h:3"), true));

    a.add_window(w);
    a.run();
}