use crate::prelude::*;

#[Window(events = CommandBarEvents, internal=true)]
struct MyWin1 {
    info: Handle<Label>,
}
impl MyWin1 {
    fn new() -> Self {
        let mut me = Self {
            base: Window::new("Win-1", Layout::new("x:1,y:1,w:20,h:7"), window::Flags::None),
            info: Handle::None,
        };
        me.info = me.add(Label::new("<none>", Layout::new("x:0,y:0,w:18")));
        me
    }
    fn set_info(&mut self, txt: &str) {
        let h_label = self.info;
        if let Some(label) = self.get_control_mut(h_label) {
            label.set_caption(txt);
        }
    }
}
impl CommandBarEvents for MyWin1 {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("F1"), "MyWin-1-CMD-1", 1u32);
        commandbar.set(key!("F2"), "MyWin-1-CMD-2", 2u32);
    }

    fn on_event(&mut self, command_id: u32) {
        match command_id {
            1 => self.set_info("Command-1 pressed"),
            2 => self.set_info("Command-2 pressed"),
            _ => {}
        }
    }
}

#[test]
fn check_command_bar_1() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0x8F46035284DF4B04)
        Key.Pressed(F1)
        Paint('MyWin-1-CMD-1 pressed')
        CheckHash(0x4CDD9A1678E2E012)
        Mouse.Move(29,9);
        Paint('MyWin-1-CMD-2 hovered')
        CheckHash(0x1EC6A227F348D572)
        Mouse.Click(29,9,left)
        Paint('MyWin-1-CMD-2 clicked')
        CheckHash(0xCF24F44D8821A501)        
    ";
    let mut a = App::debug(60, 10, script).command_bar().build().unwrap();
    a.add_window(MyWin1::new());
    a.run();
}
