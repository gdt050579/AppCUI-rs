use super::minesweeper_game::*;
use appcui::prelude::*;

#[Window(custom_events = MinesweeperGameEvents)]
pub struct MyWin {}

impl MyWin {
    pub fn new(title: &str, layout: Layout, grid_size: Size, num_mines: usize) -> Self {
        let mut win = MyWin {
            base: Window::new(title, layout, window::Flags::None),
        };
        win.add(MinesweeperGame::new(grid_size.width as usize, grid_size.height as usize, num_mines));
        win
    }
}

impl MinesweeperGameEvents for MyWin {
    fn on_event(&mut self, _: Handle<MinesweeperGame>, _: minesweepergame::Events) -> EventProcessStatus {
        self.close();
        EventProcessStatus::Processed
    }
}
