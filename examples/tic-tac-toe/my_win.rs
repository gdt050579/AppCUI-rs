use super::Board;
use appcui::prelude::*;

#[Window(events: ButtonEvents, custom_events: BoardEvents)]
pub struct MyWin {
    p1_computer: Handle<RadioBox>,
    p1_human: Handle<RadioBox>,
    p1_name: Handle<TextField>,
    p2_computer: Handle<RadioBox>,
    p2_human: Handle<RadioBox>,
    p2_name: Handle<TextField>,
    board: Handle<Board>,
    tab: Handle<Tab>,
}
impl MyWin {
    pub fn new() -> Self {
        let mut w = Self {
            base: Window::new("Tic Tac Toe", Layout::new("d:c"), window::Flags::None),
            p1_computer: Handle::None,
            p1_human: Handle::None,
            p1_name: Handle::None,
            p2_computer: Handle::None,
            p2_human: Handle::None,
            p2_name: Handle::None,
            board: Handle::None,
            tab: Handle::None,
        };
        let mut t = tab!("tabs:[MainPage,Game],d:c,type:HiddenTabs,flags:TransparentBackground");
        // first player
        let mut pnl = panel!("'Player One (X)',l:1,t:1,r:1,h:7");
        pnl.add(label!("Name,x:1,y:1,w:4"));
        w.p1_name = pnl.add(textfield!("'Player 1',l:6,t:1,r:1,h:1"));
        pnl.add(label!("Type,x:1,y:3,w:4"));
        w.p1_computer = pnl.add(radiobox!("Computer,x:6,y:3,w:12,selected:false"));
        w.p1_human = pnl.add(radiobox!("Human,x:20,y:3,w:12,selected:true"));
        t.add(0, pnl);

        // second player
        let mut pnl = panel!("'Player Two (O)',l:1,t:9,r:1,h:7");
        pnl.add(label!("Name,x:1,y:1,w:4"));
        w.p2_name = pnl.add(textfield!("'Player 2',l:6,t:1,r:1,h:1"));
        pnl.add(label!("Type,x:1,y:3,w:4"));
        w.p2_computer = pnl.add(radiobox!("Computer,x:6,y:3,w:12,selected:false"));
        w.p2_human = pnl.add(radiobox!("Human,x:20,y:3,w:12,selected:true"));
        t.add(0, pnl);

        t.add(0, button!("'&Start Game',x:50%,y:100%,a:b,w:21"));

        w.board = t.add(1, Board::new());
        w.tab = w.add(t);
        w
    }
}
impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        // there is only one button ( the start game button )
        let h = self.tab;
        if let Some(tab) = self.control_mut(h) {
            // switch to game time
            tab.set_current_tab(1);
            let b = self.board;
            if let Some(board) = self.control_mut(b) {
                board.reset_game();
            }
        }
        EventProcessStatus::Processed
    }
}
impl BoardEvents for MyWin {
    fn on_event(&mut self,handle:Handle<Board>,event:board::Events) -> EventProcessStatus {
        todo!()
    }
}