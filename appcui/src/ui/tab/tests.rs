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
    let mut w = window!("Test,a:c,w:50,h:7,flags: Sizeable");
    let mut tab = Tab::new(layout!("l:0,t:0,r:0,b:0"), tab::Flags::None);
    tab.add_tab("Page &1");
    tab.add_tab("Page &2");
    tab.add_tab("Page &3");
    tab.add(0, button!("Page1-A,r:1,b:0,w:10"));
    tab.add(0, button!("Page1-B,a:c,w:10,h:100%"));
    tab.add(1, button!("Page2-A,r:1,b:0,w:14"));
    tab.add(1, button!("Page2-B,a:c,w:14,h:100%"));
    tab.add(2, button!("Page3-A,r:1,b:0,w:20"));
    tab.add(2, button!("Page3-B,a:l,w:20,h:100%"));

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
    let mut w = window!("Test,a:c,w:50,h:7,flags: Sizeable");
    let mut tab = Tab::new(layout!("l:0,t:0,r:0,b:0"), tab::Flags::None);
    tab.add_tab("Page &1");
    tab.add_tab("Page &2");
    tab.add_tab("Page &3");
    tab.add(0, button!("Page1-A,r:1,b:0,w:10"));
    tab.add(0, button!("Page1-B,a:c,w:10,h:100%"));
    tab.add(1, button!("Page2-A,r:1,b:0,w:14"));
    tab.add(1, button!("Page2-B,a:c,w:14,h:100%"));
    tab.add(2, button!("Page3-A,r:1,b:0,w:20"));
    tab.add(2, button!("Page3-B,a:l,w:20,h:100%"));

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
    let mut w = window!("Test,d:f");
    let mut tab1 = Tab::new(layout!("l:0,t:0,r:52,b:2"), tab::Flags::None);
    tab1.add_tab("Page &1");
    tab1.add_tab("Page &2");
    tab1.add_tab("Page &3");
    tab1.add(0, button!("T1-1-A,r:1,b:0,w:10,type:flat"));
    tab1.add(0, button!("T1-1-B,a:c,w:10,h:100%,type:flat"));
    tab1.add(1, button!("T1-2-A,r:1,b:0,w:14,h:100%,type:flat"));
    tab1.add(1, button!("T1-2-B,a:c,w:14,h:100%,type:flat"));
    tab1.add(2, button!("T1-3-A,r:1,b:0,w:20,type:flat"));
    tab1.add(2, button!("T1-3-B,a:l,w:20,h:100%,type:flat"));
    w.add(tab1);

    let mut tab2 = Tab::new(layout!("l:50,t:0,r:0,b:2"), tab::Flags::None);
    tab2.add_tab("Page &1");
    tab2.add_tab("Page &2");
    tab2.add_tab("Page &3");
    tab2.add(0, button!("T2-1-A,r:1,b:0,w:10,type:flat"));
    tab2.add(0, button!("T2-1-B,a:c,w:10,h:100%,type:flat"));
    tab2.add(1, button!("T2-2-A,r:1,b:0,w:14,type:flat"));
    tab2.add(1, button!("T2-2-B,a:c,w:14,h:100%,type:flat"));
    tab2.add(2, button!("T2-3-A,r:1,b:0,w:20,type:flat"));
    tab2.add(2, button!("T2-3-B,a:l,w:20,h:100%,type:flat"));
    w.add(tab2);

    w.add(button!("OK,r:0,b:0,w:10, type: flat"));
    w.add(button!("Cancel,r:12,b:0,w:10, type: flat"));

    a.add_window(w);
    a.run();
}

#[test]
fn check_tab_on_top() {
    let script = "
        Paint.Enable(false)
        Paint('Tab on top')   
        CheckHash(0x3C196343BA4C5BCD)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:7");
    let mut tab = Tab::with_type(layout!("l:0,t:0,r:0,b:0"), tab::Flags::None, tab::Type::OnTop);
    tab.add_tab("Page &1");
    tab.add_tab("Page &2");
    tab.add_tab("Page &3");
    tab.add(0, button!("Page1-A,r:1,b:0,w:10"));
    tab.add(0, button!("Page1-B,a:c,w:10,h:100%"));
    w.add(tab);
    a.add_window(w);
    a.run();
}

#[test]
fn check_tab_on_bottom() {
    let script = "
        Paint.Enable(false)
        Paint('Tab on top')   
        CheckHash(0x453AC0EB4A1EA2E1)
        Mouse.Move(27,7)
        Paint('Hover over 2nd tab')
        CheckHash(0xFEF7E7949F3FBFE3)
        Mouse.Move(40,7)
        Paint('Hover over 3rd tab')
        CheckHash(0xAEC0A0FB2C29E77)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:7");
    let mut tab = Tab::with_type(layout!("l:0,t:0,r:0,b:0"), tab::Flags::None, tab::Type::OnBottom);
    tab.add_tab("Page &1");
    tab.add_tab("Page &2");
    tab.add_tab("Page &3");
    tab.add(0, button!("Page1-A,r:1,b:0,w:10"));
    tab.add(0, button!("Page1-B,a:c,w:10,h:100%"));
    w.add(tab);
    a.add_window(w);
    a.run();
}

#[test]
fn check_macro_build() {
    let script = "
        Paint.Enable(false)
        Paint('build with macro')   
        CheckHash(0x453AC0EB4A1EA2E1)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:7");
    let mut t = tab!("l:0,t:0,r:0,b:0,type: OnBottom,tabs=['Page &1','Page &2','Page &3']");
    t.add(0, button!("Page1-A,r:1,b:0,w:10"));
    t.add(0, button!("Page1-B,a:c,w:10,h:100%"));
    w.add(t);
    a.add_window(w);
    a.run();
}

#[test]
fn check_page_width() {
    #[Window(events = ButtonEvents, internal=true)]
    struct MyWin {
        info: Handle<Label>,
        minus: Handle<Button>,
        plus: Handle<Button>,
        tab: Handle<Tab>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win-1", layout!("d:f"), window::Flags::None),
                info: Handle::None,
                plus: Handle::None,
                minus: Handle::None,
                tab: Handle::None,
            };
            me.info = me.add(label!("'',x:4,y:0,w:5"));
            me.plus = me.add(button!("x:10,y:0,w:3,caption:'+',type:flat"));
            me.minus = me.add(button!("x:0,y:0,w:3,caption:'-',type:flat"));
            me.tab = me.add(tab!("l:1,t:3,r:1,b:0,tabs:['A','B']"));

            me.update_tab_wdth_info();

            me
        }
        fn update_tab_wdth_info(&mut self) {
            let tw = if let Some(t) = self.control(self.tab) { t.tab_width() } else { 0 };
            let txt = format!("{tw}");
            let h_label = self.info;
            if let Some(label) = self.control_mut(h_label) {
                label.set_caption(txt.as_str());
            }
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, button_handle: Handle<Button>) -> EventProcessStatus {
            let h = self.tab;
            if self.plus == button_handle {
                if let Some(t) = self.control_mut(h) {
                    let tw = t.tab_width();
                    t.set_tab_width(tw + 1);
                }
                self.update_tab_wdth_info();
                return EventProcessStatus::Processed;
            }
            if self.minus == button_handle {
                if let Some(t) = self.control_mut(h) {
                    let tw = t.tab_width();
                    t.set_tab_width(tw - 1);
                }
                self.update_tab_wdth_info();
                return EventProcessStatus::Processed;
            }
            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xB4726EB88590481A)   
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Paint('tab width is 8')  
        CheckHash(0x78CB6547CD105637)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Paint('tab width is 3 (limited)')   
        CheckHash(0x4C78D070BF0DB674)
        Mouse.Click(12,1,left)
        Mouse.Click(12,1,left)
        Paint('tab width is 5')   
        CheckHash(0xC837D11F2D6F8AA2)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_tab_width_macro() {
    let script = "
        Paint.Enable(false)
        Paint('build with macro')   
        CheckHash(0x7B4F15E5D50B4816)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:7");
    w.add(tab!("l:0,t:0,r:0,b:0,type: OnTop,tabs=['A','B','C'],tw:5"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_tab_on_left() {
    let script = "
        Paint.Enable(false)
        Paint('Tab on top')   
        CheckHash(0xCE96D04271FC5A04)
        Mouse.Move(10,5)
        Paint('Hover over 2nd tab')
        CheckHash(0x1D451DFD9954A596)
        Mouse.Move(10,6)
        Paint('Hover over 3rd tab')
        CheckHash(0x19AFCD73D259E49A)
        Mouse.Click(10,6,left)
        Paint('3rd page selected')
        CheckHash(0x51F4F150FE86E9D2)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:7");
    let mut tab = tab!("l:0,t:0,r:0,b:0,type:OnLeft,tabs:['Page &1','Page &2','Page &2']");
    tab.add(0, button!("Page1-A,r:1,b:0,w:10"));
    tab.add(0, button!("Page1-B,a:c,w:10,h:100%"));
    w.add(tab);
    a.add_window(w);
    a.run();
}

#[test]
fn check_tabsbar_on_top() {
    let script = "
        Paint.Enable(false)
        Paint('TabBar - on top')   
        CheckHash(0x72410B54ADF4591B)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:7");
    w.add(tab!("l:0,t:0,r:0,b:0,type: OnTop,tabs=['Pg &1','Pg &2','Pg &3'],tw:6,flags:TabsBar"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_tabsbar_on_bottom() {
    let script = "
        Paint.Enable(false)
        Paint('TabBar - on bottom')   
        CheckHash(0x1537B92659C4B71B)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:7");
    w.add(tab!("l:0,t:0,r:0,b:0,type: OnBottom,tabs=['Pg &1','Pg &2','Pg &3'],tw:6,flags:TabsBar"));
    a.add_window(w);
    a.run();
}
#[test]
fn check_tabsbar_on_left() {
    let script = "
        Paint.Enable(false)
        Paint('TabBar - on left')   
        CheckHash(0x2AD3E362406BFC73)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:7");
    w.add(tab!("l:0,t:0,r:0,b:0,type: OnLeft,tabs=['Pg &1','Pg &2','Pg &3'],tw:6,flags:TabsBar"));
    a.add_window(w);
    a.run();
}


#[test]
fn check_page_width_on_left() {
    #[Window(events = ButtonEvents, internal=true)]
    struct MyWin {
        info: Handle<Label>,
        minus: Handle<Button>,
        plus: Handle<Button>,
        tab: Handle<Tab>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win-1", layout!("d:f"), window::Flags::None),
                info: Handle::None,
                plus: Handle::None,
                minus: Handle::None,
                tab: Handle::None,
            };
            me.info = me.add(label!("'',x:4,y:0,w:5"));
            me.plus = me.add(button!("x:10,y:0,w:3,caption:'+',type:flat"));
            me.minus = me.add(button!("x:0,y:0,w:3,caption:'-',type:flat"));
            let mut t = tab!("l:1,t:3,r:1,b:0,tabs:['A','B'],type:OnLeft");
            t.add(0, button!("caption:'TopLeft',x:1,y:1,w:14,type:flat"));
            t.add(0, button!("caption:'BottomRight',r:1,b:1,w:18,type:flat"));
            me.tab = me.add(t);

            me.update_tab_wdth_info();

            me
        }
        fn update_tab_wdth_info(&mut self) {
            let tw = if let Some(t) = self.control(self.tab) { t.tab_width() } else { 0 };
            let txt = format!("{tw}");
            let h_label = self.info;
            if let Some(label) = self.control_mut(h_label) {
                label.set_caption(txt.as_str());
            }
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, button_handle: Handle<Button>) -> EventProcessStatus {
            let h = self.tab;
            if self.plus == button_handle {
                if let Some(t) = self.control_mut(h) {
                    let tw = t.tab_width();
                    t.set_tab_width(tw + 1);
                }
                self.update_tab_wdth_info();
                return EventProcessStatus::Processed;
            }
            if self.minus == button_handle {
                if let Some(t) = self.control_mut(h) {
                    let tw = t.tab_width();
                    t.set_tab_width(tw - 1);
                }
                self.update_tab_wdth_info();
                return EventProcessStatus::Processed;
            }
            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xFF8140618A335809)   
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Paint('tab width is 8')  
        CheckHash(0x8798CB1908367490)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Mouse.Click(2,1,left)
        Paint('tab width is 3 (limited)')   
        CheckHash(0x3D8CC6BFA80BD404)
        Mouse.Click(12,1,left)
        Mouse.Click(12,1,left)
        Paint('tab width is 5')   
        CheckHash(0xEC90EDD06FC8B9E)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_tab_caption() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0x24D39A4A0C5E00DD)
        Mouse.Click(38,3,left)
        Paint('Second page selected')
        CheckHash(0xE8E1404229A7AE4E)
        Mouse.Click(72,3,left)
        Paint('3rd page selected')
        CheckHash(0x7BEF7089EBF9BCC2)
    ";
    let mut a = App::debug(80, 20, script).build().unwrap();
    let mut w = window!("Test,a:c,w:70,h:17,flags: Sizeable");
    let mut tab = Tab::new(layout!("l:0,t:0,r:0,b:0"), tab::Flags::None);
    
    // Add initial tabs
    tab.add_tab("Page &1");
    tab.add_tab("Page &2");
    tab.add_tab("Page &3");

    tab.set_tab_width(22);
    
    // Verify initial captions
    assert_eq!(tab.tab_caption(0), Some("Page 1"));
    assert_eq!(tab.tab_caption(1), Some("Page 2"));
    assert_eq!(tab.tab_caption(2), Some("Page 3"));
    
    // Verify None for invalid indices
    assert_eq!(tab.tab_caption(3), None);
    assert_eq!(tab.tab_caption(999), None);
    
    // Change captions
    tab.set_tab_caption(1, "New Page 2");
    tab.set_tab_caption(2, "Updated Page 3");
    
    // Verify changed captions
    assert_eq!(tab.tab_caption(0), Some("Page 1"));
    assert_eq!(tab.tab_caption(1), Some("New Page 2"));
    assert_eq!(tab.tab_caption(2), Some("Updated Page 3"));
    
    // Verify None still works after changes
    assert_eq!(tab.tab_caption(3), None);
    assert_eq!(tab.tab_caption(999), None);
    
    // Add controls to tabs
    tab.add(0, button!("Page1-A,r:1,b:0,w:10"));
    tab.add(0, button!("Page1-B,a:c,w:10,h:100%"));
    tab.add(1, button!("Page2-A,r:1,b:0,w:14"));
    tab.add(1, button!("Page2-B,a:c,w:14,h:100%"));
    tab.add(2, button!("Page3-A,r:1,b:0,w:20"));
    tab.add(2, button!("Page3-B,a:l,w:20,h:100%"));

    w.add(tab);
    a.add_window(w);
    a.run();
}

#[test]
fn check_hidden_tabs() {
    let script = "
        Paint.Enable(false)
        Paint('Tabs ar hidden')   
        CheckHash(0x434174D1EB8F39A3)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:7");
    let mut tab = Tab::with_type(layout!("l:0,t:0,r:0,b:0"), tab::Flags::None, tab::Type::HiddenTabs);
    tab.add_tab("Page &1");
    tab.add_tab("Page &2");
    tab.add_tab("Page &3");
    tab.add(0, button!("Page1-A,r:1,b:0,w:10"));
    tab.add(0, button!("Page1-B,a:c,w:10,h:100%"));
    w.add(tab);
    a.add_window(w);
    a.run();
}

#[test]
fn check_hiddentabs_changepage() {
    #[Window(events = ButtonEvents, internal=true)]
    struct MyWin {
        info: Handle<Label>,
        minus: Handle<Button>,
        plus: Handle<Button>,
        tab: Handle<Tab>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", layout!("d:f"), window::Flags::None),
                info: Handle::None,
                plus: Handle::None,
                minus: Handle::None,
                tab: Handle::None,
            };
            me.info = me.add(label!("'',x:4,y:0,w:5"));
            me.plus = me.add(button!("x:4,y:0,w:3,caption:'>',type:flat"));
            me.minus = me.add(button!("x:0,y:0,w:3,caption:'<',type:flat"));
            let mut t = tab!("l:1,t:3,r:1,b:0,tabs:['A','B','C','D'],type:HiddenTabs");
            t.add(0, button!("caption:'TopLeft - A',x:1,y:1,w:14,type:flat"));
            t.add(0, button!("caption:'BottomRight - A',r:1,b:1,w:18,type:flat"));
            t.add(1, button!("caption:'TopLeft - B',x:1,y:1,w:14,type:flat"));
            t.add(1, button!("caption:'BottomRight - B',r:1,b:1,w:18,type:flat"));
            t.add(2, button!("caption:'TopLeft - C',x:1,y:1,w:14,type:flat"));
            t.add(2, button!("caption:'BottomRight - C',r:1,b:1,w:18,type:flat"));
            t.add(3, button!("caption:'TopLeft - D',x:1,y:1,w:14,type:flat"));
            t.add(3, button!("caption:'BottomRight - D',r:1,b:1,w:18,type:flat"));
            me.tab = me.add(t);
            me
        }

    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, button_handle: Handle<Button>) -> EventProcessStatus {
            let h = self.tab;
            if self.plus == button_handle {
                if let Some(t) = self.control_mut(h) {
                    let idx = t.current_tab().unwrap_or(0);
                    t.set_current_tab(idx + 1);
                }
                return EventProcessStatus::Processed;
            }
            if self.minus == button_handle {
                if let Some(t) = self.control_mut(h) {
                    let idx = t.current_tab().unwrap_or(0);
                    t.set_current_tab(idx - 1);
                }
                return EventProcessStatus::Processed;
            }
            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xA8ADF7FC20FDF239)   
        Mouse.Click(6,1,left)
        Paint('2nd page')  
        CheckHash(0x490AC90BF6434117)
        Mouse.Click(6,1,left)
        Paint('3nd page')  
        CheckHash(0x533625E8A00AF0A7)
        Mouse.Click(6,1,left)
        Paint('4th page')  
        CheckHash(0xE4E96057767B106F)
        Mouse.Click(6,1,left)
        Paint('4th page (reached the limit) - now button > has the focus')  
        CheckHash(0xC44447C148857AD8)
        Mouse.Click(2,1,left)
        Paint('3rd page')  
        CheckHash(0x8DB5DEA9AA3BE6F7)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_tab_mouse_events() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xCA9EC0919796934C)
        Mouse.Move(38,3)
        Paint('Hover over third tab')
        CheckHash(0x63EE66308B70BFE8)
        Mouse.Move(60,1)
        Mouse.Move(60,3)
        Paint('Hover over tab bar')
        CheckHash(0xCA9EC0919796934C)
        Mouse.Move(60,1)
        Paint('Mouse leaving tab bar')
        CheckHash(0xCA9EC0919796934C)
        Mouse.Drag(23,3,38,3)
        Paint('Drag from second to third tab (second tab is selected)')
        CheckHash(0x8809C1E94AAD75C)
        Mouse.DoubleClick(12,3,left)
        Paint('Double click on first tab (second tab is selected, first tab is hovered)')
        CheckHash(0x8497EC7EABF633E8)
        Mouse.Wheel(22,3,left,1)
        Paint('Mouse wheel on second tab (no hover as we are over the second tab)')
        CheckHash(0x8809C1E94AAD75C)
        Mouse.Move(60,1)
        Mouse.Move(60,3)
        Mouse.Click(60,3,left)
        Paint('Click on tab bar (but not on a tab - second tab remains selected)')
        CheckHash(0x8809C1E94AAD75C)
    ";
    let mut a = App::debug(80, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:70,h:7,flags: Sizeable");
    let mut tab = Tab::new(layout!("l:0,t:0,r:0,b:0"), tab::Flags::None);
    
    // Add tabs with wider width to make empty space more obvious
    tab.add_tab("Page 1");
    tab.add_tab("Page 2");
    tab.add_tab("Page 3");
    
    w.add(tab);
    a.add_window(w);
    a.run();
}

