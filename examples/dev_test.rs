use appcui::prelude::*;



fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Test,d:c,w:50,h:11,flags: Sizeable");
    let mut p = panel!("Test,l:10,t:1,b:1,r:1");
    let l = listbox!("d:c,w:100%,h:100%,flags: ScrollBars+HighlightSelectedItemWhenInactive, lsm:2, items=[1,2,3,4,5,6,7,8,9,10]");
    p.add(l);
    w.add(p);
    w.add(button!("Add,x:1,y:1,w:7,type:flat"));
    a.add_window(w);
    a.run();
    Ok(())
}