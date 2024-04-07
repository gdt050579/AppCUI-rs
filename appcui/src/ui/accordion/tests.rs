use crate::prelude::*;

#[test]
fn check_control_reposition() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xA54E71FE5E8F33F2)
        Mouse.Click(6,1,left)
        Paint('Maximized window')
        CheckHash(0x520775CA72A80B9A)
        Mouse.Move(10,12)
        Paint('Hover over second page')
        CheckHash(0xCE5C457C4DAAD6A0)
        Mouse.Click(10,12,left)
        Paint('Second page selected')
        CheckHash(0xCC377F34D916DAD2)
        Mouse.Click(2,0,left)
        Paint('Return to original size')
        CheckHash(0x455E671F87CAEBB2)
        Mouse.Click(40,11,left)
        Paint('3rd page selected')
        CheckHash(0x79E1692D8F6CCFA2)
        Mouse.Click(7,1,left)
        Paint('Maximize again')
        CheckHash(0x303FE6CF830F139A)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:12,flags: Sizeable");
    let mut ac = Accordion::new(Layout::new("l:0,t:0,r:0,b:0"), accordion::Flags::None);
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

#[test]
fn check_keys() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xA54E71FE5E8F33F2)
        Key.Pressed(Ctrl+Tab)
        Paint('Second panel selected')   
        CheckHash(0x455E671F87CAEBB2)
        Key.Pressed(Ctrl+Tab)
        Paint('3rd panel selected')   
        CheckHash(0x79E1692D8F6CCFA2)
        Key.Pressed(Ctrl+Shift+Tab,2)
        Paint('1st panel selected')   
        CheckHash(0xA54E71FE5E8F33F2)
        Key.Pressed(Alt+2)
        Paint('Second panel selected')   
        CheckHash(0x455E671F87CAEBB2)
        Key.Pressed(Alt+1)
        Paint('1st panel selected')   
        CheckHash(0xA54E71FE5E8F33F2)
        Key.Pressed(Alt+3)
        Paint('3rd panel selected')   
        CheckHash(0x79E1692D8F6CCFA2)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:12,flags: Sizeable");
    let mut ac = Accordion::new(Layout::new("l:0,t:0,r:0,b:0"), accordion::Flags::None);
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

#[test]
fn check_focus() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xF8B8FB1A7C6A4324)
    ";
    let mut a = App::debug(80, 15, script).build().unwrap();
    let mut w = window!("Test,d:c,w:75,h:12,flags: Sizeable");
    let mut ac1 = Accordion::new(Layout::new("x:1,y:1,w:20,h:8"), accordion::Flags::None);
    ac1.add_panel("Panel &1");
    ac1.add_panel("Panel &2");
    ac1.add_panel("Panel &3");
    ac1.add_panel("Panel &4");
    ac1.add_panel("Panel &5");

    let mut ac2 = Accordion::new(Layout::new("x:25,y:1,w:20,h:8"), accordion::Flags::None);
    ac2.add_panel("Panel &1");
    ac2.add_panel("Panel &2");
    ac2.add_panel("Panel &3");

    let mut ac3 = Accordion::new(Layout::new("x:49,y:1,w:20,h:8"), accordion::Flags::None);
    ac3.add_panel("Panel &1");
    ac3.add_panel("Panel &2");
    ac3.add_panel("Panel &3");
    ac3.set_enabled(false);

    w.add(ac1);
    w.add(ac2);
    w.add(ac3);
    a.add_window(w);
    a.run();
}


#[test]
fn check_macro() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xA54E71FE5E8F33F2)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:12,flags: Sizeable");
    let mut ac = accordion!("l:0,t:0,r:0,b:0,panels:['Panel &1','Panel &2','Panel &3']");
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