use appcui::prelude::*;
use super::tetris_game::TetrisGame;

#[Window()]
pub struct Win {
    game: Handle<TetrisGame>,
}

impl Win {
    pub fn new() -> Self {
        let mut win = Win {
            base: window!("'Tetris Game',a:c,w:80,h:35"),
            game: Handle::None,
        };

        win.game = win.add(TetrisGame::new());
        win
    }
}
