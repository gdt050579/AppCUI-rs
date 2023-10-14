use appcui::prelude::*;

#[Window(events = ButtonEvents)]
struct MyWin {
    b1: Handle<Button>,
}

impl MyWin {
    fn new(title: &str) -> Self {
        let mut win = MyWin {
            base: Window::new(title, Layout::new("d:c,w:40,h:7"), window::Flags::None),
            b1: Handle::None,
        };
        win.b1 = win.add(button!("'Show modal &window',x:50%,y:2,a:c,w:30"));
        win
    }
}

impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, button_handle: Handle<Button>) -> EventProcessStatus {
        if button_handle == self.b1 {
            // run a modal window
            //let win = MyWin::new("Modal Window");
            //win.show()
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWin::new("Regular Window"));
    app.run();
    Ok(())
}
