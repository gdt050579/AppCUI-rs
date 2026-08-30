use appcui::prelude::*;
mod mywin;
use mywin::MyWin;
mod ramit_game;

fn main() -> Result<(), appcui::system::Error> {
    App::new().size(Size::new(60, 18)).single_window().window(|| MyWin::new()).run()
} 