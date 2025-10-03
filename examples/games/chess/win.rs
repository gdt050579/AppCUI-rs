use appcui::prelude::*;
use super::chess_logic::ChessLogic;

#[Window()]
pub struct Win {
    game: Handle<ChessLogic>,
}

impl Win {
    pub fn new() -> Self {
        let mut win = Win {
            base: window!("'Chess Game',a:c,w:80,h:40"),
            game: Handle::None,
        };

        win.game = win.add(ChessLogic::new());
        win
    }
}
