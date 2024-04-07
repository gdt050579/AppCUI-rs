use appcui::prelude::*;


fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Example,d:c,w:140,h:20");
    let mut ac = accordion!("l:1,t:1,r:1,b:1,panels:['Panel &1','Panel &2','Panel &3']");
    ac.add(0, radiobox!("'Option one',x:1,y:1,w:20, selected: true"));
    ac.add(0, radiobox!("'Option two',x:1,y:2,w:20"));
    ac.add(0, radiobox!("'Option three',x:1,y:3,w:20"));
    ac.add(0, radiobox!("'Option four',x:1,y:4,w:20"));
    w.add(ac);
    a.add_window(w);
    a.run();
    Ok(())
}
