use appcui::prelude::*;
use super::connect_four_game::ConnectFourGame;

#[Window()]
pub struct Win {
    game: Handle<ConnectFourGame>,
}

impl Win {
    pub fn new() -> Self {
        let mut win = Win {
            base: window!("'Connect Four Game',a:c,w:60,h:30"),
            game: Handle::None,
        };

        win.game = win.add(ConnectFourGame::new());
        win
    }
}
