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