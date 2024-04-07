use crate::prelude::*;

//#[test]
fn check_control_reposition() {
    let script = "
        //Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xA54E71FE5E8F33F2)
        Mouse.Click(6,1,left)
        Paint('Maximized window')
        CheckHash(0x520775CA72A80B9A)
        Mouse.Move(10,12)
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
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:12,flags: Sizeable");
    let mut ac = Accordion::new(Layout::new("l:0,t:0,r:0,b:0"), tab::Flags::None);
    ac.add_panel("Panel &1");
    ac.add_panel("Panel &2");
    ac.add_panel("Panel &3");
    ac.add(0, button!("P-1-A,r:1,b:0,w:10,type:flat"));
    ac.add(0, button!("P-1-B,l:1,t:1,w:10,type:flat"));
    ac.add(1, button!("P-2-A,r:1,b:0,w:14,type:flat"));
    ac.add(1, button!("P-2-B,l:1,t:1,w:14,type:flat"));
    ac.add(2, button!("P-3-A,r:1,b:0,w:20,type:flat"));
    ac.add(2, button!("P-3-B,l:1,t:1,w:20,type:flat"));

    w.add(ac);
    a.add_window(w);
    a.run();
}