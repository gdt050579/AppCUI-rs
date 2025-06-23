
use super::ramit_game::*;
use appcui::prelude::*;

#[Window(custom_events = RamItGameEvents)]
pub struct MyWin {
    game: Handle<RamItGame>,
    score: Handle<toolbar::Label>,
}

impl MyWin {
    pub fn new() -> Self {
        let mut win = MyWin {
            base: window!("'Ram It Game',d:c,w:50,h:20"),
            game: Handle::None,
            score: Handle::None,
        };
        let g = win.toolbar().create_group(toolbar::GroupPosition::BottomLeft);
        win.score = win.toolbar().add(g, toolbar::Label::new(""));
        win.game = win.add(RamItGame::new());
        win
    }
}

impl RamItGameEvents for MyWin {
    fn on_event(&mut self, handle: Handle<RamItGame>, _: ramitgame::Events) -> EventProcessStatus {
        let sc = if let Some(game) = self.control(handle) {
            game.score()
        } else {
            0
        };
        let h = self.score;
        if let Some(score) = self.toolbar().get_mut(h) {
            score.set_content(format!("Score: {}", sc).as_str());
        }
        EventProcessStatus::Processed
    }
}
