use chrono::NaiveDateTime;
use super::NavSimulator;
use crate::utils::{Navigator,NavigatorRoot,NavigatorEntry};

#[test]
fn check_simulator() {
    let csv_data = "
    r,C:\\,10000,100000,   
    r,D:\\,123,123456,
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
    let nav = NavSimulator::with_csv(csv_data);
    let v = nav.roots();
    assert!(v.len() == 2);
    assert!(v[0].name() == "C:\\");
    assert!(v[1].name() == "D:\\");
    let e = nav.entries("C:\\");
    assert!(e.len() == 1);
    assert!(e[0].name() == "Program Files");
    assert!(e[0].size == 0);
    assert!(e[0].folder);
    assert!(e[0].created == NaiveDateTime::parse_from_str("2024-01-10 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap());    
    let e = nav.entries("C:\\Program Files\\");
    assert!(e.len() == 3);
    assert!(e[0].name() == "runme.exe");
    assert!(e[0].size == 123);
    assert!(!e[0].folder);
    assert!(e[0].created == NaiveDateTime::parse_from_str("2024-01-10 12:31:55", "%Y-%m-%d %H:%M:%S").unwrap());
    assert!(e[1].name() == "readme.txt");
    assert!(e[1].size == 123456);
    assert!(!e[1].folder);
    assert!(e[1].created == NaiveDateTime::parse_from_str("2023-02-05 09:12:25", "%Y-%m-%d %H:%M:%S").unwrap());
    assert!(e[2].name() == "Windows");
    assert!(e[2].size == 0);
    assert!(e[2].folder);
    assert!(e[2].created == NaiveDateTime::parse_from_str("2024-01-10 12:31:55", "%Y-%m-%d %H:%M:%S").unwrap());
}