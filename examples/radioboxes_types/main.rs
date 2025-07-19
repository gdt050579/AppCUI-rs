use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut win = window!("'Radiobox types',a:c,w:80,h:15, flags:Sizeable");
    let mut acc = accordion!("d:c,w:100%,h:100%,panels:['&Standard', '&Circle', '&Diamond', '&Ascii']");
    
    // Standard
    acc.add(0, radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,selected:false"));
    acc.add(0, radiobox!("'Option 2 (selected)',x:1,y:2,w:40,selected:true"));
    acc.add(0, radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,selected:false, enabled:false"));

    // Circle
    acc.add(1, radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,type=Circle,selected:false"));
    acc.add(1, radiobox!("'Option 2 (selected)',x:1,y:2,w:40,type=Circle,selected:true"));
    acc.add(1, radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,type=Circle,selected:false, enabled:false"));

    // Diamond
    acc.add(2, radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,type=Diamond,selected:false"));
    acc.add(2, radiobox!("'Option 2 (selected)',x:1,y:2,w:40,type=Diamond,selected:true"));
    acc.add(2, radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,type=Diamond,selected:false, enabled:false"));

    // Ascii
    acc.add(3, radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,type=Ascii,selected:false"));
    acc.add(3, radiobox!("'Option 2 (selected)',x:1,y:2,w:40,type=Ascii,selected:true"));
    acc.add(3, radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,type=Ascii,selected:false, enabled:false"));

    win.add(acc);
    app.add_window(win);
    app.run();
    Ok(())
} 