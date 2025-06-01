use std::path::PathBuf;

use super::Entry;
use super::NavSimulator;
use crate::utils::{fs::entry::EntryType, Navigator, NavigatorEntry, NavigatorRoot};
use crate::utils::fs::root::RootType;
use chrono::NaiveDateTime;

#[test]
fn check_simulator() {
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
    let nav = NavSimulator::with_csv(csv_data, true, "C:\\");
    let v = nav.roots();
    assert!(v.len() == 2);
    assert!(v[0].path() == "C:\\");
    assert!(v[1].path() == "D:\\");
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

#[cfg(target_os = "windows")]
#[test]
fn check_simulator_join() {
    let csv_data = "
    r,C:\\,10000,100000,Old System,cdrom   
    r,D:\\,123,123456,fast_drive,ramdisk
    d,C:\\Program Files,0,2024-01-10 12:00:00,
    f,C:\\Program Files\\runme.exe,123,2024-01-10 12:31:55,
    f,C:\\Program Files\\readme.txt,123456,2023-02-05 09:12:25,
    d,C:\\Program Files\\Windows,0,2024-01-10 12:31:55,
    f,C:\\Program Files\\Windows\\picture.png,123456,2020-03-12 12:31:55,
    f,C:\\Program Files\\Windows\\melody.mp3,0,2019-03-12 12:31:55,
    ";
    let nav = NavSimulator::with_csv(csv_data, true, "C:\\");
    let p = nav.join(
        &PathBuf::from("C:\\"),
        &Entry::new(
            "Program Files",
            0,
            NaiveDateTime::parse_from_str("2024-01-10 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            EntryType::File,
        ),
    );
    assert_eq!(p, Some(PathBuf::from("C:\\Program Files")));

    let p = nav.join(
        &PathBuf::from("C:\\Test\\xyz"),
        &Entry::new(
            "..\\a.exe",
            0,
            NaiveDateTime::parse_from_str("2024-01-10 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            EntryType::File,
        ),
    );
    assert_eq!(p, Some(PathBuf::from("C:\\Test\\a.exe")));

    let p = nav.join(
        &PathBuf::from("C:\\a/b/c/d/e/f"),
        &Entry::new(
            "./../.././././///../a.exe",
            0,
            NaiveDateTime::parse_from_str("2024-01-10 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            EntryType::File,
        ),
    );
    assert_eq!(p, Some(PathBuf::from("C:\\a\\b\\c\\a.exe")));

    let p = nav.join(
        &PathBuf::from("C:\\a/b/c\\d/e/f"),
        &Entry::new(
            "X:\\Test/T2/a.exe",
            0,
            NaiveDateTime::parse_from_str("2024-01-10 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            EntryType::File,
        ),
    );
    assert_eq!(p, Some(PathBuf::from("X:\\Test\\T2\\a.exe")));
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
    let nav = NavSimulator::with_csv(csv_data, true, "C:\\");
    assert_eq!(nav.exists(&PathBuf::from("C:\\Program Files\\Windows\\picture.png")), Some(true));
    assert_eq!(nav.exists(&PathBuf::from("D:\\Program Files\\Windows\\picture.png")), Some(false));
}

#[test]
fn check_roottype_creation() {
    assert_eq!(RootType::new("fixed"), Some(RootType::Fixed));
    assert_eq!(RootType::new("disk"), Some(RootType::Fixed));       
    assert_eq!(RootType::new("d"), Some(RootType::Fixed));
    assert_eq!(RootType::new("D"), Some(RootType::Fixed));
    assert_eq!(RootType::new("f"), Some(RootType::Fixed));
    assert_eq!(RootType::new("F"), Some(RootType::Fixed));

    assert_eq!(RootType::new("removable"), Some(RootType::Removable));
    assert_eq!(RootType::new("usb"), Some(RootType::Removable));
    assert_eq!(RootType::new("U"), Some(RootType::Removable));
    assert_eq!(RootType::new("u"), Some(RootType::Removable));
    
    assert_eq!(RootType::new("network"), Some(RootType::Network));
    assert_eq!(RootType::new("net"), Some(RootType::Network));
    assert_eq!(RootType::new("n"), Some(RootType::Network));
    assert_eq!(RootType::new("N"), Some(RootType::Network));

    assert_eq!(RootType::new("ramdisk"), Some(RootType::RamDisk));
    assert_eq!(RootType::new("ram"), Some(RootType::RamDisk));
    assert_eq!(RootType::new("r"), Some(RootType::RamDisk));
    assert_eq!(RootType::new("R"), Some(RootType::RamDisk));

    assert_eq!(RootType::new("cdrom"), Some(RootType::CdRom));  
    assert_eq!(RootType::new("cd"), Some(RootType::CdRom));
    assert_eq!(RootType::new("c"), Some(RootType::CdRom));
    assert_eq!(RootType::new("C"), Some(RootType::CdRom));

    assert_eq!(RootType::new("unknown"), Some(RootType::Unknown));
    assert_eq!(RootType::new("?"), Some(RootType::Unknown));

    assert_eq!(RootType::new("123"), None);
}

#[test]
fn check_roottype_icon() {
    assert_eq!(RootType::Fixed.icon(), '💻');
    assert_eq!(RootType::Removable.icon(), '🔌');
    assert_eq!(RootType::Network.icon(), '🖧');
    assert_eq!(RootType::RamDisk.icon(), '▦');
    assert_eq!(RootType::CdRom.icon(), '📀');
    assert_eq!(RootType::Unknown.icon(), '❓');
}   