use std::path::PathBuf;

use super::Entry;
use super::NavSimulator;
use crate::utils::{fs::entry::EntryType, Navigator, NavigatorEntry, NavigatorRoot};
use chrono::NaiveDateTime;

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
    let nav = NavSimulator::with_csv(csv_data, true);
    let v = nav.roots();
    assert!(v.len() == 2);
    assert!(v[0].name() == "C:\\");
    assert!(v[1].name() == "D:\\");
    let e = nav.entries(&PathBuf::from("C:\\"));
    assert!(e.len() == 1);
    assert!(e[0].name() == "Program Files");
    assert!(e[0].size == 0);
    assert!(e[0].entry_type == EntryType::Folder);
    assert!(e[0].created == NaiveDateTime::parse_from_str("2024-01-10 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap());
    let e = nav.entries(&PathBuf::from("C:\\Program Files\\"));
    assert!(e.len() == 3);
    assert!(e[0].name() == "runme.exe");
    assert!(e[0].size == 123);
    assert!(e[0].entry_type == EntryType::File);
    assert!(e[0].created == NaiveDateTime::parse_from_str("2024-01-10 12:31:55", "%Y-%m-%d %H:%M:%S").unwrap());
    assert!(e[1].name() == "readme.txt");
    assert!(e[1].size == 123456);
    assert!(e[1].entry_type == EntryType::File);
    assert!(e[1].created == NaiveDateTime::parse_from_str("2023-02-05 09:12:25", "%Y-%m-%d %H:%M:%S").unwrap());
    assert!(e[2].name() == "Windows");
    assert!(e[2].size == 0);
    assert!(e[2].entry_type == EntryType::Folder);
    assert!(e[2].created == NaiveDateTime::parse_from_str("2024-01-10 12:31:55", "%Y-%m-%d %H:%M:%S").unwrap());
}

#[test]
fn check_simulator_join() {
    let csv_data = "
    r,C:\\,10000,100000,   
    r,D:\\,123,123456,
    d,C:\\Program Files,0,2024-01-10 12:00:00,
    f,C:\\Program Files\\runme.exe,123,2024-01-10 12:31:55,
    f,C:\\Program Files\\readme.txt,123456,2023-02-05 09:12:25,
    d,C:\\Program Files\\Windows,0,2024-01-10 12:31:55,
    f,C:\\Program Files\\Windows\\picture.png,123456,2020-03-12 12:31:55,
    f,C:\\Program Files\\Windows\\melody.mp3,0,2019-03-12 12:31:55,
    ";
    let nav = NavSimulator::with_csv(csv_data, true);
    let p = nav.join(
        &PathBuf::from("C:\\"),
        &Entry::new(
            "Program Files",
            0,
            NaiveDateTime::parse_from_str("2024-01-10 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(), 
            EntryType::File
        )        
    );
    assert_eq!(p, Some(PathBuf::from("C:\\Program Files")));
    let p = nav.join(
        &PathBuf::from("C:\\Test\\xyz"),
        &Entry::new(
            "..\\a.exe",
            0,
            NaiveDateTime::parse_from_str("2024-01-10 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(), 
            EntryType::File
        )        
    );
    assert_eq!(p, Some(PathBuf::from("C:\\Test\\a.exe")));
    let p = nav.join(
        &PathBuf::from("C:\\a/b/c/d/e/f"),
        &Entry::new(
            "./../.././././///../a.exe",
            0,
            NaiveDateTime::parse_from_str("2024-01-10 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(), 
            EntryType::File
        )        
    );
    assert_eq!(p, Some(PathBuf::from("C:\\a\\b\\c\\a.exe")));
}


#[test]
fn check_simulator_exists() {
    let csv_data = "
    r,C:\\,10000,100000,   
    r,D:\\,123,123456,
    d,C:\\Program Files,0,2024-01-10 12:00:00,
    f,C:\\Program Files\\runme.exe,123,2024-01-10 12:31:55,
    f,C:\\Program Files\\readme.txt,123456,2023-02-05 09:12:25,
    d,C:\\Program Files\\Windows,0,2024-01-10 12:31:55,
    f,C:\\Program Files\\Windows\\picture.png,123456,2020-03-12 12:31:55,
    f,C:\\Program Files\\Windows\\melody.mp3,0,2019-03-12 12:31:55,
    ";
    let nav = NavSimulator::with_csv(csv_data, true);
    assert_eq!(nav.exists(&PathBuf::from("C:\\Program Files\\Windows\\picture.png")),Some(true));
    assert_eq!(nav.exists(&PathBuf::from("D:\\Program Files\\Windows\\picture.png")),Some(false));
}