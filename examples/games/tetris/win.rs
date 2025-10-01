use appcui::prelude::*;
use super::tetris_game::TetrisGame;

#[Window()]
pub struct Win {
    game: Handle<TetrisGame>,
}

impl Win {
    pub fn new() -> Self {
        let mut win = Win {
            base: window!("'Tetris Game',a:c,w:75,h:32"),
            game: Handle::None,
        };

        win.game = win.add(TetrisGame::new());
        win
    }
}
