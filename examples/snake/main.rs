use appcui::prelude::*;
mod mywin;
use mywin::MyWin;
mod snake_game;

fn main() -> Result<(), appcui::system::Error> {
    App::new().size(Size::new(40, 24)).single_window().window(|| MyWin::new()).run()
}