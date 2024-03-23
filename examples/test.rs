use appcui::prelude::*;

#[Window()]
struct MyWin {
    p: Handle<Password>
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'My Win',d:c,w:39,h:14"),
            p: Handle::None,
        };
        win.p = win.add(Password::new(Layout::new("x:1,y:1,w:10")));
        win.add(button!("Yes,x:1,y:4,w:11"));
        win.add(button!("No,x:13,y:4,w:11"));
        win.add(button!("Cancel,x:25,y:4,w:11"));
        win
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
