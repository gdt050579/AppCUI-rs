use crate::prelude::*;

#[test]
fn check_keyboard_normal() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0x142F4584D04C8610)
        Key.Pressed(Tab)
        Paint('AB has focus')   
        CheckHash(0xC693EA917E95659C)
        Key.Pressed(Tab)
        Paint('CD has focus')   
        CheckHash(0xE5E073DD34B95DAC)
        Key.Pressed(Enter)
        Paint('CD is selected')   
        CheckHash(0xC5A22BCDE79CB224)
        Key.Pressed(Tab)
        Paint('Back to the button')   
        CheckHash(0x4A798672ABE15E48)
    ";
    let mut a = App::debug(70, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:10");
    w.add(togglebutton!("text:AB,desc:'Push Me',x:2,y:2,w:2"));
    w.add(togglebutton!("CD,'Push Me',x:5,y:2,w:2"));
    w.add(button!("'Test',x:2,y:6,w:15"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_keyboard_underlined() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0x93A7E5D7693BF8D0)
        Key.Pressed(Tab)
        Paint('AB has focus')   
        CheckHash(0xB94DD2BA100CE5C)
        Key.Pressed(Tab)
        Paint('CD has focus')   
        CheckHash(0x69781B504B82B86C)
        Key.Pressed(Enter)
        Paint('CD is selected')   
        CheckHash(0x282B46F06C7E118C)
        Key.Pressed(Tab)
        Paint('Back to the button')   
        CheckHash(0x84C7AA7D2395DF10)
    ";
    let mut a = App::debug(70, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:10");
    w.add(togglebutton!("text:AB,desc:'Push Me',x:2,y:2,w:2,type:Underlined"));
    w.add(togglebutton!("CD,'Push Me',x:5,y:2,w:2,type:Underlined"));
    w.add(button!("'Test',x:2,y:6,w:15"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_hover_normal() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0x142F4584D04C8610)
        Mouse.Move(19,3)
        Paint('Hover over AB')
        CheckHash(0xB0F4840FAB2A1BF2)
        Mouse.Move(22,3)
        Paint('Hover over CD')
        CheckHash(0xEF3CCBA28E4FB9BD)
    ";
    let mut a = App::debug(70, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:10");
    w.add(togglebutton!("text:AB,desc:'Push Me',x:2,y:2,w:2"));
    w.add(togglebutton!("CD,'Push Me',x:5,y:2,w:2"));
    w.add(button!("'Test',x:2,y:6,w:15"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_events() {
    #[Window(events = ToggleButtonEvents, internal=true)]
    struct MyWin {
        info: Handle<Label>,
        but1: Handle<ToggleButton>,
        but2: Handle<ToggleButton>,
        but3: Handle<ToggleButton>,
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
            me.but1 = me.add(togglebutton!("<>,'Some button',x:1,y:3,w:2"));
            me.but2 = me.add(togglebutton!("(),'Some button 2',x:4,y:3,w:2"));
            let mut b3 = togglebutton!("Update,'Some button 2',x:10,y:3,w:8");
            b3.set_enabled(false);
            me.but3 = me.add(b3);
            me
        }
        fn set_info(&mut self, txt: &str) {
            let h_label = self.info;
            if let Some(label) = self.control_mut(h_label) {
                label.set_caption(txt);
            }
        }
    }
    impl ToggleButtonEvents for MyWin {
        fn on_selection_changed(&mut self, handle: Handle<ToggleButton>, selected: bool) -> EventProcessStatus {
            if handle == self.but1 {
                self.set_info(format!("Button <> was pressed -> {}", selected).as_str());
            } else if handle == self.but2 {
                self.set_info(format!("Button () was pressed -> {}", selected).as_str());
            }
            EventProcessStatus::Ignored
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Button () has focus (default)')   
        CheckHash(0xB30105347DAB4CF8)   
        Key.Pressed(Tab)
        Paint('Button <> has focus (default)') 
        CheckHash(0xA7AA3274BE5B67EC) 
        Key.Pressed(Enter)
        Paint('Message: Button <> was pressed -> true')
        CheckHash(0x144E815C6F5EC959) 
        Mouse.Move(13,6)
        Paint('Button () is hovered')
        CheckHash(0xAE9B6DC41EF0E793) 
        Mouse.Click(13,6,left)
        Paint('Button () was pressed -> Button () was pressed -> true')
        CheckHash(0x7EA0CD4C1C6DF0AE)
        Mouse.Click(13,6,left)
        Paint('Button () was pressed -> Button () was pressed -> false')
        CheckHash(0x146C22653FBA0A06)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_events_single_selection() {
    let script = "
        Paint.Enable(false)
        Paint('1.Initial State')   
        CheckHash(0x5F20CF032E36FB91)  
        Key.Pressed(Enter)
        Paint('2. Update is selected (underlined)')   
        CheckHash(0xC3E317298CA63C61)  
        Key.Pressed(Tab)
        Key.Pressed(Space)
        Paint('3. <> is selected, () & {} are not, Update is still selected')   
        CheckHash(0xB1A37854E603B011)  
        Key.Pressed(Tab)
        Key.Pressed(Space)
        Paint('4. () is selected, <> & {} are not, Update is still selected')   
        CheckHash(0x8842F6BFD83CD8A5)  
        Key.Pressed(Tab)
        Key.Pressed(Space)
        Paint('5. {} is selected, () & <> are not, Update is still selected')   
        CheckHash(0xBEFCBDB7BF12FF5D)  
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%");
    w.add(togglebutton!("<>,'Some button',x:1,y:3,w:2, group: true, type: Underlined"));
    w.add(togglebutton!("(),'Some button 2',x:4,y:3,w:2, group: true, type: Underlined"));
    w.add(togglebutton!(
        "'{}','Some button 3',x:7,y:3,w:2, single_selection: true, type: Underlined"
    ));
    w.add(togglebutton!("'Update','Some button 4',x:30,y:3,w:6, type: Underlined"));

    a.add_window(w);
    a.run();
}
