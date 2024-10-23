use appcui::prelude::*;


fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Test,d:c,w:50,h:11,flags: Sizeable");
    let t = TimePicker::new(Layout::new("d:c,w:2"), timepicker::Flags::AMPMFormat | timepicker::Flags::Miliseconds);
    w.add(t);
    w.add(button!("&Press, d:b, w:10"));
    a.add_window(w);
    a.run();
    Ok(())
}