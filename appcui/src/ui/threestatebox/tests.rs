use crate::prelude::*;

#[test]
fn check_three_state_box_simple() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xD52344AB78EF21D2)   
    ";
    let mut a = App::debug(80, 13, script).build().unwrap();
    let mut w = window!("Title,a:c,w:78,h:11");
    w.add(ThreeStateBox::new(
        "Options",
        Layout::new("x:1,y:1,w:16,h:4"),
        threestatebox::State::Unchecked,
    ));
    w.add(ThreeStateBox::new(
        "Options",
        Layout::new("x:1,y:2,w:16,h:4"),
        threestatebox::State::Checked,
    ));
    w.add(ThreeStateBox::new(
        "Options",
        Layout::new("x:1,y:3,w:16,h:4"),
        threestatebox::State::Unknown,
    ));
    a.add_window(w);
    a.run();
}

#[test]
fn check_three_state_box_macro() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xC4D48739A6B596DE)   
    ";
    let mut a = App::debug(80, 13, script).build().unwrap();
    let mut w = window!("Title,a:c,w:78,h:11");
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
            base:window!("title,a:c,w:40,h:8"),
            ts: Handle::None,
            lb:Handle::None,
        };
        w.lb = w.add(label!("xyz,x:1,y:1,w:30"));
        w.ts = w.add(ThreeStateBox::new("smth", Layout::new("x:1,y:2,w:30"), threestatebox::State::Unknown));
        w
    }
}

impl ThreeStateBoxEvents for MyWindow {
    fn on_status_changed(&mut self, _handle: Handle<ThreeStateBox>, state: threestatebox::State) -> EventProcessStatus {
        let s = match state {
            threestatebox::State::Checked => "check",
            threestatebox::State::Unchecked => "uncheck",
            threestatebox::State::Unknown => "unknown",
        };
        let h = self.lb;
        if let Some(label) = self.control_mut(h) {
            label.set_caption(s);
        }

        EventProcessStatus::Processed
    }
}
    let script = "
        Paint.Enable(false)
        Paint('InitialState (unknown)')   
        CheckHash(0x8EAE904E2D1BDB2D)
        Key.Pressed(Enter)
        Paint('Should be checked')   
        CheckHash(0x1E3C577A71C4F2C7)
        Key.Pressed(Space)
        Paint('Should be unchecked')  
        CheckHash(0xABB26831C7D2F752) 
        Mouse.Click(23,5,left)
        Paint('Should be unknown')  
        CheckHash(0x271B06247758B2FA) 
    ";
    let mut a = App::debug(80, 13, script).build().unwrap();
    a.add_window(MyWindow::new());
    a.run();
}

#[test]
fn check_methods() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0x3FA2ED544BE2898A)   
    ";
    let mut a = App::debug(80, 13, script).build().unwrap();
    let mut w = window!("Title,a:c,w:78,h:11");
    let mut t = threestatebox!("Options,x:1,y:1,w:16,h:4,state=unchecked");
    assert_eq!(t.caption(),"Options");
    assert_eq!(t.state(),threestatebox::State::Unchecked);
    t.set_caption("New caption");
    assert_eq!(t.caption(),"New caption");
    t.set_state(threestatebox::State::Unknown);
    assert_eq!(t.state(),threestatebox::State::Unknown);
    w.add(t);
    a.add_window(w);
    a.run();
}

#[test]
fn check_threestatebox_plusminus_mode() {
    let script = "
        Paint.Enable(false)
        Paint('Mode: PlusMinus')   
        CheckHash(0xFB01E18C872A6FAC)  
        CheckCursor(7,7)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:50,h:10"), window::Flags::None);
    w.add(threestatebox!("'Option 1 (not-checked)',x:1,y:1,w:40,type=PlusMinus,state=Unchecked"));
    w.add(threestatebox!("'Option 2 (checked)',x:1,y:2,w:40,type=PlusMinus,state=Checked"));
    w.add(threestatebox!("'Option 3 (disabled and not-checked)',x:1,y:3,w:40,type=PlusMinus,state:Unchecked, enabled:false"));
    w.add(threestatebox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=PlusMinus,state=Checked, enabled:false"));
    w.add(threestatebox!("'Option 5 (unknown)',x:1,y:5,w:40,type=PlusMinus,state=unknown"));
    w.add(threestatebox!("'Option 6 (disabled and unknown)',x:1,y:6,w:40,type=PlusMinus,state=unknown, enabled:false"));
    a.add_window(w);
    a.run();
}


#[test]
fn check_threestatebox_ascii_mode() {
    let script = "
        Paint.Enable(false)
        Paint('Mode: Ascii')   
        CheckHash(0xD0B1AF4745F5E714)  
        CheckCursor(8,7)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:50,h:10"), window::Flags::None);
    w.add(threestatebox!("'Option 1 (not-checked)',x:1,y:1,w:40,type=Ascii,state=Unchecked"));
    w.add(threestatebox!("'Option 2 (checked)',x:1,y:2,w:40,type=Ascii,state=Checked"));
    w.add(threestatebox!("'Option 3 (disabled and not-checked)',x:1,y:3,w:40,type=Ascii,state:Unchecked, enabled:false"));
    w.add(threestatebox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=Ascii,state=Checked, enabled:false"));
    w.add(threestatebox!("'Option 5 (unknown)',x:1,y:5,w:40,type=Ascii,state=unknown"));
    w.add(threestatebox!("'Option 6 (disabled and unknown)',x:1,y:6,w:40,type=Ascii,state=unknown, enabled:false"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_threestatebox_checkbox_mode() {
    let script = "
        Paint.Enable(false)
        Paint('Mode: Checkbox')   
        CheckHash(0x5E5E498E662812C4)  
        CheckCursor(7,7)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:50,h:10"), window::Flags::None);
    w.add(threestatebox!("'Option 1 (not-checked)',x:1,y:1,w:40,type=Checkbox,state=Unchecked"));
    w.add(threestatebox!("'Option 2 (checked)',x:1,y:2,w:40,type=Checkbox,state=Checked"));
    w.add(threestatebox!("'Option 3 (disabled and not-checked)',x:1,y:3,w:40,type=Checkbox,state:Unchecked, enabled:false"));
    w.add(threestatebox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=Checkbox,state=Checked, enabled:false"));
    w.add(threestatebox!("'Option 5 (unknown)',x:1,y:5,w:40,type=Checkbox,state=unknown"));
    w.add(threestatebox!("'Option 6 (disabled and unknown)',x:1,y:6,w:40,type=Checkbox,state=unknown, enabled:false"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_threestatebox_checkmark_mode() {
    let script = "
        Paint.Enable(false)
        Paint('Mode: Checkmark')   
        CheckHash(0xE0405A261573BF34)  
        CheckCursor(7,7)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:50,h:10"), window::Flags::None);
    w.add(threestatebox!("'Option 1 (not-checked)',x:1,y:1,w:40,type=Checkmark,state=Unchecked"));
    w.add(threestatebox!("'Option 2 (checked)',x:1,y:2,w:40,type=Checkmark,state=Checked"));
    w.add(threestatebox!("'Option 3 (disabled and not-checked)',x:1,y:3,w:40,type=Checkmark,state:Unchecked, enabled:false"));
    w.add(threestatebox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=Checkmark,state=Checked, enabled:false"));
    w.add(threestatebox!("'Option 5 (unknown)',x:1,y:5,w:40,type=Checkmark,state=unknown"));
    w.add(threestatebox!("'Option 6 (disabled and unknown)',x:1,y:6,w:40,type=Checkmark,state=unknown, enabled:false"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_threestatebox_fill_mode() {
    let script = "
        Paint.Enable(false)
        Paint('Mode: FilledBox')   
        CheckHash(0x328C09CF38889778)  
        CheckCursor(7,7)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:50,h:10"), window::Flags::None);
    w.add(threestatebox!("'Option 1 (not-checked)',x:1,y:1,w:40,type=FilledBox,state=Unchecked"));
    w.add(threestatebox!("'Option 2 (checked)',x:1,y:2,w:40,type=FilledBox,state=Checked"));
    w.add(threestatebox!("'Option 3 (disabled and not-checked)',x:1,y:3,w:40,type=FilledBox,state:Unchecked, enabled:false"));
    w.add(threestatebox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=FilledBox,state=Checked, enabled:false"));
    w.add(threestatebox!("'Option 5 (unknown)',x:1,y:5,w:40,type=FilledBox,state=unknown"));
    w.add(threestatebox!("'Option 6 (disabled and unknown)',x:1,y:6,w:40,type=FilledBox,state=unknown, enabled:false"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_threestatebox_yes_no_mode() {
    let script = "
        Paint.Enable(false)
        Paint('Mode: YesNo')   
        CheckHash(0xB7778A06B2B9C2B4)  
        CheckCursor(8,7)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:50,h:10"), window::Flags::None);
    w.add(threestatebox!("'Option 1 (not-checked)',x:1,y:1,w:40,type=YesNo,state=Unchecked"));
    w.add(threestatebox!("'Option 2 (checked)',x:1,y:2,w:40,type=YesNo,state=Checked"));
    w.add(threestatebox!("'Option 3 (disabled and not-checked)',x:1,y:3,w:40,type=YesNo,state:Unchecked, enabled:false"));
    w.add(threestatebox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=YesNo,state=Checked, enabled:false"));
    w.add(threestatebox!("'Option 5 (unknown)',x:1,y:5,w:40,type=YesNo,state=unknown"));
    w.add(threestatebox!("'Option 6 (disabled and unknown)',x:1,y:6,w:40,type=YesNo,state=unknown, enabled:false"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_three_state_box_hot_key() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xC8C53061F55AA476)   
    ";
    let mut a = App::debug(80, 13, script).build().unwrap();
    let mut w = window!("Title,a:c,w:78,h:11");
    w.add(threestatebox!("&Options,x:1,y:1,w:16,h:4,state=unchecked"));
    w.add(threestatebox!("O&ptions,x:1,y:2,w:16,h:4,state=checked"));
    w.add(threestatebox!("Op&tions,x:1,y:3,w:16,h:4,state=unknown"));
    w.add(threestatebox!("Opt&ions,x:1,y:4,w:16,h:4"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_events() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')   
        CheckHash(0xB6BB128869977206)   
        Mouse.Move(10,3)
        Paint('2. Mouse Over')   
        CheckHash(0x1DEC17777C0BBD81)
        Mouse.Move(0,0)   
        Paint('3. Mouse Exit')   
        CheckHash(0xB6BB128869977206)
        Key.Pressed(F1)
        Paint('4. Nothing happens - the key is not processed')   
        CheckHash(0xB6BB128869977206)
    ";
    let mut a = App::debug(80, 13, script).build().unwrap();
    let mut w = window!("Title,a:c,w:78,h:11");
    w.add(threestatebox!("'&Options with a lot of text',x:1,y:1,w:16,h:4,state=unchecked"));
    a.add_window(w);
    a.run();
}