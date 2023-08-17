use AppCUIProcMacro::*;

#[Window(events = ButtonEvents, internal=true)]
struct MyWin {
    info: ControlHandle<Label>,
    but1: ControlHandle<Button>,
    but2: ControlHandle<Button>,
    but3: ControlHandle<Button>,
}
impl MyWin {
    fn new() -> Self {
        let mut me = Self {
            base: Window::new("Win-1", Layout::new("d:c,w:41,h:7"), WindowFlags::None),
            info: ControlHandle::None,
            but1: ControlHandle::None,
            but2: ControlHandle::None,
            but3: ControlHandle::None,
        };
        me.info = me.add(Label::new("<none>",Layout::new("x:0,y:0,w:35")));
        me.but1 = me.add(Button::new("Button &1", Layout::new("x:1,y:3,w:11"),button::Flags::None));
        me.but2 = me.add(Button::new("Button &2", Layout::new("x:14,y:3,w:11"),button::Flags::None));
        let mut b3 = Button::new("Button &3", Layout::new("x:27,y:3,w:11"),button::Flags::None);
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
    fn on_pressed(&mut self, button_handle: Handle) {
        if self.but1 == button_handle {
            self.set_info("Button 1 presed");
        }
        if self.but2 == button_handle {
            self.set_info("Button 2 pressed");
        }
    }
}

#[test]
fn check_button_control() {
    // let script = "
    //     Paint('initial state')   
    //     //CheckHash(0xB838E6ABBF00B753)   
    //     Key.Pressed(Tab)
    //     Paint('After tab pressed') 
    //     Key.Pressed(Enter)
    //     Paint('After second button was pressed')
    // ";
    // let mut a = App::debug(
    //     60,
    //     10,
    //     InitializationFlags::None,
    //     Desktop::new(),
    //     script,
    // )
    // .unwrap();
    // a.add_window(MyWin::new());
    //a.run();
}
