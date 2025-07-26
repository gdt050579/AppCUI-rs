use appcui::prelude::*;

#[Window(events = RadioBoxEvents)]
struct MyWin {
    g1_r1: Handle<RadioBox>,
    g1_r2: Handle<RadioBox>,
    g1_r3: Handle<RadioBox>,
    g2_r1: Handle<RadioBox>,
    g2_r2: Handle<RadioBox>,
    g2_r3: Handle<RadioBox>,
    l: Handle<Label>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'My Win',a:c,w:60,h:14"),
            g1_r1: Handle::None,
            g1_r2: Handle::None,
            g1_r3: Handle::None,
            g2_r1: Handle::None,
            g2_r2: Handle::None,
            g2_r3: Handle::None,
            l: Handle::None,
        };
        win.l = win.add(label!("'<no status>',l:1,r:1,t:1"));
        let mut group_1 = panel!("'Group 1',x:1,y:3,w:26,h:7");
        win.g1_r1 = group_1.add(radiobox!("&Meters,x:1,y:1,w:20,select:true"));
        win.g1_r2 = group_1.add(radiobox!("&Centimeters,x:1,y:2,w:20"));
        win.g1_r3 = group_1.add(radiobox!("&Kilometers,x:1,y:3,w:20"));
        
        let mut group_2 = panel!("'Group 2',x:30,y:3,w:26,h:7");
        win.g2_r1 = group_2.add(radiobox!("&Red,x:1,y:1,w:20,select:true"));
        win.g2_r2 = group_2.add(radiobox!("&Green,x:1,y:2,w:20"));
        win.g2_r3 = group_2.add(radiobox!("&Blue,x:1,y:3,w:20"));

        win.add(group_1);
        win.add(group_2);
        win
    }
}

impl RadioBoxEvents for MyWin {
    fn on_selected(&mut self, handle: Handle<RadioBox>) -> EventProcessStatus {
        let mut s = String::new();
        if let Some(r) = self.control(handle) {
            s += r.caption();
        }
        if !s.is_empty() {
            let h = self.l;
            if let Some(l) = self.control_mut(h) {
                l.set_caption(&s);
            }
        }
        EventProcessStatus::Ignored
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
