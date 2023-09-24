use crate::prelude::*;
#[test]
fn check_button_control() {
    #[Window(events = ButtonEvents, internal=true)]
    struct MyWin {
        info: Handle<Label>,
        but1: Handle<Button>,
        but2: Handle<Button>,
        but3: Handle<Button>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win-1", Layout::new("d:c,w:47,h:7"), window::Flags::None),
                info: Handle::None,
                but1: Handle::None,
                but2: Handle::None,
                but3: Handle::None,
            };
            me.info = me.add(Label::new("<none>", Layout::new("x:0,y:0,w:35")));
            me.but1 = me.add(Button::new("Button &1", Layout::new("x:1,y:3,w:13"), button::Flags::None));
            me.but2 = me.add(Button::new("Button &2", Layout::new("x:16,y:3,w:13"), button::Flags::None));
            let mut b3 = Button::new("Button &3", Layout::new("x:31,y:3,w:13"), button::Flags::None);
            b3.set_enabled(false);
            me.but3 = me.add(b3);
            me
        }
        fn set_info(&mut self, txt: &str) {
            let h_label = self.info;
            if let Some(label) = self.get_control_mut(h_label) {
                label.set_text(txt);
            }
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, button_handle: Handle<Button>) -> EventProcessStatus {
            if self.but1 == button_handle {
                self.set_info("Button 1 presed");
                return EventProcessStatus::Processed;
            }
            if self.but2 == button_handle {
                self.set_info("Button 2 pressed");
                return EventProcessStatus::Processed;
            }
            return EventProcessStatus::Ignored;
        }
    }

    let script = "
        //Paint.Enable(false)
        Paint('Button 2 has focus (default)')   
        CheckHash(0x2D08048B47432DA)   
        Key.Pressed(Tab)
        Paint('Button 1 has focus (default)') 
        CheckHash(0x4AC33C576575FD4E) 
        Key.Pressed(Enter)
        Paint('After first button was pressed')
        CheckHash(0xC4D9433CFFBF4A67) 
        Mouse.Move(30,6)
        Paint('Button 2 is hovered')
        CheckHash(0xA477C84D39481B3E) 
        Mouse.Click(30,6,left)
        Paint('Second button was pressed')
        CheckHash(0x30D90A1046C4AC48)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_button_control_2() {
    #[Window(events = ButtonEvents, internal=true)]
    struct MyWin {
        add: Handle<Button>,
        reset: Handle<Button>,
        counter: i32,
    }

    impl MyWin {
        fn new() -> Self {
            let mut win = MyWin {
                base: Window::new("My Win", Layout::new("d:c,w:40,h:6"), window::Flags::None),
                add: Handle::None,
                reset: Handle::None,
                counter: 0,
            };
            win.add = win.add(Button::new("Add (0)", Layout::new("x:25%,y:2,w:13,a:c"), button::Flags::None));
            win.reset = win.add(Button::new("Reset", Layout::new("x:75%,y:2,w:13,a:c",), button::Flags::None));
            win
        }
        fn update_add_button(&mut self) {
            let h = self.add;
            let new_text = format!("Add ({})",self.counter);
            if let Some(button) = self.get_control_mut(h) {
                button.set_text(new_text.as_str());
            }
        }
    }

    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, button_handle: Handle<Button>) -> EventProcessStatus {
            if button_handle == self.add {
                self.counter+=1;
                self.update_add_button();
                return EventProcessStatus::Processed;
            }
            if button_handle == self.reset {
                self.counter = 0;
                self.update_add_button();
                return EventProcessStatus::Processed;
            }
            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial focus => Add (0) and Reset')   
        CheckHash(0xBAE7A3089D249D76) 
        Mouse.Click(20,4,left) 
        Paint('Add (1) and Reset')   
        CheckHash(0xCD7B633E46CD6D23) 
        Mouse.Click(20,4,left)
        Paint('Add (2) and Reset') 
        CheckHash(0x9194073ED8F578) 
        Mouse.Click(40,4,left) 
        Paint('After reset: Add (0) and Reset') 
        CheckHash(0xBAE7A3089D249D76) 
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_button_control_with_macro() {
    let script = "
        Paint.Enable(false)
        Paint('Initial focus => Add (0) and Reset')   
        CheckHash(0xC656986DBDA863BA)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = Window::new("Macro Test",Layout::new("d:c,w:40,h:10"),window::Flags::None);
    w.add(button!("Start,x:2,y:3,w:15"));
    w.add(button!("Disabled,x:20,y:3,w:15,enable:false"));
    w.add(button!("caption:'Not Visible',x:0,y:0,w:100%,visible:false"));
    w.add(button!("Flat,x:2,y:5,w:15,flags:flat"));
    w.add(button!("text:'Flat and disabled',x:2,y:7,w:30,flags:[flat],enable:false"));
    a.add_window(w);
    a.run();
}
