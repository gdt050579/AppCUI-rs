use appcui::prelude::*;

#[Window(events = CheckBoxEvents)]
struct MyWin {
    c1: Handle<CheckBox>,
    c2: Handle<CheckBox>,
    c3: Handle<CheckBox>,
    lb: Handle<Label>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("Checkboxes,a:c,w:40,h:10"),
            c1: Handle::None,
            c2: Handle::None,
            c3: Handle::None,
            lb: Handle::None,
        };
        win.c1 = win.add(checkbox!("'A &Single line checkbox',x:2,y:2,w:40"));
        win.c2 = win.add(checkbox!("Caption='&Inactive checkbox',x:2,y:3,w:15,enabled:false"));
        win.c3 = win.add(checkbox!("Text='A &multi line checkbox that is enabled',x:2,y:4,w:20,h:3,checked:true"));
        win.lb = win.add(Label::new("", Layout::new("x:2,y:0,w:35")));
        win
    }
    fn set_label_text(&mut self, txt: &str) {
        let h = self.lb;
        if let Some(label) = self.control_mut(h) {
            label.set_caption(txt);
        }
    }
}

impl CheckBoxEvents for MyWin {
    fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
        let id = match () {
            _ if handle == self.c1 => 1,
            _ if handle == self.c2 => 2,
            _ if handle == self.c3 => 3,
            _ => 0,
        };
        self.set_label_text(format!("Checkbox {id} check status: {checked}").as_str());
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWin::new());
    app.run();
    Ok(())
}
