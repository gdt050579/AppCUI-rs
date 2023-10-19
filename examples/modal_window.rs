use appcui::prelude::*;

#[ModalWindow(events = ButtonEvents)]
struct MyWin {
    b1: Handle<Button>,
}

impl MyWin {
    fn new(title: &str) -> Self {
        let mut win = MyWin {
            base: ModalWindow::new(title, Layout::new("d:c,w:40,h:7"), window::Flags::None),
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

#[Desktop(events=CommandBarEvents)]
struct MyDesktop {}
impl CommandBarEvents for MyDesktop {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("F1"), "Create a modal window", 1);
    }

    fn on_event(&mut self, command_id: u32) {
        if command_id==1 {

        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.run();
    Ok(())
}
