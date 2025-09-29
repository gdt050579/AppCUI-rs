use appcui::prelude::*;
use super::pacman_game::PacmanGame;

#[Window()]
pub struct PacmanWin {
    game: Handle<PacmanGame>,
}

impl PacmanWin {
    pub fn new() -> Self {
        let mut win = PacmanWin {
            base: window!("'Pacman Game',a:c,w:80,h:30"),
            game: Handle::None,
        };

        win.game = win.add(PacmanGame::new());
        win
    }
}