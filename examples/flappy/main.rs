use appcui::prelude::*;
mod mywin;
use mywin::MyWin;
mod flappy_game;

fn main() -> Result<(), appcui::system::Error> {
    App::new().size(Size::new(70, 25)).single_window().window(|| MyWin::new()).run()
} 