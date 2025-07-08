use super::flappy_game::*;
use appcui::prelude::*;

#[Window(custom_events = FlappyGameEvents)]
pub struct MyWin {
    game: Handle<FlappyGame>,
    score: Handle<toolbar::Label>,
}

impl MyWin {
    pub fn new() -> Self {
        let mut win = MyWin {
            base: window!("'Flappy Birds',d:c,w:60,h:20"),
            game: Handle::None,
            score: Handle::None,
        };
        let g = win.toolbar().create_group(toolbar::GroupPosition::BottomLeft);
        win.score = win.toolbar().add(g, toolbar::Label::new(""));
        win.game = win.add(FlappyGame::new());
        win
    }
}

impl FlappyGameEvents for MyWin {
    fn on_event(&mut self, handle: Handle<FlappyGame>, _: flappygame::Events) -> EventProcessStatus {
        let sc = if let Some(game) = self.control(handle) {
            game.score()
        } else {
            0
        };
        let h = self.score;
        if let Some(score) = self.toolbar().get_mut(h) {
            score.set_content(format!("Score: {sc}").as_str());
        }
        EventProcessStatus::Processed
    }
} 