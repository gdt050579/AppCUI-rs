use crate::prelude::*;
use chrono::NaiveTime;

#[test]
fn check_drawing_capability() {
    let script = "
        Paint.Enable(false)
        Paint('Test initialization')   
        CheckHash(0x9BD18D91075A6D13)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    let t1 = TimePicker::new(NaiveTime::from_hms_milli_opt(1, 0, 0, 123).unwrap(), Layout::new("x:1, y:1, w:2"), timepicker::Flags::HM);
    let t2 = TimePicker::new(NaiveTime::from_hms_milli_opt(2, 0, 0, 123).unwrap(), Layout::new("x:1, y:2, w:2"), timepicker::Flags::HMS);
    let t3 = TimePicker::new(NaiveTime::from_hms_milli_opt(3, 0, 0, 123).unwrap(), Layout::new("x:1, y:3, w:2"), timepicker::Flags::HMS_MS);
    let t4 = TimePicker::new(NaiveTime::from_hms_milli_opt(4, 0, 0, 123).unwrap(), Layout::new("x:1, y:4, w:2"), timepicker::Flags::HM_AMPM);
    let t5 = TimePicker::new(NaiveTime::from_hms_milli_opt(15, 0, 0, 123).unwrap(), Layout::new("x:1, y:5, w:2"), timepicker::Flags::HMS_AMPM);
    let t6 = TimePicker::new(NaiveTime::from_hms_milli_opt(16, 0, 0, 123).unwrap(), Layout::new("x:1, y:6, w:2"), timepicker::Flags::HMS_MS_AMPM);
    w.add(t1);
    w.add(t2);
    w.add(t3);
    w.add(t4);
    w.add(t5);
    w.add(t6);
    a.add_window(w);
    a.run();
}