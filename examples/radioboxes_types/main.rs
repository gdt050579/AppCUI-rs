use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_os = "windows")]
    let mut app = App::with_backend(appcui::backend::Type::WindowsVT).color_schema(false).build()?;
    #[cfg(not(target_os = "windows"))]
    let mut app = App::new().color_schema(false).build()?;

    let mut win = window!("'Radiobox types',a:c,w:80,h:15, flags:Sizeable");
    let mut acc = accordion!("d:f,panels:['&Standard', '&Circle', '&Diamond', '&Ascii', '&Bullet']");
    
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

    // Bullet
    acc.add(4, radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,type=Bullet,selected:false"));
    acc.add(4, radiobox!("'Option 2 (selected)',x:1,y:2,w:40,type=Bullet,selected:true"));
    acc.add(4, radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,type=Bullet,selected:false, enabled:false"));

    win.add(acc);
    app.add_window(win);
    app.run();
    Ok(())
} 