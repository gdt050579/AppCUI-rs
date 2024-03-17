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
}

impl ButtonEvents for MyWin {

}
impl PasswordEvents for MyWin {
    
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
