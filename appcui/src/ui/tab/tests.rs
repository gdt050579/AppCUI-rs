use crate::prelude::*;

#[test]
fn check_paint() {
    let script = "
        //Paint.Enable(false)
        Paint('Initial state')   
        //CheckHash(0xC656986DBDA863BA)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:10");
    let mut tab = Tab::new(Layout::new("x:1,y:1,w:44,h:6"),tab::Flags::None);
    tab.add_tab("Page &1");
    tab.add_tab("Page &2");
    tab.add_tab("Page &3");
    w.add(tab);
    a.add_window(w);
    a.run();
}