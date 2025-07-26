use vline::Flags;
use crate::prelude::*;

#[test]
fn check_create(){
    let script = "
        Paint.Enable(false)
        Paint('Initial State')
        CheckHash(0xB375CF2B6D717392)
    ";
    let mut a = App::debug(60, 40, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:25,flags:Sizeable");

    w.add(VLine::new( layout!("x:1,y:1,h:10"), Flags::None));
    w.add(VLine::new( layout!("x:3,y:1,h:20"), Flags::DoubleLine));
    a.add_window(w);
    a.run();
}

#[test]
fn check_procmacro(){
    let script = "
        Paint.Enable(false)
        Paint('Initial State')
        CheckHash(0xB375CF2B6D717392)
    ";
    let mut a = App::debug(60, 40, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:25,flags:Sizeable");
    
    w.add(vline!("x:1,y:1,h:10"));
    w.add(vline!("x:3,y:1,h:20, flags:DoubleLine"));
    a.add_window(w);
    a.run();
}