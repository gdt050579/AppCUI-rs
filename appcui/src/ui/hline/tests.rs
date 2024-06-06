use hline::Flags;

use crate::prelude::*;

#[test]
fn check_create() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')
        CheckHash(0xC5491A50D5507086)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8,flags:Sizeable");
    
    w.add(HLine::new("", Layout::new("x:1,y:1,w:10"), Flags::None));
    w.add(HLine::new("TestLine", Layout::new("x:1,y:3,w:30"), Flags::DoubleLine | Flags::HasTitle));
    a.add_window(w);
    a.run();
}

#[test]
fn check_title_too_large() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')
        CheckHash(0x50E2B35DE67D69AB)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8,flags:Sizeable");
    
    w.add(HLine::new("ThisIsAVeryLargeTitle", Layout::new("x:1,y:3,w:20"), Flags::DoubleLine | Flags::HasTitle));
    a.add_window(w);
    a.run();
}


#[test]
fn check_title_with_line_too_small() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')
        CheckHash(0x9E92EA6BBE2344EA)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8,flags:Sizeable");
    
    w.add(HLine::new("ThisIsAVeryLargeTitle", Layout::new("x:1,y:3,w:4"), Flags::DoubleLine | Flags::HasTitle));
    a.add_window(w);
    a.run();
}

#[test]
fn check_create_procmacro() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')
        CheckHash(0xC5491A50D5507086)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8,flags:Sizeable");
    
    w.add(hline!("x:1,y:1,w:10"));
    w.add(hline!("TestLine,x:1,y:3,w:30,flags:DoubleLine+HasTitle"));
    a.add_window(w);
    a.run();
}