use crate::prelude::*;

#[test]
fn check_empty_combobox() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0xD3338F888B5B6D69)
        Key.Pressed(Space)
        Paint('Opened')   
        CheckHash(0xB34B803556F629E9)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:40,h:7");
    let c = ComboBox::new(Layout::new("x:1,y:1,w:30"), combobox::Flags::None);
    w.add(c);
    a.add_window(w);
    a.run();
}

#[test]
fn check_show_description() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0x6B9CEEE64BE5336A)
        Key.Pressed(Space)
        Paint('Opened')   
        CheckHash(0x9B224EA337A5BDC6)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:60,h:7");
    let mut c = ComboBox::new(Layout::new("x:1,y:1,w:50"), combobox::Flags::ShowDescription);
    c.add_item(combobox::Item::new("Option 1", "(Description for option 1)"));
    c.add_item(combobox::Item::new("Option 2", "(Description for option 2)"));
    c.add_item(combobox::Item::new("Option 3", "(Description for option 3)"));
    c.add_item(combobox::Item::new("Option 4", "(Description for option 4)"));
    c.set_index(3);
    w.add(c);
    a.add_window(w);
    a.run();
}

#[test]
fn check_macro_creation() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0xDFFF12815B314ED0)
        Key.Pressed(Space)
        Paint('2nd opened (BMW selected)')   
        CheckHash(0x3E4501DFCFB4948A)
        Key.Pressed(Space)
        Key.Pressed(Tab)
        Key.Pressed(Space)
        Paint('1st selected')   
        CheckHash(0xCD6CE57B63BBE3DD)
        Key.Pressed(Down)
        Key.Pressed(Enter)
        Paint('1st selected (Item-1)')   
        CheckHash(0x9D5F6C4A639F82FC)
    ";
    let mut a = App::debug(40, 12, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:60,h:7");
    w.add(combobox!("x:1,y:1,w:30,items=['Item 1','Item 2','Item 3']"));
    w.add(combobox!("x:1,y:3,w:30,items=['Toyota','Dacia','BMW'],index:2"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_open_unselected_combobox() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0xD3338F888B5B6D69)
        Key.Pressed(Space)
        Paint('Opened (nothing selected)')   
        CheckHash(0xB6B348402DAF8E22)
        Key.Pressed(Space)
        Paint('closed (nothing selected)')   
        CheckHash(0xD3338F888B5B6D69)
        Key.Pressed(Space)
        Paint('Opened (nothing selected)')   
        CheckHash(0xB6B348402DAF8E22)
        Key.Pressed(Down)
        Paint('First item selected')   
        CheckHash(0xB28AF987ED8D20B0)
        Key.Pressed(Down)
        Paint('Second item selected')   
        CheckHash(0x43E2E38DE5C81DC7)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:40,h:7");
    let mut c = ComboBox::new(Layout::new("x:1,y:1,w:30"), combobox::Flags::None);
    c.add("option 1");
    c.add("option 2");
    c.add("option 3");
    w.add(c);
    a.add_window(w);
    a.run();
}

#[test]
fn check_select_item_from_unselected() {
    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0xD3338F888B5B6D69)
        Key.Pressed(Space)
        Paint('Opened (nothing selected)')   
        CheckHash(0xB6B348402DAF8E22)
        Mouse.Click(10,5,left)
        Paint('Second item selected')   
        CheckHash(0x4A4D0C1E30FE00E4)        
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:40,h:7");
    let mut c = ComboBox::new(Layout::new("x:1,y:1,w:30"), combobox::Flags::None);
    c.add("option 1");
    c.add("option 2");
    c.add("option 3");
    w.add(c);
    a.add_window(w);
    a.run();
}

#[test]
fn check_clear_items_when_closed() {
    #[Window(events=CommandBarEvents,commands:A, internal:true)]
    struct MyWin {
        h: Handle<ComboBox>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Win,x:1,y:1,w:38,h:8"),
                h: Handle::None,
            };
            let mut c = ComboBox::new(Layout::new("x:1,y:1,w:30"), combobox::Flags::None);
            c.add("option 1");
            c.add("option 2");
            c.add("option 3");
            c.set_index(1);
            w.h = w.add(c);
            w
        }
    }
    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Clear", mywin::Commands::A);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            if command_id == mywin::Commands::A {
                let h = self.h;
                if let Some(cb) = self.control_mut(h) {
                    cb.clear();
                }
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('initial state (option 2)')   
        CheckHash(0x677F305E06C6CC2A)
        Key.Pressed(F1)
        Paint('Items are clear')   
        CheckHash(0xA48FE4D6600B53E3)
    ";
    let mut a = App::debug(40, 10, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_clear_items_when_opened() {
    #[Window(events=CommandBarEvents,commands:A, internal:true)]
    struct MyWin {
        h: Handle<ComboBox>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Win,x:1,y:1,w:38,h:8"),
                h: Handle::None,
            };
            let mut c = ComboBox::new(Layout::new("x:1,y:1,w:30"), combobox::Flags::None);
            c.add("option 1");
            c.add("option 2");
            c.add("option 3");
            c.set_index(1);
            w.h = w.add(c);
            w
        }
    }
    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Clear", mywin::Commands::A);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            if command_id == mywin::Commands::A {
                let h = self.h;
                if let Some(cb) = self.control_mut(h) {
                    cb.clear();
                }
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('initial state (option 2)')   
        CheckHash(0x677F305E06C6CC2A)
        Key.Pressed(Space)
        Paint('Opened')
        CheckHash(0xDCB81C344C555A2D)
        Key.Pressed(F1)
        Paint('Items are clear (larger)')   
        CheckHash(0x3A0D2563A2353DAB)
        Key.Pressed(Space)
        Paint('Items are clear (closed)')   
        CheckHash(0xA48FE4D6600B53E3)
        Key.Pressed(Space)
        Paint('Items are clear (opened and smaller)')   
        CheckHash(0x7D591AE22C8A7DFB)
    ";
    let mut a = App::debug(40, 10, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_complex_behavior() {
    #[Window(events = ComboBoxEvents+CommandBarEvents, commands: Add+Clear, internal: true)]
    struct MyWin {
        h: Handle<ComboBox>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("x:1,y:1,w:30,h:6,caption:Win"),
                h: Handle::None,
            };
            w.h = w.add(ComboBox::new(Layout::new("x:1,y:1,w:26"), combobox::Flags::None));
            w
        }
    }
    impl ComboBoxEvents for MyWin {
        fn on_selection_changed(&mut self, handle: Handle<ComboBox>) -> EventProcessStatus {
            let title = if let Some(cb) = self.control_mut(handle) {
                if let Some(item) = cb.selected_item() {
                    item.value().to_string()
                } else {
                    String::from("[None]")
                }
            } else {
                String::from("?")
            };
            self.set_title(&title);
            EventProcessStatus::Processed
        }
    }
    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Add", mywin::Commands::Add);
            commandbar.set(key!("F2"), "Clear", mywin::Commands::Clear);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            let handle = self.h;
            if let Some(cb) = self.control_mut(handle) {
                match command_id {
                    mywin::Commands::Add => {
                        let s = format!("Option {}", cb.count() + 1);
                        cb.add(&s);
                    }
                    mywin::Commands::Clear => {
                        cb.clear();
                    }
                }
            }
        }
    }

    let script = "
            Paint.Enable(false)
            Paint('Initial state')
            CheckHash(0xEF14CE41B51A027A)
            Mouse.Move(27,3)
            Mouse.Hold(27,3,left)
            Paint('State_7')
            CheckHash(0x24f37694ee4cea2)
            Mouse.Release(27,3,left)
            Mouse.Move(27,2)
            Mouse.Hold(27,2,left)
            Paint('State_8')
            CheckHash(0x98d4aa04e3bb5506)
            Mouse.Release(27,2,left)
            Paint('State_9')
            CheckHash(0xef14ce41b51a027a)
            Mouse.Move(4,19)
            Paint('State_10')
            CheckHash(0xd978c3b8b53dd98a)
            Mouse.Move(3,19)
            Mouse.Hold(3,19,left)
            Paint('State_11')
            CheckHash(0x32a7a8079d00b3e2)
            Mouse.Release(3,19,left)
            Paint('State_12')
            CheckHash(0xd978c3b8b53dd98a)
            Mouse.Hold(3,19,left)
            Paint('State_13')
            CheckHash(0x32a7a8079d00b3e2)
            Mouse.Release(3,19,left)            
            Paint('State_14')
            CheckHash(0xd978c3b8b53dd98a)
            Mouse.Move(27,3)
            Paint('State_15')
            CheckHash(0xef14ce41b51a027a)
            Mouse.Hold(27,3,left)
            Paint('State_16')
            CheckHash(0x4862e7f64490578d)
            Mouse.Release(27,3,left)
            Mouse.Move(21,5)
            Paint('State_17')
            CheckHash(0x47cfa8393abbe4cd)
            Mouse.Move(17,6)
            Paint('State_18')
            CheckHash(0x365daeb90efe297d)
            Mouse.Move(14,6)
            Mouse.Hold(14,6,left)
            Paint('State_19')
            CheckHash(0xb7cfa3c4c6bc1bd)
            Mouse.Release(14,6,left)
            Key.Pressed(F1,3)
            Key.Pressed(Space)
            Paint('State_20')
            CheckHash(0x88d92072819f686f)
            Key.Pressed(Left,3)
            Key.Pressed(Down)
            Paint('State_21')
            CheckHash(0x17abf7f60725e53f)
            Key.Pressed(Down)
            Paint('State_22')
            CheckHash(0xe4fbc1f9aeb2ad7)
            Key.Pressed(Enter)
            Paint('State_23')
            CheckHash(0x57a60816885fc225)
            Key.Pressed(F1)
            Key.Pressed(Space)
            Paint('State_24')
            CheckHash(0xba979690bf150b9e)
            Key.Pressed(F2)
            Paint('State_25')
            CheckHash(0x847a0a45181eaada)
            Key.Pressed(Up)
            Key.Pressed(M)
            Key.Pressed(Up)
            Key.Pressed(Down)
            Key.Pressed(Space)
            Paint('State_26')
            CheckHash(0xce9427b743a13f4e)
            Key.Pressed(Space)
            Paint('State_27')
            CheckHash(0x809b4661c1e058c6)
            Key.Pressed(Space)
            Paint('State_28')
            CheckHash(0xce9427b743a13f4e)
            Key.Pressed(F1,2)
            Key.Pressed(Space)
            Paint('State_29')
            CheckHash(0x38e5395a8c33a931)
            Key.Pressed(Down)
            Paint('State_30')
            CheckHash(0x870199ddc880e2a)
            Key.Pressed(Down)
            Paint('State_31')
            CheckHash(0x976f560bd889d68a)
            Key.Pressed(Enter)
            Paint('State_32')
            CheckHash(0xb7cfa3c4c6bc1bd)
            Key.Pressed(F2)
            Paint('State_33')
            CheckHash(0xfb5994be8d656874)
            Key.Pressed(Space)
            Paint('State_34')
            CheckHash(0x9a27eb95c77d169c)
            Key.Pressed(Space)
            Paint('State_35')
            CheckHash(0xfb5994be8d656874)
            Key.Pressed(F1,8)
            Key.Pressed(Space)
            Paint('State_36')
            CheckHash(0x993ffc50ddb79434)
            Key.Pressed(Down)
            Paint('State_37')
            CheckHash(0xf33a5205ffde9dbd)
            Key.Pressed(Down)
            Paint('State_38')
            CheckHash(0xc3ddebebbdfc8d1d)
            Key.Pressed(Down)
            Paint('State_39')
            CheckHash(0xd0064f9f78fe354d)
            Key.Pressed(Down)
            Paint('State_40')
            CheckHash(0x872e0421927e63f5)
            Key.Pressed(Down)
            Paint('State_41')
            CheckHash(0xbaecde2d61ea0175)
            Key.Pressed(Down)
            Paint('State_42')
            CheckHash(0x39526f9c882c6e4d)
            Key.Pressed(Down)
            Paint('State_43')
            CheckHash(0x30e17dcad167b695)
            Key.Pressed(Down)
            Paint('State_44')
            CheckHash(0xe5a977411e9b259d)
            Key.Pressed(Enter)
            Paint('State_45')
            CheckHash(0xc41002a96ff6939d)
            Key.Pressed(Escape)
            Paint('State_46')
            CheckHash(0x2d11fcbd531710fd)
    ";

    let mut a = App::debug(60, 20, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_value_and_try_value() {
    let script = "
        Paint.Enable(false)
        Paint('initial state with selected value')   
        CheckHash(0x25A5B9F75E16317D)
        Key.Pressed(Down)
        Paint('Option 3 selected')   
        CheckHash(0xD5F489B17E3182B8)
        Key.Pressed(Space)
        Key.Pressed(Up,2)
        Key.Pressed(Enter)
        Paint('Option 1 selected')   
        CheckHash(0xF6273124ADFC889A)
        Key.Pressed(F1)
        Paint('No items, no selection')   
        CheckHash(0x64D6F4DC72C8C23A)
    ";

    #[Window(events=CommandBarEvents+ComboBoxEvents,commands:A, internal:true)]
    struct MyWin {
        combo_handle: Handle<ComboBox>,
        output_handle: Handle<Label>,
    }

    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Win,x:1,y:1,w:38,h:10"),
                combo_handle: Handle::None,
                output_handle: Handle::None,
            };

            let mut c = ComboBox::new(Layout::new("x:1,y:1,w:30"), combobox::Flags::None);
            c.add("option 1");
            c.add("option 2");
            c.add("option 3");
            c.set_index(1); // Select "option 2" initially

            // Create a label to display values from value() and try_value()
            let l = Label::new("", Layout::new("x:1,y:3,w:36,h:4"));

            w.combo_handle = w.add(c);
            w.output_handle = w.add(l);

            // Update the label with initial values
            w.update_output_label();

            w
        }

        fn update_output_label(&mut self) {
            let h = self.combo_handle;
            let output_text = if let Some(combo) = self.control_mut(h) {
                let try_value_result = match combo.try_value() {
                    Some(value) => format!("try_value(): Some(\"{}\")", value),
                    None => "try_value(): None".to_string(),
                };

                let value_result = if combo.has_selection() {
                    format!("value(): \"{}\"", combo.value())
                } else {
                    "value(): would panic!".to_string()
                };

                format!("{}\n{}", value_result, try_value_result)
            } else {
                "Error: ComboBox not found".to_string()
            };

            let h = self.output_handle;
            if let Some(label) = self.control_mut(h) {
                label.set_caption(&output_text);
            }
        }
    }

    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Clear Items", mywin::Commands::A);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            if command_id == mywin::Commands::A {
                let h = self.combo_handle;
                if let Some(cb) = self.control_mut(h) {
                    cb.clear();
                    self.update_output_label();
                }
            }
        }
    }

    impl ComboBoxEvents for MyWin {
        fn on_selection_changed(&mut self, _handle: Handle<ComboBox>) -> EventProcessStatus {
            self.update_output_label();
            EventProcessStatus::Processed
        }
    }

    let mut a = App::debug(40, 12, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_selected_item_and_index() {
    let script = "
        Paint.Enable(false)
        Paint('initial state with option 2 selected')   
        CheckHash(0x7363825F4B6E3EB3)
        Key.Pressed(Down)
        Paint('Option 3 selected')   
        CheckHash(0x8DFD8437672B64FC)
        Key.Pressed(Space)
        Key.Pressed(Up,2)
        Key.Pressed(Enter)
        Paint('Option 1 selected')   
        CheckHash(0x294E16814CE44163)
        Key.Pressed(F1)
        Paint('Modified option 1')   
        CheckHash(0xE3DA449FEC38F268)
        Key.Pressed(F2)
        Paint('No items, no selection')   
        CheckHash(0xF61AF9C9E2DC1793)
    ";

    #[Window(events=CommandBarEvents+ComboBoxEvents,commands:A+B, internal:true)]
    struct MyWin {
        combo_handle: Handle<ComboBox>,
        output_handle: Handle<Label>,
    }

    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Win,x:1,y:1,w:78,h:10"),
                combo_handle: Handle::None,
                output_handle: Handle::None,
            };

            let mut c = ComboBox::new(Layout::new("x:1,y:1,w:30"), combobox::Flags::ShowDescription);
            c.add_item(combobox::Item::new("option 1", "first item"));
            c.add_item(combobox::Item::new("option 2", "second item"));
            c.add_item(combobox::Item::new("option 3", "third item"));
            c.set_index(1); // Select "option 2" initially

            // Create a label to display info about selected_item and index
            let l = Label::new("", Layout::new("x:1,y:3,w:76,h:5"));

            w.combo_handle = w.add(c);
            w.output_handle = w.add(l);

            // Update the label with initial values
            w.update_output_label();

            w
        }

        fn update_output_label(&mut self) {
            let h = self.combo_handle;
            let output_text = if let Some(combo) = self.control_mut(h) {
                let index = combo.index();

                let selected_item_result = match combo.selected_item() {
                    Some(item) => format!("selected_item(): Some({}, \"{}\")", item.value(), item.description()),
                    None => "selected_item(): None".to_string(),
                };

                let selected_item_mut_result = if combo.has_selection() {
                    "selected_item_mut(): Available"
                } else {
                    "selected_item_mut(): Would return None"
                };

                format!("index(): {:?}\n{}\n{}", index, selected_item_result, selected_item_mut_result)
            } else {
                "Error: ComboBox not found".to_string()
            };

            let h = self.output_handle;
            if let Some(label) = self.control_mut(h) {
                label.set_caption(&output_text);
            }
        }

        // Modify the selected item's description using selected_item_mut
        fn modify_selected_item(&mut self) {
            let h = self.combo_handle;
            if let Some(combo) = self.control_mut(h) {
                if let Some(item) = combo.selected_item_mut() {
                    // Modify the description of the selected item
                    item.set_description("MODIFIED description");

                    // Also test setting the value
                    item.set_value(&format!("MODIFIED {}", item.value()));
                }
            }
            self.update_output_label();
        }
    }

    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Modify Item", mywin::Commands::A);
            commandbar.set(key!("F2"), "Clear Items", mywin::Commands::B);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            match command_id {
                mywin::Commands::A => self.modify_selected_item(),
                mywin::Commands::B => {
                    let h = self.combo_handle;
                    if let Some(cb) = self.control_mut(h) {
                        cb.clear();
                        self.update_output_label();
                    }
                }
            }
        }
    }

    impl ComboBoxEvents for MyWin {
        fn on_selection_changed(&mut self, _handle: Handle<ComboBox>) -> EventProcessStatus {
            self.update_output_label();
            EventProcessStatus::Processed
        }
    }

    let mut a = App::debug(80, 12, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}
