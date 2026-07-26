use crate::prelude::*;


#[test]
fn check_hyperlink_control_with_macro() {
    let script = "
        Paint.Enable(false)
        Paint('1.initial state')
        CheckHash(0x33C839A5D930A167)
        Mouse.Move(12,2)
        Mouse.Hold(12,2,left)
        Paint('2.button apasat peste')
        CheckHash(0x1F8A1799D9614CF7)
        Mouse.Release(12,2,left)
        Paint('3. dupa release')
        CheckHash(0x33C839A5D930A167)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = Window::new("Macro Test", layout!("a:c,w:40,h:10"), window::Flags::None);
    w.add(hyperlink!("'Wiki',url:'www.wikipedia.com',x:1,y:1,w:9"));
    a.add_window(w);
    a.run();
}