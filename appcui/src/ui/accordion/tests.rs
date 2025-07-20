use crate::prelude::*;

#[test]
fn check_control_reposition() {
    let script = "
        Paint.Enable(false) 
        Paint('Initial state')   
        CheckHash(0x59375A32B72A3ACA)
        Mouse.Click(6,1,left)
        Paint('Maximized window')
        CheckHash(0x13C58E8EFB3C0782)
        Mouse.Move(10,12)
        Paint('Hover over second page')
        CheckHash(0x35F8EDC3268D2518)
        Mouse.Click(10,12,left)
        Paint('Second page selected')
        CheckHash(0x4F7D31365BB4312)
        Mouse.Click(2,0,left)
        Paint('Return to original size')
        CheckHash(0xA450FC3636F665B2)
        Mouse.Click(40,11,left)
        Paint('3rd page selected')
        CheckHash(0x6C08EA349DACC442)
        Mouse.Click(7,1,left)
        Paint('Maximize again')
        CheckHash(0x2D8DF67B09FC3F82)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:12,flags: Sizeable");
    let mut ac = Accordion::new(layout!("l:0,t:0,r:0,b:0"), accordion::Flags::None);
    ac.add_panel("Panel &1");
    ac.add_panel("Panel &2");
    ac.add_panel("Panel &3");
    ac.add(0, button!("P-1-A,r:1,b:0,w:10,type:flat"));
    ac.add(0, button!("P-1-B,l:1,t:1,w:10,type:flat"));
    ac.add(1, button!("P-2-A,r:1,b:0,w:14,type:flat"));
    ac.add(1, button!("P-2-B,l:1,t:1,w:14,type:flat"));
    ac.add(2, button!("P-3-A,r:1,b:0,w:20,type:flat"));
    ac.add(2, button!("P-3-B,l:1,t:1,w:20,type:flat"));

    w.add(ac);
    a.add_window(w);
    a.run();
}

#[test]
fn check_keys() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0x59375A32B72A3ACA)
        Key.Pressed(Ctrl+Tab)
        Paint('Second panel selected')   
        CheckHash(0xA450FC3636F665B2)
        Key.Pressed(Ctrl+Tab)
        Paint('3rd panel selected')   
        CheckHash(0x6C08EA349DACC442)
        Key.Pressed(Ctrl+Shift+Tab,2)
        Paint('1st panel selected')   
        CheckHash(0x59375A32B72A3ACA)
        Key.Pressed(Alt+2)
        Paint('Second panel selected')   
        CheckHash(0xA450FC3636F665B2)
        Key.Pressed(Alt+1)
        Paint('1st panel re-selected')   
        CheckHash(0x59375A32B72A3ACA)
        Key.Pressed(Alt+3)
        Paint('3rd panel selected')   
        CheckHash(0x6C08EA349DACC442)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:12,flags: Sizeable");
    let mut ac = Accordion::new(layout!("l:0,t:0,r:0,b:0"), accordion::Flags::None);
    ac.add_panel("Panel &1");
    ac.add_panel("Panel &2");
    ac.add_panel("Panel &3");
    ac.add(0, button!("P-1-A,r:1,b:0,w:10,type:flat"));
    ac.add(0, button!("P-1-B,l:1,t:1,w:10,type:flat"));
    ac.add(1, button!("P-2-A,r:1,b:0,w:14,type:flat"));
    ac.add(1, button!("P-2-B,l:1,t:1,w:14,type:flat"));
    ac.add(2, button!("P-3-A,r:1,b:0,w:20,type:flat"));
    ac.add(2, button!("P-3-B,l:1,t:1,w:20,type:flat"));

    w.add(ac);
    a.add_window(w);
    a.run();
}

#[test]
fn check_focus() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xF8B8FB1A7C6A4324)
    ";
    let mut a = App::debug(80, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:75,h:12,flags: Sizeable");
    let mut ac1 = Accordion::new(layout!("x:1,y:1,w:20,h:8"), accordion::Flags::None);
    ac1.add_panel("Panel &1");
    ac1.add_panel("Panel &2");
    ac1.add_panel("Panel &3");
    ac1.add_panel("Panel &4");
    ac1.add_panel("Panel &5");

    let mut ac2 = Accordion::new(layout!("x:25,y:1,w:20,h:8"), accordion::Flags::None);
    ac2.add_panel("Panel &1");
    ac2.add_panel("Panel &2");
    ac2.add_panel("Panel &3");

    let mut ac3 = Accordion::new(layout!("x:49,y:1,w:20,h:8"), accordion::Flags::None);
    ac3.add_panel("Panel &1");
    ac3.add_panel("Panel &2");
    ac3.add_panel("Panel &3");
    ac3.set_enabled(false);

    w.add(ac1);
    w.add(ac2);
    w.add(ac3);
    a.add_window(w);
    a.run();
}

#[test]
fn check_macro() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0x59375A32B72A3ACA)
        Mouse.Click(20,10,left)
        Paint('Second panel')
        CheckHash(0xA450FC3636F665B2)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:12,flags: Sizeable");
    let mut ac = accordion!("l:0,t:0,r:0,b:0,panels:['Panel &1','Panel &2','Panel &3']");
    ac.add(0, button!("P-1-A,r:1,b:0,w:10,type:flat"));
    ac.add(0, button!("P-1-B,l:1,t:1,w:10,type:flat"));
    ac.add(1, button!("P-2-A,r:1,b:0,w:14,type:flat"));
    ac.add(1, button!("P-2-B,l:1,t:1,w:14,type:flat"));
    ac.add(2, button!("P-3-A,r:1,b:0,w:20,type:flat"));
    ac.add(2, button!("P-3-B,l:1,t:1,w:20,type:flat"));

    w.add(ac);
    a.add_window(w);
    a.run();
}

#[test]
fn check_transparent_background() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xA1CFB89BF40E15DA)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:12,flags: Sizeable");
    let mut ac = accordion!("l:0,t:0,r:0,b:0,panels:['Panel &1','Panel &2','Panel &3'],flags: TransparentBackground");
    ac.add(0, button!("P-1-A,r:1,b:1,w:10,type:flat"));
    ac.add(0, button!("P-1-B,l:1,t:1,w:10,type:flat"));

    w.add(ac);
    a.add_window(w);
    a.run();
}

#[test]
fn check_panel_caption_methods() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state with first panel selected')   
        CheckHash(0x78692C9C0A6DE6BF)
        Key.Pressed(Alt+2)
        Paint('Second panel selected')   
        CheckHash(0x473964A3F8A36110)
        Key.Pressed(F1)
        Paint('Second panel caption modified')   
        CheckHash(0xC24FAFD8AF0898DE)
        Key.Pressed(Alt+3)
        Paint('Third panel selected')   
        CheckHash(0xEC7547FE4ECDF8F7)
        Key.Pressed(F2)
        Paint('Third panel caption modified')   
        CheckHash(0xED7B71552EA39A84)
        Key.Pressed(Alt+1)
        Paint('First panel selected')   
        CheckHash(0x5D75C84BED87F85A)
        Key.Pressed(Alt+4)
        Paint('Fourth panel selected')   
        CheckHash(0x4CA226EB7A9D7B23)
        Key.Pressed(F3)
        Paint('Panel information displayed')   
        CheckHash(0x5CB1016FE86004B6)
    ";

    #[Window(events=CommandBarEvents,commands:A+B+C, internal:true)]
    struct MyWin {
        accordion_handle: Handle<Accordion>,
        info_handle: Handle<Label>,
    }

    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,x:1,y:1,w:78,h:15"),
                accordion_handle: Handle::None,
                info_handle: Handle::None,
            };

            let mut acc = Accordion::new(layout!("x:1,y:1,w:70,h:10"), accordion::Flags::None);
            acc.add_panel("Panel &1");
            acc.add_panel("Panel &2");
            acc.add_panel("Panel &3");
            acc.add_panel("Panel &4");

            // Add some content to each panel
            acc.add(0, Button::new("P1", layout!("x:2,y:2,w:20"), button::Type::Flat));
            acc.add(1, Button::new("P2", layout!("x:2,y:2,w:20"), button::Type::Flat));
            acc.add(2, Button::new("Pa", layout!("x:2,y:2,w:10"), button::Type::Flat));
            acc.add(2, Button::new("Pb", layout!("x:14,y:2,w:10"), button::Type::Flat));
            acc.add(3, Button::new("PX", layout!("x:2,y:2,w:20"), button::Type::Flat));

            // Create a label to display panel information
            let l = Label::new("", layout!("x:1,y:12,w:70,h:2"));

            w.accordion_handle = w.add(acc);
            w.info_handle = w.add(l);

            // Update the info label with initial panel info
            w.update_panel_info();

            w
        }

        fn update_panel_info(&mut self) {
            let h = self.accordion_handle;
            let info_text = if let Some(accordion) = self.control_mut(h) {
                // Get current panel
                let current = match accordion.current_panel() {
                    Some(idx) => format!("Current panel: {idx}"),
                    None => "No current panel".to_string(),
                };

                // Get all panel captions
                let mut captions = String::new();
                for i in 0..4 {
                    if let Some(caption) = accordion.panel_caption(i) {
                        captions.push_str(&format!("Panel {}: '{}', ", i + 1, caption));
                    }
                }

                format!("{current}\n{captions}")
            } else {
                "Error: Accordion not found".to_string()
            };

            let h = self.info_handle;
            if let Some(label) = self.control_mut(h) {
                label.set_caption(&info_text);
            }
        }

        fn modify_second_panel_caption(&mut self) {
            let h = self.accordion_handle;
            if let Some(accordion) = self.control_mut(h) {
                accordion.set_panel_caption(1, "MODIFIED &2");
                self.update_panel_info();
            }
        }

        fn modify_third_panel_caption(&mut self) {
            let h = self.accordion_handle;
            if let Some(accordion) = self.control_mut(h) {
                accordion.set_panel_caption(2, "CHANGED &3");
                self.update_panel_info();
            }
        }
    }

    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Modify Panel 2", mywin::Commands::A);
            commandbar.set(key!("F2"), "Modify Panel 3", mywin::Commands::B);
            commandbar.set(key!("F3"), "Update Info", mywin::Commands::C);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            match command_id {
                mywin::Commands::A => self.modify_second_panel_caption(),
                mywin::Commands::B => self.modify_third_panel_caption(),
                mywin::Commands::C => self.update_panel_info(),
            }
        }
    }

    let mut a = App::debug(80, 20, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_not_process_input() {
    let script = "
        Paint.Enable(false) 
        Paint('Initial state')   
        CheckHash(0x59375A32B72A3ACA)
        Key.Pressed('F1')
        Key.Pressed('F2')
        Mouse.DoubleClick(20,2,left)
        Mouse.Drag(20,2,40,2)
        Mouse.Wheel(20,2,up,1)
        Mouse.Wheel(20,2,down,1)
        Mouse.Wheel(20,2,left,1)
        Mouse.Wheel(20,2,right,1)
        Paint('Nothing changes - no input processing')   
        CheckHash(0x59375A32B72A3ACA)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:12,flags: Sizeable");
    let mut ac = Accordion::new(layout!("l:0,t:0,r:0,b:0"), accordion::Flags::None);
    assert_eq!(ac.current_panel(), None);

    ac.add_panel("Panel &1");
    ac.add_panel("Panel &2");
    ac.add_panel("Panel &3");

    ac.add(0, button!("P-1-A,r:1,b:0,w:10,type:flat"));
    ac.add(0, button!("P-1-B,l:1,t:1,w:10,type:flat"));
    ac.add(1, button!("P-2-A,r:1,b:0,w:14,type:flat"));
    ac.add(1, button!("P-2-B,l:1,t:1,w:14,type:flat"));
    ac.add(2, button!("P-3-A,r:1,b:0,w:20,type:flat"));
    ac.add(2, button!("P-3-B,l:1,t:1,w:20,type:flat"));

    assert_eq!(ac.panel_caption(100), None);

    w.add(ac);
    a.add_window(w);
    a.run();
}

#[test]
fn check_events() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state with first panel selected')   
        CheckHash(0xA4066CC38F5343F2)
        Key.Pressed(Alt+3)
        Paint('2. 3rd panel selected (New:2, Old:0)')   
        CheckHash(0x74301CAF6DB078D5)
        Mouse.Click(20,4,left)
        Paint('3. 2nd panel selected (New:1, Old:2)')   
        CheckHash(0x784FFF55A62C24F5)
        Key.Pressed(Alt+2)
        Paint('4. Nothing changed')   
        CheckHash(0x784FFF55A62C24F5)
    ";

    #[Window(events=AccordionEvents,internal:true)]
    struct MyWin {
        accordion_handle: Handle<Accordion>,
        info_handle: Handle<Label>,
    }

    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,x:1,y:1,w:78,h:15"),
                accordion_handle: Handle::None,
                info_handle: Handle::None,
            };

            let mut acc = Accordion::new(layout!("x:1,y:1,w:70,h:10"), accordion::Flags::None);
            acc.add_panel("Panel &1");
            acc.add_panel("Panel &2");
            acc.add_panel("Panel &3");
            acc.add_panel("Panel &4");

            // Add some content to each panel
            acc.add(0, Button::new("P1", layout!("x:2,y:2,w:20"), button::Type::Flat));
            acc.add(1, Button::new("P2", layout!("x:2,y:2,w:20"), button::Type::Flat));
            acc.add(2, Button::new("Pa", layout!("x:2,y:2,w:10"), button::Type::Flat));
            acc.add(2, Button::new("Pb", layout!("x:14,y:2,w:10"), button::Type::Flat));
            acc.add(3, Button::new("PX", layout!("x:2,y:2,w:20"), button::Type::Flat));

            // Create a label to display panel information
            let l = Label::new("", layout!("x:1,y:12,w:70,h:2"));

            w.accordion_handle = w.add(acc);
            w.info_handle = w.add(l);

            w
        }
    }

    impl AccordionEvents for MyWin {
        fn on_panel_changed(&mut self, _handle: Handle<Accordion>, new_panel_index: u32, old_panel_index: u32) -> EventProcessStatus {
            let s= format!("New: {new_panel_index}, Old: {old_panel_index}");
            let h = self.info_handle;
            if let Some(label) = self.control_mut(h) {
                label.set_caption(&s);
            }
            EventProcessStatus::Processed
        }
    }

    let mut a = App::debug(80, 20, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}
