use crate::prelude::*;

#[test]
fn check() {
    let script = "
        //Paint.Enable(false)        
        Key.Pressed(Tab)        
        Paint('Abc')
        Key.Pressed(A)
        Paint('after')
        Key.Pressed(B)
        Paint('after')
        Key.Pressed(C)
        Paint('after')        
    ";
    let mut a = App::debug(80, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:60,h:10");
    let p = PathFinder::new(
        r#"c:\program files\windows\start\go\hello\games"#, 
        Layout::new("x:1,y:1,w:30"), 
        pathfinder::Flags::None);
    w.add(p);
    w.add(button!("test,x:1,y:3,w:6"));
    a.add_window(w);    
    a.run();    
}