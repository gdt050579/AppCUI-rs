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
    let mut vs = VSplitter::new(0.5,Layout::new("d:c,w:100%,h:100%"),vsplitter::ResizeBehavior::PreserveAspectRatio);
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
    let mut vs = VSplitter::new(24,Layout::new("d:c,w:100%,h:100%"),vsplitter::ResizeBehavior::PreserveAspectRatio);
    vs.set_min_width(vsplitter::Panel::Left, 5);
    vs.set_min_width(vsplitter::Panel::Right, 5);
    vs.add(vsplitter::Panel::Left,panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right,panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_buttons() {
    let script = "
        Paint.Enable(false)        
        Paint('Initial state')   
        CheckHash(0xE2548C89F72469D9)
        Mouse.Click(30,2,left)
        Paint('2. Left most')   
        CheckHash(0xB617F953476DE6E2)
        Mouse.Click(11,3,left)
        Paint('3. Right most')   
        CheckHash(0x1E6CE665E12C30FE)
        Mouse.Click(48,2,left)
        Paint('4. Left most')   
        CheckHash(0xB617F953476DE6E2)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(24,Layout::new("d:c,w:100%,h:100%"),vsplitter::ResizeBehavior::PreserveAspectRatio);
    vs.set_min_width(vsplitter::Panel::Left, 5);
    vs.set_min_width(vsplitter::Panel::Right, 5);
    vs.add(vsplitter::Panel::Left,panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right,panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_movement() {
    let script = "
        Paint.Enable(false)        
        //Error.Disable(true)
        Paint('Initial state')   
        CheckHash(0xE2548C89F72469D9)
        Mouse.Move(30,2)
        Paint('2. Left Button Hovered')   
        CheckHash(0x1904C6CB28C38017)
        Mouse.Move(30,3)
        Paint('3. Right Button Hovered')   
        CheckHash(0xC28A2015A90E8933)
        Mouse.Move(30,5)
        Paint('4. Splitter bar hovered')   
        CheckHash(0x16A315627F3D6011)
        Mouse.Drag(30,5,25,5)
        Paint('5. Splitter bar moved')   
        CheckHash(0xE4252AD91821669D)
        Mouse.Hold(25,5,left)
        Paint('6. Splitter bar pressed')   
        CheckHash(0xC2F66E9DABD4BB05)
        Mouse.Move(35,5)
        Paint('7. Splitter bar pressed (right)')   
        CheckHash(0xBB6A43024EE639B5)
        Mouse.Release(35,5,left)
        Paint('8. Splitter bar hovered')   
        CheckHash(0x101F59917065395D)
        Mouse.Move(30,5)
        Paint('9. Splitter bar not-hovered')   
        CheckHash(0x550B43D780CCCBDD)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(24,Layout::new("d:c,w:100%,h:100%"),vsplitter::ResizeBehavior::PreserveAspectRatio);
    vs.set_min_width(vsplitter::Panel::Left, 5);
    vs.set_min_width(vsplitter::Panel::Right, 5);
    vs.add(vsplitter::Panel::Left,panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right,panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}