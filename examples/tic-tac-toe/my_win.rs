use appcui::prelude::*;

#[Window()]
pub struct MyWin {
    rb_computer: Handle<RadioBox>,
    rb_human: Handle<RadioBox>,
    rb_easy: Handle<RadioBox>,
    rb_normal: Handle<RadioBox>,
    rb_hard: Handle<RadioBox>,
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
        };
        let mut t = tab!("tabs:[MainPage,Game],d:c,type:HiddenTabs,flags:TransparentBackground");
        // first player
        let mut p1 = panel!("'First player',l:1,t:1,r:1,h:3");
        w.rb_computer = p1.add(radiobox!("&Computer,x:1,y:0,w:20,selected:true"));
        w.rb_human = p1.add(radiobox!("&Human,x:30,y:0,w:20"));
        t.add(0,p1);

        // dificulty level
        let mut p2 = panel!("'Dificulty',l:1,t:5,r:1,h:3");
        w.rb_easy = p2.add(radiobox!("&Easy,x:1,y:0,w:10,selected:true"));
        w.rb_normal = p2.add(radiobox!("&Normal,x:15,y:0,w:10"));
        w.rb_hard = p2.add(radiobox!("H&ard,x:30,y:0,w:10"));
        t.add(0,p2);

        w.add(button!("'&Start Game',x:50%,y:100%,a:b,w:21"));

        w.add(t);
        w
    }
}
