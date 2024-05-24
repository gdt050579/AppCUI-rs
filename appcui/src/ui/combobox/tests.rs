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
    let c = ComboBox::new(Layout::new("x:1,y:1,w:30"),combobox::Flags::None);
    w.add(c);
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
    let mut c = ComboBox::new(Layout::new("x:1,y:1,w:30"),combobox::Flags::None);
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
    let mut c = ComboBox::new(Layout::new("x:1,y:1,w:30"),combobox::Flags::None);
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
        h: Handle<ComboBox>
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Win,x:1,y:1,w:38,h:8"),
                h: Handle::None,
            };
            let mut c = ComboBox::new(Layout::new("x:1,y:1,w:30"),combobox::Flags::None);
            c.add("option 1");
            c.add("option 2");
            c.add("option 3");
            c.set_index(1);
            w.h = w.add(c);
            w
        }
    }
    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self,commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"),"Clear",mywin::Commands::A);
        }
    
        fn on_event(&mut self,command_id:mywin::Commands) {
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
        h: Handle<ComboBox>
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Win,x:1,y:1,w:38,h:8"),
                h: Handle::None,
            };
            let mut c = ComboBox::new(Layout::new("x:1,y:1,w:30"),combobox::Flags::None);
            c.add("option 1");
            c.add("option 2");
            c.add("option 3");
            c.set_index(1);
            w.h = w.add(c);
            w
        }
    }
    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self,commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"),"Clear",mywin::Commands::A);
        }
    
        fn on_event(&mut self,command_id:mywin::Commands) {
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