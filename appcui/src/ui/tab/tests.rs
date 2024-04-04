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

#[test]
fn check_key_control() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0x52DCC8DF3E55C403)
        Key.Pressed(Ctrl+Tab)
        Paint('2nd page')
        CheckHash(0x44CDC0ABE77E55F3)
        Key.Pressed(Ctrl+Tab)
        Paint('3rd page')
        CheckHash(0x4D5C8439170A28E7)
        Key.Pressed(Ctrl+Tab)
        Paint('first page')
        CheckHash(0x52DCC8DF3E55C403)
        Key.Pressed(Ctrl+Shift+Tab)
        Paint('3rd page again')
        CheckHash(0x4D5C8439170A28E7)
        Key.Pressed(Ctrl+Tab,2)
        Paint('2nd page (again)')
        CheckHash(0x44CDC0ABE77E55F3)
        Key.Pressed(Ctrl+Shift+Tab)
        Paint('first page again')
        CheckHash(0x52DCC8DF3E55C403)
        Key.Pressed(Alt+3)
        Paint('3rd page again (hotkey)')
        CheckHash(0x4D5C8439170A28E7)
        Key.Pressed(Alt+2)
        Paint('2nd page - hotkey')
        CheckHash(0x44CDC0ABE77E55F3)
        Key.Pressed(Alt+1)
        Paint('1st page - hotkey')
        CheckHash(0x52DCC8DF3E55C403)        
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

#[test]
fn check_switch_between_tabcontrols() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state - cancel button has focus')   
        CheckHash(0xE80270A89ED9265B)
        Key.Pressed(Tab)
        Paint('First tab has focus (T1-1-A)')
        CheckHash(0xD4F10F62888C0B67)
        Key.Pressed(Tab)
        Paint('First tab has focus (T1-1-B)')
        CheckHash(0x99DDF4F12CE42523)
        Key.Pressed(Ctrl+Tab)
        Paint('Change Tab1 focus - Page 2 (T1-2-B)')
        CheckHash(0x79A2DE7C7976B3D7)
        Key.Pressed(Tab)
        Paint('Tab2 focus (T2-1-A)')
        CheckHash(0x5BCE19978646CF83)
        Key.Pressed(Tab,2)
        Paint('OK button has focus')
        CheckHash(0xFBE8E74E4A5AD143)           
    ";
    let mut a = App::debug(100, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%");
    let mut tab1 = Tab::new(Layout::new("l:0,t:0,r:52,b:2"),tab::Flags::None);
    tab1.add_tab("Page &1");
    tab1.add_tab("Page &2");
    tab1.add_tab("Page &3");
    tab1.add(0, button!("T1-1-A,r:1,b:0,w:10,type:flat"));
    tab1.add(0, button!("T1-1-B,d:c,w:10,type:flat"));    
    tab1.add(1, button!("T1-2-A,r:1,b:0,w:14,type:flat"));
    tab1.add(1, button!("T1-2-B,d:c,w:14,type:flat")); 
    tab1.add(2, button!("T1-3-A,r:1,b:0,w:20,type:flat"));
    tab1.add(2, button!("T1-3-B,d:l,w:20,type:flat"));  
    w.add(tab1); 

    let mut tab2 = Tab::new(Layout::new("l:50,t:0,r:0,b:2"),tab::Flags::None);
    tab2.add_tab("Page &1");
    tab2.add_tab("Page &2");
    tab2.add_tab("Page &3");
    tab2.add(0, button!("T2-1-A,r:1,b:0,w:10,type:flat"));
    tab2.add(0, button!("T2-1-B,d:c,w:10,type:flat"));    
    tab2.add(1, button!("T2-2-A,r:1,b:0,w:14,type:flat"));
    tab2.add(1, button!("T2-2-B,d:c,w:14,type:flat")); 
    tab2.add(2, button!("T2-3-A,r:1,b:0,w:20,type:flat"));
    tab2.add(2, button!("T2-3-B,d:l,w:20,type:flat"));  
    w.add(tab2); 

    w.add(button!("OK,r:0,b:0,w:10, type: flat"));
    w.add(button!("Cancel,r:12,b:0,w:10, type: flat"));

    a.add_window(w);
    a.run();
}