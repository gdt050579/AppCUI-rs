use appcui::prelude::*;

mod my_win;
use my_win::MyWin;



fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(60, 24)).single_window().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}   