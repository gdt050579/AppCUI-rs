use crate::prelude::*;

struct TestItem {}
impl listview::ListItem for TestItem {}

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
