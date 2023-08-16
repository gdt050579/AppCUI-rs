use AppCUIProcMacro::*;

#[Window(events = ButtonEvents)]
struct MyWin {
    info: ControlHandle<Label>,
    but1: ControlHandle<Button>,
    but2: ControlHandle<Button>,
}
impl MyWin {
    fn new() -> Self {
        let mut me = Self {
            base: Window::new("Win-1", Layout::new("x:1,y:1,w:20,h:7"), WindowFlags::None),
            info: ControlHandle::None,
            but1: ControlHandle::None,
            but2: ControlHAndle::None,
        };
        me.info = me.add(Label::new("<none>",Layout::new("x:1,y:1,w:18")));
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

}

#[test]
fn check_button_control() {
    let script = "
        //Paint('initial state')
        CheckHash(0x8F46035284DF4B04)
        Key.Pressed(F1)
        //Paint('MyWin-1-CMD-1 pressed')
        CheckHash(0x4CDD9A1678E2E012)
        Mouse.Move(29,9);
        //Paint('MyWin-1-CMD-2 hovered')
        CheckHash(0x1EC6A227F348D572)
        Mouse.Click(29,9,left)
        //Paint('MyWin-1-CMD-2 clicked')
        CheckHash(0xCF24F44D8821A501)        
    ";
    let mut a = App::debug(
        60,
        10,
        InitializationFlags::CommandBar,
        Desktop::new(),
        script,
    )
    .unwrap();
    a.add_window(MyWin1::new());
    a.run();
}
