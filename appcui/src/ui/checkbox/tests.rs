use crate::prelude::*;

#[test]
fn check_checkbox_creation() {
    let script = "
        Paint.Enable(false)
        Paint('some checkboxes')   
        CheckHash(0x9F68D7DDA0CF6014)  
        CheckCursor(13,5) 
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(CheckBox::new("Single line", Layout::new("x:1,y:1,w:20"), false));
    w.add(CheckBox::new("Single line (checked initially)", Layout::new("x:1,y:2,w:40"), true));
    w.add(CheckBox::new(
        "A &multi line checkbox that can be checked or not",
        Layout::new("x:1,y:3,w:20,h:3"),
        true,
    ));

    a.add_window(w);
    a.run();
}

#[test]
fn check_checkbox_events() {
    #[Window(events = CheckBoxEvents, internal = true)]
    struct MyWin {
        c1: Handle<CheckBox>,
        c2: Handle<CheckBox>,
        c3: Handle<CheckBox>,
        lb: Handle<Label>,
    }

    impl MyWin {
        fn new() -> Self {
            let mut win = MyWin {
                base: window!("Checkboxes,d:c,w:40,h:10"),
                c1: Handle::None,
                c2: Handle::None,
                c3: Handle::None,
                lb: Handle::None,
            };
            win.c1 = win.add(checkbox!("'A &Single line checkbox',x:2,y:2,w:40"));
            win.c2 = win.add(checkbox!("Caption='&Inactive checkbox',x:2,y:3,w:15,enabled:false"));
            win.c3 = win.add(checkbox!("Text='A &multi line checkbox that is enabled',x:2,y:4,w:20,h:3,checked:true"));
            win.lb = win.add(Label::new("", Layout::new("x:2,y:0,w:35")));
            win
        }
        fn set_label_text(&mut self, txt: &str) {
            let h = self.lb;
            if let Some(label) = self.control_mut(h) {
                label.set_caption(txt);
            }
        }
    }

    impl CheckBoxEvents for MyWin {
        fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
            let id = match () {
                _ if handle == self.c1 => 1,
                _ if handle == self.c2 => 2,
                _ if handle == self.c3 => 3,
                _ => 0,
            };
            self.set_label_text(format!("Checkbox {} check status: {}", id, checked).as_str());
            EventProcessStatus::Processed
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0xC20CC4F5C86E807C)  
        CheckCursor(14,6) 
        Key.Pressed(Tab)
        Key.Pressed(Space)
        // the label should say: Checkbox 1 check status: true
        Paint('1st checkbox checked')
        CheckHash(0x4C5E4A9F456071D)  
        CheckCursor(14,4) 
        Key.Pressed(Alt+M)
        // the label should say:  Checkbox 3 check status: false
        Paint('3st checkbox un-checked')
        CheckHash(0x627909865CF67619)  
        CheckCursor(14,6) 
        Key.Pressed(Tab,2)
        Key.Pressed(Enter)
        // the label should say: Checkbox 3 check status: true
        Paint('3st checkbox checked')
        CheckHash(0x85943026144CC0B3)  
        CheckCursor(14,6) 
        Key.Pressed(Alt+S)
        Mouse.Move(20,6)
        Paint('Hover on 3rd checkbox')
        CheckHash(0x9C2A13455CBA7587)  
        Mouse.Click(20,6,left)
        Paint('3rd checkbox clicked')
        CheckHash(0x55B82C4FACD71F83)  
        CheckCursor(14,6) 
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}


#[test]
fn check_checkbox_set_checked() {
    #[Window(events = ButtonEvents, internal=true)]
    struct MyWin {
        b: Handle<Button>,
        c: Handle<CheckBox>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: window!("Win-1,d:c,w:40,h:8"), 
                b: Handle::None,
                c: Handle::None,
            };
            me.b = me.add(button!("'Enable/Disable',l:1,r:1,b:1"));
            me.c = me.add(checkbox!("'Inactive checkbox',l:1,r:1,t:1,enabled:false"));
            me
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
            let handle = self.c;
            let c = self.control_mut(handle).unwrap();
            c.set_checked(!c.is_checked());
            EventProcessStatus::Processed
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0x4AAF8B869A8E1A2A)
        Key.Pressed(Tab)
        Key.Pressed(Space)
        Paint('now checked')  
        CheckHash(0x1450786DA8CA5E16) 
        Mouse.Move(30,5)
        Mouse.Click(30,5,left)
        Paint('now un-checked')  
        CheckHash(0x4AAF8B869A8E1A2A) 
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run(); 
}

#[test]
fn check_checkbox_mouse_click() {
    let script = "
        Paint.Enable(false)
        Paint('some checkboxes')   
        CheckHash(0x2B9842F2F866679D)  
        CheckCursor(13,3) 
        Mouse.Click(20,3,left)
        Paint('checkbox-1 checked')  
        CheckHash(0x361D0DE3AC431283)   
        Mouse.Click(20,3,left)
        Paint('checkbox-1 un-checked')  
        CheckHash(0x2B9842F2F866679D) 
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    w.add(CheckBox::new("Single line", Layout::new("x:1,y:1,w:20"), false));
    a.add_window(w);
    a.run();
}


#[test]
fn check_checkbox_key_pressed() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x26DF8C4F9A5A4F70)  
        Key.Pressed(Space)
        Paint('checkbox-1 un-checked')  
        CheckHash(0x46C11E31F0B114B6)  
        Key.Pressed(Space)
        Paint('checkbox-1 checked') 
        CheckHash(0x26DF8C4F9A5A4F70)  
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:11");
    w.add(checkbox!("'option outside panel',x:1,y:8,w:35,checked:true"));
    a.add_window(w);
    a.run();
}
