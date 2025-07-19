use crate::prelude::*;

#[test]
fn check_view_zero() {
    let script = "
        Paint.Enable(false)
        Paint('0%')   
        CheckHash(0x1CB2372527F63465)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
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
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
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
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
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
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
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
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
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
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
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
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
    let mut p = ProgressBar::new(u64::MAX,Layout::new("x:1,y:1,w:20"), progressbar::Flags::None);
    p.update_progress(0xFFFF_FFFF_FFFF_FFF0);
    assert_eq!(p.processed(),0xFFFF_FFFF_FFFF_FFF0);
    assert_eq!(p.count(),0xFFFF_FFFF_FFFF_FFFF);
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_show_eta_for_no_items() {
    let script = "
        Paint.Enable(false)
        Paint('0% + eta')   
        CheckHash(0x47E85613777BE97)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
    let mut p = ProgressBar::new(u64::MAX,Layout::new("x:1,y:1,w:36,h:2"), progressbar::Flags::None);
    p.update_progress(0);
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_show_eta() {
    let script = "
        Paint.Enable(false)
        Paint('20% + 00:02:40')   
        CheckHash(0xE6AB2456FCAA33CE)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
    let mut p = ProgressBar::new(10,Layout::new("x:1,y:1,w:36,h:2"), progressbar::Flags::None);
    p.update_progress(2);
    p.update_eta_with_elapsed_time(40);
    // 40 seconds for 2 items -> remaining is 160 seconds for the rest of the 8 items
    // should print 2 min and 40 sec
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_show_eta_more_than_one_week() {
    let script = "
        Paint.Enable(false)
        Paint('10% + >1 week')   
        CheckHash(0xE238233C43771F70)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
    let mut p = ProgressBar::new(10,Layout::new("x:1,y:1,w:36,h:2"), progressbar::Flags::None);
    p.update_progress(1);
    p.update_eta_with_elapsed_time(300000);
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_show_eta_one_day() {
    let script = "
        Paint.Enable(false)
        Paint('50% + >1 day')   
        CheckHash(0xC976C1D0A758949)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
    let mut p = ProgressBar::new(10,Layout::new("x:1,y:1,w:36,h:2"), progressbar::Flags::None);
    p.update_progress(5);
    p.update_eta_with_elapsed_time(87400);
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_show_eta_four_day() {
    let script = "
        Paint.Enable(false)
        Paint('20% + >4 days')   
        CheckHash(0x699487060EFC4429)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:9"), window::Flags::None);
    let mut p = ProgressBar::new(10,Layout::new("x:1,y:1,w:36,h:2"), progressbar::Flags::None);
    p.update_progress(2);
    p.update_eta_with_elapsed_time(87400);
    w.add(p);
    a.add_window(w);
    a.run();
}

#[test]
fn check_macro() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xF35890A91AA2DA45)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:9");
    let mut c = progressbar!("value: 2, x:1,y:1,w:36,h:2, total: 10, text: 'Running...'");
    c.update_eta_with_elapsed_time(1);
    w.add(c);
    a.add_window(w);
    a.run();
}

#[test]
fn check_hide_percentage() {
    let script = "
        Paint.Enable(false)
        Paint('Just running... (20% is not printed), eta is 4 seconds')   
        CheckHash(0xA5C51B4854FAD972)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:9");
    let mut c = progressbar!("value: 2, x:1,y:1,w:36,h:2, total: 10, text: 'Running...', flags: HidePercentage");
    c.update_eta_with_elapsed_time(1);
    w.add(c);
    a.add_window(w);
    a.run();
}

#[test]
fn check_paused() {
    let script = "
        Paint.Enable(false)
        Paint('Just running... (20% is not printed), and paused')   
        CheckHash(0x6316D1E95B82EFCB)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:9");
    let c = progressbar!("value: 2, x:1,y:1,w:36,h:2, total: 10, text: 'Running...', paused: true");
    assert!(c.is_paused());
    w.add(c);
    a.add_window(w);
    a.run();
}

#[test]
fn check_resume() {
    let script = "
        Paint.Enable(false)
        Paint('50%, Running... text and 10 sec ETA')    
        CheckHash(0x560946B21ADA059E)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Title,a:c,w:40,h:9");
    let mut c = progressbar!("value: 2, x:1,y:1,w:36,h:2, total: 10, text: 'Running...', paused: true");
    c.update_progress(5); // should call resume
    c.update_eta_with_elapsed_time(10);
    assert!(!c.is_paused());
    w.add(c);
    a.add_window(w);
    a.run();
}