use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = window!("'Radiobox types',d:c,w:80,h:15, flags:Sizeable");
    let mut a = accordion!("d:c,w:100%,h:100%,panels:['&Standard', '&Circle', '&Diamond', '&Square', '&Star', '&Dot']");
    
    // Standard
    a.add(0, radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,selected:false"));
    a.add(0, radiobox!("'Option 2 (selected)',x:1,y:2,w:40,selected:true"));
    a.add(0, radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,selected:false, enabled:false"));
    a.add(0, radiobox!("'Option 4 (disabled and selected)',x:1,y:4,w:40,selected:true, enabled:false"));

    // Circle
    a.add(1, radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,type=Circle,selected:false"));
    a.add(1, radiobox!("'Option 2 (selected)',x:1,y:2,w:40,type=Circle,selected:true"));
    a.add(1, radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,type=Circle,selected:false, enabled:false"));
    a.add(1, radiobox!("'Option 4 (disabled and selected)',x:1,y:4,w:40,type=Circle,selected:true, enabled:false"));

    // Diamond
    a.add(2, radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,type=Diamond,selected:false"));
    a.add(2, radiobox!("'Option 2 (selected)',x:1,y:2,w:40,type=Diamond,selected:true"));
    a.add(2, radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,type=Diamond,selected:false, enabled:false"));
    a.add(2, radiobox!("'Option 4 (disabled and selected)',x:1,y:4,w:40,type=Diamond,selected:true, enabled:false"));

    // Square
    a.add(3, radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,type=Square,selected:false"));
    a.add(3, radiobox!("'Option 2 (selected)',x:1,y:2,w:40,type=Square,selected:true"));
    a.add(3, radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,type=Square,selected:false, enabled:false"));
    a.add(3, radiobox!("'Option 4 (disabled and selected)',x:1,y:4,w:40,type=Square,selected:true, enabled:false"));

    // Star
    a.add(4, radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,type=Star,selected:false"));
    a.add(4, radiobox!("'Option 2 (selected)',x:1,y:2,w:40,type=Star,selected:true"));
    a.add(4, radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,type=Star,selected:false, enabled:false"));
    a.add(4, radiobox!("'Option 4 (disabled and selected)',x:1,y:4,w:40,type=Star,selected:true, enabled:false"));

    // Dot
    a.add(5, radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,type=Dot,selected:false"));
    a.add(5, radiobox!("'Option 2 (selected)',x:1,y:2,w:40,type=Dot,selected:true"));
    a.add(5, radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,type=Dot,selected:false, enabled:false"));
    a.add(5, radiobox!("'Option 4 (disabled and selected)',x:1,y:4,w:40,type=Dot,selected:true, enabled:false"));

    w.add(a);
    app.add_window(w);
    app.run();
    Ok(())
} 