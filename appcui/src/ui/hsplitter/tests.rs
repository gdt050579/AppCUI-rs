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