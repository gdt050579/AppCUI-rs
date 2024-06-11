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
    let mut vs = HSplitter::new(0.5,Layout::new("d:c,w:100%,h:100%"),hsplitter::ResizeBehavior::PreserveAspectRatio);
    vs.add(hsplitter::Panel::Top,panel!("Top,l:1,r:1,t:1,b:1"));
    vs.add(hsplitter::Panel::Bottom,panel!("Bottom,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}