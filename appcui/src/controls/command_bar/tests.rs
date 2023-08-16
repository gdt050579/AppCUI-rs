use super::super::command_bar::*;
use super::super::common::traits::*;
use super::super::menu::*;
use super::super::Layout;
use crate::controls::*;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;
use AppCUIProcMacro::*;

#[Window(overwrite = CommandBarEvents)]
struct MyWin1 {
    info: ControlHandle<Label>,
}
impl MyWin1 {
    fn new() -> Self {
        let mut me = Self {
            base: Window::new("Win-1", Layout::new("x:1,y:1,w:20,h:7"), WindowFlags::None),
            info: ControlHandle::None,
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
impl CommandBarEvents for MyWin1 {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("F1"), "MyWin-1-CMD-1", 1);
        commandbar.set(key!("F2"), "MyWin-1-CMD-2", 2);
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
