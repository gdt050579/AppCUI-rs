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
        CheckHash(0xCB62EDEF1A6707EB)
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

#[test]
fn check_ampm_increase_decrease(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State (12:00 AM)')
        CheckHash(0x19DF675B90685F2)
        Key.Pressed(Up)
        Paint('2. 01:00 AM')
        CheckHash(0x6C914F11E31D4DC)
        Key.Pressed(Up,10)
        Paint('3. 11:00 AM')
        CheckHash(0x96524CCB86D71521)
        Key.Pressed(Up)
        Paint('4. 12:00 PM')
        CheckHash(0x9D854A9EB065FAF)
        Key.Pressed(Up)
        Paint('5. 01:00 PM')
        CheckHash(0xEBD5BF5BE7BF4821)
        Key.Pressed(Up,10)
        Paint('6. 11:00 PM')
        CheckHash(0xB145A260BD49A1DC)
        Key.Pressed(Up)
        Paint('7. 12:00 AM')
        CheckHash(0x19DF675B90685F2)
        Key.Pressed(Down)
        Paint('8. 11:00 PM')
        CheckHash(0xB145A260BD49A1DC)
        Key.Pressed(Down,10)
        Paint('9. 01:00 PM')
        CheckHash(0xEBD5BF5BE7BF4821)
        Key.Pressed(Down)
        Paint('10. 12:00 PM')
        CheckHash(0x9D854A9EB065FAF)
        Key.Pressed(Down)
        Paint('11. 11:00 AM')
        CheckHash(0x96524CCB86D71521)
        Key.Pressed(Down,10)
        Paint('12. 01:00 AM')
        CheckHash(0x6C914F11E31D4DC)
        Key.Pressed(Down)
        Paint('13. Back Initial State (12:00 AM)')
        CheckHash(0x19DF675B90685F2)

    ";
    let mut a = App::debug(40, 8, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    w.add(TimePicker::new(NaiveTime::from_hms_opt(00, 00, 00).unwrap(), layout!("x:1,y:1,w:10"), timepicker::Flags::AMPM));
    a.add_window(w);
    a.run();
}


#[test]
fn check_hover(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')
        CheckHash(0x83A02CBD3FA27F3C)
        Mouse.Move(3,2)
        Paint('2. Hover over hour')
        CheckHash(0x9165FB4ACE07083C)
        Mouse.Move(7,2)
        Paint('3. Hover over minute')
        CheckHash(0xD036C671CD4183C)
        Mouse.Move(10,4)
        Paint('4. Hover over second')
        CheckHash(0x131028AABF358BC)
        Mouse.Move(10,6)
        Paint('5. Hover over AM/PM')
        CheckHash(0x74ECDDBEE33D26BC)
        Mouse.Move(9,8)
        Paint('6. Hover over Seconds')
        CheckHash(0xD67CD5859044473C)
        Mouse.Move(12,8)
        Paint('7. Hover over AM/PM')
        CheckHash(0x688DCBBB5ADCA03C)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    w.add(TimePicker::new(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:1,w:10"), timepicker::Flags::None));
    w.add(TimePicker::new(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds));
    w.add(TimePicker::new(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:5,w:10"), timepicker::Flags::AMPM));
    w.add(TimePicker::new(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:7,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM));

    a.add_window(w);
    a.run();
}

#[test]
fn check_hover_disabled(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Move(3,2)
        Paint('2. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Move(4,2)
        Paint('3. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Move(5,2)
        Paint('4. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Move(6,2)
        Paint('5. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Move(7,2)
        Paint('6. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Move(8,2)
        Paint('7. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Move(9,2)
        Paint('8. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Move(10,2)
        Paint('9. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Move(11,2)
        Paint('10. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Move(12,2)
        Paint('11. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Move(13,2)
        Paint('12. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Move(14,2)
        Paint('13. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Move(15,2)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    let mut tp = TimePicker::new(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:1,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM);
    tp.set_enabled(false);
    w.add(tp);
    a.add_window(w);
    a.run();
}

#[test]
fn check_click_disabled(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Click(3,2,left)
        Paint('2. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Click(4,2,left)
        Paint('3. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Click(5,2,left)
        Paint('4. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Click(6,2,left)
        Paint('5. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Click(7,2,left)
        Paint('6. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Click(8,2,left)
        Paint('7. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Click(9,2,left)
        Paint('8. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Click(10,2,left)
        Paint('9. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Click(11,2,left)
        Paint('10. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Click(12,2,left)
        Paint('11. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Click(13,2,left)
        Paint('12. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Click(14,2,left)
        Paint('13. Hover (nothing happens)')
        CheckHash(0x75B2BA9BB5E7F1FE)
        Mouse.Click(15,2,left)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    let mut tp = TimePicker::new(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:1,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM);
    tp.set_enabled(false);
    w.add(tp);
    a.add_window(w);
    a.run();
}