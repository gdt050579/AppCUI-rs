use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().theme(Theme::new(Themes::DarkGray)).color_schema(false).build()?;
    let mut w = window!("Test,a:c,w:60,h:15,type:Panel");
    let mut p1 = panel!("Options,x:1,y:1,w:25,h:8,type:Sunken");
    let mut p2 = panel!("Cheeckboxes,x:32,y:1,w:25,h:8,type:Sunken");
    p1.add(radiobox!("'Option &1',x:1,y:1,w:20,type:Target,selected: true"));
    p1.add(radiobox!("'Option &2',x:1,y:2,w:20,type:Target"));
    p1.add(radiobox!("'Option &3',x:1,y:3,w:20,type:Target"));
    p1.add(radiobox!("'Option &4',x:1,y:4,w:20,type:Target,enable:false"));
    w.add(p1);
    p2.add(checkbox!("'Checkbox &1',x:1,y:1,w:20,type:FilledBox,checked: true"));
    p2.add(checkbox!("'Checkbox &2',x:1,y:2,w:20,type:FilledBox"));
    p2.add(checkbox!("'Checkbox &3',x:1,y:3,w:20,type:FilledBox,checked: true"));
    p2.add(checkbox!("'Checkbox &4',x:1,y:4,w:20,type:FilledBox,checked: true,enable: false"));
    w.add(p2);
    w.add(button!("&Ok,r:12,b:0,w:10,type:Raised"));
    w.add(button!("&Cancel,r:0,b:0,w:10,type:Raised"));
    w.add(button!("&Inactive,l:0,b:0,w:10,type:Raised,enable:false"));
    app.add_window(w);
    app.run();
    Ok(())
}
