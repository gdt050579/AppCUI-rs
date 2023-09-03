use appcui::prelude::*;

#[Window(events = ToolBarEvents)]
struct MyWin {
    lb1: Handle<toolbar::Label>,
    cb1: Handle<toolbar::CheckBox>,
    cb2: Handle<toolbar::CheckBox>,
    opt1: Handle<toolbar::SingleChoice>,
    opt2: Handle<toolbar::SingleChoice>,
    opt3: Handle<toolbar::SingleChoice>,
}
impl MyWin {
    fn new() -> Self {
        let mut me = Self {
            base: Window::new("Toolbars", Layout::new("d:c,w:100%,h:100%"), window::Flags::None),
            lb1: Handle::None,
            cb1: Handle::None,
            cb2: Handle::None,
            opt1: Handle::None,
            opt2: Handle::None,
            opt3: Handle::None,
        };
        me.lb1 = me.get_toolbar().add(toolbar::Label::new(toolbar::Gravity::TopLeft,"<None>"));
        me.cb1 = me.get_toolbar().add(toolbar::CheckBox::new(toolbar::Gravity::BottomLeft, "CheckBox &1", false));
        me.cb2 = me.get_toolbar().add(toolbar::CheckBox::new(toolbar::Gravity::BottomLeft, "CheckBox &2", false));
        me.opt1 = me.get_toolbar().add(toolbar::SingleChoice::new(toolbar::Gravity::BottomLeft, "Opy &A", 1));
        me.opt2 = me.get_toolbar().add(toolbar::SingleChoice::new(toolbar::Gravity::BottomLeft, "Opy &B", 1));
        me.opt3 = me.get_toolbar().add(toolbar::SingleChoice::new(toolbar::Gravity::BottomLeft, "Opy &C", 1));
        
        me
    }
    fn set_info(&mut self, text: &str) {
        let h = self.lb1;
        if let Some(status) = self.get_toolbar().get_mut(h) {
            status.set_text(text);
        }
    }
}
impl ToolBarEvents for MyWin {
    fn on_button_clicked(&mut self, _handle: Handle<toolbar::Button>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

    fn on_checkbox_clicked(&mut self, handle: Handle<toolbar::CheckBox>, checked: bool) -> EventProcessStatus {
        if handle == self.cb1 {
            self.set_info(format!{"CheckBox 1 is {checked}"}.as_str());
        }
        if handle == self.cb2 {
            self.set_info(format!{"CheckBox 2 is {checked}"}.as_str());
        }
        EventProcessStatus::Processed
    }

    fn on_choice_selected(&mut self, handle: Handle<toolbar::SingleChoice>) -> EventProcessStatus {
        if handle == self.opt1 {
            self.set_info("Option 1 is selected");
        }
        if handle == self.opt2 {
            self.set_info("Option 2 is selected");
        }
        if handle == self.opt3 {
            self.set_info("Option 3 is selected");
        }
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWin::new());
    app.run();
    Ok(())
}
