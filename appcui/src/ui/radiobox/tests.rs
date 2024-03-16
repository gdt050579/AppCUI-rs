use crate::prelude::*;

#[test]
fn check_radiobox_creation() {
    let script = "
        Paint.Enable(false)
        Paint('some radioboxes')   
        CheckHash(0xDCBAB7277D546411)  
        CheckCursor(13,5) 
        Key.Pressed(Space)
        Paint('third radiobox checked')  
        CheckHash(0x9896DC05AC121749) 
        CheckCursor(13,5) 
        Key.Pressed(Tab,2)
        Paint('second radiobox selected')  
        CheckHash(0x16DDCF9DB5BA6AB1) 
        CheckCursor(13,4) 
        Key.Pressed(Space)
        Paint('second radiobox checked')  
        CheckHash(0x640BD0AB0F16893D)
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
        CheckHash(0xDCBAB7277D546411)  
        CheckCursor(13,5) 
        Key.Pressed(Space)
        Paint('third radiobox checked')  
        CheckHash(0x9896DC05AC121749) 
        CheckCursor(13,5) 
        Key.Pressed(Tab,2)
        Paint('second radiobox selected')  
        CheckHash(0x16DDCF9DB5BA6AB1) 
        CheckCursor(13,4) 
        Key.Pressed(Space)
        Paint('second radiobox checked')  
        CheckHash(0x640BD0AB0F16893D)
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
            if let Some(r) = self.get_control(handle) {
                s += r.caption();
            }
            if s.len() > 0 {
                let h = self.l;
                if let Some(l) = self.get_control_mut(h) {
                    l.set_caption(&s);
                }
            }
            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0xdb01f50869759c19)
        Paint('State_3')
        CheckHash(0xdb01f50869759c19)
        Mouse.Move(33,10)
        Paint('State_4')
        CheckHash(0x8a0e0ef9f45e1535)
        Mouse.Move(26,10)
        Mouse.Hold(26,10,left)
        Paint('State_5')
        CheckHash(0x2130db6ff3a32fe9)
        Mouse.Release(26,10,left)
        Paint('State_6')
        CheckHash(0x6fdebf262a222867)
        Mouse.Move(26,11)
        Paint('State_7')
        CheckHash(0x339fee3bb426a662)
        Mouse.Move(25,11)
        Mouse.Hold(25,11,left)
        Paint('State_8')
        CheckHash(0xeedd9367b4fc8307)
        Mouse.Release(25,11,left)
        Paint('State_9')
        CheckHash(0x2f44f6fc465f5c10)
        Mouse.Move(27,10)
        Paint('State_10')
        CheckHash(0xfda5250f7bc1c2e4)
        Mouse.Move(34,10)
        Paint('State_11')
        CheckHash(0x2f44f6fc465f5c10)
        Mouse.Move(43,9)
        Paint('State_12')
        CheckHash(0xd2dd420faf38fe35)
        Mouse.Move(46,10)
        Paint('State_13')
        CheckHash(0xc0ac23c798fbc058)
        Mouse.Hold(46,10,left)
        Paint('State_14')
        CheckHash(0x23f121b699896710)
        Mouse.Release(46,10,left)
        Paint('State_15')
        CheckHash(0x266e830fde0ef789)
        Mouse.Move(47,11)
        Paint('State_16')
        CheckHash(0xdf9e2a313b178c64)
        Mouse.Move(48,11)
        Mouse.Hold(48,11,left)
        Paint('State_17')
        CheckHash(0xca7c0e45efb53ed9)
        Mouse.Release(48,11,left)
        Paint('State_18')
        CheckHash(0x2862c921e66e39ff)
        Mouse.Move(72,11)
        Key.Pressed(Alt+M)
        Paint('State_19')
        CheckHash(0xf6dfe0a9d05da72d)
        Key.Pressed(Alt+R)
        Paint('State_20')
        CheckHash(0x3c1e63362f51205)
        Key.Pressed(Alt+C)
        Paint('State_21')
        CheckHash(0x6fdebf262a222867)
        Key.Pressed(Alt+K)
        Paint('State_22')
        CheckHash(0x2f44f6fc465f5c10)
        Key.Pressed(Tab)
        Paint('State_23')
        CheckHash(0x38707c96cbad29d8)
        Key.Pressed(Tab)
        Paint('State_24')
        CheckHash(0x23f121b699896710)
        Key.Pressed(Tab)
        Paint('State_25')
        CheckHash(0xb372411a55203be8)
        Key.Pressed(Tab)
        Paint('State_26')
        CheckHash(0xd4e833e75ee28d18)
        Key.Pressed(Space)
        Paint('State_27')
        CheckHash(0x8696b1d76b96b445)
        Key.Pressed(Tab)
        Paint('State_28')
        CheckHash(0xe726f9d5b0e4cae5)
        Key.Pressed(Tab)
        Paint('State_29')
        CheckHash(0xc4c52ff260a018ad)
        Key.Pressed(Space)
        Paint('State_30')
        CheckHash(0x2f44f6fc465f5c10)
        Key.Pressed(Tab)
        Paint('State_31')
        CheckHash(0x38707c96cbad29d8)
        Key.Pressed(Tab)
        Paint('State_32')
        CheckHash(0x23f121b699896710)
        Key.Pressed(Space)
        Paint('State_33')
        CheckHash(0x266e830fde0ef789)
        Key.Pressed(Tab)
        Paint('State_34')
        CheckHash(0xca7c0e45efb53ed9)
        Key.Pressed(Enter)
        Paint('State_35')
        CheckHash(0x2862c921e66e39ff)
        Key.Pressed(Escape)
        Paint('State_36')
        CheckHash(0x483ede4555d977a5)    
    ";
    let mut a = App::debug(80, 20, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}
