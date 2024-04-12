use appcui::prelude::*;

#[Window(events = ButtonEvents+KeySelectorEvents)]
struct MyWin {
    reset: Handle<Button>,
    ks: Handle<KeySelector>,
    lb: Handle<Label>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'Key Selector example',d:c,w:40,h:9"),
            reset: Handle::None,
            ks: Handle::None,
            lb: Handle::None,
        };
        win.reset = win.add(button!("&Reset,x:50%,y:6,a:c,w:15"));
        win.ks = win.add(keyselector!("x:1,y:3,w:36"));
        win.lb = win.add(label!("<none>,x:1,y:1,w:35"));
        win
    }
    fn update_info(&mut self) {
        let key = self.control(self.ks).and_then(|obj| Some(obj.key())).unwrap_or(Key::None);
        let s = if key == Key::None {
            "<none>".to_string()
        } else {
            format!("New key is: {}{}", key.modifier.name(), key.code.name())
        };
        let h = self.lb;
        if let Some(label) = self.control_mut(h) {
            label.set_caption(&s);
        }
    }
}

impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
        // reset button was pressed
        let h = self.ks;
        if let Some(k) = self.control_mut(h) {
            k.set_key(Key::None);
        }
        self.update_info();
        EventProcessStatus::Processed
    }
}
impl KeySelectorEvents for MyWin {
    fn on_key_changed(&mut self, _handle: Handle<KeySelector>, _new_key: Key, _old_key: Key) -> EventProcessStatus {
        self.update_info();
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWin::new());
    app.run();
    Ok(())
}
