use appcui::prelude::*;
use super::game_2048_logic::Game2048Logic;

#[Window()]
pub struct Win {
    game: Handle<Game2048Logic>,
}

impl Win {
    pub fn new() -> Self {
        let mut win = Win {
            base: window!("'2048 Game',a:c,w:67,h:29"),
            game: Handle::None,
        };

        win.game = win.add(Game2048Logic::new());
        win
    }
}

