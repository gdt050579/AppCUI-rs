use appcui::prelude::*;
use super::chess_logic::ChessLogic;

#[Window()]
pub struct Win {
    game: Handle<ChessLogic>,
}

impl Win {
    pub fn new() -> Self {
        let mut win = Win {
            base: window!("'Chess Game',a:c,w:30,h:15"),
            game: Handle::None,
        };

        win.game = win.add(ChessLogic::new());
        win
    }
}
