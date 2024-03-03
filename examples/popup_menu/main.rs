use appcui::prelude::*;
mod my_custom_control;
use my_custom_control::MyCustomControl;

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(80, 24)).menu().build()?;
    let mut w = window!("Title,d:c,w:76,h:10");
    w.add(MyCustomControl::new(Layout::new("d:c,w:4,h:2")));
    w.add(label!("'Press the right mouse button on the square below to show a popup menu',x:37,y:1,a:c,w:70,h:1"));
    a.add_window(w);
    a.run();
    Ok(())
}  