use super::Board;
use appcui::prelude::*;

#[Window(events: ButtonEvents)]
pub struct MyWin {
    rb_computer: Handle<RadioBox>,
    rb_human: Handle<RadioBox>,
    rb_easy: Handle<RadioBox>,
    rb_normal: Handle<RadioBox>,
    rb_hard: Handle<RadioBox>,
    board: Handle<Board>,
    tab: Handle<Tab>,
}
impl MyWin {
    pub fn new() -> Self {
        let mut w = Self {
            base: Window::new("Tic Tac Toe", Layout::new("d:c"), window::Flags::None),
            rb_computer: Handle::None,
            rb_human: Handle::None,
            rb_easy: Handle::None,
            rb_normal: Handle::None,
            rb_hard: Handle::None,
            board: Handle::None,
            tab: Handle::None,
        };
        let mut t = tab!("tabs:[MainPage,Game],d:c,type:HiddenTabs,flags:TransparentBackground");
        // first player
        let mut p1 = panel!("'First player',l:1,t:1,r:1,h:3");
        w.rb_computer = p1.add(radiobox!("&Computer,x:1,y:0,w:20,selected:true"));
        w.rb_human = p1.add(radiobox!("&Human,x:30,y:0,w:20"));
        t.add(0, p1);

        // dificulty level
        let mut p2 = panel!("'Dificulty',l:1,t:5,r:1,h:3");
        w.rb_easy = p2.add(radiobox!("&Easy,x:1,y:0,w:10,selected:true"));
        w.rb_normal = p2.add(radiobox!("&Normal,x:15,y:0,w:10"));
        w.rb_hard = p2.add(radiobox!("H&ard,x:30,y:0,w:10"));
        t.add(0, p2);

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
