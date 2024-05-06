use appcui::prelude::*;

mod my_win;
mod board;
use my_win::MyWin;
pub use board::Board;



fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(40, 24)).single_window().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}   