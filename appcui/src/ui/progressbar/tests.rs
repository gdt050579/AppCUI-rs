use crate::prelude::*;

#[test]
fn check_view_zero() {
    let script = "
        Paint.Enable(false)
        Paint('0%')   
        CheckHash(0x1CB2372527F63465)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    let p = ProgressBar::new(100,Layout::new("x:1,y:1,w:20"), progressbar::Flags::None);
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_view_50() {
    let script = "
        Paint.Enable(false)
        Paint('50%')   
        CheckHash(0xB55672AEE746FC88)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    let mut p = ProgressBar::new(500,Layout::new("x:1,y:1,w:20"), progressbar::Flags::None);
    p.update_progress(250);
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_view_85_and_text() {
    let script = "
        Paint.Enable(false)
        Paint('Running...  85%')   
        CheckHash(0xC5FB66591549844C)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    let mut p = ProgressBar::new(500,Layout::new("x:1,y:1,w:20"), progressbar::Flags::None);
    p.update_progress(425);
    p.update_text("Running...");
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_view_100_and_text() {
    let script = "
        Paint.Enable(false)
        Paint('Completed  100%')   
        CheckHash(0x6684876DD9D4CEAD)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    let mut p = ProgressBar::new(500,Layout::new("x:1,y:1,w:20"), progressbar::Flags::None);
    p.update_progress(600);
    p.update_text("Completed");
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_view_0_items() {
    let script = "
        Paint.Enable(false)
        Paint('---%')   
        CheckHash(0x638F828C63689B2C)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    let p = ProgressBar::new(0,Layout::new("x:1,y:1,w:20"), progressbar::Flags::None);
    w.add(p);
    a.add_window(w);
    a.run();
}


#[test]
fn check_methods() {
    let script = "
        Paint.Enable(false)
        Paint('2%')   
        CheckHash(0xED37B7847F40AB5B)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    let mut p = ProgressBar::new(0,Layout::new("x:1,y:1,w:20"), progressbar::Flags::None);
    p.reset(100);
    p.update_progress(2);
    assert_eq!(p.processed(),2);
    assert_eq!(p.count(),100);
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_large_number_of_items() {
    let script = "
        Paint.Enable(false)
        Paint('99%')   
        CheckHash(0x5ACEE3CF7DDDB08C)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);
    let mut p = ProgressBar::new(u64::MAX,Layout::new("x:1,y:1,w:20"), progressbar::Flags::None);
    p.update_progress(0xFFFF_FFFF_FFFF_FFF0);
    assert_eq!(p.processed(),0xFFFF_FFFF_FFFF_FFF0);
    assert_eq!(p.count(),0xFFFF_FFFF_FFFF_FFFF);
    w.add(p);
    a.add_window(w);
    a.run();
}