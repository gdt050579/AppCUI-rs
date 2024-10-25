use appcui::prelude::*;
use chrono::NaiveTime;


fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Test,d:c,w:50,h:11,flags: Sizeable");
    let t = TimePicker::new(NaiveTime::from_hms_milli_opt(11, 0, 0, 123).unwrap(), Layout::new("d:c,w:2"), timepicker::Flags::AMPMFormat | timepicker::Flags::Miliseconds);
    w.add(t);
    w.add(button!("&Press, d:b, w:10"));
    a.add_window(w);
    a.run();
    Ok(())
}