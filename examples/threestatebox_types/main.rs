use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = window!("'ThreeStateBox types',a:c,w:80,h:15, flags:Sizeable");
    let mut a = accordion!("d:c,w:100%,h:100%,panels:['&Standard', '&Ascii', 'Check&Box', 'Check&Mark', '&FilledBox', '&Yes/No', '&PlusMinus']");
    
    // Standard
    a.add(0, threestatebox!("'Option 1 (checked)',x:1,y:1,w:40,state:checked"));
    a.add(0, threestatebox!("'Option 2 (unchecked)',x:1,y:2,w:40,state:unchecked"));
    a.add(0, threestatebox!("'Option 3 (unknown)',x:1,y:3,w:40,state:unknown"));
    a.add(0, threestatebox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,state:checked,enabled:false"));

    // Ascii
    a.add(1, threestatebox!("'Option 1 (checked)',x:1,y:1,w:40,type=Ascii,state:checked"));
    a.add(1, threestatebox!("'Option 2 (unchecked)',x:1,y:2,w:40,type=Ascii,state:unchecked"));
    a.add(1, threestatebox!("'Option 3 (unknown)',x:1,y:3,w:40,type=Ascii,state:unknown"));
    a.add(1, threestatebox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=Ascii,state:checked,enabled:false"));

    // CheckBox
    a.add(2, threestatebox!("'Option 1 (checked)',x:1,y:1,w:40,type=Checkbox,state:checked"));
    a.add(2, threestatebox!("'Option 2 (unchecked)',x:1,y:2,w:40,type=Checkbox,state:unchecked"));
    a.add(2, threestatebox!("'Option 3 (unknown)',x:1,y:3,w:40,type=Checkbox,state:unknown"));
    a.add(2, threestatebox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=Checkbox,state:checked,enabled:false"));

    // CheckMark
    a.add(3, threestatebox!("'Option 1 (checked)',x:1,y:1,w:40,type=Checkmark,state:checked"));
    a.add(3, threestatebox!("'Option 2 (unchecked)',x:1,y:2,w:40,type=Checkmark,state:unchecked"));
    a.add(3, threestatebox!("'Option 3 (unknown)',x:1,y:3,w:40,type=Checkmark,state:unknown"));
    a.add(3, threestatebox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=Checkmark,state:checked,enabled:false"));

    // FilledBox
    a.add(4, threestatebox!("'Option 1 (checked)',x:1,y:1,w:40,type=FilledBox,state:checked"));
    a.add(4, threestatebox!("'Option 2 (unchecked)',x:1,y:2,w:40,type=FilledBox,state:unchecked"));
    a.add(4, threestatebox!("'Option 3 (unknown)',x:1,y:3,w:40,type=FilledBox,state:unknown"));
    a.add(4, threestatebox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=FilledBox,state:checked,enabled:false"));

    // Yes/No
    a.add(5, threestatebox!("'Option 1 (checked)',x:1,y:1,w:40,type=YesNo,state:checked"));
    a.add(5, threestatebox!("'Option 2 (unchecked)',x:1,y:2,w:40,type=YesNo,state:unchecked"));
    a.add(5, threestatebox!("'Option 3 (unknown)',x:1,y:3,w:40,type=YesNo,state:unknown"));
    a.add(5, threestatebox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=YesNo,state:checked,enabled:false"));

    // PlusMinus
    a.add(6, threestatebox!("'Option 1 (checked)',x:1,y:1,w:40,type=PlusMinus,state:checked"));
    a.add(6, threestatebox!("'Option 2 (unchecked)',x:1,y:2,w:40,type=PlusMinus,state:unchecked"));
    a.add(6, threestatebox!("'Option 3 (unknown)',x:1,y:3,w:40,type=PlusMinus,state:unknown"));
    a.add(6, threestatebox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=PlusMinus,state:checked,enabled:false"));

    w.add(a);
    app.add_window(w);
    app.run();
    Ok(())
} 