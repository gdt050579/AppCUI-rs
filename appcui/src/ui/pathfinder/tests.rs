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
    f,C:\\Program Files\\Windows\\1script.bat,10000,2023-08-11 11:11:11,
    f,C:\\Program Files\\Windows\\2script.bat,10000,2023-08-11 11:11:11,
    f,C:\\Program Files\\Windows\\3script.bat,10000,2023-08-11 11:11:11,
    f,C:\\Program Files\\Windows\\4script.bat,10000,2023-08-11 11:11:11,
    f,C:\\Program Files\\Windows\\5script.bat,10000,2023-08-11 11:11:11,
    f,C:\\Program Files\\Windows\\6script.bat,10000,2023-08-11 11:11:11,
    f,C:\\Program Files\\Windows\\7script.bat,10000,2023-08-11 11:11:11,
    f,C:\\Program Files\\Windows\\8script.bat,10000,2023-08-11 11:11:11,
    f,C:\\Program Files\\Windows\\9script.bat,10000,2023-08-11 11:11:11,
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
fn test_while_developing() {
    let nav = NavSimulator::with_csv(CSV_DATA, true, "C:\\");
    let mut a = App::new().build().unwrap();
    let mut w = window!("Test,d:c,w:60,h:15");
    let p = GenericPathFinder::with_navigator(
        r#"C:\Program Files\"#,
        Layout::new("x:1,y:1,w:40"),
        pathfinder::Flags::None ,
        nav);
    w.add(p);
    w.add(button!("test,x:1,y:3,w:6"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_display_out_of_focus() {
    let nav = NavSimulator::with_csv(CSV_DATA, true, "C:\\");
    let script = "
        Paint('Initial')
        CheckHash(0x30CF64266AE3901)
        Key.Pressed(Tab)
        Paint('After Focus on control')
        CheckHash(0x4DE8E80C248B41E0)
        Key.TypeText('W')
        Key.Pressed(Down)
        Key.Pressed(Enter)
        Paint('Selected Windows folder')
        CheckHash(0x19156F9829B5EE07)
        Key.Pressed(Tab)
        Paint('Windows folder out of focus')
        CheckHash(0xAD1DF8ADAB3DA0B2)
        Key.Pressed(Tab)
        Key.TypeText('\\S')
        Key.Pressed(Down)
        Key.Pressed(Enter)
        Paint('Selected System32 folder')
        CheckHash(0xEC4AB50225FA43DF)
        Key.Pressed(Tab)
        Paint('Out of focus System32')
        CheckHash(0xEC9D919B63523643)
        Key.Pressed(Tab)
        Key.TypeText('\\')
        Key.Pressed(Down)
        Key.Pressed(Enter)
        Paint('After Selecting cmd.exe')
        CheckHash(0x8BAD279C7FDC5DBA)
        Key.Pressed(Tab)
        Paint('Out of focus cmd.exe')
        CheckHash(0x44EF2500D0F0F67B)
    ";
    let mut a = App::debug(80, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:60,h:15");
    let p = GenericPathFinder::with_navigator(
        r#"C:\Program Files\"#,
        Layout::new("x:1,y:1,w:40"),
        pathfinder::Flags::CaseSensitive,
        nav);
    w.add(p);
    w.add(button!("test,x:1,y:3,w:6"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_suggestion_box_navigation() {
    let nav = NavSimulator::with_csv(CSV_DATA, true, "C:\\");
    let script = "
        Paint('Initial')
        CheckHash(0xF4B84D62A7A75EB9)
        Key.TypeText('W')
        Key.Pressed(Down)
        Key.Pressed(Enter)
        Paint('Selected Windows folder')
        CheckHash(0x32A633F34E3C3E6)
        Key.TypeText('\\')
        Key.Pressed(Down)
        Paint('After scrolling down 1 position in Windows folder')
        CheckHash(0x8DB1A80682BC6DDA)
        Key.Pressed(Down, 6)
        Paint('After scrolling down 6 positions in Windows folder')
        CheckHash(0x8C830A815CAADE05)
        Key.Pressed(Down, 10)
        Paint('After scrolling down to last posistion in Windows folder(tried some extra scrolling)')
        CheckHash(0x55010DC8BEFC2295)
        Key.Pressed(Enter)
        Paint('Selected System32 folder')
        CheckHash(0x73883FF79A85162E)
        Key.TypeText('\\')
        Key.Pressed(Down, 6)
        Paint('After scrolling down 6 positions in system32 folder')
        CheckHash(0xEEB27ECF560D30DA)
        Key.Pressed(Enter)
        Paint('Selected calc.exe')
        CheckHash(0xE63D49EF994D5EE8)
    ";
    let mut a = App::debug(80, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:60,h:15");
    let p = GenericPathFinder::with_navigator(
        r#"C:\Program Files\"#,
        Layout::new("x:1,y:1,w:40"),
        pathfinder::Flags::CaseSensitive,
        nav);
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_suggestion_box_top_navigation() {
    let nav = NavSimulator::with_csv(CSV_DATA, true, "C:\\");
    let script = "
        Paint('Initial')
        CheckHash(0x19EF566F050504B1)
        Key.TypeText('W')
        Key.Pressed(Up)
        Key.Pressed(Enter)
        Paint('Selected Windows folder')
        CheckHash(0x627F3F320AF29A46)
        Key.TypeText('\\')
        Key.Pressed(Up)
        Paint('After scrolling up 1 position in Windows folder')
        CheckHash(0x1AF2DB0C4EEA0AD0)
        Key.Pressed(Up, 6)
        Paint('After scrolling up 6 positions in Windows folder')
        CheckHash(0xED6AC6635BF5A82F)
        Key.Pressed(Up, 10)
        Paint('After scrolling up to last posistion in Windows folder(tried some extra scrolling)')
        CheckHash(0xB222EDCA2054BE9F)
        Key.Pressed(Enter)
        Paint('Selected System32 folder')
        CheckHash(0xCA1419927C8D3C0E)
        Key.TypeText('\\')
        Key.Pressed(Up, 6)
        Paint('After scrolling up 6 positions in system32 folder')
        CheckHash(0x56EB8ED8FE370968)
        Key.Pressed(Enter)
        Paint('Selected calc.exe')
        CheckHash(0xB1EB01FFE3998E28)
    ";
    let mut a = App::debug(80, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:60,h:10");
    let p = GenericPathFinder::with_navigator(
        r#"C:\Program Files\"#,
        Layout::new("x:1,y:5,w:40"),
        pathfinder::Flags::CaseSensitive,
        nav);
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_case_sensitive() {
    let nav = NavSimulator::with_csv(CSV_DATA, true, "C:\\");
    let script = "
        Paint('Initial')
        Key.TypeText('w')
        Paint('No suggestion should appear because Windows starts with capital letter')
        CheckHash(0xAD6CCAFEE3E4844D)
    ";
    let mut a = App::debug(80, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:60,h:15");
    let p = GenericPathFinder::with_navigator(
        r#"C:\Program Files\"#,
        Layout::new("x:1,y:1,w:40"),
        pathfinder::Flags::CaseSensitive,
        nav);
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_case_insensitive() {
    let nav = NavSimulator::with_csv(CSV_DATA, true, "C:\\");
    let script = "
        Paint('Initial')
        Key.TypeText('w')
        Paint('Suggestion Windows should appear even though w was inserted')
        CheckHash(0xF9D5B8F930ABA6DA)
    ";
    let mut a = App::debug(80, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:60,h:15");
    let p = GenericPathFinder::with_navigator(
        r#"C:\Program Files\"#,
        Layout::new("x:1,y:1,w:40"),
        pathfinder::Flags::None ,
        nav);
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_readonly_flag() {
    let nav = NavSimulator::with_csv(CSV_DATA, true, "C:\\");
    let script = "
        Paint('Initial')
        CheckHash(0xF4B84D62A7A75EB9)
        Key.TypeText('w')
        Paint('After trying to write text')
        CheckHash(0xF4B84D62A7A75EB9)
        Key.Pressed(Backspace)
        Paint('After trying to delete text with backspace')
        CheckHash(0xF4B84D62A7A75EB9)
    ";
    let mut a = App::debug(80, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:60,h:15");
    let p = GenericPathFinder::with_navigator(
        r#"C:\Program Files\"#,
        Layout::new("x:1,y:1,w:40"),
        pathfinder::Flags::ReadOnly ,
        nav);
    w.add(p);
    a.add_window(w);
    a.run();
}