use vline::Flags;
use crate::prelude::*;

#[test]
fn check_create(){
    let script = "
        Paint.Enable(false)
        Paint('Initial State')
        CheckHash(0xB375CF2B6D717392)
    ";
    App::new().size(Size::new(60, 40)).debug_script(script).window(|| {
        let mut w = window!("Title,a:c,w:40,h:25,flags:Sizeable");

        w.add(VLine::new( layout!("x:1,y:1,h:10"), Flags::None));
        w.add(VLine::new( layout!("x:3,y:1,h:20"), Flags::DoubleLine));
        w
    }).run().unwrap();
}

#[test]
fn check_procmacro(){
    let script = "
        Paint.Enable(false)
        Paint('Initial State')
        CheckHash(0xB375CF2B6D717392)
    ";
    App::new().size(Size::new(60, 40)).debug_script(script).window(|| {
        let mut w = window!("Title,a:c,w:40,h:25,flags:Sizeable");

        w.add(vline!("x:1,y:1,h:10"));
        w.add(vline!("x:3,y:1,h:20, flags:DoubleLine"));
        w
    }).run().unwrap();
}

#[test]
fn check_merge_borders(){
    let script = "
        Paint.Enable(false)
        Paint('Initial State')
        CheckHash(0x17B0E89C8C223D61)
    ";
    App::new().size(Size::new(60, 40)).debug_script(script).window(|| {
        let mut w = window!("Title,a:c,w:40,h:25,flags:Sizeable");

        w.add(VLine::new( layout!("x:7,y:0,h:100%"), Flags::MergeBorders));
        w.add(VLine::new( layout!("x:33,y:0,h:100%"), Flags::DoubleLine | Flags::MergeBorders));
        w
    }).run().unwrap();
}