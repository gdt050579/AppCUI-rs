use appcui::prelude::*;
mod mywin;
use mywin::MyWin;
mod ramit_game;

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(60, 18)).single_window().build()?;
    a.add_window(MyWin::new());
    a.run(); 
    Ok(())
} 