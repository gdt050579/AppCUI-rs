use crate::prelude::*;

#[test]
fn check_control_reposition() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0x52DCC8DF3E55C403)
        Mouse.Click(6,2,left)
        Paint('Maximized window')
        CheckHash(0x800E288E7DB0F28B)
        Mouse.Move(20,1)
        Paint('Hover over second page')
        CheckHash(0x7D995CC4CB874301)
        Mouse.Click(20,1,left)
        Paint('Second page selected')
        CheckHash(0x565503C15E91F6DB)
        Mouse.Click(2,0,left)
        Paint('Return to original size')
        CheckHash(0x44CDC0ABE77E55F3)
        Mouse.Click(40,3,left)
        Paint('3rd page selected')
        CheckHash(0x4D5C8439170A28E7)
        Mouse.Click(6,2,left)
        Paint('Maximize again')
        CheckHash(0x8FB38F9341D9899F)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:7,flags: Sizeable");
    let mut tab = Tab::new(Layout::new("l:0,t:0,r:0,b:0"),tab::Flags::None);
    tab.add_tab("Page &1");
    tab.add_tab("Page &2");
    tab.add_tab("Page &3");
    tab.add(0, button!("Page1-A,r:1,b:0,w:10"));
    tab.add(0, button!("Page1-B,d:c,w:10"));    
    tab.add(1, button!("Page2-A,r:1,b:0,w:14"));
    tab.add(1, button!("Page2-B,d:c,w:14")); 
    tab.add(2, button!("Page3-A,r:1,b:0,w:20"));
    tab.add(2, button!("Page3-B,d:l,w:20"));  

    w.add(tab); 
    a.add_window(w);
    a.run();
}