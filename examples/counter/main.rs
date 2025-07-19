use appcui::prelude::*;

#[Window(events = ButtonEvents)]
struct CounterWindow {
    counter: i32
}

impl CounterWindow {
    fn new() -> Self {
        let mut w = Self {
            base: window!("'Counter window',a:c,w:30,h:5"),
            counter: 1            
        };
        w.add(button!("'1',d:b,w:20"));
        w
    }
}
impl ButtonEvents for CounterWindow {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        self.counter += 1;
        let text = format!("{}",self.counter);
        if let Some(button) = self.control_mut(handle) {
            button.set_caption(&text);
        }
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(CounterWindow::new());
    a.run();
    Ok(())
}