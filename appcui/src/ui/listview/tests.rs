use crate::prelude::*;

struct TestItem {}
impl listview::ListItem for TestItem {
    fn render_method(&self, _column_index: u32)->Option<listview::RenderMethod> {
        Some(listview::RenderMethod::Text("abc"))
    }
}

#[test]
fn check_create() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0x668442408FDE500C)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]"
    ));
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_navigate_with_keys() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x668442408FDE500C)
        Key.Pressed(Ctrl+Right)
        Paint('2. Column-1 selected')
        CheckHash(0x644101CC8184BE50)
        Key.Pressed(Ctrl+Right)
        Paint('3.cColumn-2 selected')
        CheckHash(0x9C1B04EFDBB90110)
        Key.Pressed(Ctrl+Right)
        Paint('4. Column-3 selected')
        CheckHash(0xB5979A0999E0E4B8)
        Key.Pressed(Ctrl+Right)
        Paint('5. Column-3 remains selected')
        CheckHash(0xB5979A0999E0E4B8)
        Key.Pressed(Ctrl+Left)
        Paint('6. Column-2 selected')
        CheckHash(0x9C1B04EFDBB90110)
        Key.Pressed(Ctrl+Left)
        Paint('7. Column-1 selected')
        CheckHash(0x644101CC8184BE50)
        Key.Pressed(Ctrl+Left)
        Paint('8. Column-1 re-selected')
        CheckHash(0x644101CC8184BE50)
        Key.Pressed(Escape)
        Paint('9. Initial state')
        CheckHash(0x668442408FDE500C)
        Key.Pressed(Escape)
        Paint('10. Window is closed')
        CheckHash(0x3900AF2CBDF4157D)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]"
    ));
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_resize_with_keys() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x668442408FDE500C)
        Key.Pressed(Ctrl+Right)
        Paint('2. Column-1 selected')
        CheckHash(0x644101CC8184BE50)
        Key.Pressed(Right)
        Paint('3. Column-1 size increased by 1')
        CheckHash(0xC8CF6D36FDDFF890)
        Key.Pressed(Left)
        Paint('4. Column-1 size decreased by 1')
        CheckHash(0x644101CC8184BE50)
        Key.Pressed(Right,14)
        Paint('5. Column-1 size increased by 14')
        CheckHash(0xD53CDF5512B92450)
        Key.Pressed(Right,1)
        Paint('6. Column-1 size increased by 1, horzontal scroll not active')
        CheckHash(0x225D535A77E87490)
        Key.Pressed(Right,1)
        Paint('7. Column-1 size increased by 1, horzontal scroll is ACTIVE')
        CheckHash(0x2BEF9B5B61807A46)
        Mouse.Click(52,9,left)
        Paint('8. Scroll to right, last column is fully visible,Header starts with Name')
        CheckHash(0x2D4C94CE23C61191)
        Mouse.Click(52,9,left)
        Paint('9. Identical to the previous one')
        CheckHash(0x2D4C94CE23C61191)
        Mouse.Click(52,9,left)
        Paint('10. Identical to the previous one')
        CheckHash(0x2D4C94CE23C61191)
        Mouse.Click(6,9,left)
        Paint('11. Scrool to first element, horzontal scroll is ACTIVE')
        CheckHash(0x60E537F7DC6646FD)
        Mouse.Click(6,9,left)
        Paint('12. Scrool to first element, horzontal scroll is ACTIVE (identical to step 11')
        CheckHash(0x60E537F7DC6646FD)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]"
    ));
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_ensure_visible_when_changing_columns() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xA6F8DC05418FA9A5)
        Key.Pressed(Ctrl+Right)
        Paint('2. C1 selected')
        CheckHash(0xA854AE3EEBECBE79)
        Key.Pressed(Ctrl+Right)
        Paint('3. C2 selected')
        CheckHash(0x1B2BF1D59A37B3C1)
        Key.Pressed(Ctrl+Right)
        Paint('4. C3 selected (header starts with C1-10, C3 is fully visible at the end)')
        CheckHash(0x7A19C8ECD3975FC9)
        Key.Pressed(Ctrl+Right)
        Paint('5. C4 selected (C4 is visible at the end of the header)')
        CheckHash(0x12531D79D88BFB31)
        Key.Pressed(Ctrl+Right)
        Paint('6. C5 selected (C5 is visible at the end of the header)')
        CheckHash(0x2610F5BA8B809EAB)
        Key.Pressed(Ctrl+Right)
        Paint('7. C5 remains selected (same as 6)')
        CheckHash(0x2610F5BA8B809EAB)
        Key.Pressed(Ctrl+Left)
        Paint('8. C4 selected (view is not changed)')
        CheckHash(0xBFEE2A312CD05D53)
        Key.Pressed(Ctrl+Left)
        Paint('9. C3 selected (from the start of the header)')
        CheckHash(0xF0A53B98F0DD9917)
        Key.Pressed(Ctrl+Left)
        Paint('10. C2 selected (from the start of the header)')
        CheckHash(0xEC65923D88D014E2)
        Key.Pressed(Ctrl+Left)
        Paint('11. C1 selected (from the start of the header)')
        CheckHash(0xA854AE3EEBECBE79)
        Key.Pressed(Ctrl+Left)
        Paint('12. C1 remains selected (view is not changed)')
        CheckHash(0xA854AE3EEBECBE79)
   ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{C1-10,10},{C2-12,12},{C3-14,14},{C4-16,16},{C5-10,10}]"
    ));
    a.add_window(w);
    a.run();
}


#[test]
fn check_column_resize_outside_visible() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xA6F8DC05418FA9A5)
        Key.Pressed(Ctrl+Right,3)
        Paint('2. C3 selected')
        CheckHash(0x7A19C8ECD3975FC9)
        Key.Pressed(Right);
        Paint('3. C3 increased by 1')
        CheckHash(0x1205B31BB5A2E8E6)
        Key.Pressed(Right,4);
        Paint('4. C3 increased by 4, C2 line under T from Test')
        CheckHash(0xBE314AF843DA7B17)
        Key.Pressed(Right,18);
        Paint('5. C3 increased by 18, C3 has the entire width of the window')
        CheckHash(0x88EB41332D598118)
        Key.Pressed(Right);
        Paint('6. C3 increased by 1, Header starts with C3..., right line visible')
        CheckHash(0xFE373AA6CB9FCB9C)
        Key.Pressed(Right);
        Paint('7. C3 increased by 1, Header starts with 3..., right line visible')
        CheckHash(0xC99586142C1EE75B)
        Key.Pressed(Left,2);
        Paint('8. C3 decreased by 2 - should take the entire width of the window')
        CheckHash(0x88EB41332D598118)
        Key.Pressed(Left,20);
        Paint('9. C3 decreased by 20, Line under letter T from Test')
        CheckHash(0x886948F2CFA28E07)
        Key.Pressed(Ctrl+Left);
        Paint('10. C2 is visible')
        CheckHash(0x1D6B48ED61CF0D6A)
   ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{C1-10,10},{C2-12,12},{C3-14,14},{C4-16,16},{C5-10,10}]"
    ));
    a.add_window(w);
    a.run();
}


#[test]
fn check_column_move_scroll_when_enter_column_resize_mode() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xA6F8DC05418FA9A5)
        Key.Pressed(Ctrl+Right,3)
        Paint('2. C3 selected')
        CheckHash(0x7A19C8ECD3975FC9)
        Key.Pressed(Right,10);
        Paint('3. C3 increased by 10')
        CheckHash(0x5836CFFB8679E7A7)
        Key.Pressed(Escape)
        Paint('4. No column selected')
        CheckHash(0x4663870AFB3F133B)
        Key.Pressed(Ctrl+Right)
        Paint('5. Fist column selected and scroll moved')
        CheckHash(0xA854AE3EEBECBE79)
   ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{C1-10,10},{C2-12,12},{C3-14,14},{C4-16,16},{C5-10,10}]"
    ));
    a.add_window(w);
    a.run();
}
#[test]
fn check_column_scroll_update_when_control_is_resized() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (h-scroll inactive)')
        CheckHash(0x9CC1A20A64CF25BC)
        Mouse.Drag(49,9,40,9)
        Paint('2. Resize window (h-scroll active)')
        CheckHash(0x96AD3061803219B9)   
        Key.Pressed(Right,100)     
        Paint('3. Move h-scroll to end (starts with | C2...)')
        CheckHash(0x1C85B6AB50E32DB9) 
        Mouse.Drag(40,9,30,9)  
        Paint('4. Move h-scroll to middle but still (starts with | C2...)')
        CheckHash(0x53F55F8778EDFF3F) 
        Mouse.Drag(30,9,60,9)  
        Paint('5. Scroll bar is not inactive (starts with C1 ...)')
        CheckHash(0x6CA64A075D529384) 
   ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{C1,6},{C2,6},{C3,6},{C4,6},{C5,6}]"
    ));
    a.add_window(w);
    a.run();
}


#[test]
fn check_column_click() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (h-scroll inactive)')
        CheckHash(0xD91C4D1725F901B4)
        Mouse.Move(20,2)
        Paint('2. C1-hover')
        CheckHash(0xBCF1AC194596E088)   
        Mouse.Move(26,2)
        Paint('3. C2-hover')
        CheckHash(0x8B967934E497DCF4)   
        Mouse.Move(40,2)
        Paint('4. C3-hover')
        CheckHash(0xF97A147330A43988)   
        Mouse.Click(40,2,left)
        Paint('5. C3-hover, clicked, sorted up')
        CheckHash(0xB1A45A7496098B32)   
        Mouse.Click(40,2,left)
        Paint('6. C3-hover, clicked, sorted down')
        CheckHash(0x93AB741282C03DC4)   
        Mouse.Move(44,2)
        Paint('7. C4-hover, C3-selected and sorted down')
        CheckHash(0xC0D6BDF3C435DEA6)   
        Mouse.Click(44,2,left)
        Paint('8. C4-hover, clicked, sorted down, scroll move to see C3 entirely at the end')
        CheckHash(0x33D2C0CF30EAEE5C)   
        Mouse.Move(0,0)
        Paint('9. C4-clicked, sorted down, scroll move to see C3 entirely at the end')
        CheckHash(0x70C7CCCB3A1B805D)   
   ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{C1,10,L},{C2,10,C},{C3,10,R},{C4,10},{C5,10}]"
    ));
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_left_right_scroll() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xD91C4D1725F901B4)
        Key.Pressed(Right)
        Paint('2. Scroll to right')
        CheckHash(0xF2231DF83A92FFDC)   
        Key.Pressed(Right,2)
        Paint('3. Scroll to right (C1 is not visible)')
        CheckHash(0x3073E33BB34F3EDE)   
        Key.Pressed(Right,12)
        Paint('4. Scroll to right (C2 is first on header)')
        CheckHash(0x903858619D4CE328)   
        Key.Pressed(Right,2)
        Paint('5. Scroll to right most - C5 is fully visible')
        CheckHash(0xA1F242FCA8DDE105)   
        Key.Pressed(Right)
        Paint('6. Scroll to right most - nothing changes')
        CheckHash(0xA1F242FCA8DDE105)   
        Key.Pressed(Left)
        Paint('7. Scroll to left - C5 bar is no longer visible')
        CheckHash(0xDD6D52AB8FFB4FCF)   
        Key.Pressed(Left,10)
        Paint('8. Scroll to left - 10 pos')
        CheckHash(0x5F47E2CF754E559A)   
        Key.Pressed(Left,10)
        Paint('9. Scroll to left - back to the initial state')
        CheckHash(0xD91C4D1725F901B4)   
   ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{C1,10,L},{C2,10,C},{C3,10,R},{C4,10},{C5,10}]"
    ));
    a.add_window(w);
    a.run();
}


