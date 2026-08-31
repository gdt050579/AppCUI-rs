use appcui::prelude::*;
mod mywin;
use mywin::MyWin;
mod flappy_game;

fn main() -> Result<(), appcui::system::Error> {
    App::single_window(|| MyWin::new()).size(Size::new(70, 25)).run()
}
