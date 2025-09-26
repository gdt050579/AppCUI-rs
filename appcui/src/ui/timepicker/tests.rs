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
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 30, 20).unwrap(), layout!("x:1,y:1,w:10"), timepicker::Flags::Seconds));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(00, 00, 00).unwrap(), layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(23, 59, 59).unwrap(), layout!("x:1,y:5,w:10"), timepicker::Flags::Seconds));

    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 30, 20).unwrap(), layout!("x:20,y:1,w:10"), timepicker::Flags::None));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(00, 00, 00).unwrap(), layout!("x:20,y:3,w:10"), timepicker::Flags::None));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(23, 59, 59).unwrap(), layout!("x:20,y:5,w:10"), timepicker::Flags::None));

    a.add_window(w);
    a.run();
}

#[test]
fn check_create_proc_macro(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')
        CheckHash(0xFFD1CBFB9AAB7083)
        CheckCursor(22,6)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    w.add(timepicker!("'12:30:20',x:1,y:1,w:10, flags:Seconds"));
    w.add(timepicker!("'00:00:00',x:1,y:3,w:10, flags:Seconds"));
    w.add(timepicker!("time = '23:59:59',x:1,y:5,w:10, flags:Seconds"));

    w.add(timepicker!("'12:30:20',x:20,y:1,w:10"));
    w.add(timepicker!("time:'00:00:00',x:20,y:3,w:10"));
    w.add(timepicker!("'23:59:59',x:20,y:5,w:10"));

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
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 30, 20).unwrap(), layout!("x:1,y:1,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(00, 00, 00).unwrap(), layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(23, 59, 59).unwrap(), layout!("x:1,y:5,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM));

    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 30, 20).unwrap(), layout!("x:20,y:1,w:10"), timepicker::Flags::AMPM));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(00, 00, 00).unwrap(), layout!("x:20,y:3,w:10"), timepicker::Flags::AMPM));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(23, 59, 59).unwrap(), layout!("x:20,y:5,w:10"), timepicker::Flags::AMPM));

    a.add_window(w);
    a.run();
}

#[test]
fn check_create_ampm_proc_macro(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')
        CheckHash(0xCB62EDEF1A6707EB)
        CheckCursor(22,6)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    w.add(timepicker!("'12:30:20',x:1,y:1,w:10, flags:[Seconds , AMPM]"));
    w.add(timepicker!("'00:00:00',x:1,y:3,w:10, flags:[Seconds , AMPM]"));
    w.add(timepicker!("'23:59:59',x:1,y:5,w:10, flags:[Seconds , AMPM]"));

    w.add(timepicker!("'12:30:20',x:20,y:1,w:10, flags:AMPM"));
    w.add(timepicker!("'00:00:00',x:20,y:3,w:10, flags:AMPM"));
    w.add(timepicker!("'23:59:59',x:20,y:5,w:10, flags:AMPM"));

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
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(00, 00, 00).unwrap(), layout!("x:1,y:1,w:10"), timepicker::Flags::AMPM));
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
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:1,w:10"), timepicker::Flags::None));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:5,w:10"), timepicker::Flags::AMPM));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:7,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM));

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
    let mut tp = TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:1,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM);
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
    let mut tp = TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:1,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM);
    tp.set_enabled(false);
    w.add(tp);
    a.add_window(w);
    a.run();
}

#[test]
fn check_click(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')
        CheckHash(0x83A02CBD3FA27F3C)
        Mouse.Click(3,2,left)
        Paint('2. Click over hour')
        CheckHash(0x7789D00B3A71A68C)
        Mouse.Click(7,2,left)
        Paint('3. Click over minute')
        CheckHash(0x4B567FD49ED4D168)
        Mouse.Click(10,4,left)
        Paint('4. Click over second')
        CheckHash(0x611B56AAD489AD24)
        Mouse.Click(10,6,left)
        Paint('5. Click over AM/PM')
        CheckHash(0x65AB7C7FA7DE5904)
        Mouse.Click(9,8,left)
        Paint('6. Click over Seconds')
        CheckHash(0x459556EC8605D30C)
        Mouse.Click(12,8,left)
        Paint('7. Click over AM/PM')
        CheckHash(0x67B7DC6BBB8594A4)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:1,w:10"), timepicker::Flags::None));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:5,w:10"), timepicker::Flags::AMPM));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:7,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM));

    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_wheel(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')
        CheckHash(0x2145A7390DE6C876)
        Mouse.Click(3,4,left)
        Paint('2. Click over hour')
        CheckHash(0x2145A7390DE6C876)
        Mouse.Wheel(3,4,down,1)
        Paint('3. [11]:34:56 AM')
        CheckHash(0xF94B1178150B8024)
        Mouse.Wheel(3,4,right,1)
        Paint('4. 11:[34]:56 AM')
        CheckHash(0x92D241E3DDF9B660)
        Mouse.Wheel(3,4,up,1)
        Paint('5. 11:[35]:56 AM')
        CheckHash(0x82DB3D673044AA15)
        Mouse.Wheel(3,4,right,2)
        Paint('6. 11:35:56 [AM]')
        CheckHash(0xC1D49D6F828BFAED)
        Mouse.Wheel(3,4,down,1)
        Paint('7. 11:35:56 [PM]')
        CheckHash(0x530048738DB2B7AC)
        Mouse.Wheel(3,4,left,1)
        Paint('8. 11:35:[56] PM')
        CheckHash(0x8F45804C5D0EFEF4)
        Mouse.Wheel(3,4,down,20)
        Paint('9. 11:35:[36] PM')
        CheckHash(0xF2C083E20A6F25F2)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM));

    a.add_window(w);
    a.run();
}


#[test]
fn check_api(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State (12:35:57 PM)')
        CheckHash(0x680415F8F44A3404)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    let mut tp = TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM);
    let time = tp.time();
    assert_eq!(time, NaiveTime::from_hms_opt(12, 34, 56).unwrap());
    tp.set_time(NaiveTime::from_hms_opt(13, 35, 57).unwrap());
    let time = tp.time();
    assert_eq!(time, NaiveTime::from_hms_opt(13, 35, 57).unwrap());
    w.add(tp);
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigate(){
    let script = "
        Paint.Enable(false)
        Paint('1. [12]:34:56 PM')
        CheckHash(0x83A02CBD3FA27F3C)
        Key.Pressed(Right)
        Paint('2. 12:[34]:56 PM')
        CheckHash(0x7CD91537680532B8)
        Key.Pressed(Right)
        Paint('3. 12:34:[56] PM')
        CheckHash(0x459556EC8605D30C)
        Key.Pressed(Right)
        Paint('4. 12:34:56 [PM]')
        CheckHash(0x67B7DC6BBB8594A4)
        Key.Pressed(Left)
        Paint('5. 12:34:[56] PM')
        CheckHash(0x459556EC8605D30C)
        Key.Pressed(Left)
        Paint('6. 12:[34]:56 PM')
        CheckHash(0x7CD91537680532B8)
        Key.Pressed(Tab)
        Paint('7. [12]:34')
        CheckHash(0x7789D00B3A71A68C)
        Key.Pressed(Right)
        Paint('8. 12:[34]')
        CheckHash(0x4B567FD49ED4D168)
        Key.Pressed(Right)
        Paint('9. [12]:34')
        CheckHash(0x7789D00B3A71A68C)
        Key.Pressed(Tab)
        Paint('10. [12]:34:56')
        CheckHash(0xD78F6C87E94E4D14)
        Key.Pressed(Left)
        Paint('11. 12:34:[56]')
        CheckHash(0x611B56AAD489AD24)
        Key.Pressed(Left)
        Paint('12. 12:[34]:56')
        CheckHash(0x32E38D9E69C17D40)
        Key.Pressed(Tab)
        Paint('13. [12]:34 PM')
        CheckHash(0x15181D772142F434)
        Key.Pressed(Right)
        Paint('14. 12:[34] PM')
        CheckHash(0x2EC560ACEDCEC8B8)
        Key.Pressed(Right)
        Paint('15. 12:34 [PM]')
        CheckHash(0x65AB7C7FA7DE5904)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:1,w:10"), timepicker::Flags::None));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:5,w:10"), timepicker::Flags::AMPM));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:7,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM));
    a.add_window(w);
    a.run();
}

#[test]
fn check_change_time(){
    let script = "
        Paint.Enable(false)
        Paint('1. [12]:34:56 PM')
        CheckHash(0x83A02CBD3FA27F3C)
        Key.TypeText('0')
        Paint('2. [02]:34:56 PM')
        CheckHash(0xD8346764C90F384C)
        CheckCursor(4,8)
        Key.Pressed(Tab)
        Paint('3. [12]:34')
        CheckHash(0x31E27232BB1AEC7C)
        CheckCursor(3,2)
        Key.TypeText('2312')
        Paint('4. [23]:12')
        CheckHash(0x8E94DDEA55D3486E)
        CheckCursor(3,2)
        Key.Pressed(Tab)
        Key.TypeText('111')
        Paint('5. 11:[14]:56')
        CheckHash(0x10B2A67452174DE7)
        CheckCursor(7,4)
        Key.TypeText('9')
        Paint('6. 11:19:[56] (nothing happens)')
        CheckHash(0xF9D4D25F0B3FA7A6)
        CheckCursor(9,4)
        Key.Pressed(Right)
        Paint('7. [11]:19:56')
        CheckHash(0xDACE1E3FB44C23F6)
        Key.TypeText('3')
        Paint('8. [11]:19:56 (nothing happens)')
        CheckHash(0xDACE1E3FB44C23F6)
        Key.TypeText('19')
        Paint('9. 19:[19]:56')
        CheckHash(0x346D911F8B082ABE)
        Key.Pressed(Left)
        Paint('10. [19]:19:56')
        CheckHash(0x108AB93793B66EFE)
        Key.TypeText('2')
        // 2 is valid for the first digit, but 2 follow by 9 is not so it will be normalized to 23
        Paint('11. [23]:19:56')
        CheckHash(0x197F2C35E0E53D1B)
        CheckCursor(4,4)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:1,w:10"), timepicker::Flags::None));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:5,w:10"), timepicker::Flags::AMPM));
    w.add(TimePicker::with_time(NaiveTime::from_hms_opt(12, 34, 56).unwrap(), layout!("x:1,y:7,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM));
    a.add_window(w);
    a.run();
}

#[test]
fn check_increase_decrease_all(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State ([12]:35:56 PM)')
        CheckHash(0xEED06B87151F60D7)
        Key.Pressed(Right)
        Paint('2. 12:[35]:56 PM')
        CheckHash(0x310B64668BFC170F)
        Key.Pressed(Up,24)
        Paint('3. 12:[59]:56 PM')
        CheckHash(0x8FD62572B6007DBD)
        Key.Pressed(Up,3)
        Paint('4. 12:[02]:56 PM')
        CheckHash(0x129B44CE9C58A5E3)
        Key.Pressed(Down,2)
        Paint('5. 12:[00]:56 PM')
        CheckHash(0x42B8BD8E52E1C56D)
        Key.Pressed(Right)
        Paint('6. 12:00:[56] PM')
        CheckHash(0xD3EB6464EB926369)
        Key.Pressed(Up,4)
        Paint('7. 12:00:[00] PM')
        CheckHash(0xE0A2CAE753F97FE)
        Key.Pressed(Down,3)
        Paint('8. 12:00:[57] PM')
        CheckHash(0x66A9A3A0283BFAD4)
        Key.Pressed(Right)
        Paint('9. 12:00:57 [PM]')
        CheckHash(0x37FE294B74AD6CC0)
        Key.Pressed(Up)
        Paint('10. 12:00:57 [AM]')
        CheckHash(0xE0D9634B57728281)
        Key.Pressed(Up)
        Paint('11. 12:00:57 [PM]')
        CheckHash(0x37FE294B74AD6CC0)
        Key.Pressed(Down)
        Paint('12. 12:00:57 [AM]')
        CheckHash(0xE0D9634B57728281)
        Key.Pressed(Down)
        Paint('13. 12:00:57 [PM]')
        CheckHash(0x37FE294B74AD6CC0)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    let tp = TimePicker::with_time(NaiveTime::from_hms_opt(12, 35, 56).unwrap(), layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM);
    w.add(tp);
    a.add_window(w);
    a.run();
}

#[test]
fn check_type_all(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State ([12]:35:56)')
        CheckHash(0xB2A7252480733ACE)
        Key.TypeText('070811')
        Paint('2. 07:08:11')
        CheckHash(0xEC277E0EFAB776F7)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    let tp = TimePicker::with_time(NaiveTime::from_hms_opt(12, 35, 56).unwrap(), layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds);
    w.add(tp);
    a.add_window(w);
    a.run();
}

#[test]
fn check_delete_digit(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State ([12]:35:56)')
        CheckHash(0xB2A7252480733ACE)
        Key.TypeText('0')
        Paint('2. [02]:35:56')
        CheckHash(0xC5759BE5B8776213)
        CheckCursor(4,4)
        Key.Pressed(Backspace)
        Paint('3. [02]:35:56')
        CheckHash(0xC5759BE5B8776213)
        CheckCursor(3,4)
        Key.TypeText('2')
        Paint('4. [22]:35:56')
        CheckHash(0xFECE992CDAFDEDE5)
        CheckCursor(4,4)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    let tp = TimePicker::with_time(NaiveTime::from_hms_opt(12, 35, 56).unwrap(), layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds);
    w.add(tp);
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigate_rotate(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State ([12]:35:56 PM)')
        CheckHash(0xEED06B87151F60D7)
        Key.Pressed(Left)
        Paint('2. 12:35:56 [PM]')
        CheckHash(0xA7CA1AB0DBDD600F)
        Key.Pressed(Right)
        Paint('3. [12]:35:56 AM')
        CheckHash(0xEED06B87151F60D7)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    let tp = TimePicker::with_time(NaiveTime::from_hms_opt(12, 35, 56).unwrap(), layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM);
    w.add(tp);
    a.add_window(w);
    a.run();
}


#[test]
fn check_create_with_string(){
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State ([12]:35:56 PM)')
        CheckHash(0xEED06B87151F60D7)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("TimePicker,d:fill");
    let tp = TimePicker::new("12:35:56", layout!("x:1,y:3,w:10"), timepicker::Flags::Seconds | timepicker::Flags::AMPM);
    w.add(tp);
    a.add_window(w);
    a.run();
}