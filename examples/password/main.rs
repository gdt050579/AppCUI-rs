use appcui::prelude::*;

#[Window(events = ButtonEvents+PasswordEvents)]
struct MyWin {
    p: Handle<Password>,
    b_ok: Handle<Button>,
    b_cancel: Handle<Button>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'Login',d:c,w:40,h:8"),
            p: Handle::None,
            b_ok: Handle::None,
            b_cancel: Handle::None
        };
        win.add(label!("'Enter the password:',x:1,y:1,w:36,h:1"));
        win.b_ok = win.add(button!("&Ok,x:5,y:4,w:11"));
        win.b_cancel = win.add(button!("&Cancel,x:22,y:4,w:11"));
        win.p = win.add(password!("x:1,y:2,w:36"));

        win
    }
    fn check_password(&mut self) {
        let p = self.p;
        if let Some(pass) = self.control(p) {
            if pass.password() == "admin" {
                dialogs::message("Login", "Correct password. Let's start !");
            } else {
                if !dialogs::retry("Login", "Invalid password. Try again ?") {
                    self.close();
                }
            }
        }
    }
}

impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        match () {
            _ if handle == self.b_cancel => {
                self.close();
                EventProcessStatus::Processed
            }
            _ if handle == self.b_ok => {
                self.check_password();
                EventProcessStatus::Processed
            }
            _ => { EventProcessStatus::Ignored }
        }
    }
}
impl PasswordEvents for MyWin {
    fn on_accept(&mut self, _: Handle<Password>) -> EventProcessStatus {
        self.check_password();
        EventProcessStatus::Processed
    }

    fn on_cancel(&mut self, _: Handle<Password>) -> EventProcessStatus {
        self.close();
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
