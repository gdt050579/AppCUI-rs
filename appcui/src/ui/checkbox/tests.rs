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
            if let Some(label) = self.get_control_mut(h) {
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
