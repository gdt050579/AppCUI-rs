use crate::prelude::*;

#[test]
fn check() {    
    let script = "
        //Paint.Enable(false)        
        Paint('Initial')
        Key.Pressed(Tab)        
        Paint('After Tab')
        Key.Pressed(A)         
        Key.Pressed(Backspace)
        Key.Pressed(Down, 6)         
        Paint('after')                
    ";
    let mut a = App::debug(80, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:60,h:15");
    let p = PathFinder::<crate::utils::fs::Navigator>::new(
        r#"D:\work\Projects\BDAntiCryptoLockerUnified\RemovalToolUnifiedDropper\RemovalToolUnifiedDropper.sln"#, 
        Layout::new("x:1,y:1,w:30"), 
        pathfinder::Flags::None);
    w.add(p);
    w.add(button!("test,x:1,y:3,w:6"));
    a.add_window(w);    
    a.run();    
}
    
#[test]
fn run() -> Result<(), crate::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Test,d:c,w:100,h:15");
    let p = PathFinder::<crate::utils::fs::Navigator>::new(
        r#"D:\work\Projects\BDAntiCryptoLockerUnified\RemovalToolUnifiedDropper\RemovalToolUnifiedDropper.sln"#, 
        Layout::new("x:1,y:1,w:60"), 
        pathfinder::Flags::None);           
    w.add(p);   
    w.add(button!("test,x:1,y:4,w:6"));
    a.add_window(w);    
    a.run();
    Ok(())    
}       