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
    a.add_window(CallbackWin::new(|| {
        dialogs::error("Error", "An error has occured while running the code.")
    }));
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
    a.add_window(CallbackWin::new(|| {
        dialogs::error(
            "Error",
            "An error has occured while running the code. Because of this certain operations are no longer possible.",
        )
    }));
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

#[test]
fn check_retry_error() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Retry');
        CheckHash(0x7C19591705072D6D)
        Key.Pressed(Enter)
        Paint('Back to initial State (result is cancel)')   
        CheckHash(0x937CE126B66578D9)
        Key.Pressed(Enter)
        Paint('Back to initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Retry (second time)');
        CheckHash(0x7C19591705072D6D)
        Key.Pressed(Escape)
        Paint('Back to initial State (after escape)')   
        Key.Pressed(Enter)
        Paint('Back to initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Retry (third time)');
        CheckHash(0x7C19591705072D6D)
        Key.Pressed(Tab)
        Key.Pressed(Enter)
        Paint('Now we need to retry');
        CheckHash(0xC88A5ABECB445F81)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| {
        if dialogs::retry("Error", "An error occured. Retry ?") {
            dialogs::message("Response", "We should retry.")
        } else {
            dialogs::message("Response", "Stop the action.")
        }
    }));
    a.run();
}

#[test]
fn check_alert() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Message box');
        CheckHash(0xBCA6A406AE5AE98E)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| dialogs::alert("Error", "A problem occured while running the code.")));
    a.run();
}

#[test]
fn check_proceed() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('No (do not continue)');
        CheckHash(0x24B39A0A49793368)
        Key.Pressed(Enter)
        Paint('Back to initial State (result is cancel)')   
        CheckHash(0x937CE126B66578D9)
        Key.Pressed(Enter)
        Paint('Back to initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Retry (second time)');
        CheckHash(0x24B39A0A49793368)
        Key.Pressed(Escape)
        Paint('Back to initial State (after escape)')   
        Key.Pressed(Enter)
        Paint('Back to initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Retry (third time)');
        CheckHash(0x24B39A0A49793368)
        Key.Pressed(Tab)
        Paint('Yes button selected')
        CheckHash(0x7DB1B8B269F1DAB8)
        Key.Pressed(Enter)
        Paint('Now we should continue');
        CheckHash(0xF8045E482E522D83)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| {
        if dialogs::proceed("Alert", "An problem occured.\nContinue anyway ?") {
            dialogs::message("Response", "We should continue.")
        } else {
            dialogs::message("Response", "Stop the action.")
        }
    }));
    a.run();
}

#[test]
fn check_message() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Message box');
        CheckHash(0xE2E128A51D518819)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| dialogs::message("Success", "Operation completed succesifully.")));
    a.run();
}