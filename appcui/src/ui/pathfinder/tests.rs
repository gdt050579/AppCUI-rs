use pathfinder::pathfinder::GenericPathFinder;
use crate::{prelude::*, utils::fs::NavSimulator};

const CSV_DATA: &str = "
    r,C:\\,10000,100000,SYSTEM,fixed
    r,D:\\,123,123456,USB Drive,removable
    d,C:\\Program Files,0,2024-01-10 12:00:00,
    f,C:\\Program Files\\runme.exe,123,2024-01-10 12:31:55,
    f,C:\\Program Files\\readme.txt,123456,2023-02-05 09:12:25,
    d,C:\\Program Files\\Windows,0,2024-01-10 12:31:55,
    f,C:\\Program Files\\Windows\\picture.png,123456,2020-03-12 12:31:55,
    f,C:\\Program Files\\Windows\\melody.mp3,0,2019-03-12 12:31:55,
    f,C:\\Program Files\\Windows\\script.bat,10000,2023-08-11 11:11:11,
    d,C:\\Program Files\\Windows\\System32,0,2020-03-12 12:31:55,
    f,C:\\Program Files\\Windows\\System32\\cmd.exe,123456,2020-03-12 22:15:45,
    f,C:\\Program Files\\Windows\\System32\\notepad.exe,123456,2020-05-14 12:18:55,
    f,C:\\Program Files\\Windows\\System32\\calc.exe,123456,2022-05-14 12:19:35,
    f:D:\\runme.exe,123,2024-01-10 12:31:55,
    f:D:\\readme.txt,123456,2023-02-05 09:12:25,
    d:D:\\Windows,0,2024-01-10 12:31:55,
    f:D:\\Windows\\picture.png,123456,2020-03-12 12:31:55,
    f:D:\\Windows\\melody.mp3,0,2019-03-12 12:31:55,
    ";
#[test]
fn check_with_simulator() {

    let nav = NavSimulator::with_csv(CSV_DATA, true);

    let script = "
        //Paint.Enable(false)
        Paint('Initial')
        Key.Pressed(Tab)
        Paint('After Focus')
        Key.Pressed(S)
        Paint('After pressing inexisting path prefix')
        Key.Pressed(Backspace)
        Paint('Print path suggestions')
        Key.Pressed(Shift+W)
        Paint('Restrict path suggestions')
        Key.Pressed(Down, 6)
        Paint('Got to last selection')
        Key.Pressed(Enter)
        Paint('Selected last')
        Key.Pressed(Tab)
        Paint('After losing focus')
    ";
    //let mut a = App::debug(80, 20, script).build().unwrap();
    let mut a = App::new().build().unwrap();
    let mut w = window!("Test,d:c,w:60,h:15");
    let p = GenericPathFinder::with_navigator(
        r#"C:\Program Files\"#,
        Layout::new("x:1,y:1,w:40"),
        pathfinder::Flags::None,
        nav);
    w.add(p);
    w.add(button!("test,x:1,y:3,w:6"));
    a.add_window(w);
    a.run();
}