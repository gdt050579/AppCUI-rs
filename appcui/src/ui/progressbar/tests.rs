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