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
