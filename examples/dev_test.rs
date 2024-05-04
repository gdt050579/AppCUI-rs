use appcui::prelude::*;


#[Window()]
struct MyWin {
    index: u32,
}
impl MyWin {
    fn new() -> Self {
        Self {
            base: window!("Test,x:1,y:1,w:30,h:10"),
            index: 1,
        }
    }
}


fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().single_window().command_bar().menu_bar().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
