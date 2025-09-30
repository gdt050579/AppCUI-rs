use appcui::prelude::*;
use super::pacman_game::PacmanGame;

#[Window()]
pub struct Win {
    game: Handle<PacmanGame>,
}

impl Win {
    pub fn new() -> Self {
        let mut win = Win {
            base: window!("'Pacman Game',a:c,w:57,h:26"),
            game: Handle::None,
        };

        win.game = win.add(PacmanGame::new());
        win
    }
}