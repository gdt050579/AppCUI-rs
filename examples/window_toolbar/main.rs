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
            base: Window::new("Toolbars", layout!("d:f"), window::Flags::None),
            lb1: Handle::None,
            cb1: Handle::None,
            cb2: Handle::None,
            opt1: Handle::None,
            opt2: Handle::None,
            opt3: Handle::None,
        };
        let labels_group = me.toolbar().create_group(toolbar::GroupPosition::TopLeft);
        me.lb1 = me.toolbar().add(labels_group, toolbar::Label::new("<None>"));
        let checkboxes_group = me.toolbar().create_group(toolbar::GroupPosition::BottomLeft);
        me.cb1 = me.toolbar().add(checkboxes_group, toolbar::CheckBox::new("CheckBox &1", false));
        me.cb2 = me.toolbar().add(checkboxes_group, toolbar::CheckBox::new("CheckBox &2", false));
        let buttons_group = me.toolbar().create_group(toolbar::GroupPosition::BottomLeft);
        me.opt1 = me.toolbar().add(buttons_group, toolbar::SingleChoice::new("Opt &A"));
        me.opt2 = me.toolbar().add(buttons_group, toolbar::SingleChoice::new("Opt &B"));
        me.opt3 = me.toolbar().add(buttons_group, toolbar::SingleChoice::new("Opt &C"));

        me
    }
    fn set_info(&mut self, text: &str) {
        let h = self.lb1;
        if let Some(status) = self.toolbar().get_mut(h) {
            status.set_content(text);
        }
    }
}
impl ToolBarEvents for MyWin {
    fn on_button_clicked(&mut self, _handle: Handle<toolbar::Button>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

    fn on_checkbox_clicked(&mut self, handle: Handle<toolbar::CheckBox>, checked: bool) -> EventProcessStatus {
        if handle == self.cb1 {
            self.set_info(format! {"CheckBox 1 is {checked}"}.as_str());
        }
        if handle == self.cb2 {
            self.set_info(format! {"CheckBox 2 is {checked}"}.as_str());
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
