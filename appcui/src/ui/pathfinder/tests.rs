use crate::{prelude::*, utils::fs::NavSimulator};

#[test]
fn check() {
    let script = "
        //Paint.Enable(false)
        Paint('Initial')
        Key.Pressed(Tab)
        Paint('After Tab')
        Key.Pressed(P)
        Paint('after P')
        Key.Pressed(Backspace)
        Paint('after backspace')
    ";
    let mut a = App::debug(80, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:60,h:15");
    let p = PathFinder::<crate::utils::fs::Navigator>::new(
        r#"D:\work\Projects\"#,
        Layout::new("x:1,y:1,w:30"),
        pathfinder::Flags::None);
    w.add(p);
    w.add(button!("test,x:1,y:3,w:6"));
    a.add_window(w);
    a.run();
}

#[test]
fn run() -> Result<(), crate::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Test,d:c,w:100,h:15");
    let p = PathFinder::<crate::utils::fs::Navigator>::new(
        r#"D:\work\Projects\BDAntiCryptoLockerUnified\RemovalToolUnifiedDropper\RemovalToolUnifiedDropper.sln"#,
        Layout::new("x:1,y:1,w:60"),
        pathfinder::Flags::None);
    w.add(p);
    w.add(button!("test,x:1,y:4,w:6"));
    a.add_window(w);
    a.run();
    Ok(())
}

#[test]
fn check_with_simulator() {
    let csv_data = "
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
    let nav = NavSimulator::with_csv(csv_data, true);

    // let script = "
    //     //Paint.Enable(false)
    //     Paint('Initial')
    //     Key.Pressed(Tab)
    //     Paint('After Tab')
    //     Key.Pressed(W)
    //     Key.Pressed(Backspace)
    //     Key.Pressed(Down, 6)
    //     Paint('after')
    // ";
    // let mut a = App::debug(80, 20, script).build().unwrap();
    let mut a = App::new().build().unwrap();
    let mut w = window!("Test,d:c,w:60,h:15");
    let p = PathFinder::<crate::utils::fs::NavSimulator>::with_navigator(
        r#"C:\Program Files\"#,
        Layout::new("x:1,y:1,w:30"),
        pathfinder::Flags::None,
        nav);
    w.add(p);
    w.add(button!("test,x:1,y:3,w:6"));
    a.add_window(w);
    a.run();
}