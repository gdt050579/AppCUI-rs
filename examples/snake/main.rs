use appcui::prelude::*;
mod mywin;
use mywin::MyWin;
mod snake_game;

fn main() -> Result<(), appcui::system::Error> {
    App::new().window(MyWin::new).size(Size::new(40, 24)).run()
}