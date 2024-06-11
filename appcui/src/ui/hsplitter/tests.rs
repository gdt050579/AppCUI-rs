use crate::prelude::*;

#[test]
fn check_create() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xEFFB94F1E8AA29ED)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:51,h:11,flags: Sizeable");
    let mut hs = HSplitter::new(0.5,Layout::new("d:c,w:100%,h:100%"),hsplitter::ResizeBehavior::PreserveAspectRatio);
    hs.add(hsplitter::Panel::Top,panel!("Top,l:1,r:1,t:1,b:1"));
    hs.add(hsplitter::Panel::Bottom,panel!("Bottom,l:1,r:1,t:1,b:1"));
    w.add(hs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_create_procmacro() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xEFFB94F1E8AA29ED)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:51,h:11,flags: Sizeable");
    let mut hs = hsplitter!("pos:50%,d:c,w:100%,h:100%");
    hs.add(hsplitter::Panel::Top,panel!("Top,l:1,r:1,t:1,b:1"));
    hs.add(hsplitter::Panel::Bottom,panel!("Bottom,l:1,r:1,t:1,b:1"));
    w.add(hs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_keyboard_movement() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (spliter at y:15)')   
        CheckHash(0xA9EBF9E5AF1C996E)
        Key.Pressed(Ctrl+Alt+Up)
        Paint('2. Top panel smaller by 1 (spliter at y:14)')   
        CheckHash(0x1B65D92441AE29EE)
        Key.Pressed(Ctrl+Alt+Down)
        Paint('3. Back to original state (spliter at y:15)')   
        CheckHash(0xA9EBF9E5AF1C996E)
        Key.Pressed(Ctrl+Alt+Up,100)
        Paint('4. Top most (spliter at y:6)')   
        CheckHash(0xFD38E7685209A5EE)
        Key.Pressed(Ctrl+Alt+Down,100)
        Paint('5. Bottom most (spliter at y:24)')   
        CheckHash(0x2FA69898CA587A1E)
        Key.Pressed(Ctrl+Alt+Shift+Up)
        Paint('6. Top most (spliter at y:6)')   
        CheckHash(0xFD38E7685209A5EE)
        Key.Pressed(Ctrl+Alt+Shift+Down)
        Paint('7. Bottom most (spliter at y:24)')   
        CheckHash(0x2FA69898CA587A1E)
    ";
    let mut a = App::debug(60, 30, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:30,flags: Sizeable");
    let mut hs = HSplitter::new(14,Layout::new("d:c,w:100%,h:100%"),hsplitter::ResizeBehavior::PreserveAspectRatio);
    hs.set_min_height(hsplitter::Panel::Top, 5);
    hs.set_min_height(hsplitter::Panel::Bottom, 4);
    hs.add(hsplitter::Panel::Top,panel!("Top,l:1,r:1,t:1,b:1"));
    hs.add(hsplitter::Panel::Bottom,panel!("Bottom,l:1,r:1,t:1,b:1"));
    w.add(hs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_keyboard_movement_with_procmacro() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (spliter at y:15)')   
        CheckHash(0xA9EBF9E5AF1C996E)
        Key.Pressed(Ctrl+Alt+Up)
        Paint('2. Top panel smaller by 1 (spliter at y:14)')   
        CheckHash(0x1B65D92441AE29EE)
        Key.Pressed(Ctrl+Alt+Down)
        Paint('3. Back to original state (spliter at y:15)')   
        CheckHash(0xA9EBF9E5AF1C996E)
        Key.Pressed(Ctrl+Alt+Up,100)
        Paint('4. Top most (spliter at y:6)')   
        CheckHash(0xFD38E7685209A5EE)
        Key.Pressed(Ctrl+Alt+Down,100)
        Paint('5. Bottom most (spliter at y:24)')   
        CheckHash(0x2FA69898CA587A1E)
        Key.Pressed(Ctrl+Alt+Shift+Up)
        Paint('6. Top most (spliter at y:6)')   
        CheckHash(0xFD38E7685209A5EE)
        Key.Pressed(Ctrl+Alt+Shift+Down)
        Paint('7. Bottom most (spliter at y:24)')   
        CheckHash(0x2FA69898CA587A1E)
    ";
    let mut a = App::debug(60, 30, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:30,flags: Sizeable");
    let mut hs = hsplitter!("pos:14,d:c,w:100%,h:100%,mth:5,mbh:4");
    hs.add(hsplitter::Panel::Top,panel!("Top,l:1,r:1,t:1,b:1"));
    hs.add(hsplitter::Panel::Bottom,panel!("Bottom,l:1,r:1,t:1,b:1"));
    w.add(hs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_buttons() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (spliter at y:15)')   
        CheckHash(0xA9EBF9E5AF1C996E)
        Mouse.Move(7,15)
        Paint('2. Hover over Up button')   
        CheckHash(0x14FD6D04583F4D18)
        Mouse.Move(8,15)
        Paint('3. Hover over Down button')   
        CheckHash(0xE36BC3FF8A484AFC)
        Mouse.Click(7,15,left)
        Paint('4. Top most (spliter at y:6)')   
        CheckHash(0xFD38E7685209A5EE)
        Mouse.Click(8,6,left)
        Paint('5. Bottom most (spliter at y:24)')   
        CheckHash(0x2FA69898CA587A1E)
    ";
    let mut a = App::debug(60, 30, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:30,flags: Sizeable");
    let mut hs = HSplitter::new(14,Layout::new("d:c,w:100%,h:100%"),hsplitter::ResizeBehavior::PreserveAspectRatio);
    hs.set_min_height(hsplitter::Panel::Top, 5);
    hs.set_min_height(hsplitter::Panel::Bottom, 4);
    hs.add(hsplitter::Panel::Top,panel!("Top,l:1,r:1,t:1,b:1"));
    hs.add(hsplitter::Panel::Bottom,panel!("Bottom,l:1,r:1,t:1,b:1"));
    w.add(hs);
    a.add_window(w);
    a.run();
}