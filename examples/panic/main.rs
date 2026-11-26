use appcui::prelude::*;

#[Window(events = ButtonEvents)]
struct PanicWindow {
    panic_button: Handle<Button>,
}

impl PanicWindow {
    fn new() -> Self {
        let mut win = Self {
            base: window!("'Panic',a:c,w:40,h:7"),
            panic_button: Handle::None,
        };
        win.panic_button = win.add(button!("'&Panic',a:c,w:16"));
        win
    }
}

impl ButtonEvents for PanicWindow {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.panic_button {
            panic!("This is an example to show how terminal is being restore in case of a panic");
        }
        EventProcessStatus::Ignored
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().window(PanicWindow::new).run()
}
