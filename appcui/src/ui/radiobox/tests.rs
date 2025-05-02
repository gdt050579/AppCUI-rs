use crate::prelude::*;

#[test]
fn check_radiobox_creation() {
    let script = "
        Paint.Enable(false)
        Paint('some radioboxes')   
        CheckHash(0xADBC43945D44CAEE)  
        CheckCursor(13,5) 
        Key.Pressed(Space)
        Paint('third radiobox checked')  
        CheckHash(0x349455E173D3F586) 
        CheckCursor(13,5) 
        Key.Pressed(Tab,2)
        Paint('second radiobox selected')  
        CheckHash(0xAA97B2213D654C16) 
        CheckCursor(13,4) 
        Key.Pressed(Space)
        Paint('second radiobox checked')  
        CheckHash(0x3BA39654D6BE3162)
        CheckCursor(13,4) 
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(RadioBox::new("Selection &One", Layout::new("x:1,y:1,w:20"), true));
    w.add(RadioBox::new("Selection &Two", Layout::new("x:1,y:2,w:40"), false));
    w.add(RadioBox::new("Selection T&hree", Layout::new("x:1,y:3,w:40"), false));

    a.add_window(w);
    a.run();
}

#[test]
fn check_radiobox_macro_creation() {
    let script = "
        Paint.Enable(false)
        Paint('some radioboxes')   
        CheckHash(0xADBC43945D44CAEE)  
        CheckCursor(13,5) 
        Key.Pressed(Space)
        Paint('third radiobox checked')  
        CheckHash(0x349455E173D3F586) 
        CheckCursor(13,5) 
        Key.Pressed(Tab,2)
        Paint('second radiobox selected')  
        CheckHash(0xAA97B2213D654C16) 
        CheckCursor(13,4) 
        Key.Pressed(Space)
        Paint('second radiobox checked')  
        CheckHash(0x3BA39654D6BE3162)
        CheckCursor(13,4) 
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(radiobox!("'Selection &One',x:1,y:1,w:20,select: true"));
    w.add(radiobox!("'Selection &Two',x:1,y:2,w:40"));
    w.add(radiobox!("caption:'Selection T&hree',x:1,y:3,w:40,selected: false"));

    a.add_window(w);
    a.run();
}

#[test]
fn check_shortkeys() {
    #[Window(events = RadioBoxEvents, internal: true)]
    struct MyWin {
        g1_r1: Handle<RadioBox>,
        g1_r2: Handle<RadioBox>,
        g1_r3: Handle<RadioBox>,
        g2_r1: Handle<RadioBox>,
        g2_r2: Handle<RadioBox>,
        g2_r3: Handle<RadioBox>,
        l: Handle<Label>,
    }

    impl MyWin {
        fn new() -> Self {
            let mut win = MyWin {
                base: window!("'My Win',d:c,w:60,h:14"),
                g1_r1: Handle::None,
                g1_r2: Handle::None,
                g1_r3: Handle::None,
                g2_r1: Handle::None,
                g2_r2: Handle::None,
                g2_r3: Handle::None,
                l: Handle::None,
            };
            win.l = win.add(label!("'<no status>',l:1,r:1,t:1"));
            let mut group_1 = panel!("'Group 1',x:1,y:3,w:26,h:7");
            win.g1_r1 = group_1.add(radiobox!("&Meters,x:1,y:1,w:20,select:true"));
            win.g1_r2 = group_1.add(radiobox!("&Centimeters,x:1,y:2,w:20"));
            win.g1_r3 = group_1.add(radiobox!("&Kilometers,x:1,y:3,w:20"));

            let mut group_2 = panel!("'Group 2',x:30,y:3,w:26,h:7");
            win.g2_r1 = group_2.add(radiobox!("&Red,x:1,y:1,w:20,select:true"));
            win.g2_r2 = group_2.add(radiobox!("&Green,x:1,y:2,w:20"));
            win.g2_r3 = group_2.add(radiobox!("&Blue,x:1,y:3,w:20"));

            win.add(group_1);
            win.add(group_2);
            win
        }
    }

    impl RadioBoxEvents for MyWin {
        fn on_selected(&mut self, handle: Handle<RadioBox>) -> EventProcessStatus {
            let mut s = String::new();
            if let Some(r) = self.control(handle) {
                s += r.caption();
            }
            if !s.is_empty() {
                let h = self.l;
                if let Some(l) = self.control_mut(h) {
                    l.set_caption(&s);
                }
            }
            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0xD7E773EB5F49A999)
        Paint('State_3')
        CheckHash(0xD7E773EB5F49A999)
        Mouse.Move(33,10)
        Paint('State_4')
        CheckHash(0x63BBFD755F4EFB45)
        Mouse.Move(26,10)
        Mouse.Hold(26,10,left)
        Paint('State_5')
        CheckHash(0x1F77A9D73501BD29)
        Mouse.Release(26,10,left)
        Paint('State_6')
        CheckHash(0x7315C053C705A14F)
        Mouse.Move(26,11)
        Paint('State_7')
        CheckHash(0x16E0BC6567A865A)
        Mouse.Move(25,11)
        Mouse.Hold(25,11,left)
        Paint('State_8')
        CheckHash(0xC7A8325AE08BA837)
        Mouse.Release(25,11,left)
        Paint('State_9')
        CheckHash(0xC7C6E08882B89070)
        Mouse.Move(27,10)
        Paint('State_10')
        CheckHash(0xB5C00D0B9C294FCC)
        Mouse.Move(34,10)
        Paint('State_11')
        CheckHash(0xC7C6E08882B89070)
        Mouse.Move(43,9)
        Paint('State_12')
        CheckHash(0x168B19668C630410)
        Mouse.Move(46,10)
        Paint('State_13')
        CheckHash(0x7E76ECACD7F93820)
        Mouse.Hold(46,10,left)
        Paint('State_14')
        CheckHash(0x5A3ECDF8F7E83148)
        Mouse.Release(46,10,left)
        Paint('State_15')
        CheckHash(0xF80CD34AE9A21C41)
        Mouse.Move(47,11)
        Paint('State_16')
        CheckHash(0x36FFC2CCA07864D4)
        Mouse.Move(48,11)
        Mouse.Hold(48,11,left)
        Paint('State_17')
        CheckHash(0x4184A9B25FF37279)
        Mouse.Release(48,11,left)
        Paint('State_18')
        CheckHash(0x744900DB66314C6F)
        Mouse.Move(72,11)
        Key.Pressed(Alt+M)
        Paint('State_19')
        CheckHash(0xA71DF7DA996F045D)
        Key.Pressed(Alt+R)
        Paint('State_20')
        CheckHash(0xC356E7EC1CB4240D)
        Key.Pressed(Alt+C)
        Paint('State_21')
        CheckHash(0x7315C053C705A14F)
        Key.Pressed(Alt+K)
        Paint('State_22')
        CheckHash(0xC7C6E08882B89070)
        Key.Pressed(Tab)
        Paint('State_23')
        CheckHash(0xA3EC30DFD690BEC8)
        Key.Pressed(Tab)
        Paint('State_24')
        CheckHash(0x5A3ECDF8F7E83148)
        Key.Pressed(Tab)
        Paint('State_25')
        CheckHash(0x90583D032CABF10)
        Key.Pressed(Tab)
        Paint('State_26')
        CheckHash(0xEFC782640FAFE5F0)
        Key.Pressed(Space)
        Paint('State_27')
        CheckHash(0xDB266D68A8C6C0FD)
        Key.Pressed(Tab)
        Paint('State_28')
        CheckHash(0x3927C193E03BD6A5)
        Key.Pressed(Tab)
        Paint('State_29')
        CheckHash(0x929B52F2AD98C46D)
        Key.Pressed(Space)
        Paint('State_30')
        CheckHash(0xC7C6E08882B89070)
        Key.Pressed(Tab)
        Paint('State_31')
        CheckHash(0xA3EC30DFD690BEC8)
        Key.Pressed(Tab)
        Paint('State_32')
        CheckHash(0x5A3ECDF8F7E83148)
        Key.Pressed(Space)
        Paint('State_33')
        CheckHash(0xF80CD34AE9A21C41)
        Key.Pressed(Tab)
        Paint('State_34')
        CheckHash(0x4184A9B25FF37279)
        Key.Pressed(Enter)
        Paint('State_35')
        CheckHash(0x744900DB66314C6F)
        Key.Pressed(Escape)
        Paint('State_36')
        CheckHash(0x483ede4555d977a5)    
    ";
    let mut a = App::debug(80, 20, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}
#[test]
fn check_radiobox_ascii_mode() {
    let script = "
        Paint.Enable(false)
        Paint('Ascii mode')   
        CheckHash(0x7ECA8EE81C4146D5)  
        CheckCursor(8,4)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:50,h:9"), window::Flags::None);
    w.add(radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,type=Ascii,select:false"));
    w.add(radiobox!("'Option 2 (selected)',x:1,y:2,w:40,type=Ascii,select:true"));
    w.add(radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,type=Ascii,select:false, enabled:false"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_radiobox_circle_mode() {
    let script = "
        Paint.Enable(false)
        Paint('Circle mode')   
        CheckHash(0xF471A26E9DA373E8)  
        CheckCursor(7,4)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:50,h:9"), window::Flags::None);
    w.add(radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,type=Circle,select:false"));
    w.add(radiobox!("'Option 2 (selected)',x:1,y:2,w:40,type=Circle,select:true"));
    w.add(radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,type=Circle,select:false, enabled:false"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_radiobox_diamond_mode() {
    let script = "
        Paint.Enable(false)
        Paint('Diamond mode')   
        CheckHash(0xAA65B0D6A6E73527)  
        CheckCursor(7,4)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:50,h:9"), window::Flags::None);
    w.add(radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,type=Diamond,select:false"));
    w.add(radiobox!("'Option 2 (selected)',x:1,y:2,w:40,type=Diamond,select:true"));
    w.add(radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,type=Diamond,select:false, enabled:false"));
    a.add_window(w);
    a.run();
}


#[test]
fn check_radiobox_is_selected() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xAA65B0D6A6E73527)  
        CheckCursor(7,4)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:50,h:9"), window::Flags::None);
    let r1 = radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,type=Diamond,select:false");
    let r2 = radiobox!("'Option 2 (selected)',x:1,y:2,w:40,type=Diamond,select:true");
    assert!(!r1.is_selected());
    assert!(r2.is_selected());
    w.add(r1);
    w.add(r2);
    w.add(radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,type=Diamond,select:false, enabled:false"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_radiobox_set_caption() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xF4D4B61AB48E7B81)  
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:50,h:9"), window::Flags::None);
    let mut rb = radiobox!("'x',x:1,y:1,w:40,type=Ascii,select:false");
    rb.set_caption("Test &caption");
    assert_eq!(rb.caption(), "Test caption");
    w.add(rb);
    a.add_window(w);
    a.run();
}


#[test]
fn check_radiobox_show_tooltip() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state => ( ) A real')   
        CheckHash(0xB4EB92A5F09397A6)  
        Mouse.Move(15,3)
        Paint('Tool tip shown');
        CheckHash(0xE5A899040E15763F)  
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:50,h:9"), window::Flags::None);
    w.add(radiobox!("'A really large text',x:1,y:1,w:10,type=Ascii,select:false"));
    a.add_window(w);
    a.run();
}