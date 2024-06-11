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

#[test]
fn check_mouse_drag() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (spliter at y:15)')   
        CheckHash(0xA9EBF9E5AF1C996E)
        Mouse.Move(10,15)
        Paint('2. Hover over splitter')   
        CheckHash(0xDD8DB62E8EE34896)
        Mouse.Hold(10,15,left)
        Paint('3. Press mouse over the splitter')   
        CheckHash(0xEF2B96EADD67B66)
        Mouse.Move(10,11)
        Paint('4. Move splitter upper (spliter at y:11)')   
        CheckHash(0x7851C97E8F4CF2A6)
        Mouse.Move(10,0)
        Paint('5. Top most (spliter at y:6) and highligheed')   
        CheckHash(0x3E5331A6A833E486)
        Mouse.Move(10,15)
        Paint('6. Move to original position (spliter at y:15)')   
        CheckHash(0xEF2B96EADD67B66)
        Mouse.Move(10,55)
        Paint('7. Bottom Most (spliter at y:24)')   
        CheckHash(0x14CA40672CB2CAF6)
        Mouse.Move(10,17)
        Paint('8. Move to another pos (spliter at y:17)')   
        CheckHash(0x1C2EDF83228C0096)
        Mouse.Release(10,17,left)
        Paint('9. Same position (no selection, but highlighed) (spliter at y:17)')   
        CheckHash(0x63F99D1AAC4D3606)
        Mouse.Move(10,6)
        Paint('10. Same position (no selection, no highlighed) (spliter at y:17)')   
        CheckHash(0xC1C21089B42AE13E)
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
fn check_resize_preserve_aspect_ratio() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (spliter at y:9)')   
        CheckHash(0x86FBFACC75BE4F5D)
        Mouse.Drag(54,19,54,8)
        Paint('Same height (spliter at y:5)')   
        CheckHash(0x4944D9828A81B459)
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:20,flags: Sizeable");
    w.add(hsplitter!("50%,d:c,w:100%,h:100%"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_resize_preserve_bottom_size() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (spliter at y:13) - bottom height = 5')   
        CheckHash(0x73FBA06DE697449D)
        Mouse.Drag(54,19,54,8)
        Paint('Same height (spliter at y:2) - bottom height = 5')   
        CheckHash(0x68B1E9E699BB2E9)
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:20,flags: Sizeable");
    w.add(hsplitter!("75%,d:c,w:100%,h:100%,rb:PreserveBottomPanelSize"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_resize_preserve_top_size() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (spliter at y:5) - top height = 4')   
        CheckHash(0xB75D3BE3E241C5D)
        Mouse.Drag(54,19,54,8)
        Paint('Same height (spliter at y:5) - top height = 4')   
        CheckHash(0x8138F6FBC5169A49)
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:20,flags: Sizeable");
    w.add(hsplitter!("25%,d:c,w:100%,h:100%,rb:PreserveTopPanelSize"));
    a.add_window(w);
    a.run();
}