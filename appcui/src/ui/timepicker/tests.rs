use crate::prelude::*;
use chrono::NaiveTime;

#[test]
fn check_create(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')
        CheckHash(0xFFD1CBFB9AAB7083)
        CheckCursor(22,6)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    w.add(TimePicker::new(NaiveTime::from_hms_opt(12, 30, 20).unwrap(), layout!("x:1,y:1,w:10"), timepicker::Flags::Seconds));
    w.add(TimePicker::new(NaiveTime::from_hms_opt(00, 00, 00).unwrap(), layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds));
    w.add(TimePicker::new(NaiveTime::from_hms_opt(23, 59, 59).unwrap(), layout!("x:1,y:5,w:10"), timepicker::Flags::Seconds));

    w.add(TimePicker::new(NaiveTime::from_hms_opt(12, 30, 20).unwrap(), layout!("x:20,y:1,w:10"), timepicker::Flags::None));
    w.add(TimePicker::new(NaiveTime::from_hms_opt(00, 00, 00).unwrap(), layout!("x:20,y:3,w:10"), timepicker::Flags::None));
    w.add(TimePicker::new(NaiveTime::from_hms_opt(23, 59, 59).unwrap(), layout!("x:20,y:5,w:10"), timepicker::Flags::None));

    a.add_window(w);
    a.run();
}


#[test]
fn check_create_ampm(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')
        CheckHash(0x7E6FFF4577329B6F)
        CheckCursor(22,6)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    w.add(TimePicker::new(NaiveTime::from_hms_opt(12, 30, 20).unwrap(), layout!("x:1,y:1,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM));
    w.add(TimePicker::new(NaiveTime::from_hms_opt(00, 00, 00).unwrap(), layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM));
    w.add(TimePicker::new(NaiveTime::from_hms_opt(23, 59, 59).unwrap(), layout!("x:1,y:5,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM));

    w.add(TimePicker::new(NaiveTime::from_hms_opt(12, 30, 20).unwrap(), layout!("x:20,y:1,w:10"), timepicker::Flags::AMPM));
    w.add(TimePicker::new(NaiveTime::from_hms_opt(00, 00, 00).unwrap(), layout!("x:20,y:3,w:10"), timepicker::Flags::AMPM));
    w.add(TimePicker::new(NaiveTime::from_hms_opt(23, 59, 59).unwrap(), layout!("x:20,y:5,w:10"), timepicker::Flags::AMPM));

    a.add_window(w);
    a.run();
}