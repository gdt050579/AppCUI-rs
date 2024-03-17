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
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
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
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
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
                base: Window::new("Win-1", Layout::new("d:c,w:47,h:7"), window::Flags::None),
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