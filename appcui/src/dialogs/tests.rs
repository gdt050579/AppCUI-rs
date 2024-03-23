use crate::dialogs;
use crate::prelude::*;

#[Window(events=ButtonEvents, internal: true)]
struct CallbackWin {
    f: fn(),
}
impl CallbackWin {
    fn new(f: fn()) -> Self {
        let mut w = Self {
            base: window!("Test,d:c,w:20,h:6"),
            f,
        };
        w.add(button!("'Press Me',d:c,w:15"));
        w
    }
}
impl ButtonEvents for CallbackWin {
    fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
        (self.f)();
        EventProcessStatus::Processed
    }
}

#[test]
fn check_small_error() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Message box');
        CheckHash(0x847C1B71CF1BAB79)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| dialogs::error("Error", "123")));
    a.run();
}


#[test]
fn check_large_error() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Message box');
        CheckHash(0x563DF7AC2DDD7DAE)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| dialogs::error("Error", "An error has occured while running the code.")));
    a.run();
}

#[test]
fn check_very_large_error() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Message box');
        CheckHash(0xF83F2AE0FC4EC4ED)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| dialogs::error("Error", "An error has occured while running the code. Because of this certain operations are no longer possible.")));
    a.run();
}

#[test]
fn check_too_large_error() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        // should print:
        // An error has occured while running the code.
        // Because of this certain operations are no 
        // longer possible. All connection to the
        Paint('Message box');
        CheckHash(0xD8F2736351150900)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| dialogs::error("Error", "An error has occured while running the code. Because of this certain operations are no longer possible. All connection to the database have been stop and the file system has been reverted to its original state before this operation has started !")));
    a.run();
}

#[test]
fn check_multi_line_error() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x7948D22070789869)
        Key.Pressed(Enter)
        Paint('Message box');
        CheckHash(0x3E34FCDDB9B49649)
    ";
    let mut a = App::debug(60, 16, script).build().unwrap();
    a.add_window(CallbackWin::new(|| dialogs::error("Error", "An error has occured during the last operarion. To recover perform the following:\n1. Run the diagnostics\n2. Restart the computer\n3. Reboot")));
    a.run();
}

#[test]
fn check_return_from_error() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Message box');
        CheckHash(0x847C1B71CF1BAB79)
        Key.Pressed(Enter)
        Paint('Back to initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| dialogs::error("Error", "123")));
    a.run();
}
