use crate::prelude::*;

#[test]
fn check_behavior() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x7E4294B3BDFB617F)
        CheckCursor(13,3)
        Key.TypeText(mypass)
        Paint('mypass password typed')
        CheckHash(0x1D678EBEA102AF7F)
        CheckCursor(19,3)
        Mouse.Move(20,3)
        Paint('mouse hover - tooltip visible')
        CheckHash(0xB170FB8F3BA64841)
        Mouse.Hold(20,3,left)
        Paint('Show password, no tooltip')
        CheckHash(0x7FBDE9790EAFF51C)
        Mouse.Release(20,3,left)
        Paint('Pass is hidden again, no tooltip')
        CheckHash(0x1D678EBEA102AF7F)
        Key.Pressed(Backspace,3)
        Paint('delete last 3 chars')
        CheckHash(0x4DDE3EBA59D1D595)
        Mouse.Hold(20,3,left)
        Paint('Show password (myp), no tooltip')
        CheckHash(0x2205012F056CA905)
        Mouse.Release(20,3,left)
        Paint('Hide password')
        CheckHash(0x4DDE3EBA59D1D595)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
    w.add(Password::new(Layout::new("x:1,y:1,w:30")));

    a.add_window(w);
    a.run();
}

#[test]
fn check_macro() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0xA761B1792C262886)
        CheckCursor(16,5)
        Mouse.Move(20,5)
        Mouse.Hold(20,5,left)
        Paint('Pass: abc visible')
        CheckHash(0xBA3DB90189C89092)
        Mouse.Release(20,5,left)
        Paint('pass hidden')
        CheckHash(0xA761B1792C262886)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
    w.add(password!("x:1,y:1,w:30"));
    w.add(password!("x:1,y:3,w:30,pass:123"));

    a.add_window(w);
    a.run();
}

#[test]
fn check_events() {
    #[Window(events = PasswordEvents, internal=true)]
    struct MyWin {
        info: Handle<Label>,
        pass: Handle<Password>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win-1", Layout::new("a:c,w:47,h:7"), window::Flags::None),
                info: Handle::None,
                pass: Handle::None,
            };
            me.info = me.add(Label::new("<none>", Layout::new("x:0,y:0,w:35")));
            me.pass = me.add(password!("x:1,y:2,w:30"));
            me
        }
        fn set_info(&mut self, txt: &str) {
            let h_label = self.info;
            if let Some(label) = self.control_mut(h_label) {
                label.set_caption(txt);
            }
        }
    }
    impl PasswordEvents for MyWin {
        fn on_accept(&mut self, handle: Handle<Password>) -> EventProcessStatus {
            if handle == self.pass {
                if let Some(p) = self.control(handle) {
                    self.set_info(format!("Acepted: {}",p.password()).as_str());
                    return EventProcessStatus::Processed;
                }
            }
            EventProcessStatus::Ignored
        }
    
        fn on_cancel(&mut self, handle: Handle<Password>) -> EventProcessStatus {
            if handle == self.pass {
                if let Some(p) = self.control(handle) {
                    self.set_info(format!("Canceled: {}",p.password()).as_str());                    
                }
                if let Some(p) = self.control_mut(handle) {
                    p.set_password("");
                    return EventProcessStatus::Processed;
                }
            }
            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0xEAA261A438D9EC3D)
        Key.TypeText(admin1234)
        Paint('password written')
        CheckHash(0x2DCA169E5EAD70E7)
        CheckCursor(19,5)
        Key.Pressed(Enter)
        Paint('Accepted')
        CheckHash(0xD99A474066933448)
        Key.Pressed(Escape)
        Paint('Cancel and empty password')
        CheckHash(0x205EBB9FC7AB10A0)
        CheckCursor(10,5)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_enter_leave_control() {
    let script = "
        Paint.Enable(false)
        Paint('1. initial state')   
        CheckHash(0xE7D1F03B892A5765)
        Mouse.Move(27,5)
        Paint('2. Mouse over - tooltip visible')   
        CheckHash(0x68C60E61C9E69834)
        Mouse.Move(27,3)
        Paint('3. Mouse left - tooltip in-visible')   
        CheckHash(0xE7D1F03B892A5765)
        Mouse.Wheel(27,5,left,1)
        Paint('4. Mouse wheel - nothing happen (same hash as step 2)')   
        CheckHash(0x68C60E61C9E69834)
        Mouse.DoubleClick(27,5,left)
        Paint('5. Mouse double-click - tooltip in-visible')   
        CheckHash(0xE7D1F03B892A5765)        
        Mouse.Drag(27,5,30,5)
        Paint('6. Mouse drag - nothing happen (same hash as step 5)')   
        CheckHash(0xE7D1F03B892A5765)        
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
    w.add(password!("x:1,y:3,w:30,pass:123"));

    a.add_window(w);
    a.run();
}

#[test]
fn check_invalid_keys() {
    let script = "
        Paint.Enable(false)
        Paint('1. initial state')   
        CheckHash(0x4EEF18715284B1DF)  
        Key.Pressed(F10)
        Paint('2. F10 pressed - nothing happen')   
        CheckHash(0x4EEF18715284B1DF)  
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
    // width is 5 so that the control will be resized
    // the password is 1234567890, so the control will recompute the visible with on paint
    w.add(password!("x:1,y:3,w:5,pass:1234567890"));

    a.add_window(w);
    a.run();
}

#[test]
fn check_no_events_implementation() {
    let script = "
        Paint.Enable(false)
        Paint('1. initial state')   
        CheckHash(0xE7D1F03B892A5765)
        Key.Pressed(Enter)
        Paint('2. Enter pressed - nothing happen (the on_enter event is not implemented)')    
        CheckHash(0xE7D1F03B892A5765)
        Key.Pressed(Escape)
        Paint('3. Escape pressed - nothing happen (the on_cancel event is not implemented)')    
        CheckHash(0xE7D1F03B892A5765)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
    w.add(password!("x:1,y:3,w:30,pass:123"));

    a.add_window(w);
    a.run();
}