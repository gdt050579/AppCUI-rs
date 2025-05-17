use appcui::prelude::*;
mod mywin;
use mywin::MyWin;
mod flappy_game;

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(70, 25)).single_window().build()?;
    a.add_window(MyWin::new());
    a.run(); 
    Ok(())
} 