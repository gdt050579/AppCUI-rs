use appcui::prelude::*;

mod my_win;
mod board;
use my_win::MyWin;
pub use board::Board;



fn main() -> Result<(), appcui::system::Error> {
    App::single_window(|| MyWin::new()).size(Size::new(40, 24)).run()
}   