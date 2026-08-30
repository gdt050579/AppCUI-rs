use appcui::prelude::*;

mod my_win;
mod board;
use my_win::MyWin;
pub use board::Board;



fn main() -> Result<(), appcui::system::Error> {
    App::new().size(Size::new(40, 24)).single_window().window(|| MyWin::new()).run()
}   