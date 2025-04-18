use appcui::prelude::*;
use super::snake_game::SnakeGame;


#[Window()]
pub struct MyWin {
    game: Handle<SnakeGame>,
}

impl MyWin {
    pub fn new() -> Self {
        let mut win = MyWin {
            base: window!("'Snake Game',d:c,w:50,h:20"),
            game: Handle::None,
        };

        win.game = win.add(SnakeGame::new());
        win
    }
}
