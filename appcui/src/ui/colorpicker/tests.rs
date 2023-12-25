use crate::prelude::*;

#[test]
fn check_colorpicker_header_size() {
    let script = "
    //Paint.Enable(false)
    Paint('initial state')   
";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:11");
    w.add(ColorPicker::new(Color::Black, Layout::new("x:1,y:1,w:20")));
    a.add_window(w);
    a.run();
}
