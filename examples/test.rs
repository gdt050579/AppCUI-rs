use appcui::prelude::*;

#[Window()]
struct MyWin {
    p: Handle<Password>
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'My Win',d:c,w:60,h:14"),
            p: Handle::None,
        };
        win.p = win.add(Password::new("abc",Layout::new("x:1,y:1,w:10")));
        win.add(button!("test,x:1,y:4,w:10"));
        win
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
