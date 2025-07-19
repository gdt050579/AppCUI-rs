use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = window!("'Checkbox types',a:c,w:80,h:15, flags:Sizeable");
    let mut a = accordion!("a:c,w:100%,h:100%,panels:['&Standard', '&Ascii', 'Check&Box', 'Check&Mark', '&FilledBox', '&Yes/No', '&PlusMinus']");
    
    // Standard
    a.add(0, checkbox!("'Option 1 (not-checked)',x:1,y:1,w:40,checked:false"));
    a.add(0, checkbox!("'Option 2 (checked)',x:1,y:2,w:40,checked:true"));
    a.add(0, checkbox!("'Option 3 (disabled and not-checked)',x:1,y:3,w:40,checked:false, enabled:false"));
    a.add(0, checkbox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,checked:true, enabled:false"));

    // Ascii
    a.add(1, checkbox!("'Option 1 (not-checked)',x:1,y:1,w:40,type=Ascii,checked:false"));
    a.add(1, checkbox!("'Option 2 (checked)',x:1,y:2,w:40,type=Ascii,checked:true"));
    a.add(1, checkbox!("'Option 3 (disabled and not-checked)',x:1,y:3,w:40,type=Ascii,checked:false, enabled:false"));
    a.add(1, checkbox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=Ascii,checked:true, enabled:false"));

    // CheckBox
    a.add(2, checkbox!("'Option 1 (not-checked)',x:1,y:1,w:40,type=Checkbox,checked:false"));
    a.add(2, checkbox!("'Option 2 (checked)',x:1,y:2,w:40,type=Checkbox,checked:true"));
    a.add(2, checkbox!("'Option 3 (disabled and not-checked)',x:1,y:3,w:40,type=Checkbox,checked:false, enabled:false"));
    a.add(2, checkbox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=Checkbox,checked:true, enabled:false"));

    // CheckMark
    a.add(3, checkbox!("'Option 1 (not-checked)',x:1,y:1,w:40,type=Checkmark,checked:false"));
    a.add(3, checkbox!("'Option 2 (checked)',x:1,y:2,w:40,type=Checkmark,checked:true"));
    a.add(3, checkbox!("'Option 3 (disabled and not-checked)',x:1,y:3,w:40,type=Checkmark,checked:false, enabled:false"));
    a.add(3, checkbox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=Checkmark,checked:true, enabled:false"));

    // FilledBox
    a.add(4, checkbox!("'Option 1 (not-checked)',x:1,y:1,w:40,type=FilledBox,checked:false"));
    a.add(4, checkbox!("'Option 2 (checked)',x:1,y:2,w:40,type=FilledBox,checked:true"));
    a.add(4, checkbox!("'Option 3 (disabled and not-checked)',x:1,y:3,w:40,type=FilledBox,checked:false, enabled:false"));
    a.add(4, checkbox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=FilledBox,checked:true, enabled:false"));

    // Yes/No
    a.add(5, checkbox!("'Option 1 (not-checked)',x:1,y:1,w:40,type=YesNo,checked:false"));
    a.add(5, checkbox!("'Option 2 (checked)',x:1,y:2,w:40,type=YesNo,checked:true"));
    a.add(5, checkbox!("'Option 3 (disabled and not-checked)',x:1,y:3,w:40,type=YesNo,checked:false, enabled:false"));
    a.add(5, checkbox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=YesNo,checked:true, enabled:false"));

    // PlusMinus
    a.add(6, checkbox!("'Option 1 (not-checked)',x:1,y:1,w:40,type=PlusMinus,checked:false"));
    a.add(6, checkbox!("'Option 2 (checked)',x:1,y:2,w:40,type=PlusMinus,checked:true"));
    a.add(6, checkbox!("'Option 3 (disabled and not-checked)',x:1,y:3,w:40,type=PlusMinus,checked:false, enabled:false"));
    a.add(6, checkbox!("'Option 4 (disabled and checked)',x:1,y:4,w:40,type=PlusMinus,checked:true, enabled:false"));

    w.add(a);
    app.add_window(w);
    app.run();
    Ok(())
}