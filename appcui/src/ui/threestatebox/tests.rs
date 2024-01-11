use crate::prelude::*;

#[test]
fn check_three_state_box_simple() {
    let script = "
        Paint.Enable(false)
        Paint('Border panel')   
        CheckHash(0xFDADC2B0D10B716)   
    ";
    let mut a = App::debug(80, 13, script).build().unwrap();
    let mut w = window!("Title,d:c,w:78,h:11");
    w.add(ThreeStateBox::new(
        "Options",
        Layout::new("x:1,y:1,w:16,h:4"),
        threestatebox::threestatebox::ThreeStateBoxSelection::Unchecked,
    ));
    w.add(ThreeStateBox::new(
        "Options",
        Layout::new("x:1,y:2,w:16,h:4"),
        threestatebox::threestatebox::ThreeStateBoxSelection::Checked,
    ));
    w.add(ThreeStateBox::new(
        "Options",
        Layout::new("x:1,y:3,w:16,h:4"),
        threestatebox::threestatebox::ThreeStateBoxSelection::Unknown,
    ));
    a.add_window(w);
    a.run();
}

#[test]
fn check_three_state_box_macro() {
    let script = "
        Paint.Enable(false)
        Paint('Border panel')   
        CheckHash(0xD306E351C22E772D)   
    ";
    let mut a = App::debug(80, 13, script).build().unwrap();
    let mut w = window!("Title,d:c,w:78,h:11");
    w.add(threestatebox!("Options,x:1,y:1,w:16,h:4,state=unchecked"));
    w.add(threestatebox!("Options,x:1,y:2,w:16,h:4,state=checked"));
    w.add(threestatebox!("Options,x:1,y:3,w:16,h:4,state=unknown"));
    w.add(threestatebox!("Options,x:1,y:4,w:16,h:4"));
    a.add_window(w);
    a.run();
}


#[test]
fn check_three_state_box_events() {
    #[Window(events=ThreeStateBoxEvents,internal=true)]
    struct MyWindow {
        ts: Handle<ThreeStateBox>,
        lb: Handle<Label>,
    }
impl MyWindow {
    fn new() -> MyWindow {
        let mut w = MyWindow{
            base:window!("title,d:c,w:40,h:8"),
            ts: Handle::None,
            lb:Handle::None,
        };
        w.lb = w.add(label!("xyz,x:1,y:1,w:30"));
        w.ts = w.add(ThreeStateBox::new("smth", Layout::new("x:1,y:2,w:30"), threestatebox::ThreeStateBoxSelection::Unknown));
        w
    }
}

impl ThreeStateBoxEvents for MyWindow {
    fn on_status_changed(&mut self, _handle: Handle<ThreeStateBox>, state: threestatebox::ThreeStateBoxSelection) -> EventProcessStatus {
        let s = match state {
            threestatebox::ThreeStateBoxSelection::Checked => "check",
            threestatebox::ThreeStateBoxSelection::Unchecked => "uncheck",
            threestatebox::ThreeStateBoxSelection::Unknown => "unknown",
        };
        let h = self.lb;
        if let Some(label) = self.get_control_mut(h) {
            label.set_caption(s);
        }

        EventProcessStatus::Processed
    }
}
    let script = "
        Paint.Enable(false)
        Paint('InitialState')   
        CheckHash(0x2F49AA1C9B3366DA)
        Key.Pressed(Enter)
        Paint('Should be checked')   
        CheckHash(0x8B9C885B114D3E08)
        Key.Pressed(Space)
        Paint('Should be unchecked')  
        CheckHash(0x1AAC5A7A4743F4B6) 
        Mouse.Click(23,5,left)
        Paint('Should be unknown')  
        CheckHash(0x2672FD6E1578804D) 
    ";
    let mut a = App::debug(80, 13, script).build().unwrap();
    a.add_window(MyWindow::new());
    a.run();
}
