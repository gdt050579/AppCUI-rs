use crate::prelude::*;

#[test]
fn check_checkbox_creation() {
    let script = "
        Paint.Enable(false)
        Paint('some checkboxes')   
        CheckHash(0xAA393B73091205A4)  
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
        CheckHash(0x72AB7B1C9D82138F)  
        CheckCursor(14,6) 
        Key.Pressed(Tab)
        Key.Pressed(Space)
        // the label should say: Checkbox 1 check status: true
        Paint('1st checkbox checked')
        CheckHash(0x3362FB928BD2D9E1)  
        CheckCursor(14,4) 
        Key.Pressed(Alt+M)
        // the label should say:  Checkbox 3 check status: false
        Paint('3st checkbox un-checked')
        CheckHash(0xC8621B6DA2CF9B2A)  
        CheckCursor(14,6) 
        Key.Pressed(Tab,2)
        Key.Pressed(Enter)
        // the label should say: Checkbox 3 check status: true
        Paint('3st checkbox checked')
        CheckHash(0x7AA563D9C8C952BF)  
        CheckCursor(14,6) 
        Key.Pressed(Alt+S)
        Mouse.Move(20,6)
        Paint('Hover on 3rd checkbox')
        CheckHash(0xCBC5502338859BBD)  
        Mouse.Click(20,6,left)
        Paint('3rd checkbox clicked')
        CheckHash(0x6EC587D729BC484B)  
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
        CheckHash(0x4513048D3C5B3BE2)
        Key.Pressed(Tab)
        Key.Pressed(Space)
        Paint('now checked')  
        CheckHash(0x53F15D136C5350FE) 
        Mouse.Move(30,5)
        Mouse.Click(30,5,left)
        Paint('now un-checked')  
        CheckHash(0x4513048D3C5B3BE2) 
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
        CheckHash(0xAC58A676F4FD3979)  
        CheckCursor(13,3) 
        Mouse.Click(20,3,left)
        Paint('checkbox-1 checked')  
        CheckHash(0x274A7809D33F85E4)   
        Mouse.Click(20,3,left)
        Paint('checkbox-1 un-checked')  
        CheckHash(0xAC58A676F4FD3979) 
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
        //Paint.Enable(false)
        Error.Disable(true)
        Paint('initial state')   
        CheckHash(0xA6C95C5C0E59C1BF)  
        Key.Pressed(Space)
        Paint('checkbox-1 un-checked')  
        CheckHash(0x4225719E0F724212)  
        Key.Pressed(Space)
        Paint('checkbox-1 checked') 
        CheckHash(0xA6C95C5C0E59C1BF)  
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:11");
    w.add(checkbox!("'option outside panel',x:1,y:8,w:35,checked:true"));
    a.add_window(w);
    a.run();
}
