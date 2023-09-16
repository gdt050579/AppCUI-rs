use crate::prelude::*;

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

#[test]
fn check_button_control() {
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
