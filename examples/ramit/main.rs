use appcui::prelude::*;
mod mywin;
use mywin::MyWin;
mod ramit_game;

fn main() -> Result<(), appcui::system::Error> {
    App::single_window(|| MyWin::new()).size(Size::new(60, 18)).run()
} 