use appcui::prelude::*;


fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Test,d:c,w:60,h:10");
    let tg1 = ToggleButton::new("Aa", "Case sensitive", Layout::new("x:1,y:1,w:2,h:1"), togglebutton::Type::Normal);
    let tg2 = ToggleButton::new("<>", "Block the size of the window", Layout::new("x:3,y:1,w:2,h:1"), togglebutton::Type::Normal);
    let tg3 = ToggleButton::new("Auto-Update", "Block the size of the window", Layout::new("x:5,y:1,w:13,h:1"), togglebutton::Type::Normal);
    w.add(tg1);
    w.add(tg2);     
    w.add(tg3);     
    a.add_window(w);
    a.run();
    Ok(())
}
