use appcui::prelude::*;


fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().app_bar().build()?;
    let mut w = window!("'Test',a:c,w:30,h:10");
    let tp = TimePicker::new(12, 30, 0, layout!("x:1,y:1,w:10,h:1"), timepicker::Flags::Seconds);
    w.add(tp);  
    app.add_window(w);
    app.run();
    Ok(())
}
