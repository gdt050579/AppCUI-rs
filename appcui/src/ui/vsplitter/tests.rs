use crate::prelude::*;

#[test]
fn check_control_reposition() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xE2548C89F72469D9)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(0.5,Layout::new("d:c,w:100%,h:100%"),vsplitter::Flags::None);
    vs.add(vsplitter::Panel::Left,panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right,panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_keyboard_movement() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xE2548C89F72469D9)
        Key.Pressed(Ctrl+Alt+Left)
        Paint('2. Left panel smaller by 1')   
        CheckHash(0x3A5186CCBBF6BF3D)
        Key.Pressed(Ctrl+Alt+Right)
        Paint('3. Back to original state')   
        CheckHash(0xE2548C89F72469D9)
        Key.Pressed(Ctrl+Alt+Left,100)
        Paint('4. Left most')   
        CheckHash(0xB617F953476DE6E2)
        Key.Pressed(Ctrl+Alt+Right,100)
        Paint('5. Right most')   
        CheckHash(0x1E6CE665E12C30FE)
        Key.Pressed(Ctrl+Alt+Shift+Left)
        Paint('6. Left most')   
        CheckHash(0xB617F953476DE6E2)
        Key.Pressed(Ctrl+Alt+Shift+Right)
        Paint('7. Right most')   
        CheckHash(0x1E6CE665E12C30FE)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(24,Layout::new("d:c,w:100%,h:100%"),vsplitter::Flags::None);
    vs.set_min_width(vsplitter::Panel::Left, 5);
    vs.set_min_width(vsplitter::Panel::Right, 5);
    vs.add(vsplitter::Panel::Left,panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right,panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}