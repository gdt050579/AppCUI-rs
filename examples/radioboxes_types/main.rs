use appcui::prelude::*;

fn build_window() -> Window {
    let mut win = window!("'Radiobox types',a:c,w:80,h:15, flags:Sizeable");
    let mut acc = accordion!("d:f,panels:['&Standard', '&Circle', '&Diamond', '&Ascii', '&Bullet', '&Target']");

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

    // Target
    acc.add(5, radiobox!("'Option 1 (not-selected)',x:1,y:1,w:40,type=Target,selected:false"));
    acc.add(5, radiobox!("'Option 2 (selected)',x:1,y:2,w:40,type=Target,selected:true"));
    acc.add(5, radiobox!("'Option 3 (disabled and not-selected)',x:1,y:3,w:40,type=Target,selected:false, enabled:false"));

    win.add(acc);
    win
}

fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_os = "windows")]
    {
        App::new()
            .backend(appcui::backend::Type::WindowsVT)
            .color_schema(false)
            .window(build_window)
            .run()
    }
    #[cfg(not(target_os = "windows"))]
    {
        App::new().color_schema(false).window(build_window).run()
    }
}
