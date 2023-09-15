use crate::prelude::*;

use crate::{
    input::{KeyCode, KeyModifier},
    system::App,
    ui::{Desktop, Layout},
};

use super::{
    toolbar::{self, GroupPosition},
    Window,
};

#[test]
fn check_window_title() {
    let script = "
        //Paint.Enable(false)
        Paint('title')
        CheckHash(0xA0CFD68A45B1786C)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("Title", Layout::new("d:c,w:20,h:10"), window::Flags::None));
    a.run();
}
#[test]
fn check_window_full_title_1() {
    let script = "
        Paint.Enable(false)
        Paint('full title')
        CheckHash(0xF410B9650F4ADF18)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("1234567890A", Layout::new("d:c,w:20,h:10"), window::Flags::None));
    a.run();
}
#[test]
fn check_window_full_title_2() {
    let script = "
        Paint.Enable(false)
        Paint('full title')
        CheckHash(0xA0CFD68A45B1786C)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("Title", Layout::new("d:c,w:20,h:10"), window::Flags::None));
    a.run();
}
#[test]
fn check_window_full_title_3() {
    let script = "
        Paint.Enable(false)
        Paint('full title')
        CheckHash(0xEEBF652BB26E9C4C)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("ABC", Layout::new("d:c,w:12,h:8"), window::Flags::None));
    a.run();
}
#[test]
fn check_window_minimize_title_1() {
    let script = "
        //Paint.Enable(false)
        Paint('Title = ABCD...IJKL')
        CheckHash(0x671DB3CA4AD392AE)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("ABCDEFGHIJKL", Layout::new("d:c,w:20,h:10"), window::Flags::None));
    a.run();
}
#[test]
fn check_window_minimize_title_2() {
    let script = "
        Paint.Enable(false)
        Paint('Title = AB...KL')
        CheckHash(0x7F7F1F564130F50E)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("ABCDEFGHIJKL", Layout::new("d:c,w:16,h:8"), window::Flags::None));
    a.run();
}
#[test]
fn check_window_minimize_title_3() {
    let script = "
        Paint.Enable(false)
        Paint('Title = A...L')
        CheckHash(0x6CB0EAB5DDA0E087)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("ABCDEFGHIJKL", Layout::new("d:c,w:14,h:6"), window::Flags::None));
    a.run();
}
#[test]
fn check_window_minimize_title_4() {
    let script = "
        Paint.Enable(false)
        Paint('Shorten title')
        CheckHash(0x3A1C142AE9968A2F)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    a.add_window(Window::new("ABCDEFGHIJKL", Layout::new("d:c,w:12,h:6"), window::Flags::None));
    a.run();
}

#[test]
fn check_multiple_items_top_bar() {
    let script = "
    //Paint.Enable(false)
    Paint('tags')
    //CheckHash(0xBB2962251DDB2240)
    ";
    let mut a = App::debug(80, 10, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:70,h:10"), window::Flags::Sizeable);
    w.set_tag("ABC");
    w.set_hotkey(key!("F1"));
    let g = w.get_toolbar().create_group(GroupPosition::TopLeft);
    w.get_toolbar().add(g, toolbar::Label::new("Lb-1"));
    w.get_toolbar().add(g, toolbar::Label::new("Lb-2"));
    let g = w.get_toolbar().create_group(GroupPosition::TopLeft);
    w.get_toolbar().add(g, toolbar::Label::new("Single"));
    let g = w.get_toolbar().create_group(GroupPosition::TopRight);
    w.get_toolbar().add(g, toolbar::Label::new("Lb-1"));
    w.get_toolbar().add(g, toolbar::Label::new("Lb-2"));
    let g = w.get_toolbar().create_group(GroupPosition::TopRight);
    w.get_toolbar().add(g, toolbar::Label::new("AB"));  
    let g = w.get_toolbar().create_group(GroupPosition::BottomLeft);
    w.get_toolbar().add(g, toolbar::Label::new("Lb-1"));
    w.get_toolbar().add(g, toolbar::Label::new("Lb-2"));
    let g = w.get_toolbar().create_group(GroupPosition::BottomLeft);
    w.get_toolbar().add(g, toolbar::Label::new("Single"));
    let g = w.get_toolbar().create_group(GroupPosition::BottomRight);
    w.get_toolbar().add(g, toolbar::Label::new("Lb-1"));
    w.get_toolbar().add(g, toolbar::Label::new("Lb-2"));
    let g = w.get_toolbar().create_group(GroupPosition::BottomRight);
    w.get_toolbar().add(g, toolbar::Label::new("AB"));  

    a.add_window(w);
    a.run();
}

#[test]
fn check_window_tag_1() {
    let script = "
        //Paint.Enable(false)
        Paint('tags')
        CheckHash(0xBB2962251DDB2240)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), window::Flags::None);
    w.set_tag("AB");
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_tag_2() {
    let script = "
        Paint.Enable(false)
        Paint('title should be visible')
        CheckHash(0xE2CB87CCC6FD9E4A)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), window::Flags::None);
    w.set_tag("ABCD");
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_tag_and_split_title_1() {
    let script = "
        //Paint.Enable(false)
        Paint('title split with 3 special chars')
        CheckHash(0x34902E0B6D58F035)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), window::Flags::None);
    w.set_tag("ABCDE");
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_tag_and_split_title_2() {
    let script = "
        Paint.Enable(false)
        Paint('title split with 3 special chars')
        CheckHash(0xA52995587B045766)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), window::Flags::None);
    w.set_tag("ABCDEF");
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_tag_and_title_first_letter() {
    let script = "
        Paint.Enable(false)
        Paint('title first letter and special char with 3 points')
        CheckHash(0x6F914F802B3B7B5D)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), window::Flags::None);
    w.set_tag("ABCDEFG");
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_tag_and_title_not_visible() {
    let script = "
        Paint.Enable(false)
        Paint('title not visible')
        CheckHash(0xA2C91CB6A1484009)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), window::Flags::None);
    w.set_tag("ABCDEFGH");
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_hotkey_1() {
    let script = "
        Paint.Enable(false)
        Paint('hotkey')
        CheckHash(0x4454159FD9AA73E9)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), window::Flags::None);
    w.set_hotkey(key!("Alt+F1"));
    a.add_window(w);
    a.run();
}
#[test]
fn check_window_hotkey_2() {
    let script = "
        //Paint.Enable(false)
        Paint('hotkey')
        CheckHash(0xC9D2F0E450475385)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), window::Flags::None);
    w.set_hotkey(KeyCode::Enter);
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_hotkey_and_tag() {
    let script = "
        Paint.Enable(false)
        Paint('hotkey & tag')
        CheckHash(0x8F6D9DF3500A2D7A)
    ";
    let mut a = App::debug(20, 10, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:20,h:10"), window::Flags::None);
    w.set_hotkey(key!("Alt+1"));
    w.set_tag("XYZ");
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_resize() {
    let script = "
        //Paint.Enable(false)
        Paint('initial state')
        CheckHash(0x6E5585BA8803D312)
        Mouse.Move(39,7)
        Paint('over the resize handler')
        CheckHash(0x18BDFDDEEC0AAC26)
        Mouse.Hold(39,7,left)
        Paint('click on resize handler')
        CheckHash(0x1B338A6C37E2546A)
        Mouse.Move(41,8)
        Paint('resized')
        CheckHash(0x36E28C864436D546)
        Mouse.Move(51,8)
        Paint('even bigger')
        CheckHash(0xF74E3A7D92E695E2)
        Mouse.Move(47,6)
        Paint('smaller')
        CheckHash(0x1628BC4542D3DC12)
        Mouse.Release(47,6,left)
        Paint('after release of handle')
        CheckHash(0x6A9DA986C039579A)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let w = Window::new("Title", Layout::new("d:c,w:20,h:5"), window::Flags::Sizeable);
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_move() {
    let script = "
        Paint.Enable(false)
        Mouse.Move(30,3)
        Paint('over the title')
        CheckHash(0x6E5585BA8803D312)
        Mouse.Hold(30,3,left)
        Paint('press on window title')
        CheckHash(0x5EEA06B578ABD9A)
        Mouse.Move(15,1)
        Paint('window moved')
        CheckHash(0x1EBF887394C43672)
        Mouse.Move(1,1)
        Paint('window moved outside')
        CheckHash(0x8C8185EC85BF07AE)
        Mouse.Release(1,1,left)
        Paint('after release of handle')
        CheckHash(0x922FBAECBC6B2C2)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let w = Window::new("Title", Layout::new("d:c,w:20,h:5"), window::Flags::Sizeable);
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_move_2() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')
        CheckHash(0xB1471A30B30F5C6C)
        Mouse.Drag(30,3,35,5)
        Paint('window was moved')
        CheckHash(0x419533D4BBEFE538)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let w = Window::new("Title", Layout::new("d:c,w:20,h:5"), window::Flags::None);
    a.add_window(w);
    a.run();
}

#[test]
fn check_window_on_layout_changed() {
    #[Window(events = WindowEvents,internal = true)]
    struct MyWin {
        info: Handle<Label>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", Layout::new("d:c,w:40,h:7"), window::Flags::Sizeable),
                info: Handle::None,
            };
            me.info = me.add(Label::new("", Layout::new("x:1,y:1,w:36,h:3")));
            me
        }
    }
    impl WindowEvents for MyWin {
        fn on_layout_changed(&mut self, old_layout: Rect, new_layout: Rect) {
            let label_handle = self.info;
            let size = self.get_size();
            let client_size = self.get_client_size();
            if let Some(label) = self.get_control_mut(label_handle) {
                label.set_text(
                    format!(
                        "Previous rect : {},{} - {}x{}\nNew rect      : {},{} - {}x{}\nClient size   : {}x{}",
                        old_layout.get_left(),
                        old_layout.get_top(),
                        old_layout.get_width(),
                        old_layout.get_height(),
                        new_layout.get_left(),
                        new_layout.get_top(),
                        new_layout.get_width(),
                        new_layout.get_height(),
                        client_size.width,
                        client_size.height
                    )
                    .as_str(),
                );
            }
            assert!(size.width == new_layout.get_width());
            assert!(size.height == new_layout.get_height());
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('initial (pos=10,2 with 40x7)')
        CheckHash(0xDA18EF6A52B3C090)
        Mouse.Move(49,8)
        Mouse.Hold(49,8,left)
        Mouse.Move(51,9)
        Mouse.Release(51,9,left)
        Paint('Resize to 42x8')
        CheckHash(0x99F321F831E005F6)
        Mouse.Move(28,2)
        Mouse.Hold(28,2,left)
        Mouse.Move(26,1)
        Mouse.Release(26,1,left)
        Paint('Move to 8,1 with 42x8 size')
        CheckHash(0x7123D2EB32E9E49A)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_window_on_activate_deactivate() {
    #[Window(events = WindowEvents,internal = true)]
    struct MyWin {
        info: Handle<Label>,
    }
    impl MyWin {
        fn new(layout: &str) -> Self {
            let mut me = Self {
                base: Window::new("Win", Layout::new(layout), window::Flags::None),
                info: Handle::None,
            };
            me.info = me.add(Label::new("<no-state>", Layout::new("x:1,y:1,w:16")));
            me
        }
    }
    impl WindowEvents for MyWin {
        fn on_activate(&mut self) {
            let h = self.info;
            if let Some(label) = self.get_control_mut(h) {
                label.set_text("Activated");
            }
        }

        fn on_deactivate(&mut self) {
            let h = self.info;
            if let Some(label) = self.get_control_mut(h) {
                label.set_text("Deactivated");
            }
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('initial state (left=no-state,right=activated)')
        CheckHash(0xAC18C0AB5493591E)
        Mouse.Click(10,1,left)
        Paint('left=activated, right=deactivated')
        CheckHash(0xCE0FE1E047B8B238)
        Mouse.Click(40,1,left)
        Paint('left=deactivated, right=activated')
        CheckHash(0x5EBC9C1734091B4C)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new("x:1,y:1,w:25,h:6"));
    a.add_window(MyWin::new("x:30,y:1,w:25,h:6"));
    a.run();
}

#[test]
fn check_window_toolbar_label() {
    let script = "
        //Paint.Enable(false)
        Paint('multiple label')
        CheckHash(0x7DF82A0072CCF28F)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:8"), window::Flags::None);
    let g = w.get_toolbar().create_group(GroupPosition::BottomLeft);
    w.get_toolbar().add(g, toolbar::Label::new("Label 1"));
    w.get_toolbar().add(g, toolbar::Label::new("Label 2"));
    let g = w.get_toolbar().create_group(GroupPosition::BottomRight);
    w.get_toolbar().add(g, toolbar::Label::new("Label 3"));

    a.add_window(w);
    a.run();
}

#[test]
fn check_window_toolbar_button() {
    let script = "
        //Paint.Enable(false)
        Paint('buttons')
        CheckHash(0x52243E492A4813E1)
        Mouse.Move(16,8)
        Paint('Mouse on button start')
        CheckHash(0x11C054FAECF9D51F)
        Mouse.Move(21,8)
        Paint('Mouse on button stop')
        CheckHash(0x63DCB4AE11E49B9)
        Mouse.Hold(21,8,left)
        Paint('Mouse press over stop button')
        CheckHash(0xEFCCAC14BDDC389)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:8"), window::Flags::None);
    let g = w.get_toolbar().create_group(GroupPosition::BottomLeft);
    w.get_toolbar().add(g, toolbar::Button::new("Start"));
    w.get_toolbar().add(g, toolbar::Button::new("Stop"));
    let g = w.get_toolbar().create_group(GroupPosition::BottomRight);
    w.get_toolbar().add(g, toolbar::Button::new("Exit"));

    a.add_window(w);
    a.run();
}

#[test]
fn check_window_toolbar_button_events() {
    #[Window(events = ToolBarEvents,internal = true)]
    struct MyWin {
        info: Handle<Label>,
        run_button: Handle<toolbar::Button>,
        stop_button: Handle<toolbar::Button>,
        exit_button: Handle<toolbar::Button>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", Layout::new("d:c,w:40,h:8"), window::Flags::None),
                info: Handle::None,
                run_button: Handle::None,
                stop_button: Handle::None,
                exit_button: Handle::None,
            };
            me.info = me.add(Label::new("<no-state>", Layout::new("x:1,y:1,w:16")));
            let g = me.get_toolbar().create_group(GroupPosition::BottomLeft);
            me.run_button = me.get_toolbar().add(g, toolbar::Button::new("&Run"));
            me.stop_button = me.get_toolbar().add(g, toolbar::Button::new("&Stop"));
            me.exit_button = me.get_toolbar().add(g, toolbar::Button::new("E&xit"));
            me
        }
        fn set_info(&mut self, info: &str) {
            let h = self.info;
            if let Some(label) = self.get_control_mut(h) {
                label.set_text(info);
            }
        }
    }
    impl ToolBarEvents for MyWin {
        fn on_button_clicked(&mut self, handle: Handle<toolbar::Button>) -> EventProcessStatus {
            if handle == self.run_button {
                self.set_info("Run pressed");
                return EventProcessStatus::Processed;
            }
            if handle == self.stop_button {
                self.set_info("Stop pressed");
                return EventProcessStatus::Processed;
            }
            if handle == self.exit_button {
                self.set_info("Exit pressed");
                return EventProcessStatus::Processed;
            }

            EventProcessStatus::Ignored
        }
    }

    let script = "
        //Paint.Enable(false)
        Paint('initial state (no button pressed)')
        CheckHash(0x95EE0B3C635338CD)
        Mouse.Click(14,8,left)
        Paint('Run button pressed')
        //CheckHash(0)
        Mouse.Click(18,8,left)
        Paint('Stop button pressed')
        //CheckHash(0)
        Mouse.Click(22,8,left)
        Paint('Exit button pressed')
        //CheckHash(0)
        Key.Pressed(Alt+R)
        Paint('Run button pressed (with hotkey)')
        //CheckHash(0)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_window_toolbar_checkbox() {
    let script = "
        //Paint.Enable(false)
        Paint('checkboxes')
        CheckHash(0xE3AB27FF068AB418)
        Mouse.Move(16,8)
        Paint('Mouse over Task 1')
        CheckHash(0xC0598DC79AC6EE95)
        Mouse.Move(23,8)
        Paint('Mouse over Task 2')
        CheckHash(0x921CBA73AC67EB4C)
        Mouse.Click(23,8,left)
        Paint('Task 2 checked')
        //CheckHash(0x11C054FAECF9D51F)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:8"), window::Flags::None);
    let g = w.get_toolbar().create_group(GroupPosition::BottomLeft);
    w.get_toolbar().add(g, toolbar::CheckBox::new("Task &1", true));
    w.get_toolbar().add(g, toolbar::CheckBox::new("Task &2", false));
    let g = w.get_toolbar().create_group(GroupPosition::BottomRight);
    w.get_toolbar().add(g, toolbar::CheckBox::new("Enable &All", false));

    a.add_window(w);
    a.run();
}

#[test]
fn check_window_toolbar_checkbox_events() {
    #[Window(events = ToolBarEvents,internal = true)]
    struct MyWin {
        cb: Handle<toolbar::CheckBox>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", Layout::new("d:c,w:50,h:6"), window::Flags::None),
                cb: Handle::None,
            };
            let g = me.get_toolbar().create_group(GroupPosition::TopLeft);
            me.cb = me.get_toolbar().add(g, toolbar::CheckBox::new("No State", false));
            me
        }
    }
    impl ToolBarEvents for MyWin {
        fn on_checkbox_clicked(&mut self, handle: Handle<toolbar::CheckBox>, checked: bool) -> EventProcessStatus {
            if handle == self.cb {
                let h = self.cb;
                if let Some(checkbox) = self.get_toolbar().get_mut(h) {
                    if checked {
                        checkbox.set_text("&Checked");
                    } else {
                        checkbox.set_text("&Not checked");
                    }
                    return EventProcessStatus::Processed;
                }
            }
            EventProcessStatus::Ignored
        }
    }

    let script = "
        // Paint.Enable(false)
        Paint('checbox = <No State>')
        CheckHash(0x7A0F1259C3B555F4)
        Mouse.Click(12,2,left)
        Paint('checkbox = Checked')
        CheckHash(0x33348CAE85FD1833)
        Mouse.Click(12,2,left)
        Paint('checkbox = Not checked')
        CheckHash(0x3B3091F01E62C367)
        Key.Pressed(Alt+N)
        Paint('Checkbox is checked again')
        CheckHash(0x33348CAE85FD1833)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_window_toolbar_single_choice() {
    let script = "
        //Paint.Enable(false)
        Paint('checkboxes')
        CheckHash(0x4C5631E5B117880C)
        Mouse.Move(5,8)
        Paint('Mouse over Opt 1')
        CheckHash(0xFE6ED16B8C8DEC4A)
        Mouse.Move(40,8)
        Paint('Mouse over Opt C')
        CheckHash(0x230896B85C5EDB2)
        Mouse.Move(11,8)
        Paint('Mouse over Opt 2')
        CheckHash(0xB7BC6D5A47FA1902)
        Mouse.Click(11,8,left)
        Paint('Opt 2 selected')
        //CheckHash(0x11C054FAECF9D51F)
        Mouse.Click(6,8,left)
        Paint('Opt 1 selected')
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:58,h:8"), window::Flags::None);
    let g = w.get_toolbar().create_group(GroupPosition::BottomLeft);
    w.get_toolbar().add(g, toolbar::SingleChoice::new("Opt &1", 1));
    w.get_toolbar().add(g, toolbar::SingleChoice::new("Opt &2", 1));
    w.get_toolbar().add(g, toolbar::SingleChoice::new("Opt &3", 1));
    let g = w.get_toolbar().create_group(GroupPosition::BottomRight);
    w.get_toolbar().add(g, toolbar::SingleChoice::new("Opt &A", 2));
    w.get_toolbar().add(g, toolbar::SingleChoice::new("Opt &B", 2));
    w.get_toolbar().add(g, toolbar::SingleChoice::new("Opt &C", 2));

    a.add_window(w);
    a.run();
}

#[test]
fn check_window_toolbar_singlechoice_events() {
    #[Window(events = ToolBarEvents,internal = true)]
    struct MyWin {
        opt1: Handle<toolbar::SingleChoice>,
        opt2: Handle<toolbar::SingleChoice>,
        opt3: Handle<toolbar::SingleChoice>,
        optA: Handle<toolbar::SingleChoice>,
        optB: Handle<toolbar::SingleChoice>,
        optC: Handle<toolbar::SingleChoice>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", Layout::new("d:c,w:58,h:8"), window::Flags::None),
                opt1: Handle::None,
                opt2: Handle::None,
                opt3: Handle::None,
                optA: Handle::None,
                optB: Handle::None,
                optC: Handle::None,
            };
            let g = me.get_toolbar().create_group(GroupPosition::BottomLeft);
            me.opt1 = me.get_toolbar().add(g, toolbar::SingleChoice::new("Opt &1", 1));
            me.opt2 = me.get_toolbar().add(g, toolbar::SingleChoice::new("Opt &2", 1));
            me.opt3 = me.get_toolbar().add(g, toolbar::SingleChoice::new("Opt &3", 1));
            let g = me.get_toolbar().create_group(GroupPosition::BottomRight);
            me.optA = me.get_toolbar().add(g, toolbar::SingleChoice::new("Opt &A", 2));
            me.optB = me.get_toolbar().add(g, toolbar::SingleChoice::new("Opt &B", 2));
            me.optC = me.get_toolbar().add(g, toolbar::SingleChoice::new("Opt &C", 2));
            me
        }
    }
    impl ToolBarEvents for MyWin {
        fn on_choice_selected(&mut self, handle: Handle<toolbar::SingleChoice>) -> EventProcessStatus {
            if handle == self.opt1 {
                let h = self.optA;
                if let Some(opt_a) = self.get_toolbar().get_mut(h) {
                    opt_a.select();
                }
                return EventProcessStatus::Processed;
            }
            if handle == self.opt2 {
                let h = self.optB;
                if let Some(opt_b) = self.get_toolbar().get_mut(h) {
                    opt_b.select();
                }
                return EventProcessStatus::Processed;
            }
            EventProcessStatus::Ignored
        }
    }

    let script = "
        //Paint.Enable(false)
        Paint('original status')
        //CheckHash(0x4C5631E5B117880C)
        Mouse.Move(5,8)
        Paint('Mouse over Opt 1')
        //CheckHash(0xFE6ED16B8C8DEC4A)
        Mouse.Click(5,8,left)
        Paint('Opt 1 selected => Opt A selected as a result')
        //CheckHash(0x11C054FAECF9D51F)
        Mouse.Click(10,8,left)
        Paint('Opt 2 selected => Opt B selected as a result')
        //CheckHash(0x11C054FAECF9D51F)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}
