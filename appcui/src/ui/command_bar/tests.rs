use crate::prelude::*;

#[test]
fn check_commands() {
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

    #[Window(events = CommandBarEvents, internal=true,commands=[Command1,Command2])]
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
            if let Some(label) = self.control_mut(h_label) {
                label.set_caption(txt);
            }
        }
    }
    impl CommandBarEvents for MyWin1 {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "MyWin-1-CMD-1", mywin1::Commands::Command1);
            commandbar.set(key!("F2"), "MyWin-1-CMD-2", mywin1::Commands::Command2);
        }

        fn on_event(&mut self, command_id: mywin1::Commands) {
            match command_id {
                mywin1::Commands::Command1 => self.set_info("Command-1 pressed"),
                mywin1::Commands::Command2 => self.set_info("Command-2 pressed"),
            }
        }
    }

    let mut a = App::debug(60, 10, script).command_bar().build().unwrap();
    a.add_window(MyWin1::new());
    a.run();
}

#[test]
fn check_state_commands() {
    let script = "
        Paint.Enable(false)
        Paint('F1 State: Hidden')
        CheckHash(0x548C746494090210)
        Key.Pressed(F1)
        Paint('F1 State: Visible   F2 Option: OFF')
        CheckHash(0xEF008634FADFABDE)
        Key.Pressed(F2)
        Paint('F1 State: Visible   F2 Option: ON')
        CheckHash(0xBBF7A34221338C40)
        Key.Pressed(F1)
        Paint('F1 State: Hidden')
        CheckHash(0x548C746494090210)
        Key.Pressed(F1)
        Paint('F1 State: Visible   F2 Option: ON')
        CheckHash(0xBBF7A34221338C40)
    ";

    #[Window(events = CommandBarEvents, internal=true,commands=[ChangeOption,ShowState])]
    struct MyWin {
        option: bool,
        state_is_visible: bool
    }
    impl MyWin {
        fn new() -> Self {
            Self {
                base: window!("Win,x:1,y:1,w:20,h:7"),
                option: false,
                state_is_visible: false,
            }
        }
    }
    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            if self.state_is_visible {
                commandbar.set(key!("F1"), "State: Visible", mywin::Commands::ShowState);
                if self.option {
                    commandbar.set(key!("F2"), "Option: ON", mywin::Commands::ChangeOption);
                } else {
                    commandbar.set(key!("F2"), "Option: OFF", mywin::Commands::ChangeOption);
                }
            } else {
                commandbar.set(key!("F1"), "State: Hidden", mywin::Commands::ShowState);
            }
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            match command_id {
                mywin::Commands::ShowState => self.state_is_visible = !self.state_is_visible,
                mywin::Commands::ChangeOption => self.option = !self.option
            }
            self.request_update();
        }
    }

    let mut a = App::debug(60, 10, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}
