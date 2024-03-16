use appcui::prelude::*;

#[ModalWindow(events = ButtonEvents+WindowEvents,response=i32)]
struct MyWin {
    b1: Handle<Button>,
    b2: Handle<Button>,
    b3: Handle<Button>,
    lb: Handle<Label>,
    counter: i32,
}

impl MyWin {
    fn new(title: &str, counter: i32) -> Self {
        let mut win = MyWin {
            base: ModalWindow::new(title, Layout::new("d:c,w:40,h:9"), window::Flags::None),
            b1: Handle::None,
            b2: Handle::None,
            b3: Handle::None,
            lb: Handle::None,
            counter,
        };
        win.b1 = win.add(button!("'Show modal &window',x:50%,y:2,a:c,w:30"));
        win.b2 = win.add(Button::new(
            format!("Counter = {}", counter).as_str(),
            Layout::new("x:50%,y:4,a:c,w:30"),
            button::Type::Normal,
        ));
        win.b3 = win.add(button!("E&xit,x:50%,y:6,a:c,w:30"));
        win.lb = win.add(Label::new("", Layout::new("x:0,y:0,w:100%")));
        win
    }
    fn update_counter(&mut self) {
        let handle = self.b2;
        let counter = self.counter;
        if let Some(b2) = self.control_mut(handle) {
            b2.set_caption(format!("Counter = {}", counter).as_str());
        }
    }
}

impl WindowEvents for MyWin {
    fn on_accept(&mut self) {
        self.exit_with(self.counter * 3);
    }
}
impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, button_handle: Handle<Button>) -> EventProcessStatus {
        if button_handle == self.b1 {
            let response = MyWin::new(format!("{}", self.counter + 1).as_str(), self.counter + 1).show();
            let handle = self.lb;
            if let (Some(r), Some(lb)) = (response, self.control_mut(handle)) {
                lb.set_caption(format!("Reponse from modal window: {}", r).as_str());
            } else {
                if response.is_none() {
                    if let Some(lb) = self.control_mut(handle) {
                        lb.set_caption("Exit with None from modal window !");
                    }
                }
            }
            return EventProcessStatus::Processed;
        }
        if button_handle == self.b2 {
            self.counter += 1;
            self.update_counter();
            return EventProcessStatus::Processed;
        }
        if button_handle == self.b3 {
            self.exit_with(self.counter * 2);
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}

#[Desktop(events=CommandBarEvents, commands=ShowModal)]
struct MyDesktop {}
impl MyDesktop {
    fn new() -> Self {
        Self { base: Desktop::new() }
    }
}
impl CommandBarEvents for MyDesktop {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("F1"), "Create a modal window", mydesktop::Commands::ShowModal);
    }

    fn on_event(&mut self, command_id: mydesktop::Commands) {
        if command_id == mydesktop::Commands::ShowModal {
            let _response = MyWin::new("1", 1).show();
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let app = App::new().desktop(MyDesktop::new()).command_bar().build()?;
    app.run();
    Ok(())
}
