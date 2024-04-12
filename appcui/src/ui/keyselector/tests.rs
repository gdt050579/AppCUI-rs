use crate::prelude::*;

#[test]
fn check_on_paint() {
    let script = "
        Paint.Enable(false)
        Paint('tests')   
        CheckHash(0x7309311AC45F730F)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("test,d:c,w:40,h:10");
    w.add(KeySelector::new(
        Key::new(KeyCode::F1, KeyModifier::Ctrl),
        Layout::new("x:1,y:1,w:35,h:1"),
        keyselector::Flags::None,
    ));
    w.add(KeySelector::new(
        Key::new(KeyCode::Insert, KeyModifier::Ctrl | KeyModifier::Alt),
        Layout::new("x:1,y:3,w:35,h:1"),
        keyselector::Flags::None,
    ));
    let mut ks = KeySelector::new(
        Key::new(KeyCode::Escape, KeyModifier::Ctrl | KeyModifier::Shift),
        Layout::new("x:1,y:5,w:35,h:1"),
        keyselector::Flags::AcceptEscape,
    );
    ks.set_enabled(false);
    w.add(ks);
    a.add_window(w);
    a.run();
}

#[test]
fn check_macro() {
    let script = "
        Paint.Enable(false)
        Paint('macro tests')   
        CheckHash(0x598A2D33EC5D8CF0)
    ";
    let mut a = App::debug(60, 14, script).build().unwrap();
    let mut w = window!("test,d:c,w:40,h:14");
    w.add(keyselector!("F1,x:1,y:1,w:35,h:1"));
    w.add(keyselector!("x:1,y:3,w:35,h:1,key:'Ctrl+Alt+Insert'"));
    w.add(keyselector!("Ctrl+Shift+Escape,x:1,y:5,w:35,h:1,enable:false,flags:AcceptEscape"));
    w.add(keyselector!("Ctrl+Alt+K,x:1,y:7,w:35,h:1,flags:AcceptTab+ReadOnly"));
    w.add(keyselector!("x:1,y:9,w:35,h:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_tab_key() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(20,2,left)
        Mouse.Move(0,0)
        Paint('initial state (focus on first)')   
        CheckHash(0x1C6AC89BA2743EFE)
        Key.Pressed(Tab)
        // focus should be changed because the first keyselector is readonly
        Paint('Focus on second keyselector') 
        CheckHash(0x3111DF60C332BBAE)
        Key.Pressed(F1)  
        Paint('Second key selector now has F1') 
        CheckHash(0x38469025C1B0B847)
        Key.Pressed(Tab)
        Paint('Focus on 3rd keyselector') 
        CheckHash(0x8A84DADCB73BA9D7)
        Key.Pressed(Space)
        Paint('3rd key selector now has Space') 
        CheckHash(0x82CD8C72C8299691)
        Key.Pressed(Tab)
        // since 3rd keyselector accepts TAB - it should intercept it
        Paint('3rd key selector now has Tab') 
        CheckHash(0x1563BCFE7E234062)
        Mouse.Click(20,2,left)
        Mouse.Move(0,0)
        Paint('Back to first')  
        CheckHash(0x3F7B3D010FD1DF02)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("test,d:c,w:40,h:10");
    w.add(keyselector!("F1,x:1,y:1,w:35,h:1,flags:AcceptTab+ReadOnly"));
    w.add(keyselector!("x:1,y:3,w:35,h:1"));
    w.add(keyselector!("x:1,y:5,w:35,h:1,flags:AcceptTab"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_enter_key() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(20,2,left)
        Mouse.Move(0,0)
        Paint('initial state (focus on first)')   
        CheckHash(0xD32B06D73947F7A0)
        Key.Pressed(F5)
        Paint('Now the key is F5')   
        CheckHash(0x3FDB25EF5AE3C344)
        Key.Pressed(Enter)
        Paint('Now the key is Enter')   
        CheckHash(0x9E7ABA2EFC88CBF)
        Key.Pressed(Tab)
        Paint('Focus on the second keyselector')   
        CheckHash(0x18530A81C49D2DAF)
        Key.Pressed(Enter)
        Paint('Nothing changes - Enter is not captured')   
        CheckHash(0x18530A81C49D2DAF)
        Key.Pressed(Insert)
        Paint('Insert is the second keyselector')   
        CheckHash(0x5657C532B015EC26)
        Key.Pressed(Tab)
        Paint('Focus on the 3rd keyselector')   
        CheckHash(0xE96A44D9FC272DB6)
        Key.Pressed(Enter)
        Paint('Nothing changes - 3rd report is readonly')   
        CheckHash(0xE96A44D9FC272DB6)
        Key.Pressed(F3)
        Paint('Nothing changes - 3rd report is readonly (2)')   
        CheckHash(0xE96A44D9FC272DB6)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("test,d:c,w:40,h:10");
    w.add(keyselector!("F1,x:1,y:1,w:35,h:1,flags:AcceptEnter"));
    w.add(keyselector!("x:1,y:3,w:35,h:1"));
    w.add(keyselector!("F2,x:1,y:5,w:35,h:1,flags:[AcceptEnter,ReadOnly]"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_esc_key() {
    let script = "
        Paint.Enable(false)
        Mouse.Click(20,2,left)
        Mouse.Move(0,0)
        Paint('initial state (focus on first)')   
        CheckHash(0xD32B06D73947F7A0)
        Key.Pressed(F5)
        Paint('Now the key is F5')   
        CheckHash(0x3FDB25EF5AE3C344)
        Key.Pressed(Escape)
        Paint('Now the key is Escape')   
        CheckHash(0x4A4FCD3F96196F36)
        Key.Pressed(Tab,2)
        Paint('Focus on the 3rd keyselector')   
        CheckHash(0x5DB16390FC968356)
        Key.Pressed(Escape)
        Paint('Window is closed (escape is captured)')   
        CheckHash(0x734FECAF52FDE955)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("test,d:c,w:40,h:10");
    w.add(keyselector!("F1,x:1,y:1,w:35,h:1,flags:AcceptEscape"));
    w.add(keyselector!("x:1,y:3,w:35,h:1"));
    w.add(keyselector!("F2,x:1,y:5,w:35,h:1,flags:[AcceptEscape,ReadOnly]"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_events() {
    #[Window(events = KeySelectorEvents, internal=true)]
    struct MyWin {
        info: Handle<Label>,
        ks: Handle<KeySelector>,
    } 
    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: Window::new("Win", Layout::new("d:c,w:57,h:7"), window::Flags::None),
                info: Handle::None,
                ks: Handle::None,
            };
            me.info = me.add(Label::new("<none>", Layout::new("x:1,y:1,w:55,h:2")));
            me.ks = me.add(keyselector!("F1,x:1,y:3,w:35"));
            me
        }
        fn set_info(&mut self, txt: &str) {
            let h_label = self.info;
            if let Some(label) = self.control_mut(h_label) {
                label.set_caption(txt);
            }
        }
    }
    impl KeySelectorEvents for MyWin {
        fn on_key_changed(&mut self, _handle: Handle<KeySelector>, new_key: Key, old_key: Key) -> EventProcessStatus {
            let s = format!(
                "Old: {}{}\nNew: {}{}",
                old_key.modifier.name(),
                old_key.code.name(),
                new_key.modifier.name(),
                new_key.code.name()
            );
            self.set_info(&s);
            EventProcessStatus::Processed
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('initial state')   
        CheckHash(0xF7B889598DA3905D)
        Key.Pressed(Ctrl+Alt+Shift+F5)
        Paint('Now the key is F5')   
        CheckHash(0x8EDEC6861A791691)
   ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}
